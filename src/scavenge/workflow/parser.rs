use std::marker::PhantomData;

use yaml_peg::parser::{Loader, PError};
use yaml_peg::repr::Repr;
use yaml_peg::{Map,Yaml};

use crate::document::DocumentPointer;
use crate::document::{Annotation, Annotations};

pub enum ParseFailure {
    InvalidDocument(PError),
    Empty,
    NotAWorkflow,
    TooManyDocuments(Vec<DocumentPointer>),
    NotAMap(String),
}

pub fn parse<'a, R>(l: &'a Loader<'a, R>, a: &'a mut Annotations) -> Result<(), ParseFailure>
where
    R: Repr,
{
    let documents = l.parse().map_err(|e| ParseFailure::InvalidDocument(e))?;
    match documents[..] {
        [] => Err(ParseFailure::Empty),
        [n] => {
            match &'a n.yaml() {
                Yaml::Map(m) => Ok(WorkflowParser::new(a).parse(m)),
                _ => Err(ParseFailure::NotAMap("Root yaml node must be an object".to_owned())),
                 
            }
        }
        _ => Err(ParseFailure::TooManyDocuments(vec![])),
    }
}

struct WorkflowParser<'a, R>
where
    R: Repr,
{
    _x: PhantomData<R>,
    annotations: &'a mut Annotations,
}

impl<'a, R> WorkflowParser<'a, R>
where
    R: Repr,
{
    pub fn new(a: &'a mut Annotations) -> WorkflowParser<'a, R> {
        WorkflowParser {
            annotations: a,
            _x: PhantomData,
        }
    }
    pub fn parse(mut self, l: &'a yaml_peg::Map<R>) -> () {}

    fn annotate(&mut self, a: Annotation) {
        self.annotations.add(a)
    }
}
