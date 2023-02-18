use super::Concurrency;
use super::Permission;
use crate::scavenge::ast::*;

possum_node_type!(
    #[derive(Debug)]
    struct Job {
        name: String,
        permissions: Permission,
        needs: PossumSeq<String>,
        cond: String,
        runs_on: String,
        environment: Environment,
        concurrency: Concurrency,
        outputs: PossumMap<String, String>,
        env: PossumMap<String, String>,
        steps: PossumSeq<Step>,
        timeout_minutes: u64,
        continue_on_error: bool,
        uses: String,
        with: With,
    }
);

#[derive(Debug)]
pub enum Environment {
    Bare(PossumNode<String>),
    Env {
        name: PossumNode<String>,
        url: PossumNode<String>,
    },
}

#[derive(Debug)]
pub enum With {
    Args(PossumNode<PossumMap<String, String>>),
    Container {
        args: PossumNode<String>,
        entrypoint: PossumNode<String>,
    },
}

possum_node_type!(
    #[derive(Debug)]
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

#[derive(Debug)]
pub enum StepWith {
    Container {
        args: PossumNode<String>,
        entrypoint: PossumNode<String>,
    },
    Args(PossumMap<String, String>),
}
