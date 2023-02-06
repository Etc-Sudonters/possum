use super::base;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Step {
    id: Option<Id>,
    name: Option<String>,
    condition: Option<Condition>,
    env: Option<base::Env>,
    working_directory: Option<String>,
    timeout_minutes: Option<u16>,
    #[serde(flatten)]
    run: Uses,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Condition(base::Expression);

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Id(String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Uses {
    Action(Action),
    Script(Script),
    //TODO: This also supports running from a container
    //Container(Container)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    uses: base::Reference,
    with: Option<HashMap<String, base::Argument>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    // assume all scripts will use some kind of interpolation
    run: String,
    shell: Option<String>,
}
