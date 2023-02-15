mod on;
use yaml_peg::repr::Repr;
use yaml_peg::{Map, Node as YamlNode};

use super::on::Trigger;
use super::Workflow;
use crate::document::{Annotation, Annotations};
use crate::scavenge::ast::{PossumNode, PossumNodeKind};
use crate::scavenge::extraction::Extract;
use crate::scavenge::parser::Parser;
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
    fn parse(mut self, root: YamlNode<R>) -> PossumNode<Workflow> {
        match root.extract_map() {
            Ok(m) => {
                self.parse_map(m);
                PossumNodeKind::Value(self.workflow)
            }
            Err(e) => PossumNodeKind::Invalid(e.to_string()),
        }
        .at(root.pos().into())
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
                Err(err) => self.annotate(Annotation::fatal(key.pos().into(), &err.to_string())),
            }
        }
    }

    fn visit_root_key(&mut self, raw_key: String, key: YamlNode<R>, value: YamlNode<R>) {
        // we can't currently detect repeated keys ):
        match raw_key.as_str() {
            "name" => {
                self.workflow.name = Some(self.name(value));
            }
            "run_name" => {
                self.workflow.run_name = Some(self.run_name(value));
            }
            "on" => {
                self.workflow.on = Some(on::parse(value));
            }
            "jobs" => {
                self.jobs(value);
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

    fn name(&mut self, n: YamlNode<R>) -> PossumNode<String> {
        match n.extract_str() {
            Ok(s) => PossumNodeKind::Value(s.to_owned()),
            Err(e) => PossumNodeKind::Invalid(e.to_string()),
        }
        .at(n.pos().into())
    }

    fn run_name(&mut self, n: YamlNode<R>) -> PossumNode<String> {
        match n.extract_str() {
            Ok(s) => PossumNodeKind::Expr(s.to_owned()),
            Err(e) => PossumNodeKind::Invalid(e.to_string()),
        }
        .at(n.pos().into())
    }

    fn jobs(&mut self, n: YamlNode<R>) {}
}