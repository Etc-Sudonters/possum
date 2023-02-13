use super::Concurrency;
use super::Permission;
use crate::scavenge::ast::*;

possum_node!(
    #[derive(Debug)]
    struct Job {
        id: String,
        name: String,
        permissions: Permission,
        needs: Seq<String>,
        cond: String,
        runs_on: String,
        environment: Environment,
        concurrency: Concurrency,
        outputs: Map<String, String>,
        env: Map<String, String>,
        steps: Seq<Step>,
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
    Args(PossumNode<Map<String, String>>),
    Container {
        args: PossumNode<String>,
        entrypoint: PossumNode<String>,
    },
}

possum_node!(
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
    Args(Map<String, String>),
}
