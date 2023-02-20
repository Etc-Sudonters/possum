use super::input::InputParser;
use crate::document::Annotations;
use crate::document::AsDocumentPointer;
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::Extract;
use crate::scavenge::parser::Builder;
use crate::scavenge::parser::ObjectParser;
use crate::scavenge::parser::SeqParser;
use crate::scavenge::parser::StringParser;
use crate::scavenge::parser::TransformParser;
use crate::scavenge::MapParser;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::workflow::on::{self, Globbed};
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

pub struct EventParser<'a>(&'a mut Annotations);

impl<'a> EventParser<'a> {
    pub fn new(annotations: &'a mut Annotations) -> EventParser<'a> {
        EventParser(annotations)
    }
}

struct EventBuilder;

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
        event: &mut on::Event,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer,
        R: Repr,
    {
        match key {
            "branches" => {
                event.branches = Some(
                    SeqParser::new(&mut TransformParser::new(&mut StringParser, &Globbed::new))
                        .parse_node(value)
                        .at(value),
                );
            }

            "branches-ignore" => {
                event.branches_ignore = Some(
                    SeqParser::new(&mut TransformParser::new(&mut StringParser, &Globbed::new))
                        .parse_node(value)
                        .at(value),
                );
            }
            "paths" => {
                event.paths = Some(
                    SeqParser::new(&mut TransformParser::new(&mut StringParser, &Globbed::new))
                        .parse_node(value)
                        .at(value),
                );
            }
            "paths-ignore" => {
                event.paths_ignore = Some(
                    SeqParser::new(&mut TransformParser::new(&mut StringParser, &Globbed::new))
                        .parse_node(value)
                        .at(value),
                );
            }
            "tags" => {
                event.tags = Some(
                    SeqParser::new(&mut TransformParser::new(&mut StringParser, &Globbed::new))
                        .parse_node(value)
                        .at(value),
                );
            }
            "tags-ignore" => {
                event.tags_ignore = Some(
                    SeqParser::new(&mut TransformParser::new(&mut StringParser, &Globbed::new))
                        .parse_node(value)
                        .at(value),
                );
            }
            "inputs" => {
                event.inputs = Some(
                    MapParser::new(&mut StringParser, &mut InputParser::new())
                        .parse_node(value)
                        .at(value),
                );
            }
            "outputs" => {
                event.outputs = Some(
                    MapParser::new(&mut StringParser, &mut WorkflowOutputParser)
                        .parse_node(value)
                        .at(value),
                );
            }
            "secrets" => {
                event.secrets = Some(
                    MapParser::new(&mut StringParser, &mut InheritedSecretParser)
                        .parse_node(value)
                        .at(value),
                );
            }
            s => annotations.add(UnexpectedKey::from(s).at(value)),
        }
    }
}

struct InheritedSecretParser;
struct WorkflowOutputParser;

impl<R> Parser<R, on::InheritedSecret> for InheritedSecretParser
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::InheritedSecret>
    where
        R: Repr,
    {
        match root.extract_map() {
            Err(u) => PossumNodeKind::Invalid(u.to_string()),
            Ok(m) => {
                todo!()
            }
        }
    }
}

impl<R> Parser<R, on::WorkflowOutput> for WorkflowOutputParser
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::WorkflowOutput>
    where
        R: Repr,
    {
        match root.extract_map() {
            Err(u) => PossumNodeKind::Invalid(u.to_string()),
            Ok(m) => {
                todo!()
            }
        }
    }
}
