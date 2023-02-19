use yaml_peg::parser::{Loader, PError};
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

use super::ast::{PossumNode, PossumNodeKind};
use crate::document::{Annotation, AsDocumentPointer, DocumentPointer};
use crate::scavenge::ast::PossumMap;
use crate::scavenge::extraction::Extract;
use std::fmt::Display;
use std::marker::PhantomData;

pub struct UnexpectedKey<'a, S, P>(&'a S, &'a P)
where
    P: AsDocumentPointer,
    S: Display;

impl<'a, S, P> UnexpectedKey<'a, S, P>
where
    P: AsDocumentPointer,
    S: Display,
{
    pub fn at(s: &'a S, p: &'a P) -> UnexpectedKey<'a, S, P> {
        UnexpectedKey(s, p)
    }
}

impl<'a, S, P> Into<Annotation> for UnexpectedKey<'a, S, P>
where
    P: AsDocumentPointer,
    S: Display,
{
    fn into(self) -> Annotation {
        Annotation::error(self.1, self.0)
    }
}

#[derive(Debug)]
pub enum ParseFailure {
    InvalidDocument(PError),
    Empty,
    TooManyDocuments(Vec<DocumentPointer>),
    CouldntOpen,
}

pub fn parse_single_document<'a, R, T, P>(
    mut loader: Loader<'a, R>,
    mut parser: P,
) -> Result<PossumNode<T>, ParseFailure>
where
    R: Repr + 'a,
    P: Parser<'a, R, T>,
{
    let documents = loader
        .parse()
        .map_err(|e| ParseFailure::InvalidDocument(e))?;

    if documents.len() == 0 {
        return Err(ParseFailure::Empty);
    } else if documents.len() > 1 {
        return Err(ParseFailure::TooManyDocuments(
            documents
                .iter()
                .map(|n| DocumentPointer(n.pos() as usize))
                .collect(),
        ));
    };

    let root = documents.get(0).unwrap();
    Ok(parser.parse_node(root).at(root))
}

pub trait Parser<'a, R, T>
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<T>
    where
        R: Repr;
}

pub struct StringParser;

impl<'a, R> Parser<'a, R, String> for StringParser
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<String>
    where
        R: Repr,
    {
        root.extract_str().map(ToOwned::to_owned).into()
    }
}

type ParserFactory<'a, R, T> = Box<dyn FnMut() -> Box<dyn Parser<'a, R, T>>>;

pub struct MapParser<'a, R, T>
where
    R: Repr + 'a,
{
    _x: PhantomData<R>,
    parser: ParserFactory<'a, R, T>,
}

impl<'a, R, T> MapParser<'a, R, T>
where
    R: Repr + 'a,
{
    pub fn new(factory: ParserFactory<'a, R, T>) -> MapParser<'a, R, T> {
        MapParser {
            _x: PhantomData,
            parser: factory,
        }
    }

    pub fn strings() -> MapParser<'a, R, String> {
        MapParser {
            _x: PhantomData,
            parser: Box::new(|| Box::new(StringParser)),
        }
    }
}

impl<'a, R, T> Parser<'a, R, PossumMap<String, T>> for MapParser<'a, R, T>
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<PossumMap<String, T>>
    where
        R: Repr,
    {
        use PossumNodeKind::{Invalid, Value};
        match root.extract_map() {
            Err(u) => Invalid(u.to_string()),
            Ok(m) => {
                let mut map = PossumMap::empty();
                let mut parser = (*self.parser)();

                for (key, value) in m.iter() {
                    let k: PossumNodeKind<String> = key.extract_str().map(ToOwned::to_owned).into();
                    let v: PossumNodeKind<T> = parser.parse_node(root);

                    map.insert(k.at(key), v.at(value));
                }

                Value(map)
            }
        }
    }
}
