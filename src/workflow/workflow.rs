use super::{base, job, triggers};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug, Deserialize)]
pub struct Workflow {
    on: triggers::On,
    name: String,
    run_name: Option<base::Expression>,
    env: Option<base::Env>,
    defaults: Option<base::Defaults>,
    concurrency: Option<base::Concurrency>,
    jobs: HashMap<String, job::Job>,
}
