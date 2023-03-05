use crate::scavenge::ast::PossumNode;
use crate::document::{Annotatable, AsDocumentPointer, Annotation};

pub trait LintRule<T> {
    fn lint(&self, root: &PossumNode<T>, annotations: &mut impl Annotatable);
}

pub trait LintViolation {
    fn at(&self, loc: &impl AsDocumentPointer) -> Annotation;
}
