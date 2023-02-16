use crate::scavenge::ast::{PossumNode, PossumNodeKind, PossumSeq};
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
    event: on::Event,
    _x: PhantomData<&'a R>,
}

impl<'a, R> EventParser<'a, R>
where
    R: Repr + 'a,
{
    pub fn new() -> EventParser<'a, R> {
        EventParser {
            event: on::Event::default(),
            _x: PhantomData,
        }
    }

    fn parse_map(&mut self, root: &YamlMap<R>) {
        for (key, value) in root.iter() {
            match key.extract_str() {
                Ok(s) => self.visit_event_key(s.to_lowercase(), value),
                Err(err) => todo!(),
            }
        }
    }

    fn visit_event_key(&mut self, key: String, value: &YamlNode<R>) {
        match key.as_str() {
            "branches" => {
                self.event.branches = Some(get_globbed_paths(value));
            }
            "branches-ignore" => {
                self.event.branches_ignore = Some(get_globbed_paths(value));
            }
            "paths" => {
                self.event.paths = Some(get_globbed_paths(value));
            }
            "paths-ignore" => {
                self.event.paths_ignore = Some(get_globbed_paths(value));
            }
            "tags" => {
                self.event.tags = Some(get_globbed_paths(value));
            }
            "tags-ignore" => {
                self.event.tags_ignore = Some(get_globbed_paths(value));
            }
            "inputs" => {}
            "outputs" => {}
            "secrets" => {}
            _ => {}
        }

        fn get_globbed_paths<'a, R>(root: &YamlNode<R>) -> PossumNode<PossumSeq<Globbed>>
        where
            R: Repr + 'a,
        {
            use PossumNodeKind::{Invalid, Value};
            match root.extract_seq() {
                Ok(seq) => Value(EventParser::globbed_paths(seq)),
                Err(u) => Invalid(u.to_string()),
            }
            .at(root.pos().into())
        }
    }

    fn globbed_paths(root: &YamlSeq<R>) -> PossumSeq<Globbed> {
        root.into_iter()
            .map(|n| {
                match n.extract_str() {
                    Ok(s) => PossumNodeKind::Value(Globbed::new(s)),
                    Err(u) => PossumNodeKind::Invalid(u.to_string()),
                }
                .at(n.pos().into())
            })
            .collect()
    }
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
            Ok(m) => {
                self.parse_map(m);
                PossumNodeKind::Value(self.event)
            }
            Err(u) => PossumNodeKind::Invalid(u.to_string()),
        }
    }
}
