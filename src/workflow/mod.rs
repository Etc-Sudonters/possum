pub mod job;
pub mod lints;
pub mod on;
pub mod parser;

pub use self::parser::WorkflowParser;
use crate::scavenge::ast::*;
use std::default::Default;

possum_node_type! {
    #[derive(Debug, Default)]
    struct Workflow {
        name: String,
        run_name: String,
        on: on::Trigger,
        jobs: PossumMap<String, job::Job>,
    }
}

#[derive(Debug)]
pub enum Concurrency {
    Concurrency(String),
    Group {
        group: PossumNode<String>,
        cancel_in_progress: PossumNode<bool>,
    },
}

#[derive(Debug)]
pub enum Permission {
    GlobalGrant(PossumNode<Grant>),
    GlobalRevoke,
    IndividualGrants(PossumNode<PossumMap<String, Grant>>),
}

#[derive(Debug)]
pub enum Grant {
    Read,
    Write,
}
