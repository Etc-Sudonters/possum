use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct ActivityType(String);

impl ActivityType {
    pub fn new(s: String) -> ActivityType {
        ActivityType(s)
    }
}

impl Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type ActivityTypes = HashSet<ActivityType>;
