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
        strategy: Strategy,
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

possum_node_type!(
    #[derive(Debug, Default)]
    struct Strategy {
        matrix: Matrix,
        fail_fast: bool,
        max_parallel: f64,
    }
);

// Matrix has include and exclude embedded within the map itself, for some reason
// that means we can't use possum_node_type! here because entries isn't actually a node itself
#[derive(Debug, Default)]
pub struct Matrix {
    pub entries: PossumMap<String, PossumSeq<MatrixInput>>,
    pub include: Option<PossumNode<PossumSeq<PossumMap<String, MatrixInput>>>>,
    pub exclude: Option<PossumNode<PossumSeq<PossumMap<String, MatrixInput>>>>,
}

#[derive(Debug)]
pub enum MatrixInput {
    Str(String),
    Number(f64),
    Bool(bool),
}
