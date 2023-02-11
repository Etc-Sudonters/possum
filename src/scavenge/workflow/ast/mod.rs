mod builder;
#[macro_use]
mod core;
pub mod job;
pub mod on;
pub use self::core::*;

node! {
    struct Workflow {
        name: String,
        run_name: String,
        on: on::Trigger,
        jobs: job::Job,
    }
}

node!(
    struct Step {
        id: String,
        cond: String,
        name: String,
        uses: String,
        run: String,
        shell: String,
        with: StepWith,
    }
);

pub enum StepWith {
    Container {
        args: Node<String>,
        entrypoint: Node<String>,
    },
    Args(Map<String, String>),
}

pub enum Concurrency {
    Concurrency(String),
    Group {
        group: Node<String>,
        cancel_in_progress: Node<bool>,
    },
}

pub enum Permission {
    GlobalGrant(Node<Grant>),
    GlobalRevoke,
    IndividualGrants(Node<Map<String, Grant>>),
}

pub enum Grant {
    Read,
    Write,
}
