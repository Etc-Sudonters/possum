use super::document::DocumentPointer;
use std::convert::AsRef;
use std::fmt::Display;
use strum::Display;

#[derive(Debug)]
pub struct Annotations(Vec<Annotation>);

impl Annotations {
    pub fn new() -> Annotations {
        Annotations(Vec::with_capacity(16))
    }

    pub fn add(&mut self, a: Annotation) {
        self.0.push(a)
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
    fn from_parts(level: AnnotationLevel, msg: &str, pointer: DocumentPointer) -> Annotation {
        Annotation(level, msg.to_owned(), pointer)
    }

    pub fn info(pointer: DocumentPointer, msg: &str) -> Annotation {
        Annotation::from_parts(AnnotationLevel::Info, msg, pointer)
    }

    pub fn warn(pointer: DocumentPointer, msg: &str) -> Annotation {
        Annotation::from_parts(AnnotationLevel::Warn, msg, pointer)
    }

    pub fn error(pointer: DocumentPointer, msg: &str) -> Annotation {
        Annotation::from_parts(AnnotationLevel::Error, msg, pointer)
    }

    pub fn fatal(pointer: DocumentPointer, msg: &str) -> Annotation {
        Annotation::from_parts(AnnotationLevel::Fatal, msg, pointer)
    }
}
