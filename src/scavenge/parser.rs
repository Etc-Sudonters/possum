use yaml_peg::parser::{Loader, PError};
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

use crate::document::DocumentPointer;

#[derive(Debug)]
pub enum ParseFailure {
    InvalidDocument(PError),
    Empty,
    TooManyDocuments(Vec<DocumentPointer>),
    NotAMap(String, DocumentPointer),
    CouldntOpen,
}

pub fn parse<'a, R, T, P>(mut loader: Loader<'a, R>, parser: P) -> Result<T, ParseFailure>
where
    R: Repr + 'a,
    P: Parser<'a, R, T>,
{
    let documents = loader
        .parse()
        .map_err(|e| ParseFailure::InvalidDocument(e))?;

    match documents[..] {
        [] => Err(ParseFailure::Empty),
        [root] => Ok(parser.parse(root)),
        _ => Err(ParseFailure::TooManyDocuments(vec![])),
    }
}

pub trait Parser<'a, R, T>
where
    R: Repr,
{
    fn parse(self, root: &'a YamlNode<R>) -> Result<T, ParseFailure>
    where
        R: Repr;
}
