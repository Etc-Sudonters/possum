use crate::{document::{Annotation, AsDocumentPointer}, lint::LintViolation};
pub mod jobs;
pub mod workflows;

pub struct MissingProperty<'a>(&'a str);

impl<'a> LintViolation for MissingProperty<'a> {

    fn at(&self, loc: &impl AsDocumentPointer) -> Annotation
    {
        Annotation::error(loc, &format!("missing required property: {}", self.0))
    }
}

