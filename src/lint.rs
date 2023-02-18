use crate::document::Annotation;

pub trait Linter<T> {
    fn lint<I>(&self, target: &I) -> Vec<Annotation>
    where
        I: AsRef<T>;
}
