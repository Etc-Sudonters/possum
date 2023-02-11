use crate::document::Annotation;
use std::convert::Into;
use yaml_peg::{parser, repr::Repr};

use super::visitor::WorkflowVisitor;
use crate::workflow::Builder as WorkflowBuilder;

pub enum ParseFailure {
    TooManyDocuments(u8),
}

pub struct ParseResult<T> {
    result: Option<T>,
    annotations: Vec<Annotation>,
}

struct Parser {
    b: WorkflowBuilder,
    annos: Vec<Annotation>,
}

pub fn parse<'a, T, V, R, L>(
    loader: &parser::Loader<'a, R>,
    visitor: V,
) -> Result<ParseResult<T>, ParseFailure>
where
    R: Repr,
    V: WorkflowVisitor + Into<T>,
{
    todo!()
}
