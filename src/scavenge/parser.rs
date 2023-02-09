use crate::workflow;
use std::str::FromStr;
use thiserror::Error;
use yaml_peg::parser::Loader;
use yaml_peg::repr::{RcRepr, Repr};
use yaml_peg::{Node, Yaml};

#[derive(Debug, Error)]
#[error("{0}")]
pub struct WorkflowParseError(String);

pub fn parse_workflow(doc: &[u8]) -> Result<workflow::Workflow, WorkflowParseError> {
    let mut loader: Loader<RcRepr> = Loader::new(doc);
    let workflow_documents = loader
        .parse()
        .map_err(|_| make_parse_err("Failed to parse document"))?;

    let count = workflow_documents.len();
    if count > 1 {
        return Err(WorkflowParseError(format!(
            "Expected exactly 1 workflow document in file, found {count}"
        )));
    }

    let workflow_document = workflow_documents
        .first()
        .unwrap()
        .as_map()
        .map_err(parse_error("Expected workflow document to be a map"))?;

    let mut builder = workflow::Builder::new();

    for (key, value) in workflow_document.iter() {
        let key = get_string(key)
            .map(|s| s.to_lowercase())
            .map_err(map_key_must_be_strings())?;

        match key.as_str() {
            "on" => {
                parse_workflow_on(&mut builder, value)?;
            }
            _ => {}
        }
    }

    builder
        .try_into()
        .map_err(parse_error("failed to map builder into workflow"))
}

fn make_parse_err(s: &str) -> WorkflowParseError {
    WorkflowParseError(s.to_owned())
}

fn get_string<'a, R: Repr>(n: &'a Node<R>) -> Result<&'a String, ()> {
    match n.yaml() {
        Yaml::Str(s) => Ok(s),
        _ => Err(()),
    }
}
fn parse_workflow_on<'a, R: Repr>(
    b: &mut workflow::Builder,
    n: &'a Node<R>,
) -> Result<(), WorkflowParseError> {
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

fn failed_to_parse_to_event<'a, T>(
    unknown: &'a String,
) -> impl FnOnce(T) -> WorkflowParseError + 'a {
    move |_| WorkflowParseError(format!("failed to parse {unknown} to event"))
}

fn map_key_must_be_strings<'a, T>() -> impl FnOnce(T) -> WorkflowParseError + 'a {
    |_| make_parse_err("map keys must be strings")
}

fn parse_error<'a, T>(msg: &'a str) -> impl FnOnce(T) -> WorkflowParseError + 'a {
    |_| make_parse_err(msg)
}
