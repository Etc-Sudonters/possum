use crate::workflow;
use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;
use yaml_peg::parser::Loader;
use yaml_peg::repr::{RcRepr, Repr};
use yaml_peg::{Node, Yaml};

use super::visitor::{self, ParserVisitor};

#[derive(Debug)]
pub enum Error {
    InvalidYaml,
    MapKeyMustBeString,
    Other(String),
    UnexpectedDocumentCount(usize),
    EventParse(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        let msg = match self {
            InvalidYaml => format!("Invalid YAML encountered"),
            MapKeyMustBeString => format!("Map key must be string"),
            Other(s) => s.to_owned(),
            UnexpectedDocumentCount(n) => format!("Expected exactly 1 document, found {n}"),
            EventParse(s) => format!("failed to parse event: {s}"),
        };
        write!(f, "{}", msg)
    }
}

pub struct WorkflowBuilderVisitor<R: Repr> {
    builder: workflow::Builder,
    _spooky: PhantomData<R>,
}

impl<R: Repr> WorkflowBuilderVisitor<R> {
    fn new() -> WorkflowBuilderVisitor<R> {
        WorkflowBuilderVisitor {
            builder: workflow::Builder::new(),
            _spooky: PhantomData,
        }
    }
}

impl<R> visitor::ParserVisitor<R> for WorkflowBuilderVisitor<R>
where
    R: Repr,
{
    fn visit_on_str(&mut self, on: &str) {
        if let Ok(evt) = workflow::Event::from_str(on) {
            self.builder.responds_to(evt)
        }
    }

    fn visit_on_seq(&mut self, on: &yaml_peg::Seq<R>) {
        for node in on.iter() {
            self.visit_on(node);
        }
    }

    fn visit_on_map(&mut self, on: &yaml_peg::Map<R>) {
        for (key, _) in on.iter() {
            self.visit_on(key);
        }
    }

    fn visit_name(&mut self, name: &str) {
        self.builder.has_name(name.to_owned())
    }

    fn visit_run_name(&mut self, run_name: &str) {
        self.builder.has_run_name(run_name.to_owned())
    }
}

pub fn parse_workflow(doc: &[u8]) -> Result<workflow::Workflow, Error> {
    let mut loader: Loader<RcRepr> = Loader::new(doc);
    let workflow_documents = loader
        .parse()
        .map_err(|_| make_parse_err("Failed to parse document"))?;

    let count = workflow_documents.len();
    if count > 1 {
        return Err(Error::UnexpectedDocumentCount(count));
    }

    let workflow_document = workflow_documents
        .first()
        .unwrap()
        .as_map()
        .map_err(|_| Error::Other("Expected document to be map".to_owned()))?;

    let mut visitor = WorkflowBuilderVisitor::new();

    for (key, value) in workflow_document.iter() {
        let key = get_string(key)
            .map(|s| s.to_lowercase())
            .map_err(map_key_must_be_strings())?;

        match key.as_str() {
            "on" => visitor.visit_on(value),
            "name" => {
                if let Yaml::Str(name) = value.yaml() {
                    visitor.visit_name(name)
                }
            }
            _ => {}
        }
    }

    visitor
        .builder
        .try_into()
        .map_err(|e| Error::Other(format!("failed to build into workflow: {e}")))
}

fn make_parse_err(s: &str) -> Error {
    Error::Other(s.to_owned())
}

fn get_string<'a, R: Repr>(n: &'a Node<R>) -> Result<&'a String, ()> {
    match n.yaml() {
        Yaml::Str(s) => Ok(s),
        _ => Err(()),
    }
}
fn parse_workflow_on<'a, R: Repr>(b: &mut workflow::Builder, n: &'a Node<R>) -> Result<(), Error> {
    match n.yaml() {
        Yaml::Str(s) => {
            b.responds_to(
                workflow::Event::from_str(s).map_err(failed_to_parse_to_event(&format!(
                    "Failed to parse {s} to event"
                )))?,
            );
            Ok(())
        }
        Yaml::Seq(seq) => {
            for node in seq.iter() {
                let event_name = get_string(node).map_err(map_key_must_be_strings())?;
                b.responds_to(
                    workflow::Event::from_str(&event_name)
                        .map_err(failed_to_parse_to_event(event_name))?,
                );
            }
            Ok(())
        }
        Yaml::Map(map) => {
            for (key, _) in map.iter() {
                let event_name = get_string(key).map_err(map_key_must_be_strings())?;
                b.responds_to(
                    workflow::Event::from_str(&event_name)
                        .map_err(failed_to_parse_to_event(event_name))?,
                )
            }
            Ok(())
        }
        _ => Err(make_parse_err("unexpected type for on")),
    }
}

fn failed_to_parse_to_event<'a, T>(unknown: &'a String) -> impl FnOnce(T) -> Error + 'a {
    move |_| Error::EventParse(unknown.to_owned())
}

fn map_key_must_be_strings<'a, T>() -> impl FnOnce(T) -> Error + 'a {
    |_| Error::MapKeyMustBeString
}
