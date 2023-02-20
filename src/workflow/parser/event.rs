use super::input::InputParser;
use crate::document::Annotation;
use crate::document::Annotations;
use crate::document::AsDocumentPointer;
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::Extract;
use crate::scavenge::parser::SeqParser;
use crate::scavenge::parser::StringParser;
use crate::scavenge::parser::TransformParser;
use crate::scavenge::MapParser;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::workflow::on::{self, Globbed};
use yaml_peg::repr::Repr;
use yaml_peg::{Map as YamlMap, Node as YamlNode};

pub struct EventParser<'a> {
    annotations: &'a mut Annotations,
}

impl<'a, R> Parser<R, on::Event> for EventParser<'a>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::Event>
    where
        R: Repr,
    {
        match root.extract_map() {
            Ok(m) => PossumNodeKind::Value(self.parse_map(m)),
            Err(u) => PossumNodeKind::Invalid(u.to_string()),
        }
    }
}

impl<'a> EventParser<'a> {
    pub fn new(a: &'a mut Annotations) -> EventParser<'a> {
        EventParser { annotations: a }
    }

    fn annotate<A>(&mut self, annotation: A)
    where
        A: Into<Annotation>,
    {
        self.annotations.add(annotation);
    }

    fn parse_map<R>(&mut self, root: &YamlMap<R>) -> on::Event
    where
        R: Repr,
    {
        let mut evt = on::Event::new();
        for (key, value) in root.iter() {
            match key.extract_str() {
                Ok(s) => self.visit_event_key(&mut evt, s.to_lowercase(), value, key),
                Err(err) => self.annotate(Annotation::error(key, &err)),
            }
        }

        evt
    }

    fn visit_event_key<P, R>(
        &mut self,
        event: &mut on::Event,
        key: String,
        value: &YamlNode<R>,
        p: &P,
    ) where
        P: AsDocumentPointer,
        R: Repr,
    {
        match key.to_lowercase().as_str() {
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
                    MapParser::new(&mut InputParser::new())
                        .parse_node(value)
                        .at(value),
                );
            }
            "outputs" => {
                event.outputs = Some(
                    MapParser::new(&mut WorkflowOutputParser)
                        .parse_node(value)
                        .at(value),
                );
            }
            "secrets" => {
                event.secrets = Some(
                    MapParser::new(&mut InheritedSecretParser)
                        .parse_node(value)
                        .at(value),
                );
            }
            s => self.annotate(UnexpectedKey::at(&s.to_owned(), p)),
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
