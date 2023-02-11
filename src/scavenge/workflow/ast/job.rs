use super::core::*;
use super::Concurrency;
use super::Permission;
use super::Step;

node!(
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

pub enum Environment {
    Bare(Node<String>),
    Env {
        name: Node<String>,
        url: Node<String>,
    },
}

pub enum With {
    Args(Node<Map<String, String>>),
    Container {
        args: Node<String>,
        entrypoint: Node<String>,
    },
}
