use std::collections::HashMap;
use std::default::Default;

use super::ActivityTypes;
use super::Event;
#[derive(Debug)]
pub enum Schedule {
    Cron(String),
}

#[derive(Debug)]
pub struct PullRequest {
    branches: Vec<String>,
    ignored_branches: Vec<String>,
    paths: Vec<String>,
    ignored_paths: Vec<String>,
    types: ActivityTypes,
}

impl Default for PullRequest {
    fn default() -> Self {
        PullRequest {
            branches: Vec::new(),
            ignored_branches: Vec::new(),
            paths: Vec::new(),
            ignored_paths: Vec::new(),
            types: Event::default_activity_types("pull_request"),
        }
    }
}

#[derive(Debug)]
pub struct Push {
    branches: Vec<String>,
    ignored_branches: Vec<String>,
    paths: Vec<String>,
    ignored_paths: Vec<String>,
    tags: Vec<String>,
    ignored_tags: Vec<String>,
    types: ActivityTypes,
}

impl Default for Push {
    fn default() -> Self {
        Push {
            branches: Vec::new(),
            ignored_branches: Vec::new(),
            paths: Vec::new(),
            ignored_paths: Vec::new(),
            tags: Vec::new(),
            ignored_tags: Vec::new(),
            types: Event::default_activity_types("push"),
        }
    }
}

#[derive(Debug)]
pub struct CallInputEntry<T> {
    required: bool,
    description: String,
    default: Option<T>,
}

#[derive(Debug)]
pub enum CallInput {
    Bool(CallInputEntry<bool>),
    Str(CallInputEntry<String>),
    Number(CallInputEntry<f64>),
    Unsure(CallInputEntry<String>),
}

#[derive(Debug, Default)]
pub struct WorkflowCall {
    inputs: HashMap<String, CallInput>,
    outputs: HashMap<String, String>,
    secrets: HashMap<String, bool>,
}

#[derive(Debug)]
pub struct DispatchInputEntry<T> {
    description: String,
    default: Option<T>,
    required: bool,
}

#[derive(Debug)]
pub enum DispatchInput {
    Bool(DispatchInputEntry<bool>),
    Str(DispatchInputEntry<String>),
    Number(DispatchInputEntry<f64>),
    Choice(DispatchInputEntry<String>, Vec<String>),
    Unsure(DispatchInputEntry<String>),
}

#[derive(Debug, Default)]
pub struct WorkflowDispatch {
    inputs: HashMap<String, DispatchInput>,
}

#[derive(Debug, Default)]
pub struct WorkflowRun {
    branches: Vec<String>,
    ignored_branches: Vec<String>,
    types: ActivityTypes,
    workflows: Vec<String>,
}
