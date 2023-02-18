use super::input::InputParser;
use crate::document::Annotation;
use crate::document::Annotations;
use crate::document::AsDocumentPointer;
use crate::scavenge::ast::{PossumMap, PossumNode, PossumNodeKind, PossumSeq};
use crate::scavenge::extraction::Extract;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::workflow::on::{self, Globbed};
use std::marker::PhantomData;
use yaml_peg::repr::Repr;
use yaml_peg::{Map as YamlMap, Node as YamlNode, Seq as YamlSeq};

pub struct EventParser<'a, R>
where
    R: Repr + 'a,
{
    _x: PhantomData<&'a R>,
    annotations: &'a mut Annotations,
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
    pub fn new(a: &'a mut Annotations) -> EventParser<'a, R> {
        EventParser {
            _x: PhantomData,
            annotations: a,
        }
    }

    fn annotate<A>(&mut self, annotation: A)
    where
        A: Into<Annotation>,
    {
        self.annotations.add(annotation);
    }

    fn parse_map(&mut self, root: &YamlMap<R>) -> on::Event {
        let mut evt = on::Event::new();
        for (key, value) in root.iter() {
            match key.extract_str() {
                Ok(s) => self.visit_event_key(&mut evt, s.to_lowercase(), value, key),
                Err(err) => self.annotate(Annotation::error(key, &err)),
            }
        }

        evt
    }

    fn visit_event_key<P>(&mut self, event: &mut on::Event, key: String, value: &YamlNode<R>, p: &P)
    where
        P: AsDocumentPointer,
    {
        use PossumNodeKind::{Invalid, Value};
        match key.to_lowercase().as_str() {
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
                            |out| Value(self.outputs(out)),
                        )
                        .at(value.pos()),
                );
            }
            "secrets" => {
                event.secrets = Some(
                    value
                        .extract_map()
                        .map_or_else(
                            |unexpected| Invalid(unexpected.to_string()),
                            |secrets| Value(self.secrets(secrets)),
                        )
                        .at(value.pos()),
                );
            }
            s => self.annotate(UnexpectedKey::new(&s.to_owned(), p)),
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
        use PossumNodeKind::*;
        let mut inputs = PossumMap::empty();
        for (key, value) in root.iter() {
            let k = key
                .extract_str()
                .map_or_else(|u| Invalid(u.to_string()), |s| Value(s.to_owned()))
                .at(key.pos());

            let v = InputParser::new().parse_node(value).at(value.pos());

            inputs.insert(k, v)
        }
        inputs
    }

    fn outputs(&mut self, root: &YamlMap<R>) -> PossumMap<String, on::WorkflowOutput> {
        use PossumNodeKind::*;
        let mut outputs = PossumMap::empty();
        for (key, value) in root.iter() {
            let k = key
                .extract_str()
                .map_or_else(
                    |unexpected| Invalid(unexpected.to_string()),
                    |key| Value(key.to_owned()),
                )
                .at(key.pos());

            let v = match value.extract_map() {
                Ok(m) => self.output(m),
                Err(u) => Invalid(u.to_string()),
            }
            .at(value.pos());

            outputs.insert(k, v);
        }
        outputs
    }

    fn secrets(&mut self, root: &YamlMap<R>) -> PossumMap<String, on::InheritedSecret> {
        use PossumNodeKind::*;
        let mut secrets = PossumMap::empty();
        for (key, secret) in root.iter() {
            let k = key
                .extract_str()
                .map_or_else(
                    |unexpected| Invalid(unexpected.to_string()),
                    |key| Value(key.to_owned()),
                )
                .at(key.pos());

            let v = match secret.extract_map() {
                Ok(secret) => self.secret(secret),
                Err(unexpected) => Invalid(unexpected.to_string()),
            }
            .at(secret.pos());

            secrets.insert(k, v);
        }
        secrets
    }

    fn output(&mut self, map: &YamlMap<R>) -> PossumNodeKind<on::WorkflowOutput> {
        use PossumNodeKind::*;
        let mut output = on::WorkflowOutput::default();

        for (key, value) in map.iter() {
            let v = value
                .extract_str()
                .map_or_else(
                    |unexpected| Invalid(unexpected.to_string()),
                    |v| Value(v.to_owned()),
                )
                .at(value.pos());

            match key.extract_str() {
                Ok(s) => match s.to_lowercase().as_str() {
                    "description" => {
                        output.description = Some(v);
                    }
                    "value" => {
                        output.value = Some(v);
                    }
                    s => self.annotate(UnexpectedKey::new(&s.to_owned(), key)),
                },
                Err(unexpected) => self.annotate(Annotation::fatal(key, &unexpected)),
            }
        }

        PossumNodeKind::Value(output)
    }

    fn secret(&mut self, map: &YamlMap<R>) -> PossumNodeKind<on::InheritedSecret> {
        use PossumNodeKind::*;
        let mut secret = on::InheritedSecret::default();

        for (key, value) in map.iter() {
            match key.extract_str() {
                Err(unexpected) => self.annotate(unexpected.at(key)),
                Ok(name) => match name.to_lowercase().as_str() {
                    "description" => {
                        let description = match value.extract_str() {
                            Ok(s) => Value(s.to_owned()),
                            Err(u) => Invalid(u.to_string()),
                        }
                        .at(value.pos());

                        secret.description = Some(description);
                    }
                    "required" => {
                        let required = match value.extract_bool() {
                            Ok(b) => Value(b.clone()),
                            Err(u) => Invalid(u.to_string()),
                        }
                        .at(value.pos());

                        secret.required = Some(required);
                    }
                    s => panic!("unexpected key {s}"),
                },
            }
        }

        Value(secret)
    }
}
