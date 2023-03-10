use super::input::InputParser;
use crate::document::Annotations;
use crate::document::AsDocumentPointer;
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::parsers::BoolParser;
use crate::scavenge::parsers::Builder;
use crate::scavenge::parsers::MapParser;
use crate::scavenge::parsers::ObjectParser;
use crate::scavenge::parsers::SeqParser;
use crate::scavenge::parsers::StringParser;
use crate::scavenge::parsers::TransformableParser;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::workflow::on::{self, Globbed};
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

pub struct EventParser<'a>(&'a mut Annotations);
struct InheritedSecretParser<'a>(&'a mut Annotations);
struct WorkflowOutputParser<'a>(&'a mut Annotations);

impl<'a> EventParser<'a> {
    pub fn new(annotations: &'a mut Annotations) -> EventParser<'a> {
        EventParser(annotations)
    }
}

impl<'a, R> Parser<R, on::Event> for EventParser<'a>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::Event>
    where
        R: Repr,
    {
        ObjectParser::new(EventBuilder::default, &mut self.0).parse_node(root)
    }
}

#[derive(Default)]
struct EventBuilder {
    event: on::Event,
}

impl Into<on::Event> for EventBuilder {
    fn into(self) -> on::Event {
        self.event
    }
}

impl Builder<on::Event> for EventBuilder {
    fn build<'a, P, R>(
        &mut self,
        key: &str,
        value: &YamlNode<R>,
        _: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer + 'a,
        R: Repr,
    {
        let mut glob_parser = SeqParser::new(StringParser.to(Globbed::new));
        match key {
            "branches" => {
                self.event.branches = Some(glob_parser.parse_node(value).at(value));
            }

            "branches-ignore" => {
                self.event.branches_ignore = Some(glob_parser.parse_node(value).at(value));
            }
            "paths" => {
                self.event.paths = Some(glob_parser.parse_node(value).at(value));
            }
            "paths-ignore" => {
                self.event.paths_ignore = Some(glob_parser.parse_node(value).at(value));
            }
            "tags" => {
                self.event.tags = Some(glob_parser.parse_node(value).at(value));
            }
            "tags-ignore" => {
                self.event.tags_ignore = Some(glob_parser.parse_node(value).at(value));
            }
            "inputs" => {
                self.event.inputs = Some(
                    MapParser::new(StringParser, InputParser::new(annotations))
                        .parse_node(value)
                        .at(value),
                );
            }
            "outputs" => {
                self.event.outputs = Some(
                    MapParser::new(StringParser, WorkflowOutputParser(annotations))
                        .parse_node(value)
                        .at(value),
                );
            }
            "secrets" => {
                self.event.secrets = Some(
                    MapParser::new(StringParser, InheritedSecretParser(annotations))
                        .parse_node(value)
                        .at(value),
                );
            }
            s => annotations.add(UnexpectedKey::from(s).at(value)),
        }
    }
}

impl<'a, R> Parser<R, on::InheritedSecret> for InheritedSecretParser<'a>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::InheritedSecret>
    where
        R: Repr,
    {
        ObjectParser::new(InheritedSecretBuilder::default, self.0).parse_node(root)
    }
}

impl<'a, R> Parser<R, on::WorkflowOutput> for WorkflowOutputParser<'a>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::WorkflowOutput>
    where
        R: Repr,
    {
        ObjectParser::new(WorkflowOutputBuilder::default, self.0).parse_node(root)
    }
}

#[derive(Default)]
struct InheritedSecretBuilder {
    secret: on::InheritedSecret,
}

impl Into<on::InheritedSecret> for InheritedSecretBuilder {
    fn into(self) -> on::InheritedSecret {
        self.secret
    }
}

impl Builder<on::InheritedSecret> for InheritedSecretBuilder {
    fn build<'a, P, R>(
        &mut self,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer + 'a,
        R: Repr,
    {
        match key {
            "description" => {
                self.secret.description = Some(StringParser.parse_node(value).at(value));
            }
            "required" => {
                self.secret.required = Some(BoolParser.parse_node(value).at(value));
            }
            unexpected @ _ => annotations.add(UnexpectedKey::from(unexpected).at(pointer)),
        }
    }
}

#[derive(Default)]
struct WorkflowOutputBuilder {
    output: on::WorkflowOutput,
}

impl Into<on::WorkflowOutput> for WorkflowOutputBuilder {
    fn into(self) -> on::WorkflowOutput {
        self.output
    }
}
impl Builder<on::WorkflowOutput> for WorkflowOutputBuilder {
    fn build<'a, P, R>(
        &mut self,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer + 'a,
        R: Repr,
    {
        match key {
            "description" => {
                self.output.description = Some(StringParser.parse_node(value).at(value));
            }
            "value" => {
                self.output.value = Some(StringParser.parse_node(value).at(value));
            }
            unexpected @ _ => {
                annotations.add(UnexpectedKey::from(unexpected).at(pointer));
            }
        }
    }
}
