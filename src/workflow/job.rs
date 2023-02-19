use super::Concurrency;
use super::Permission;
use crate::scavenge::ast::*;

possum_node_type!(
    #[derive(Debug, Default)]
    struct Job {
        name: String,
        permissions: Permission,
        needs: PossumSeq<String>,
        cond: String,
        runs_on: PossumSeq<String>,
        environment: Environment,
        concurrency: Concurrency,
        outputs: PossumMap<String, String>,
        env: PossumMap<String, String>,
        steps: PossumSeq<Step>,
        timeout_minutes: f64,
        continue_on_error: bool,
        uses: String,
        with: PossumMap<String, String>,
    }
);

#[derive(Debug)]
pub enum Environment {
    Bare(String),
    Env {
        name: Option<PossumNode<String>>,
        url: Option<PossumNode<String>>,
    },
}

possum_node_type!(
    #[derive(Debug, Default)]
    struct Step {
        id: String,
        cond: String,
        name: String,
        uses: String,
        run: String,
        shell: String,
        with: PossumMap<String, String>,
        env: PossumMap<String, String>,
    }
);
