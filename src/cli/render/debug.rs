use crate::project::Project;
use std::fmt::Display;

pub struct DebugRender(pub Project);

impl Display for DebugRender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}
