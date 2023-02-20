use yaml_peg::parser::{Loader, PError};
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

use super::ast::{PossumNode, PossumNodeKind};
use crate::document::{Annotation, Annotations, AsDocumentPointer, DocumentPointer};
use crate::scavenge::ast::{PossumMap, PossumSeq};
use crate::scavenge::extraction::Extract;
use std::fmt::Display;

#[derive(Debug)]
pub enum ParseFailure {
    InvalidDocument(PError),
    Empty,
    TooManyDocuments(Vec<DocumentPointer>),
    CouldntOpen,
}

pub struct UnexpectedKey<'a>(&'a str);
pub struct StringParser;
pub struct BoolParser;
pub struct NumberParser;
pub type InnerParser<'a, R, T> = &'a mut dyn Parser<R, T>;
pub struct StringMapParser(StringParser, StringParser);
pub struct MapParser<'a, R, K, V>
where
    R: Repr,
{
    keys: InnerParser<'a, R, K>,
    values: InnerParser<'a, R, V>,
}

pub struct SeqParser<'a, R, T>(&'a mut dyn Parser<R, T>)
where
    R: Repr;
pub struct TransformParser<'a, R, T, U>
where
    R: Repr,
{
    parser: InnerParser<'a, R, T>,
    transform: &'a dyn Fn(T) -> U,
}
pub struct FlatMapParser<'a, R, T, U>
where
    R: Repr,
{
    parser: InnerParser<'a, R, T>,
    transform: &'a dyn Fn(T) -> PossumNodeKind<U>,
}
pub struct Pluralize<'a, R, T>(InnerParser<'a, R, T>);

impl<'a, R, T> Pluralize<'a, R, T> {
    pub fn new(inner: InnerParser<'a, R, T>) -> Pluralize<'a, R, T> {
        Pluralize(inner)
    }
}
pub struct ObjectParser<'a, B, F, T>
where
    B: Builder<T>,
    F: Fn() -> T,
{
    builder: B,
    default: F,
    annotations: &'a mut Annotations,
}
pub struct OrParser<'a, R, T>
where
    R: Repr,
{
    lhs: InnerParser<'a, R, T>,
    rhs: InnerParser<'a, R, T>,
    default: &'a dyn Fn(&YamlNode<R>) -> PossumNodeKind<T>,
}

pub trait Builder<T> {
    fn build<'a, P, R>(
        &mut self,
        item: &mut T,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer,
        R: Repr;
}

impl<'a, R, T> Parser<R, PossumSeq<T>> for Pluralize<'a, R, T>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<PossumSeq<T>>
    where
        R: Repr,
    {
        PossumNodeKind::Value(self.0.parse_node(root).at(root).into())
    }
}

impl<'a> UnexpectedKey<'a> {
    pub fn at<P>(self, loc: &P) -> Annotation
    where
        P: AsDocumentPointer,
    {
        Annotation::error(loc, &self)
    }
}

impl<'a> Display for UnexpectedKey<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected key: {}", self.0)
    }
}

impl<'a> From<&'a str> for UnexpectedKey<'a> {
    fn from(value: &'a str) -> Self {
        UnexpectedKey(value)
    }
}

pub fn parse_single_document<'a, R, T, P>(
    mut loader: Loader<'a, R>,
    parser: &mut P,
) -> Result<PossumNode<T>, ParseFailure>
where
    R: Repr,
    P: Parser<R, T>,
{
    let documents = loader
        .parse()
        .map_err(|e| ParseFailure::InvalidDocument(e))?;

    if documents.is_empty() {
        Err(ParseFailure::Empty)?
    }

    if documents.len() > 1 {
        Err(ParseFailure::TooManyDocuments(
            documents
                .iter()
                .map(AsDocumentPointer::as_document_pointer)
                .collect(),
        ))?
    }

    let documents = &documents;
    let d = documents.get(0).unwrap();
    Ok(parser.parse_node(d).at(d))
}

pub trait Parser<R, T>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<T>
    where
        R: Repr;
}

impl<R> Parser<R, String> for StringParser
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<String>
    where
        R: Repr,
    {
        root.extract_str().map(ToOwned::to_owned).into()
    }
}

impl<R> Parser<R, bool> for BoolParser
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<bool>
    where
        R: Repr,
    {
        root.extract_bool().into()
    }
}

impl<R> Parser<R, f64> for NumberParser
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<f64>
    where
        R: Repr,
    {
        root.extract_number().into()
    }
}

impl<'a, R, K, V> MapParser<'a, R, K, V>
where
    R: Repr,
{
    pub fn new(
        keys: InnerParser<'a, R, K>,
        values: InnerParser<'a, R, V>,
    ) -> MapParser<'a, R, K, V> {
        MapParser { keys, values }
    }
}

impl<'a, R, K, V> Parser<R, PossumMap<K, V>> for MapParser<'a, R, K, V>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<PossumMap<K, V>>
    where
        R: Repr,
    {
        use PossumNodeKind::{Invalid, Value};
        match root.extract_map() {
            Err(u) => Invalid(u.to_string()),
            Ok(m) => {
                let mut map = PossumMap::empty();

                for (key, value) in m.iter() {
                    let k: PossumNodeKind<K> = self.keys.parse_node(key);
                    let v: PossumNodeKind<V> = self.values.parse_node(value);

                    map.insert(k.at(key), v.at(value));
                }

                Value(map)
            }
        }
    }
}

impl StringMapParser {
    pub fn new() -> StringMapParser {
        StringMapParser(StringParser, StringParser)
    }
}

impl<R> Parser<R, PossumMap<String, String>> for StringMapParser
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<PossumMap<String, String>>
    where
        R: Repr,
    {
        MapParser::new(&mut self.0, &mut self.1).parse_node(root)
    }
}

impl<'a, R, T> SeqParser<'a, R, T>
where
    R: Repr,
{
    pub fn new(parser: &mut dyn Parser<R, T>) -> SeqParser<R, T> {
        SeqParser(parser)
    }
}

impl<'a, R, T> Parser<R, PossumSeq<T>> for SeqParser<'a, R, T>
where
    R: Repr,
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

impl<'a, R, T, U> TransformParser<'a, R, T, U>
where
    R: Repr,
{
    pub fn new(
        parser: InnerParser<'a, R, T>,
        transform: &'a dyn Fn(T) -> U,
    ) -> TransformParser<'a, R, T, U> {
        TransformParser { parser, transform }
    }
}

impl<'a, R, T, U> Parser<R, U> for TransformParser<'a, R, T, U>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<U>
    where
        R: Repr,
    {
        self.parser.parse_node(root).map(self.transform)
    }
}

impl<'a, R, T, U> FlatMapParser<'a, R, T, U>
where
    R: Repr,
{
    pub fn new(
        parser: InnerParser<'a, R, T>,
        transform: &'a dyn Fn(T) -> PossumNodeKind<U>,
    ) -> FlatMapParser<'a, R, T, U> {
        FlatMapParser { parser, transform }
    }
}

impl<'a, R, T, U> Parser<R, U> for FlatMapParser<'a, R, T, U>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<U>
    where
        R: Repr,
    {
        self.parser.parse_node(root).flatmap(self.transform)
    }
}

impl<'a, R, T> OrParser<'a, R, T>
where
    R: Repr,
{
    pub fn new(
        lhs: InnerParser<'a, R, T>,
        rhs: InnerParser<'a, R, T>,
        default: &'a dyn Fn(&YamlNode<R>) -> PossumNodeKind<T>,
    ) -> OrParser<'a, R, T> {
        OrParser { lhs, rhs, default }
    }
}

impl<'a, R, T> Parser<R, T> for OrParser<'a, R, T>
where
    R: Repr,
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

impl<'a, B, F, T> ObjectParser<'a, B, F, T>
where
    B: Builder<T>,
    F: Fn() -> T,
{
    pub fn new(
        builder: B,
        default: F,
        annotations: &'a mut Annotations,
    ) -> ObjectParser<'a, B, F, T> {
        ObjectParser {
            builder,
            default,
            annotations,
        }
    }
}

impl<'a, B, F, R, T> Parser<R, T> for ObjectParser<'a, B, F, T>
where
    R: Repr,
    B: Builder<T>,
    F: Fn() -> T,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<T>
    where
        R: Repr,
    {
        use PossumNodeKind::*;
        match root.extract_map() {
            Err(u) => Invalid(u.to_string()),
            Ok(m) => {
                let mut item = (self.default)();

                for (key, value) in m.iter() {
                    match key.extract_str() {
                        Err(u) => self.annotations.add(u.at(key)),
                        Ok(s) => self
                            .builder
                            .build(&mut item, s, value, key, self.annotations),
                    }
                }

                Value(item)
            }
        }
    }
}
