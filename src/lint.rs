use crate::{document::Annotatable, scavenge::ast::PossumNode};

pub trait Linter<T> {
    fn lint<A>(&self, root: &PossumNode<T>, annotations: &mut A)
    where
        A: Annotatable;
}
