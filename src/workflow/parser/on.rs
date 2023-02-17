use super::event::EventParser;
use crate::scavenge::ast::{PossumNodeKind, PossumSeq};
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::parser::Parser;
use crate::scavenge::yaml::YamlKind;
use crate::workflow::on::{self, EventKind};
use std::marker::PhantomData;
use std::str::FromStr;
use yaml_peg::repr::Repr;
use yaml_peg::{Map as YamlMap, Node as YamlNode, Seq as YamlSeq, Yaml};

#[derive(Default)]
pub struct OnParser<'a, R>
where
    R: Repr + 'a,
{
    _x: PhantomData<&'a R>,
}

impl<'a, R> Parser<'a, R, on::Trigger> for OnParser<'a, R>
where
    R: Repr + 'a,
{
    fn parse_node(self, root: &YamlNode<R>) -> PossumNodeKind<on::Trigger>
    where
        R: Repr,
    {
        use PossumNodeKind::{Invalid, Value};
        use YamlKind::{Map, Seq, Str};
        match YamlKind::from_yaml_node(root) {
            Map => Value(Self::configured_events(root.extract_map().unwrap()).into()),
            Seq => Value(Self::event_names(root.extract_seq().unwrap()).into()),
            Str => Value(Self::event_name(root).at(root.pos()).into()),
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
    pub fn new() -> OnParser<'a, R> {
        OnParser { _x: PhantomData }
    }

    fn configured_events(root: &YamlMap<R>) -> on::Trigger {
        root.into_iter()
            .map(|(kind, event)| {
                (
                    Self::event_kind(kind.yaml()).at(kind.pos()),
                    EventParser::new().parse_node(event).at(event.pos()),
                )
            })
            .collect()
    }

    fn event_names(root: &YamlSeq<R>) -> PossumSeq<on::EventKind> {
        root.into_iter()
            .map(|n| Self::event_name(&n).at(n.pos()))
            .collect()
    }

    fn event_name(root: &YamlNode<R>) -> PossumNodeKind<on::EventKind> {
        use PossumNodeKind::Invalid;
        use YamlKind::Str;
        match YamlKind::from_yaml_node(root) {
            Str => Self::event_kind(root.yaml()),
            _ => Invalid(
                ExpectedYaml::Only(Str)
                    .but_found(YamlKind::from_yaml_node(&root))
                    .to_string(),
            ),
        }
    }

    fn event_kind(n: &Yaml<R>) -> PossumNodeKind<on::EventKind> {
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
