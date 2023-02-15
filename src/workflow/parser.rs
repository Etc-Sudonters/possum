use yaml_peg::repr::Repr;
use yaml_peg::{Map, Node as YamlNode, Yaml};

use super::on::{self, EventKind};
use super::Workflow;
use crate::document::{Annotation, Annotations};
use crate::scavenge::ast::{PossumNode, PossumNodeKind};
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::parser::{ParseFailure, Parser};
use crate::scavenge::yaml::YamlKind;
use std::marker::PhantomData;
use std::string::ToString;

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
    R: Repr + 'a,
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
                Err(a) => self.annotate(Annotation::error(key.pos().into(), &a.to_string())),
            }
        }
    }

    fn visit_root_key(&mut self, raw_key: String, key: YamlNode<R>, value: YamlNode<R>) {
        match raw_key.as_str() {
            "on" => self.workflow.on = Some(PossumNode::new(key.pos().into(), self.on(value))),
            "jobs" => {
                self.jobs(value);
            }
            "name" => {
                self.name(value);
            }
            "run_name" => {
                self.run_name(value);
            }
            _ => {
                self.annotate(Annotation::warn(
                    key.pos().into(),
                    format!("unknown key {raw_key}").as_str(),
                ));
            }
        }
    }

    fn annotate(&mut self, a: Annotation) {
        self.annotations.add(a)
    }

    fn on(&mut self, value: YamlNode<R>) -> PossumNodeKind<on::Trigger> {
        match value.yaml() {
            Yaml::Map(m) => PossumNodeKind::Value(Default::default()),
            Yaml::Seq(seq) => PossumNodeKind::Value(
                seq.into_iter()
                    .map(|n| Self::event(n, YamlKind::Str))
                    .collect(),
            ),
            Yaml::Str(s) => PossumNodeKind::Value(Self::event(&value, YamlKind::Str).into()),
            y @ _ => PossumNodeKind::Invalid(
                ExpectedYaml::AnyOf(vec![YamlKind::Map, YamlKind::Seq, YamlKind::Str])
                    .but_found(y)
                    .to_string(),
            ),
        }
    }

    fn event(value: &YamlNode<R>, expect: YamlKind) -> PossumNode<on::Event> {
        match value.yaml() {
            Yaml::Map(m) if expect == YamlKind::Map => {
                todo!()
            }
            Yaml::Str(s) if expect == YamlKind::Str => PossumNode::new(
                value.pos().into(),
                PossumNodeKind::Value(on::Event::new(PossumNode::new(
                    value.pos().into(),
                    Self::event_name(s),
                ))),
            ),
            n @ _ => PossumNode::new(
                value.pos().into(),
                PossumNodeKind::Invalid(ExpectedYaml::Only(expect).but_found(n).to_string()),
            ),
        }
    }

    fn event_name(value: &String) -> PossumNodeKind<EventKind> {
        match EventKind::what_to_name(&value) {
            Ok(ek) => PossumNodeKind::Value(ek),
            Err(be) => PossumNodeKind::Invalid(be.to_string()),
        }
    }

    fn name(&mut self, n: YamlNode<R>) {}
    fn run_name(&mut self, n: YamlNode<R>) {}
    fn jobs(&mut self, n: YamlNode<R>) {}
}
