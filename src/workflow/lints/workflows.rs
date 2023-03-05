use crate::workflow::Workflow;
use crate::document::Annotatable;
use crate::lint::{LintRule, LintViolation};
use crate::scavenge::ast::PossumNode;
use super::MissingProperty;

pub struct MissingWorkflowProperties;

impl LintRule<Workflow> for MissingWorkflowProperties {
    fn lint(&self, root: &PossumNode<Workflow>, annotations: &mut impl Annotatable)
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
