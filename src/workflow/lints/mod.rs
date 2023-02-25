use super::Workflow;
use crate::document::{Annotatable, Annotation, AsDocumentPointer};
use crate::lint::Linter;
use crate::scavenge::ast::PossumNode;
pub mod jobs;

pub struct MissingNeededProperties;

pub struct MissingProperty<'a>(&'a str);

impl<'a> MissingProperty<'a> {
    pub fn at<P>(&self, loc: P) -> Annotation
    where
        P: AsDocumentPointer,
    {
        Annotation::error(&loc, &format!("missing required property: {}", self.0))
    }
}

impl Linter<Workflow> for MissingNeededProperties {
    fn lint<A>(&self, root: &PossumNode<Workflow>, annotations: &mut A)
    where
        A: Annotatable,
    {
        let wf = root.value().unwrap();

        if wf.on.is_none() {
            annotations.annotate(MissingProperty("on").at(root))
        }

        if wf.jobs.is_none() {
            annotations.annotate(MissingProperty("jobs").at(root))
        }
    }
}
