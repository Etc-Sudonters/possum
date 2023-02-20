use std::marker::PhantomData;

use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

use crate::{
    document::{Annotation, Annotations},
    scavenge::{
        ast::PossumNodeKind,
        extraction::Extract,
        parser::{MapParser, Parser, StringParser},
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
                    self.step.with = Some(
                        MapParser::new(&mut StringParser)
                            .parse_node(value)
                            .at(value),
                    );
                    Fallible::Success
                }

                "env" => {
                    self.step.env = Some(
                        MapParser::new(&mut StringParser)
                            .parse_node(value)
                            .at(value),
                    );

                    Fallible::Success
                }

                k @ _ => Fallible::Failure(UnexpectedKey::at(&k.to_owned(), key).into()),
            },
        }
    }
}

impl Into<job::Step> for StepBuilder {
    fn into(self) -> job::Step {
        self.step
    }
}

pub struct StepParser<'a, R>
where
    R: Repr + 'a,
{
    _x: PhantomData<R>,
    annotations: &'a mut Annotations,
}

impl<'a, R> Parser<'a, R, job::Step> for StepParser<'a, R>
where
    R: Repr + 'a,
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

impl<'a, R> StepParser<'a, R>
where
    R: Repr + 'a,
{
    pub fn new(a: &'a mut Annotations) -> StepParser<'a, R> {
        StepParser {
            _x: PhantomData,
            annotations: a,
        }
    }
    fn annotate<A>(&mut self, a: A)
    where
        A: Into<Annotation>,
    {
        self.annotations.add(a);
    }
}
