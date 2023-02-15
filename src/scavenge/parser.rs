use yaml_peg::parser::{Loader, PError};
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

use super::ast::PossumNode;
use crate::document::DocumentPointer;

#[derive(Debug)]
pub enum ParseFailure {
    InvalidDocument(PError),
    Empty,
    TooManyDocuments(Vec<DocumentPointer>),
    CouldntOpen,
}

pub fn parse<'a, R, T, P>(
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
    Ok(parser.parse(root))
}

pub trait Parser<'a, R, T>
where
    R: Repr + 'a,
{
    fn parse(self, root: YamlNode<R>) -> PossumNode<T>
    where
        R: Repr;
}
