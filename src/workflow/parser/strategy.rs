use crate::document::{Annotations, AsDocumentPointer};
use crate::scavenge::parser::Builder;
use crate::workflow::job;
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

pub struct StrategyBuilder;

impl Builder<job::Strategy> for StrategyBuilder {
    fn build<'a, P, R>(
        &mut self,
        item: &mut job::Strategy,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer,
        R: Repr,
    {
        todo!()
    }
}

pub struct MatrixBuilder;

impl Builder<job::Matrix> for MatrixBuilder {
    fn build<'a, P, R>(
        &mut self,
        item: &mut job::Matrix,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer,
        R: Repr,
    {
        todo!()
    }
}
