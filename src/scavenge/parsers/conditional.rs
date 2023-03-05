use super::concrete::ExprParser;
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::parser::Parser;
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;
use std::marker::PhantomData;


pub struct OrParser<R, T, LHS, RHS, D>
where
    R: Repr,
    LHS: Parser<R, T>,
    RHS: Parser<R, T>,
    D: Fn(&YamlNode<R>) -> PossumNodeKind<T>,
{
    lhs: LHS,
    rhs: RHS,
    default: D,
    _r: PhantomData<R>,
    _t: PhantomData<T>,
}

impl<R, T, LHS, RHS, D> OrParser<R, T, LHS, RHS, D>
where
    R: Repr,
    LHS: Parser<R, T>,
    RHS: Parser<R, T>,
    D: Fn(&YamlNode<R>) -> PossumNodeKind<T>,
{
    pub fn new(lhs: LHS, rhs: RHS, default: D) -> OrParser<R, T, LHS, RHS, D> {
        OrParser { lhs, rhs, default, _r: PhantomData, _t: PhantomData }
    }
}

impl<R, T, LHS, RHS, D> Parser<R, T> for OrParser<R, T, LHS, RHS, D>
where
    R: Repr,
    LHS: Parser<R, T>,
    RHS: Parser<R, T>,
    D: Fn(&YamlNode<R>) -> PossumNodeKind<T>,
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

pub struct MaybeExprParser<R, T, RHS, D>
where
    R: Repr,
    RHS: Parser<R, T>,
    D: Fn(&YamlNode<R>) -> PossumNodeKind<T>
    {
inner: OrParser<R, T, ExprParser<T>, RHS, D>,
_m: PhantomData<R>,
    }

impl<R, T, RHS, D> MaybeExprParser<R, T, RHS, D>
where
    R: Repr,
    RHS: Parser<R, T>,
    D: Fn(&YamlNode<R>) -> PossumNodeKind<T>,
{
    pub fn new(
        inner: RHS,
        default: D,
    ) -> MaybeExprParser<R, T,  RHS, D> {
        MaybeExprParser {
            inner:OrParser::new(ExprParser::new(), inner, default),
            _m: PhantomData,
        }
    }
}

impl<R, T, RHS, D> Parser<R, T> for MaybeExprParser<R, T, RHS, D>
where
    R: Repr,
    RHS: Parser<R, T>,
    D: Fn(&YamlNode<R>) -> PossumNodeKind<T>
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<T>
    where
        R: Repr,
    {
        self.inner.parse_node(root)
    }
}
