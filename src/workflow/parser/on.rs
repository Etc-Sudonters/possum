use super::event::EventParser;
use crate::document::Annotations;
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::ExpectedYaml;
use crate::scavenge::parser::{
    FlatMapParser, OrParser, Parser, SeqParser, StringParser, TransformParser,
};
use crate::scavenge::yaml::YamlKind;
use crate::scavenge::MapParser;
use crate::workflow::on::{self, BadEvent, EventKind};
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

pub struct OnParser<'a>(&'a mut Annotations);

impl<'a> OnParser<'a> {
    pub fn new(annotations: &'a mut Annotations) -> OnParser<'a> {
        OnParser(annotations)
    }
}

struct EventKindParser;
struct OnStringParser;
struct OnArrayParser;
struct OnMapParser<'a>(&'a mut Annotations);

impl<R> Parser<R, on::EventKind> for EventKindParser
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::EventKind>
    where
        R: Repr,
    {
        FlatMapParser::new(
            &mut StringParser,
            &|s| match EventKind::fromstr(s.as_str()) {
                Ok(ek) => PossumNodeKind::Value(ek),
                Err(_) => PossumNodeKind::Invalid(BadEvent::Unknown(s).to_string()),
            },
        )
        .parse_node(root)
    }
}

impl<R> Parser<R, on::Trigger> for OnStringParser
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::Trigger>
    where
        R: Repr,
    {
        PossumNodeKind::Value(EventKindParser.parse_node(root).at(root).into())
    }
}

impl<R> Parser<R, on::Trigger> for OnArrayParser
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::Trigger>
    where
        R: Repr,
    {
        TransformParser::new(
            &mut SeqParser::new(&mut EventKindParser),
            &Into::<on::Trigger>::into,
        )
        .parse_node(root)
    }
}

impl<'a, R> Parser<R, on::Trigger> for OnMapParser<'a>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::Trigger>
    where
        R: Repr,
    {
        MapParser::new(&mut EventKindParser, &mut EventParser::new(self.0))
            .parse_node(root)
            .map(Into::<on::Trigger>::into)
    }
}

impl<'a, R> Parser<R, on::Trigger> for OnParser<'a>
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::Trigger>
    where
        R: Repr,
    {
        let mut from_str = OnStringParser;
        let mut from_arr = OnArrayParser;
        let mut from_map = OnMapParser(&mut self.0);

        let mut rhs = OrParser::new(&mut from_arr, &mut from_map, &|r| {
            PossumNodeKind::Invalid(
                ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Seq, YamlKind::Map])
                    .but_found(r)
                    .to_string(),
            )
        });

        OrParser::new(&mut from_str, &mut rhs, &|r| {
            PossumNodeKind::Invalid(
                ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Seq, YamlKind::Map])
                    .but_found(r)
                    .to_string(),
            )
        })
        .parse_node(root)
    }
}
