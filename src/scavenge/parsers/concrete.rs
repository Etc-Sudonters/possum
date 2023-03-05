use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::Extract;
use crate::scavenge::Parser;
use std::marker::PhantomData;
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;


pub struct StringParser;
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

pub struct ExprParser<T>(PhantomData<T>);
impl<T> ExprParser<T> {
    pub fn new() -> ExprParser<T> {
        ExprParser(PhantomData)
    }
}

impl<R, T> Parser<R, T> for ExprParser<T>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<T>
    where
        R: Repr,
    {
        StringParser.parse_node(root).flatmap(PossumNodeKind::Expr)
    }
}

pub struct BoolParser;
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

pub struct NumberParser;
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
