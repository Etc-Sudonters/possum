use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::ast::PossumSeq;
use crate::scavenge::parser::Parser;
use std::marker::PhantomData;

pub struct TransformParser<R, T, U, P, F>
where
    R: Repr,
    P: Parser<R, T>,
    F: Fn(T) -> U,
{
    parser: P,
    transform: F,
    _r: PhantomData<R>,
    _t: PhantomData<T>,
    _u: PhantomData<U>,
}

impl<R, T, U, P, F> TransformParser<R, T, U, P, F>
where
    R: Repr,
    P: Parser<R, T>,
    F: Fn(T) -> U,
{
    pub fn new(parser: P, transform: F) -> TransformParser<R, T, U, P, F> {
        TransformParser {
            parser,
            transform,
            _r: PhantomData,
            _t: PhantomData,
            _u: PhantomData,
        }
    }
}

impl<R, T, U, P, F> Parser<R, U> for TransformParser<R, T, U, P, F>
where
    R: Repr,
    P: Parser<R, T>,
    F: Fn(T) -> U,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<U>
    where
        R: Repr,
    {
        self.parser.parse_node(root).map(&self.transform)
    }
}

pub trait TransformableParser<R, T, U, P, F>: Parser<R, T>
where
    R: Repr,
    P: Parser<R, T>,
    F: Fn(T) -> U,
{
    fn to(self, f: F) -> TransformParser<R, T, U, P, F>;
}

impl<R, T, U, P, F> TransformableParser<R, T, U, P, F> for P
where
    R: Repr,
    P: Parser<R, T>,
    F: Fn(T) -> U,
{
    fn to(self, f: F) -> TransformParser<R, T, U, P, F> {
        TransformParser::new(self, f)
    }
}

pub struct FlatMapParser<R, T, U, P, F>
where
    R: Repr,
    P: Parser<R, T>,
    F: Fn(T) -> PossumNodeKind<U>,
{
    parser: P,
    transform: F,
    _r: PhantomData<R>,
    _t: PhantomData<T>,
    _u: PhantomData<U>,
}
impl<R, T, U, P, F> FlatMapParser<R, T, U, P, F>
where
    R: Repr,
    P: Parser<R, T>,
    F: Fn(T) -> PossumNodeKind<U>,
{
    pub fn new(parser: P, transform: F) -> FlatMapParser<R, T, U, P, F> {
        FlatMapParser {
            parser,
            transform,
            _r: PhantomData,
            _t: PhantomData,
            _u: PhantomData,
        }
    }
}

impl<R, T, U, P, F> Parser<R, U> for FlatMapParser<R, T, U, P, F>
where
    R: Repr,
    P: Parser<R, T>,
    F: Fn(T) -> PossumNodeKind<U>,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<U>
    where
        R: Repr,
    {
        self.parser.parse_node(root).flatmap(&self.transform)
    }
}

// Pluralize parses a _single_ T into a sequence of T.
// For parsing a sequence of T, see SeqParser
pub struct Pluralize<R, T, P>
where
    R: Repr,
    P: Parser<R, T>,
{
    inner: P,
    _r: PhantomData<R>,
    _t: PhantomData<T>,
}

impl<R, T, P> Pluralize<R, T, P>
where
    R: Repr,
    P: Parser<R, T>,
{
    pub fn new(inner: P) -> Pluralize<R, T, P> {
        Pluralize {
            inner,
            _r: PhantomData,
            _t: PhantomData,
        }
    }
}

impl<R, T, P> Parser<R, PossumSeq<T>> for Pluralize<R, T, P>
where
    R: Repr,
    P: Parser<R, T>,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<PossumSeq<T>>
    where
        R: Repr,
    {
        PossumNodeKind::Value(self.inner.parse_node(root).at(root).into())
    }
}

pub trait PluralizableParser<R, T, P>
where
    R: Repr,
    P: Parser<R, T>,
{
    fn pluralize(self) -> Pluralize<R, T, P>;
}

impl<R, T, P> PluralizableParser<R, T, P> for P
where
    R: Repr,
    P: Parser<R, T>,
{
    fn pluralize(self) -> Pluralize<R, T, P> {
        Pluralize::new(self)
    }
}

pub trait FlatMappableParser<R, T, U, P, F>
where
    R: Repr,
    P: Parser<R, T>,
    F: Fn(T) -> PossumNodeKind<U>,
{
    fn flatten(self, transform: F) -> FlatMapParser<R, T, U, P, F>;
}

impl<R, T, U, P, F> FlatMappableParser<R, T, U, P, F> for P
where
    R: Repr,
    P: Parser<R, T>,
    F: Fn(T) -> PossumNodeKind<U>,
{
    fn flatten(self, transform: F) -> FlatMapParser<R, T, U, P, F> {
        FlatMapParser::new(self, transform)
    }
}
