use crate::scavenge::ast::{PossumNode, PossumNodeKind, PossumSeq};
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::parser::Parser;
use crate::scavenge::yaml::YamlKind;
use crate::workflow::on::{self, EventKind};
use std::marker::PhantomData;
use std::str::FromStr;
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;
use yaml_peg::Yaml;

pub(super) fn parse<'a, R>(root: YamlNode<R>) -> PossumNode<on::Trigger>
where
    R: Repr + 'a,
{
    let mut parser = OnParser::<'a, R>::new();
    parser.parse(root.yaml()).at(root.pos().into())
}

#[derive(Default)]
struct OnParser<'a, R>
where
    R: Repr + 'a,
{
    on: on::Trigger,
    _x: PhantomData<&'a R>,
}

impl<'a, R> Parser<'a, R, on::Trigger> for OnParser<'a, R>
where
    R: Repr + 'a,
{
    #[allow(unreachable_code)]
    fn parse(self, root: &Yaml<R>) -> PossumNodeKind<on::Trigger>
    where
        R: Repr,
    {
        use PossumNodeKind::{Invalid, Value};
        use YamlKind::{Map, Seq, Str};
        match YamlKind::from_yaml(root) {
            Map => Value(todo!()),
            Seq => Value(Self::events(root.extract_seq().unwrap())),
            Str => Value(Self::event(root, ExpectedYaml::Only(Str))),
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
    fn new() -> OnParser<'a, R> {
        OnParser {
            on: Default::default(),
            _x: PhantomData,
        }
    }

    fn events(root: Seq<R>) -> PossumSeq<on::EventKind> {
        root.into_iter()
            .map(|n| Self::event_kind(n.yaml()))
            .collect()
    }

    fn event(root: &Yaml<R>, expected: ExpectedYaml) -> PossumNodeKind<on::Event> {
        use PossumNodeKind::Value;
        use YamlKind::{Map, Str};
        match YamlKind::from_yaml(&root) {
            Map if expected == Map => todo!(),
            Str if expected == Str => {
                let kind = Self::event_kind(root);
                Value(on::Event::new(kind))
            }
            _ => todo!(),
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
