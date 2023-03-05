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
use crate::scavenge::parsers::TransformParser;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::workflow::on::{self, Globbed};
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

pub struct EventParser<'a>(&'a mut Annotations);
struct EventBuilder;
struct InheritedSecretParser<'a>(&'a mut Annotations);
struct InheritedSecretBuilder;
struct WorkflowOutputParser<'a>(&'a mut Annotations);
struct WorkflowOutputBuilder;

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
        ObjectParser::new(EventBuilder, on::Event::default, &mut self.0).parse_node(root)
    }
}

impl Builder<on::Event> for EventBuilder {
    fn build<'a, P, R>(
        &mut self,
        item: &mut on::Event,
        key: &str,
        value: &YamlNode<R>,
        _: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer,
        R: Repr,
    {
        match key {
            "branches" => {
                item.branches = Some(
                    SeqParser::new(TransformParser::new(StringParser, Globbed::new))
                        .parse_node(value)
                        .at(value),
                );
            }

            "branches-ignore" => {
                item.branches_ignore = Some(
                    SeqParser::new(TransformParser::new(StringParser, Globbed::new))
                        .parse_node(value)
                        .at(value),
                );
            }
            "paths" => {
                item.paths = Some(
                    SeqParser::new(TransformParser::new(StringParser, Globbed::new))
                        .parse_node(value)
                        .at(value),
                );
            }
            "paths-ignore" => {
                item.paths_ignore = Some(
                    SeqParser::new(TransformParser::new(StringParser, Globbed::new))
                        .parse_node(value)
                        .at(value),
                );
            }
            "tags" => {
                item.tags = Some(
                    SeqParser::new(TransformParser::new(StringParser, Globbed::new))
                        .parse_node(value)
                        .at(value),
                );
            }
            "tags-ignore" => {
                item.tags_ignore = Some(
                    SeqParser::new(TransformParser::new(StringParser, Globbed::new))
                        .parse_node(value)
                        .at(value),
                );
            }
            "inputs" => {
                item.inputs = Some(
                    MapParser::new(StringParser, InputParser::new(annotations))
                        .parse_node(value)
                        .at(value),
                );
            }
            "outputs" => {
                item.outputs = Some(
                    MapParser::new(StringParser, WorkflowOutputParser(annotations))
                        .parse_node(value)
                        .at(value),
                );
            }
            "secrets" => {
                item.secrets = Some(
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
        ObjectParser::new(
            InheritedSecretBuilder,
            &on::InheritedSecret::default,
            self.0,
        )
        .parse_node(root)
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
        ObjectParser::new(WorkflowOutputBuilder, &on::WorkflowOutput::default, self.0)
            .parse_node(root)
    }
}

impl Builder<on::InheritedSecret> for InheritedSecretBuilder {
    fn build<'a, P, R>(
        &mut self,
        item: &mut on::InheritedSecret,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer,
        R: Repr,
    {
        match key {
            "description" => {
                item.description = Some(StringParser.parse_node(value).at(value));
            }
            "required" => {
                item.required = Some(BoolParser.parse_node(value).at(value));
            }
            unexpected @ _ => annotations.add(UnexpectedKey::from(unexpected).at(pointer)),
        }
    }
}

impl Builder<on::WorkflowOutput> for WorkflowOutputBuilder {
    fn build<'a, P, R>(
        &mut self,
        item: &mut on::WorkflowOutput,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer,
        R: Repr,
    {
        match key {
            "description" => {
                item.description = Some(StringParser.parse_node(value).at(value));
            }
            "value" => {
                item.value = Some(StringParser.parse_node(value).at(value));
            }
            unexpected @ _ => {
                annotations.add(UnexpectedKey::from(unexpected).at(pointer));
            }
        }
    }
}
