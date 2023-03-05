use crate::document::{Annotatable, Annotation, AsDocumentPointer};
use crate::lint::LintRule;
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

impl LintRule<PossumMap<String, Job>> for EmptyJobs {
    fn lint(&self, root: &PossumNode<PossumMap<String, Job>>, annotations: &mut impl Annotatable)
    {
        match root.value() {
            Some(j) if j.is_empty() => annotations.annotate(Self::at(root)),
            _ => {}
        }
    }
}
