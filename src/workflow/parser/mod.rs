mod concurrency;
mod event;
mod input;
mod job;
mod on;
mod permissions;
mod step;
use yaml_peg::repr::Repr;
use yaml_peg::{Map, Node as YamlNode};

use super::job::Job;
use super::Workflow;
use crate::document::{Annotation, Annotations};
use crate::scavenge::ast::{PossumMap, PossumNode, PossumNodeKind};
use crate::scavenge::extraction::Extract;
use crate::scavenge::parser::Parser;
use crate::scavenge::UnexpectedKey;
use std::marker::PhantomData;
use std::string::ToString;

pub struct WorkflowParser<'a, R>
where
    R: Repr + 'a,
{
    _x: PhantomData<R>,
    annotations: &'a mut Annotations,
}

impl<'a, R> Parser<'a, R, Workflow> for WorkflowParser<'a, R>
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<Workflow> {
        match root.extract_map() {
            Ok(m) => PossumNodeKind::Value(self.parse_map(m)),
            Err(e) => PossumNodeKind::Invalid(e.to_string()),
        }
    }
}

impl<'a, R> WorkflowParser<'a, R>
where
    R: Repr + 'a,
{
    pub fn new(a: &'a mut Annotations) -> WorkflowParser<'a, R> {
        WorkflowParser {
            annotations: a,
            _x: PhantomData,
        }
    }

    fn parse_map(&mut self, m: &Map<R>) -> Workflow {
        let mut wf = Workflow::default();
        for (key, value) in m.into_iter() {
            match key.extract_str() {
                Ok(s) => self.visit_root_key(s.to_lowercase(), key, value, &mut wf),
                Err(err) => self.annotate(err.at(key)),
            }
        }

        wf
    }

    fn visit_root_key(
        &mut self,
        raw_key: String,
        key: &YamlNode<R>,
        value: &YamlNode<R>,
        workflow: &mut Workflow,
    ) {
        // we can't currently detect repeated keys ):
        match raw_key.as_str() {
            "name" => {
                workflow.name = Some(self.name(value));
            }
            "run_name" => {
                workflow.run_name = Some(self.run_name(value));
            }
            "on" => {
                let on = on::OnParser::new(self.annotations).parse_node(value);
                workflow.on = Some(on.at(value));
            }
            "jobs" => {
                workflow.jobs = Some(self.jobs(value));
            }
            "permissions" => {
                workflow.permissions = Some(
                    permissions::PermissionParser::new(self.annotations)
                        .parse_node(value)
                        .at(value),
                );
            }
            "concurrency" => {
                workflow.concurrency = Some(
                    concurrency::ConcurrencyParser::new(self.annotations)
                        .parse_node(value)
                        .at(value),
                );
            }
            s => self.annotate(UnexpectedKey::at(&s.to_owned(), value)),
        }
    }

    fn annotate<A>(&mut self, a: A)
    where
        A: Into<Annotation>,
    {
        self.annotations.add(a.into())
    }

    fn name(&mut self, n: &YamlNode<R>) -> PossumNode<String> {
        match n.extract_str() {
            Ok(s) => PossumNodeKind::Value(s.to_owned()),
            Err(e) => PossumNodeKind::Invalid(e.to_string()),
        }
        .at(n)
    }

    fn run_name(&mut self, n: &YamlNode<R>) -> PossumNode<String> {
        match n.extract_str() {
            Ok(s) => PossumNodeKind::Expr(s.to_owned()),
            Err(e) => PossumNodeKind::Invalid(e.to_string()),
        }
        .at(n)
    }

    fn jobs(&mut self, n: &YamlNode<R>) -> PossumNode<PossumMap<String, Job>> {
        use PossumNodeKind::*;
        match n.extract_map() {
            Ok(root) => {
                let mut jobs = PossumMap::empty();
                for (name, job) in root.iter() {
                    let k = match name.extract_str() {
                        Ok(s) => Value(s.to_owned()),
                        Err(u) => Invalid(u.to_string()),
                    }
                    .at(name);

                    let job = job::JobParser::new(self.annotations)
                        .parse_node(job)
                        .at(job);

                    jobs.insert(k, job);
                }

                Value(jobs)
            }
            Err(u) => PossumNodeKind::Invalid(u.to_string()),
        }
        .at(n)
    }
}
