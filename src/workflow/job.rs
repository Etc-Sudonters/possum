use super::{base, step};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug, Deserialize)]
#[serde(transparent)]
pub struct Id(String);

#[derive(Serialize, Debug, Deserialize)]
pub struct Matrix {
    #[serde(flatten)]
    matrix: HashMap<String, base::Argument>,
    include: base::ExprOr<MatrixInclude>,
    exclude: base::ExprOr<MatrixExclude>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct MatrixInclude;
#[derive(Serialize, Debug, Deserialize)]
pub struct MatrixExclude;

#[derive(Serialize, Debug, Deserialize)]
pub struct Strategy {
    matrix: base::ExprOr<Matrix>,
    fail_fast: bool,
    max_paralell: u32,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(transparent)]
pub struct Condition(base::Expression);

#[derive(Serialize, Debug, Deserialize)]
pub struct Job {
    name: Option<String>,
    needs: Option<Vec<Id>>,
    condition: Option<Condition>,
    env: Option<base::Env>,
    timeout_minutes: Option<u32>,
    continue_on_error: Option<bool>,
    strategy: Option<Strategy>,
    #[serde(rename = "runs-on")]
    runner: Runner,
    steps: Vec<step::Step>,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(untagged)]
pub enum Definition {
    Inline(Inline),
    ReusableWorkflow(ReusableWorkflow),
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(untagged)]
pub enum Runner {
    Runner(String),
    Runners(Vec<String>),
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Inline {}

#[derive(Serialize, Debug, Deserialize)]
pub enum Secrets {
    Inherit,
    Explicit(HashMap<String, base::Expression>),
}

#[derive(Serialize, Debug, Deserialize)]
pub struct ReusableWorkflow {
    uses: base::Reference,
    with: HashMap<String, base::Argument>,
    secrets: Secrets,
}
