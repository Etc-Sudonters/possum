use super::event;
use std::collections::HashMap;
use std::convert::TryInto;
use std::default::Default;
use std::fmt::Display;

#[derive(Default)]
pub struct Builder {
    events: HashMap<String, event::Event>,
    name: Option<String>,
    run_name: Option<String>,
}

#[derive(Debug)]
pub struct Workflow {
    name: Option<String>,
    events: HashMap<String, event::Event>,
    run_name: Option<String>,
}

pub struct WorkflowConstructionError;

impl Display for WorkflowConstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "something bad happened")
    }
}

impl TryInto<Workflow> for Builder {
    type Error = WorkflowConstructionError;

    fn try_into(self) -> Result<Workflow, Self::Error> {
        Ok(Workflow {
            events: self.events,
            name: self.name,
            run_name: self.run_name,
        })
    }
}

impl Builder {
    pub fn new() -> Builder {
        Builder::default()
    }

    pub fn responds_to(&mut self, event: event::Event) {
        self.events.insert(event.to_string(), event);
    }

    pub fn has_name(&mut self, name: String) {
        self.name = Some(name)
    }

    pub fn has_run_name(&mut self, run_name: String) {
        self.run_name = Some(run_name)
    }
}
