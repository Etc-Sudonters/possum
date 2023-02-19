use super::event::EventParser;
use crate::document::Annotations;
use crate::scavenge::ast::{PossumNodeKind, PossumSeq};
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::parser::Parser;
use crate::scavenge::yaml::YamlKind;
use crate::workflow::on::{self, EventKind};
use std::marker::PhantomData;
use std::str::FromStr;
use yaml_peg::repr::Repr;
use yaml_peg::{Map as YamlMap, Node as YamlNode, Seq as YamlSeq, Yaml};

pub struct OnParser<'a, R>
where
    R: Repr + 'a,
{
    _x: PhantomData<&'a R>,
    annotations: &'a mut Annotations,
}

impl<'a, R> Parser<'a, R, on::Trigger> for OnParser<'a, R>
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::Trigger>
    where
        R: Repr,
    {
        use PossumNodeKind::{Invalid, Value};
        use YamlKind::{Map, Seq, Str};
        match root.yaml() {
            Yaml::Map(m) => Value(self.configured_events(m)),
            Yaml::Seq(s) => Value(Self::event_names(s).into()),
            Yaml::Str(_) => Value(Self::event_name(root).at(root).into()),
            n @ _ => Invalid(
                ExpectedYaml::AnyOf(vec![Map, Seq, Str])
                    .but_found(n)
                    .to_string(),
            ),
        }
    }
}

impl<'a, R> OnParser<'a, R>
where
    R: Repr + 'a,
{
    pub fn new(a: &'a mut Annotations) -> OnParser<'a, R> {
        OnParser {
            _x: PhantomData,
            annotations: a,
        }
    }

    fn configured_events(&mut self, root: &YamlMap<R>) -> on::Trigger {
        root.into_iter()
            .map(|(kind, event)| {
                (
                    Self::event_name(kind).at(kind),
                    EventParser::new(self.annotations)
                        .parse_node(event)
                        .at(event),
                )
            })
            .collect()
    }

    fn event_names(root: &YamlSeq<R>) -> PossumSeq<on::EventKind> {
        root.into_iter()
            .map(|n| Self::event_name(&n).at(n))
            .collect()
    }

    fn event_name(n: &YamlNode<R>) -> PossumNodeKind<on::EventKind> {
        use on::BadEvent::Unknown;
        use PossumNodeKind::{Invalid, Value};
        match n.extract_str() {
            Ok(s) => match EventKind::from_str(s) {
                Ok(ek) => Value(ek),
                Err(_) => Invalid(Unknown(s.to_owned()).to_string()),
            },
            Err(n) => Invalid(n.to_string()),
        }
    }
}
