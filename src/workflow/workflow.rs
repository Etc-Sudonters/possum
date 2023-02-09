use super::event;
use std::collections::HashMap;
use std::convert::TryInto;

pub struct Builder {
    events: HashMap<String, event::Event>,
}

#[derive(Debug)]
pub struct Workflow {
    events: HashMap<String, event::Event>,
}

pub struct WorkflowConstructionError;

impl TryInto<Workflow> for Builder {
    type Error = WorkflowConstructionError;

    fn try_into(self) -> Result<Workflow, Self::Error> {
        Ok(Workflow {
            events: self.events,
        })
    }
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            events: HashMap::new(),
        }
    }

    pub fn responds_to(&mut self, event: event::Event) {
        self.events.insert(event.to_string(), event);
    }
}
