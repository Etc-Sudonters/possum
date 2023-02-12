use std::marker::PhantomData;

use yaml_peg::parser::{Loader, PError};
use yaml_peg::repr::Repr;
use yaml_peg::{Map, Node as YamlNode, Yaml};

use super::ast::{core, job, on, Step, Workflow};
use crate::document::DocumentPointer;
use crate::document::{Annotation, Annotations};
use std::default::Default;

pub enum ParseFailure {
    InvalidDocument(PError),
    Empty,
    TooManyDocuments(Vec<DocumentPointer>),
    NotAMap(String, DocumentPointer),
}

pub fn parse<'a, R>(
    loader: &'a mut Loader<'a, R>,
    annotations: &'a mut Annotations,
) -> Result<Workflow, ParseFailure>
where
    R: Repr,
{
    let documents = loader
        .parse()
        .map_err(|e| ParseFailure::InvalidDocument(e))?;

    match &documents[..] {
        [n] => match &n.yaml() {
            Yaml::Map(m) => Ok(WorkflowParser::new(annotations).parse(m)),
            _ => Err(ParseFailure::NotAMap(
                "Root yaml node must be an object".to_owned(),
                DocumentPointer(n.pos()),
            )),
        },
        [] => Err(ParseFailure::Empty),
        _ => Err(ParseFailure::TooManyDocuments(vec![])),
    }
}

struct WorkflowParser<'a, R>
where
    R: Repr,
{
    _x: PhantomData<R>,
    annotations: &'a mut Annotations,
    workflow: Workflow,
}

impl<'a, R> WorkflowParser<'a, R>
where
    R: Repr,
{
    pub fn new(a: &'a mut Annotations) -> WorkflowParser<'a, R> {
        WorkflowParser {
            annotations: a,
            workflow: Workflow::default(),
            _x: PhantomData,
        }
    }

    pub fn parse(mut self, m: &'a Map<R>) -> Workflow {
        for (key, value) in m.iter() {
            match extract_str_key(key) {
                Ok(s) => self.visit_root_key(s.to_lowercase(), key, value),
                Err(a) => self.annotate(a),
            }
        }

        self.workflow
    }

    fn visit_root_key(&mut self, raw_key: String, key: &YamlNode<R>, value: &'a YamlNode<R>) {
        match raw_key.as_str() {
            "on" => self.on(value),
            "jobs" => self.jobs(value),
            "name" => self.name(value),
            "run_name" => self.run_name(value),
            _ => self.annotate(Annotation::warn(
                DocumentPointer(key.pos()),
                format!("unknown key {raw_key}").as_str(),
            )),
        }
    }

    fn annotate(&mut self, a: Annotation) {
        self.annotations.add(a)
    }

    fn on(&mut self, n: &'a YamlNode<R>) {}
    fn name(&mut self, n: &'a YamlNode<R>) {}
    fn run_name(&mut self, n: &'a YamlNode<R>) {}
    fn jobs(&mut self, n: &'a YamlNode<R>) {}
}

fn extract_str_key<'a, R>(key: &'a YamlNode<R>) -> Result<&'a str, Annotation>
where
    R: Repr,
{
    match key.as_str() {
        Ok(s) => Ok(s),
        Err(pos) => Err(Annotation::error(
            DocumentPointer(pos),
            "workflow keys must be strings",
        )),
    }
}
