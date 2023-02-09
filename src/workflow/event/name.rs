use std::cmp::{Eq, PartialEq};
use std::convert::{From, Into};
use std::fmt::Display;
use std::hash::Hash;
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct EventName(String);

impl EventName {
    pub fn new(s: String) -> EventName {
        EventName(s.into())
    }
}

impl From<String> for EventName {
    fn from(value: String) -> Self {
        EventName::new(value)
    }
}

impl From<&String> for EventName {
    fn from(value: &String) -> Self {
        let s = value.to_owned();
        s.into()
    }
}

impl From<&str> for EventName {
    fn from(value: &str) -> Self {
        let s: String = value.into();
        s.into()
    }
}

impl Display for EventName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
