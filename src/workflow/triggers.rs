use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug, Deserialize)]
pub struct GlobPattern(String);
#[derive(Serialize, Debug, Deserialize)]
pub struct GlobbedBranch(GlobPattern);
#[derive(Serialize, Debug, Deserialize)]
pub struct GlobbedTag(GlobPattern);
#[derive(Serialize, Debug, Deserialize)]
pub struct GlobbedPath(GlobPattern);

#[derive(Serialize, Debug, Deserialize)]
pub enum Event {
    PullRequest(PullRequest),
    Push(Push),
}

#[derive(Serialize, Debug, Deserialize)]
pub struct PullRequest {
    branches: Vec<GlobbedBranch>,
    branches_ignore: Vec<GlobbedBranch>,
    tags: Vec<GlobbedTag>,
    tags_ignore: Vec<GlobbedTag>,
    paths: Vec<GlobbedPath>,
    paths_ignore: Vec<GlobbedPath>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Push {
    paths: Vec<GlobbedPath>,
    paths_ignore: Vec<GlobbedPath>,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(untagged)]
pub enum On {
    Event(String),
    Events(Vec<String>),
    ConfiguredEvents(HashMap<String, Event>),
}
