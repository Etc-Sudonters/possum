use super::document::DocumentPointer;

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
