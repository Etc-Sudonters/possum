extern crate serde;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Expression(String);

#[derive(Serialize, Deserialize, Debug)]
pub enum ExprOr<T> {
    Expr(Expression),
    Value(T),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Input {
    String(String),
    Bool(bool),
    Number(i64),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Argument {
    String(String),
    Bool(bool),
    Number(i64),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Reference(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Defaults {
    runs: RunDefaults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunDefaults {
    shell: Option<String>,
    working_directory: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Concurrency {
    Group(ConcurrencyGroup),
    Concurrency(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConcurrencyGroup {
    group: Expression,
    cancel_in_progress: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Env(HashMap<String, String>);
