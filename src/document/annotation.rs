use super::document::{AsDocumentPointer, DocumentPointer};
use std::convert::AsRef;
use std::fmt::Display;
use strum::Display;

#[derive(Debug)]
pub struct Annotations(Vec<Annotation>);

pub trait Annotatable {
    fn annotate<A>(&mut self, annotation: A)
    where
        A: Into<Annotation>;
}

impl Annotations {
    pub fn new() -> Annotations {
        Annotations(Vec::with_capacity(16))
    }

    pub fn add<A>(&mut self, a: A)
    where
        A: Into<Annotation>,
    {
        self.0.push(a.into())
    }

    pub fn entries(&self) -> std::slice::Iter<Annotation> {
        self.0.iter()
    }
}

#[derive(Debug, Display)]
pub enum AnnotationLevel {
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug)]
pub struct Annotation(AnnotationLevel, String, DocumentPointer);

impl Display for Annotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.0, self.1)
    }
}

impl AsRef<DocumentPointer> for Annotation {
    fn as_ref(&self) -> &DocumentPointer {
        &self.2
    }
}

impl Annotation {
    fn from_parts<P, I>(level: AnnotationLevel, msg: &I, pointer: &P) -> Annotation
    where
        P: AsDocumentPointer,
        I: Display,
    {
        Annotation(level, msg.to_string(), pointer.as_document_pointer())
    }

    pub fn info<P, I>(pointer: &P, msg: &I) -> Annotation
    where
        P: AsDocumentPointer,
        I: Display,
    {
        Annotation::from_parts(AnnotationLevel::Info, msg, pointer)
    }

    pub fn warn<P, I>(pointer: &P, msg: &I) -> Annotation
    where
        P: AsDocumentPointer,
        I: Display,
    {
        Annotation::from_parts(AnnotationLevel::Warn, msg, pointer)
    }

    pub fn error<P, I>(pointer: &P, msg: &I) -> Annotation
    where
        P: AsDocumentPointer,
        I: Display,
    {
        Annotation::from_parts(AnnotationLevel::Error, msg, pointer)
    }

    pub fn fatal<P, I>(pointer: &P, msg: &I) -> Annotation
    where
        P: AsDocumentPointer,
        I: Display,
    {
        Annotation::from_parts(AnnotationLevel::Fatal, msg, pointer)
    }
}
