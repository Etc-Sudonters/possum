use strum::Display;

use yaml_peg::repr::Repr;
use yaml_peg::{Map, Node as YamlNode};

use super::Workflow;
use crate::document::DocumentPointer;
use crate::document::{Annotation, Annotations};
use crate::scavenge::parser::{ParseFailure, Parser};
use std::marker::PhantomData;

pub struct WorkflowParser<'a, R>
where
    R: Repr + 'a,
{
    _x: PhantomData<R>,
    annotations: &'a mut Annotations,
    workflow: Workflow,
}

impl<'a, R> Parser<'a, R, Workflow> for WorkflowParser<'a, R>
where
    R: Repr + 'a,
{
    fn parse(mut self, root: YamlNode<R>) -> Result<Workflow, ParseFailure> {
        root.as_map()
            .map(|m| self.parse_map(m))
            .map_err(|e| ParseFailure::NotAMap(e.into()))?;
        Ok(self.workflow)
    }
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

    fn parse_map(&mut self, m: Map<R>) {
        for (key, value) in m.into_iter() {
            match key.extract_str() {
                Ok(s) => self.visit_root_key(s.to_lowercase(), key, value),
                Err(a) => self.annotate(a),
            }
        }
    }

    fn visit_root_key(&mut self, raw_key: String, key: YamlNode<R>, value: YamlNode<R>) {
        match raw_key.as_str() {
            "on" => self.on(value),
            "jobs" => self.jobs(value),
            "name" => self.name(value),
            "run_name" => self.run_name(value),
            _ => self.annotate(Annotation::warn(
                key.pos().into(),
                format!("unknown key {raw_key}").as_str(),
            )),
        }
    }

    fn annotate(&mut self, a: Annotation) {
        self.annotations.add(a)
    }

    fn on(&mut self, n: YamlNode<R>) {
        match n.extract_map() {
            Ok(m) => {}
            Err(a) => self.annotate(a),
        }
    }
    fn name(&mut self, n: YamlNode<R>) {}
    fn run_name(&mut self, n: YamlNode<R>) {}
    fn jobs(&mut self, n: YamlNode<R>) {}
}

#[derive(Display)]
enum YamlKind {
    Map,
    Seq,
    Str,
    Bool,
    Number,
    Alias,
    Null,
}

impl<'a, R> Into<YamlKind> for &'a YamlNode<R>
where
    R: Repr,
{
    fn into(self) -> YamlKind {
        use yaml_peg::Yaml::*;

        match self.yaml() {
            Null => YamlKind::Null,
            Map(_) => YamlKind::Map,
            Seq(_) => YamlKind::Seq,
            Bool(_) => YamlKind::Bool,
            Int(_) | Float(_) => YamlKind::Number,
            Str(_) => YamlKind::Str,
            Alias(_) => YamlKind::Alias,
        }
    }
}

enum YamlNumber {
    Int(i64),
    Float(f64),
}

type Extraction<T> = Result<T, Annotation>;

trait Extract<'a, R>
where
    R: Repr,
{
    fn extract_map(&'a self) -> Extraction<Map<R>>;
    fn extract_str(&'a self) -> Extraction<&'a str>;
}

impl<'a, R> Extract<'a, R> for YamlNode<R>
where
    R: Repr,
{
    fn extract_map(&'a self) -> Extraction<Map<R>> {
        self.as_map()
            .map_err(|pos| expected("map", self.into(), pos.into()))
    }

    fn extract_str(&'a self) -> Extraction<&'a str> {
        self.as_str()
            .map_err(|pos| expected("seq", self.into(), pos.into()))
    }
}

fn expected<'a>(expected: &'a str, found: YamlKind, pointer: DocumentPointer) -> Annotation {
    Annotation::error(
        pointer,
        format!("expected {} but found {}", expected, found).as_str(),
    )
}
