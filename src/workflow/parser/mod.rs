mod concurrency;
mod event;
mod input;
mod job;
mod on;
mod permissions;
mod step;
mod strategy;
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

use super::Workflow;
use crate::document::Annotations;
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::parsers::{Builder, MapParser, ObjectParser, StringParser, StringMapParser};
use crate::scavenge::{Parser, UnexpectedKey};

pub struct WorkflowParser<'a> {
    annotations: &'a mut Annotations,
}

impl<'a, R> Parser<R, Workflow> for WorkflowParser<'a>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<Workflow> {
        ObjectParser::new(WorkflowBuilder::new, self.annotations).parse_node(root)
    }
}

struct WorkflowBuilder {
    workflow: Workflow
}

impl WorkflowBuilder {
    fn new() -> WorkflowBuilder {
        WorkflowBuilder { workflow: Workflow::default() }
    }
}

impl Into<Workflow> for WorkflowBuilder {
    fn into(self) -> Workflow {
       self.workflow
    }
}

impl Builder<Workflow> for WorkflowBuilder {
    fn build<'a, P, R>(
        &mut self,
        key: &str,
        value: &YamlNode<R>,
        _: &P,
        annotations: &'a mut Annotations,
    ) where
        P: crate::document::AsDocumentPointer + 'a,
        R: Repr,
    {
        match key {
            "name" => {
                self.workflow.name = Some(StringParser.parse_node(value).at(value));
            }
            "run_name" => {
                self.workflow.run_name = Some(StringParser.parse_node(value).at(value));
            }
            "on" => {
                let on = on::OnParser::new(annotations).parse_node(value);
                self.workflow.on = Some(on.at(value));
            }
            "jobs" => {
                self.workflow.jobs = Some(
                    MapParser::new(StringParser, job::JobParser::new(annotations))
                        .parse_node(value)
                        .at(value),
                );
            }
            "permissions" => {
                self.workflow.permissions = Some(
                    permissions::PermissionParser::new(annotations)
                        .parse_node(value)
                        .at(value),
                );
            }
            "concurrency" => {
                self.workflow.concurrency = Some(
                    concurrency::ConcurrencyParser::new(annotations)
                        .parse_node(value)
                        .at(value),
                );
            }
            "env" => {
                self.workflow.env = Some(
                    StringMapParser::new().parse_node(value).at(value)
                );
            }
            s @ _ => annotations.add(UnexpectedKey::from(s).at(value)),
        }
    }
}

impl<'a> WorkflowParser<'a> {
    pub fn new(a: &'a mut Annotations) -> WorkflowParser<'a> {
        WorkflowParser { annotations: a }
    }
}
