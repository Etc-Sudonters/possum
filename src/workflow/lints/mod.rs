use super::Workflow;
use crate::lint::Linter;

pub struct MissingNeededProperties;

impl Linter<Workflow> for MissingNeededProperties {
    fn lint<I>(&self, _: &I) -> Vec<crate::document::Annotation>
    where
        I: AsRef<Workflow>,
    {
        todo!()
    }
}
