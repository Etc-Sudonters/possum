use crate::scavenge::ast::{PossumMap, PossumNode, PossumNodeKind, PossumSeq};
use crate::scavenge::extraction::Extract;
use crate::scavenge::Parser;
use crate::workflow::on::{self, Globbed};
use std::marker::PhantomData;
use yaml_peg::repr::Repr;
use yaml_peg::{Map as YamlMap, Node as YamlNode, Seq as YamlSeq};

pub struct EventParser<'a, R>
where
    R: Repr + 'a,
{
    _x: PhantomData<&'a R>,
}

impl<'a, R> Parser<'a, R, on::Event> for EventParser<'a, R>
where
    R: Repr + 'a,
{
    fn parse_node(mut self, root: &YamlNode<R>) -> PossumNodeKind<on::Event>
    where
        R: Repr,
    {
        match root.extract_map() {
            Ok(m) => PossumNodeKind::Value(self.parse_map(m)),
            Err(u) => PossumNodeKind::Invalid(u.to_string()),
        }
    }
}

impl<'a, R> EventParser<'a, R>
where
    R: Repr + 'a,
{
    pub fn new() -> EventParser<'a, R> {
        EventParser { _x: PhantomData }
    }

    fn parse_map(&mut self, root: &YamlMap<R>) -> on::Event {
        let mut evt = on::Event::new();
        for (key, value) in root.iter() {
            match key.extract_str() {
                Ok(s) => self.visit_event_key(&mut evt, s.to_lowercase(), value),
                Err(err) => todo!(),
            }
        }

        evt
    }

    fn visit_event_key(&mut self, event: &mut on::Event, key: String, value: &YamlNode<R>) {
        use PossumNodeKind::{Invalid, Value};
        match key.as_str() {
            "branches" => {
                event.branches = Some(get_globbed_paths(value));
            }
            "branches-ignore" => {
                event.branches_ignore = Some(get_globbed_paths(value));
            }
            "paths" => {
                event.paths = Some(get_globbed_paths(value));
            }
            "paths-ignore" => {
                event.paths_ignore = Some(get_globbed_paths(value));
            }
            "tags" => {
                event.tags = Some(get_globbed_paths(value));
            }
            "tags-ignore" => {
                event.tags_ignore = Some(get_globbed_paths(value));
            }
            "inputs" => {
                event.inputs = Some(
                    value
                        .extract_map()
                        .map_or_else(
                            |err| Invalid(err.to_string()),
                            |inputs| Value(Self::inputs(inputs)),
                        )
                        .at(value.pos()),
                );
            }
            "outputs" => {
                event.outputs = Some(
                    value
                        .extract_map()
                        .map_or_else(
                            |err| Invalid(err.to_string()),
                            |out| Value(Self::outputs(out)),
                        )
                        .at(value.pos()),
                );
            }
            "secrets" => {}
            _ => {}
        }

        fn get_globbed_paths<'a, R>(root: &YamlNode<R>) -> PossumNode<PossumSeq<Globbed>>
        where
            R: Repr + 'a,
        {
            match root.extract_seq() {
                Ok(seq) => Value(EventParser::globbed_paths(seq)),
                Err(u) => Invalid(u.to_string()),
            }
            .at(root.pos())
        }
    }

    fn globbed_paths(root: &YamlSeq<R>) -> PossumSeq<Globbed> {
        root.into_iter()
            .map(|n| {
                match n.extract_str() {
                    Ok(s) => PossumNodeKind::Value(Globbed::new(s)),
                    Err(u) => PossumNodeKind::Invalid(u.to_string()),
                }
                .at(n.pos())
            })
            .collect()
    }

    fn inputs(root: &YamlMap<R>) -> PossumMap<String, on::WorkflowInput> {
        todo!()
    }

    fn outputs(root: &YamlMap<R>) -> PossumMap<String, on::WorkflowOutput> {
        todo!()
    }

    fn secrets(root: &YamlMap<R>) -> PossumNode<on::InheritedSecret> {
        todo!()
    }
}
