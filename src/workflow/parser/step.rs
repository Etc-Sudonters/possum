use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

use crate::{
    document::{Annotation, Annotations},
    scavenge::{
        ast::PossumNodeKind,
        extraction::Extract,
        parser::{Parser, StringMapParser, StringParser},
        Fallible, UnexpectedKey,
    },
    workflow::job,
};

struct StepBuilder {
    step: job::Step,
}

impl StepBuilder {
    fn empty() -> StepBuilder {
        StepBuilder {
            step: job::Step::default(),
        }
    }

    fn build<'a, R>(&mut self, key: &'a YamlNode<R>, value: &'a YamlNode<R>) -> Fallible<Annotation>
    where
        R: Repr + 'a,
    {
        match key.extract_str() {
            Err(u) => Fallible::Failure(u.at(key)),
            Ok(s) => match s.to_lowercase().as_str() {
                "id" => {
                    self.step.id = Some(StringParser.parse_node(value).at(value));
                    Fallible::Success
                }

                "if" => {
                    self.step.cond = Some(StringParser.parse_node(value).at(value));
                    Fallible::Success
                }

                "name" => {
                    self.step.name = Some(StringParser.parse_node(value).at(value));
                    Fallible::Success
                }

                "uses" => {
                    self.step.uses = Some(StringParser.parse_node(value).at(value));
                    Fallible::Success
                }

                "run" => {
                    self.step.run = Some(StringParser.parse_node(value).at(value));
                    Fallible::Success
                }
                "shell" => {
                    self.step.shell = Some(StringParser.parse_node(value).at(value));
                    Fallible::Success
                }

                "with" => {
                    self.step.with = Some(StringMapParser::new().parse_node(value).at(value));
                    Fallible::Success
                }

                "env" => {
                    self.step.env = Some(StringMapParser::new().parse_node(value).at(value));

                    Fallible::Success
                }

                k @ _ => Fallible::Failure(UnexpectedKey::from(k).at(key)),
            },
        }
    }
}

impl Into<job::Step> for StepBuilder {
    fn into(self) -> job::Step {
        self.step
    }
}

pub struct StepParser<'a> {
    annotations: &'a mut Annotations,
}

impl<'a, R> Parser<R, job::Step> for StepParser<'a>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<job::Step>
    where
        R: Repr,
    {
        use PossumNodeKind::*;
        match root.extract_map() {
            Err(u) => Invalid(u.to_string()),
            Ok(m) => {
                let mut builder = StepBuilder::empty();

                for (key, value) in m.iter() {
                    match builder.build(key, value) {
                        Fallible::Success => {}
                        Fallible::Failure(a) => self.annotate(a),
                    }
                }

                Value(builder.into())
            }
        }
    }
}

impl<'a> StepParser<'a> {
    pub fn new(a: &'a mut Annotations) -> StepParser<'a> {
        StepParser { annotations: a }
    }
    fn annotate<A>(&mut self, a: A)
    where
        A: Into<Annotation>,
    {
        self.annotations.add(a);
    }
}
