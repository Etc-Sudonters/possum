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
use crate::scavenge::parser::{Builder, ObjectParser, Parser, StringParser};
use crate::scavenge::{MapParser, UnexpectedKey};

pub struct WorkflowParser<'a> {
    annotations: &'a mut Annotations,
}

impl<'a, R> Parser<R, Workflow> for WorkflowParser<'a>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<Workflow> {
        ObjectParser::new(WorkflowBuilder, Workflow::default, self.annotations).parse_node(root)
    }
}

struct WorkflowBuilder;

impl Builder<Workflow> for WorkflowBuilder {
    fn build<'a, P, R>(
        &mut self,
        item: &mut Workflow,
        key: &str,
        value: &YamlNode<R>,
        _: &P,
        annotations: &'a mut Annotations,
    ) where
        P: crate::document::AsDocumentPointer,
        R: Repr,
    {
        match key {
            "name" => {
                item.name = Some(StringParser.parse_node(value).at(value));
            }
            "run_name" => {
                item.run_name = Some(StringParser.parse_node(value).at(value));
            }
            "on" => {
                let on = on::OnParser::new(annotations).parse_node(value);
                item.on = Some(on.at(value));
            }
            "jobs" => {
                item.jobs = Some(
                    MapParser::new(&mut StringParser, &mut job::JobParser::new(annotations))
                        .parse_node(value)
                        .at(value),
                );
            }
            "permissions" => {
                item.permissions = Some(
                    permissions::PermissionParser::new(annotations)
                        .parse_node(value)
                        .at(value),
                );
            }
            "concurrency" => {
                item.concurrency = Some(
                    concurrency::ConcurrencyParser::new(annotations)
                        .parse_node(value)
                        .at(value),
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
