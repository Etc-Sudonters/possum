use yaml_peg::repr::Repr;
use yaml_peg::{Map, Node as YamlNode, Yaml};

use super::on::{self, EventKind};
use super::Workflow;
use crate::document::{Annotation, Annotations};
use crate::scavenge::ast::{PossumNode, PossumNodeKind};
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::parser::Parser;
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

#[derive(Default)]
struct OnParser<'a, R>
where
    R: Repr + 'a,
{
    on: on::Trigger,
    _x: PhantomData<&'a R>,
}

impl<'a, R> Parser<'a, R, on::Trigger> for OnParser<'a, R>
where
    R: Repr + 'a,
{
    #[allow(unreachable_code)]
    fn parse(self, root: YamlNode<R>) -> PossumNode<on::Trigger>
    where
        R: Repr,
    {
        use PossumNodeKind::{Invalid, Value};
        use YamlKind::{Map, Seq, Str};
        match YamlKind::from_yaml_node(root.yaml()) {
            Map => Value(todo!()),
            Seq => Value(todo!()),
            Str => Value(todo!()),
            n @ _ => Invalid(
                ExpectedYaml::AnyOf(vec![Map, Seq, Str])
                    .but_found(n)
                    .to_string(),
            ),
        }
        .at(root.pos().into())
    }
}

impl<'a, R> OnParser<'a, R> where R: Repr + 'a {}

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
                self.workflow.on = Some(PossumNode::new(key.pos().into(), self.on(value)));
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

    fn event(value: YamlNode<R>, expect: YamlKind) -> PossumNode<on::Event> {
        match value.yaml() {
            Yaml::Map(m) if expect == YamlKind::Map => {
                todo!()
            }
            Yaml::Str(s) if expect == YamlKind::Str => PossumNodeKind::Value(on::Event::new(
                PossumNode::new(value.pos().into(), Self::event_name(s)),
            ))
            .at(value.pos().into()),
            n @ _ => {
                PossumNodeKind::Invalid(ExpectedYaml::Only(expect).but_found(n.into()).to_string())
                    .at(value.pos().into())
            }
        }
    }

    fn event_name(value: &String) -> PossumNodeKind<EventKind> {
        match EventKind::what_to_name(&value) {
            Ok(ek) => PossumNodeKind::Value(ek),
            Err(be) => PossumNodeKind::Invalid(be.to_string()),
        }
    }

    fn jobs(&mut self, n: YamlNode<R>) {}
}
