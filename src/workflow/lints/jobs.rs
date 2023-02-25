use crate::document::{Annotatable, Annotation, AsDocumentPointer};
use crate::lint::Linter;
use crate::scavenge::ast::{PossumMap, PossumNode};
use crate::workflow::job::Job;

pub struct EmptyJobs;

impl EmptyJobs {
    fn at<P>(p: &P) -> Annotation
    where
        P: AsDocumentPointer,
    {
        Annotation::error(p, &String::from("No jobs present in this workflow"))
    }
}

impl Linter<PossumMap<String, Job>> for EmptyJobs {
    fn lint<A>(&self, root: &PossumNode<PossumMap<String, Job>>, annotations: &mut A)
    where
        A: Annotatable,
    {
        match root.value() {
            Some(j) if j.is_empty() => annotations.annotate(Self::at(root)),
            _ => {}
        }
    }
}
