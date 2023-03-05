use super::ast::{PossumNode, PossumNodeKind};
use crate::document::{Annotation, AsDocumentPointer, DocumentPointer};
use std::fmt::{Display};
use yaml_peg::parser::{Loader, PError};
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

#[derive(Debug)]
pub enum ParseFailure {
    InvalidDocument(PError),
    Empty,
    TooManyDocuments(Vec<DocumentPointer>),
    CouldntOpen(std::io::Error),
}

impl std::error::Error for ParseFailure {}

impl Display for ParseFailure {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct UnexpectedKey<'a>(&'a str);

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
