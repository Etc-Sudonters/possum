use std::marker::PhantomData;

use yaml_peg::repr::Repr;

use crate::document::{Annotation, Annotations};

use super::ast;
use super::ast::{job, on};

pub enum ParseFailure {
    TooManyDocuments(u8),
}

pub struct WorkflowParser<'a, R>
where
    R: Repr,
{
    _x: &'a PhantomData<R>,
    annotations: Annotations,
}

impl<'a, R> WorkflowParser<'a, R>
where
    R: Repr,
{
    pub fn parse(&mut self, n: &'a yaml_peg::Map<R>) {
        for (k, v) in n.iter() {}
    }

    fn annotation(&mut self, a: Annotation) {
        self.annotations.add(a)
    }

    fn visit_name(&mut self, n: &'a ast::Node<String>) {}
    fn visit_run_name(&mut self, n: &'a ast::Node<String>) {}
    fn visit_on(&mut self, n: &'a ast::Node<on::Trigger>) {}
    fn visit_job(&mut self, n: &'a ast::Node<job::Job>) {}
}
