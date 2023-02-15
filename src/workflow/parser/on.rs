use crate::scavenge::ast::{PossumNode, PossumNodeKind, PossumSeq};
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::parser::Parser;
use crate::scavenge::yaml::YamlKind;
use crate::workflow::on::{self, EventKind};
use std::marker::PhantomData;
use std::str::FromStr;
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

pub(super) fn parse<'a, R>(root: YamlNode<R>) -> PossumNode<on::Trigger>
where
    R: Repr + 'a,
{
    let mut parser = OnParser::<'a, R>::new();
    parser.parse(root)
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
    fn parse(self, root: YamlNode<R>) -> PossumNode<on::Trigger>
    where
        R: Repr,
    {
        use PossumNodeKind::{Invalid, Value};
        use YamlKind::{Map, Seq, Str};
        let loc = root.pos().into();
        match YamlKind::from_yaml(root.yaml()) {
            Map => Value(todo!()),
            Seq => Value(Self::events(root).into()),
            Str => Value(Self::event(root, ExpectedYaml::Only(Str)).into()),
            n @ _ => Invalid(
                ExpectedYaml::AnyOf(vec![Map, Seq, Str])
                    .but_found(n)
                    .to_string(),
            ),
        }
        .at(loc)
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

    fn events(root: YamlNode<R>) -> PossumNode<PossumSeq<on::Event>> {
        use PossumNodeKind::{Invalid, Value};
        let loc = root.pos().into();
        match root.extract_seq() {
            Ok(kinds) => Value(kinds.into_iter().map(Self::event_kind).collect()),
            Err(unexpected) => Invalid(unexpected.to_string()),
        }
        .at(loc)
    }

    fn event(root: YamlNode<R>, expected: ExpectedYaml) -> PossumNode<on::Event> {
        use YamlKind::{Map, Str};
        let loc = root.pos().into();
        match YamlKind::from_yaml_node(&root) {
            Map if expected == Map => todo!(),
            Str if expected == Str => {
                let kind = Self::event_kind(root);
                let evt = on::Event::new(kind);
                PossumNodeKind::Value(evt).at(loc)
            }
            _ => todo!(),
        }
    }

    fn event_kind(n: YamlNode<R>) -> PossumNode<on::EventKind> {
        use on::BadEvent::Unknown;
        use PossumNodeKind::{Invalid, Value};
        match n.extract_str() {
            Ok(s) => match EventKind::from_str(s) {
                Ok(ek) => Value(ek),
                Err(_) => Invalid(Unknown(s.to_owned()).to_string()),
            },
            Err(n) => Invalid(n.to_string()),
        }
        .at(n.pos().into())
    }
}
