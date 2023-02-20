mod concurrency;
mod event;
mod input;
mod job;
mod on;
mod permissions;
mod step;
use yaml_peg::repr::Repr;
use yaml_peg::{Map, Node as YamlNode};

use super::Workflow;
use crate::document::Annotations;
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::Extract;
use crate::scavenge::parser::{Parser, StringParser};
use crate::scavenge::{MapParser, UnexpectedKey};
use std::string::ToString;

pub struct WorkflowParser<'a> {
    annotations: &'a mut Annotations,
}

impl<'a, R> Parser<R, Workflow> for WorkflowParser<'a>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<Workflow> {
        match root.extract_map() {
            Ok(m) => PossumNodeKind::Value(self.parse_map(m)),
            Err(e) => PossumNodeKind::Invalid(e.to_string()),
        }
    }
}

impl<'a> WorkflowParser<'a> {
    pub fn new(a: &'a mut Annotations) -> WorkflowParser<'a> {
        WorkflowParser { annotations: a }
    }

    fn parse_map<R>(&mut self, m: &Map<R>) -> Workflow
    where
        R: Repr,
    {
        let mut wf = Workflow::default();
        for (key, value) in m.into_iter() {
            match key.extract_str() {
                Ok(s) => self.visit_root_key(s.to_lowercase(), key, value, &mut wf),
                Err(err) => self.annotations.add(err.at(key)),
            }
        }

        wf
    }

    fn visit_root_key<R>(
        &mut self,
        raw_key: String,
        key: &YamlNode<R>,
        value: &YamlNode<R>,
        workflow: &mut Workflow,
    ) where
        R: Repr,
    {
        // we can't currently detect repeated keys ):
        match raw_key.as_str() {
            "name" => {
                workflow.name = Some(StringParser.parse_node(value).at(value));
            }
            "run_name" => {
                workflow.run_name = Some(StringParser.parse_node(value).at(value));
            }
            "on" => {
                let on = on::OnParser::new(self.annotations).parse_node(value);
                workflow.on = Some(on.at(value));
            }
            "jobs" => {
                workflow.jobs = Some(
                    MapParser::new(&mut job::JobParser::new(self.annotations))
                        .parse_node(value)
                        .at(value),
                );
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
            s @ _ => self.annotations.add(UnexpectedKey::from(s).at(value)),
        }
    }
}
