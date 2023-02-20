use yaml_peg::parser::{Loader, PError};
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

use super::ast::{PossumNode, PossumNodeKind};
use crate::document::{Annotation, AsDocumentPointer, DocumentPointer};
use crate::scavenge::ast::{PossumMap, PossumSeq};
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

pub struct BoolParser;

impl<'a, R> Parser<'a, R, bool> for BoolParser
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<bool>
    where
        R: Repr,
    {
        root.extract_bool().into()
    }
}

pub struct MapParser<'a, R, T>
where
    R: Repr + 'a,
{
    _x: PhantomData<R>,
    parser: &'a mut dyn Parser<'a, R, T>,
}

impl<'a, R, T> MapParser<'a, R, T>
where
    R: Repr + 'a,
{
    pub fn new(parser: &'a mut dyn Parser<'a, R, T>) -> MapParser<'a, R, T> {
        MapParser {
            _x: PhantomData,
            parser,
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

                for (key, value) in m.iter() {
                    let k: PossumNodeKind<String> = key.extract_str().map(ToOwned::to_owned).into();
                    let v: PossumNodeKind<T> = self.parser.parse_node(root);

                    map.insert(k.at(key), v.at(value));
                }

                Value(map)
            }
        }
    }
}

pub struct SeqParser<'a, R, T>(&'a mut dyn Parser<'a, R, T>)
where
    R: Repr + 'a;

impl<'a, R, T> SeqParser<'a, R, T>
where
    R: Repr + 'a,
{
    pub fn new(parser: &'a mut dyn Parser<'a, R, T>) -> SeqParser<'a, R, T> {
        SeqParser(parser)
    }
}

impl<'a, R, T> Parser<'a, R, PossumSeq<T>> for SeqParser<'a, R, T>
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<PossumSeq<T>>
    where
        R: Repr,
    {
        use PossumNodeKind::{Invalid, Value};
        match root.extract_seq() {
            Err(u) => Invalid(u.to_string()),
            Ok(seq) => Value(
                seq.iter()
                    .map(|elm| self.0.parse_node(elm).at(elm))
                    .collect(),
            ),
        }
    }
}

pub struct TransformParser<'a, R, T, U>
where
    R: Repr + 'a,
{
    parser: &'a mut dyn Parser<'a, R, T>,
    transform: &'a dyn Fn(T) -> U,
}

impl<'a, R, T, U> TransformParser<'a, R, T, U>
where
    R: Repr + 'a,
{
    pub fn new(
        parser: &'a mut dyn Parser<'a, R, T>,
        transform: &'a dyn Fn(T) -> U,
    ) -> TransformParser<'a, R, T, U> {
        TransformParser { parser, transform }
    }
}

impl<'a, R, T, U> Parser<'a, R, U> for TransformParser<'a, R, T, U>
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<U>
    where
        R: Repr,
    {
        self.parser.parse_node(root).map(self.transform)
    }
}

pub struct FlatMapParser<'a, R, T, U>
where
    R: Repr + 'a,
{
    parser: &'a mut dyn Parser<'a, R, T>,
    transform: &'a dyn Fn(T) -> PossumNodeKind<U>,
}

impl<'a, R, T, U> FlatMapParser<'a, R, T, U>
where
    R: Repr + 'a,
{
    pub fn new(
        parser: &'a mut dyn Parser<'a, R, T>,
        transform: &'a dyn Fn(T) -> PossumNodeKind<U>,
    ) -> FlatMapParser<'a, R, T, U> {
        FlatMapParser { parser, transform }
    }
}

impl<'a, R, T, U> Parser<'a, R, U> for FlatMapParser<'a, R, T, U>
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<U>
    where
        R: Repr,
    {
        self.parser.parse_node(root).flatmap(self.transform)
    }
}

pub struct OrParser<'a, R, T>
where
    R: Repr + 'a,
{
    lhs: &'a mut dyn Parser<'a, R, T>,
    rhs: &'a mut dyn Parser<'a, R, T>,
    default: &'a dyn Fn(&YamlNode<R>) -> PossumNodeKind<T>,
}

impl<'a, R, T> OrParser<'a, R, T>
where
    R: Repr + 'a,
{
    pub fn new(
        lhs: &'a mut dyn Parser<'a, R, T>,
        rhs: &'a mut dyn Parser<'a, R, T>,
        default: &'a dyn Fn(&YamlNode<R>) -> PossumNodeKind<T>,
    ) -> OrParser<'a, R, T> {
        OrParser { lhs, rhs, default }
    }
}

impl<'a, R, T> Parser<'a, R, T> for OrParser<'a, R, T>
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<T>
    where
        R: Repr,
    {
        self.lhs
            .parse_node(root)
            .recover(|| self.rhs.parse_node(root))
            .recover(|| (self.default)(root))
    }
}
