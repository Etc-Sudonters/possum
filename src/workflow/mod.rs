#[derive(Debug)]
pub struct Builder {}

pub enum Trigger {
    Push,
    WorkflowDispatch,
}

pub enum Job {
    Inline(Vec<Step>),
    Workflow,
}

pub enum Step {
    Inline,
    Action,
}
