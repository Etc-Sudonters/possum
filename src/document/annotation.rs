use super::document::DocumentPointer;
use std::convert::AsRef;

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

    pub fn Info<P>(pointer: P, msg: &str) -> Annotation
    where
        P: AsRef<DocumentPointer>,
    {
        Annotation::from_parts(AnnotationLevel::Info, msg, pointer.as_ref())
    }

    pub fn Warn<P>(pointer: P, msg: &str) -> Annotation
    where
        P: AsRef<DocumentPointer>,
    {
        Annotation::from_parts(AnnotationLevel::Warn, msg, pointer.as_ref())
    }
    pub fn Error<P>(pointer: P, msg: &str) -> Annotation
    where
        P: AsRef<DocumentPointer>,
    {
        Annotation::from_parts(AnnotationLevel::Error, msg, pointer.as_ref())
    }

    pub fn Fatal<P>(pointer: P, msg: &str) -> Annotation
    where
        P: AsRef<DocumentPointer>,
    {
        Annotation::from_parts(AnnotationLevel::Fatal, msg, pointer.as_ref())
    }
}
