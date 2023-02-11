use super::document::DocumentPointer;
use std::convert::AsRef;

pub struct Annotations(Vec<Annotation>);

impl Annotations {
    pub fn new() -> Annotations {
        Annotations(Vec::with_capacity(16))
    }

    pub fn add(&mut self, a: Annotation) {
        self.0.push(a)
    }
}

pub enum AnnotationLevel {
    Info,
    Warn,
    Error,
    Fatal,
}

pub struct Annotation(AnnotationLevel, String, DocumentPointer);

impl Annotation {
    fn from_parts(level: AnnotationLevel, msg: &str, pointer: &DocumentPointer) -> Annotation {
        Annotation(level, msg.to_owned(), pointer.clone())
    }

    pub fn info<P>(pointer: P, msg: &str) -> Annotation
    where
        P: AsRef<DocumentPointer>,
    {
        Annotation::from_parts(AnnotationLevel::Info, msg, pointer.as_ref())
    }

    pub fn warn<P>(pointer: P, msg: &str) -> Annotation
    where
        P: AsRef<DocumentPointer>,
    {
        Annotation::from_parts(AnnotationLevel::Warn, msg, pointer.as_ref())
    }
    pub fn error<P>(pointer: P, msg: &str) -> Annotation
    where
        P: AsRef<DocumentPointer>,
    {
        Annotation::from_parts(AnnotationLevel::Error, msg, pointer.as_ref())
    }

    pub fn fatal<P>(pointer: P, msg: &str) -> Annotation
    where
        P: AsRef<DocumentPointer>,
    {
        Annotation::from_parts(AnnotationLevel::Fatal, msg, pointer.as_ref())
    }
}
