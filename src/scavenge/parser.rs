use yaml_peg::parser::{Loader, PError};
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

use super::ast::{PossumNode, PossumNodeKind};
use crate::document::{Annotation, AsDocumentPointer, DocumentPointer};
use std::fmt::Display;

pub struct UnexpectedKey<'a, S, P>(&'a S, &'a P)
where
    P: AsDocumentPointer,
    S: Display;

impl<'a, S, P> UnexpectedKey<'a, S, P>
where
    P: AsDocumentPointer,
    S: Display,
{
    pub fn new(s: &'a S, p: &'a P) -> UnexpectedKey<'a, S, P> {
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
    parser: P,
) -> Result<PossumNode<T>, ParseFailure>
where
    R: Repr + 'a,
    P: Parser<'a, R, T>,
{
    let mut documents = loader
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

    let root = documents.remove(0);
    Ok(parser.parse_node(&root).at(root.pos()))
}

pub trait Parser<'a, R, T>
where
    R: Repr + 'a,
{
    fn parse_node(self, root: &YamlNode<R>) -> PossumNodeKind<T>
    where
        R: Repr;
}
