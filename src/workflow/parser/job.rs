use super::concurrency::ConcurrencyParser;
use super::permissions::PermissionParser;
use crate::document::{Annotations, AsDocumentPointer};
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::parser::{
    Builder, ObjectParser, OrParser, Pluralize, SeqParser, StringParser, TransformParser,
};
use crate::scavenge::yaml::YamlKind;
use crate::scavenge::{MapParser, Parser, UnexpectedKey};
use crate::workflow::job::{self, Job};
use crate::workflow::parser::step::StepParser;
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

pub struct JobParser<'a> {
    annotations: &'a mut Annotations,
}

impl<'a, R> Parser<R, Job> for JobParser<'a>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &yaml_peg::Node<R>) -> PossumNodeKind<Job>
    where
        R: Repr,
    {
        ObjectParser::new(JobBuilder, || job::Job::default(), &mut self.annotations)
            .parse_node(root)
    }
}

impl<'a> JobParser<'a> {
    pub fn new(a: &'a mut Annotations) -> JobParser<'a> {
        JobParser { annotations: a }
    }
}

struct EnvStringParser;

impl<R> Parser<R, job::Environment> for EnvStringParser
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<job::Environment>
    where
        R: Repr,
    {
        TransformParser::new(&mut StringParser, &job::Environment::Bare).parse_node(root)
    }
}

struct EnvMapParser<'a>(&'a mut Annotations);

impl<'a> EnvMapParser<'a> {
    pub fn new(annotations: &'a mut Annotations) -> EnvMapParser<'a> {
        EnvMapParser(annotations)
    }
}

impl<'a, R> Parser<R, job::Environment> for EnvMapParser<'a>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<job::Environment>
    where
        R: Repr,
    {
        use PossumNodeKind::*;
        match root.extract_map() {
            Err(u) => Invalid(u.to_string()),
            Ok(m) => {
                let mut env_name = None;
                let mut env_url = None;

                for (key, value) in m.iter() {
                    match key.extract_str() {
                        Err(u) => self.0.add(u.at(key)),
                        Ok(s) => match s.to_lowercase().as_str() {
                            "name" => env_name = Some(Value(s.to_owned()).at(value)),
                            "url" => env_url = Some(Value(s.to_owned()).at(value)),
                            _ => self.0.add(UnexpectedKey::from(s).at(value)),
                        },
                    }
                }

                Value(job::Environment::Env {
                    name: env_name,
                    url: env_url,
                })
            }
        }
    }
}

struct JobBuilder;

impl Builder<job::Job> for JobBuilder {
    fn build<'a, P, R>(
        &mut self,
        item: &mut job::Job,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer,
        R: Repr,
    {
        use PossumNodeKind::*;
        match key {
            "permissions" => {
                item.permissions = Some(
                    PermissionParser::new(annotations)
                        .parse_node(value)
                        .at(value),
                );
            }
            "env" => {
                item.env = Some({
                    MapParser::new(&mut StringParser)
                        .parse_node(value)
                        .at(value)
                });
            }
            "with" => {
                item.with = Some(
                    MapParser::new(&mut StringParser)
                        .parse_node(value)
                        .at(value),
                );
            }
            "concurrency" => {
                item.concurrency = Some(
                    ConcurrencyParser::new(annotations)
                        .parse_node(value)
                        .at(value),
                );
            }
            "steps" => {
                item.steps = Some(
                    SeqParser::new(&mut StepParser::new(annotations))
                        .parse_node(value)
                        .at(value),
                );
            }
            "environment" => {
                item.environment = Some(
                    OrParser::new(
                        &mut EnvStringParser,
                        &mut EnvMapParser::new(annotations),
                        &|r| {
                            Invalid(
                                ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Map])
                                    .but_found(r)
                                    .to_string(),
                            )
                        },
                    )
                    .parse_node(value)
                    .at(value),
                );
            }
            "name" => {
                item.name = Some(StringParser.parse_node(value).at(value));
            }
            "needs" => {
                item.needs = Some(
                    OrParser::new(
                        &mut Pluralize::new(&mut StringParser),
                        &mut SeqParser::new(&mut StringParser),
                        &|unexpected| {
                            Invalid(
                                ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Seq])
                                    .but_found(unexpected)
                                    .to_string(),
                            )
                        },
                    )
                    .parse_node(value)
                    .at(value),
                )
            }
            "if" => {
                item.cond = Some(StringParser.parse_node(value).at(value));
            }
            "runs-on" => {
                item.runs_on = Some(
                    OrParser::new(
                        &mut Pluralize::new(&mut StringParser),
                        &mut SeqParser::new(&mut StringParser),
                        &|unexpected| {
                            Invalid(
                                ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Seq])
                                    .but_found(unexpected)
                                    .to_string(),
                            )
                        },
                    )
                    .parse_node(value)
                    .at(value),
                );
            }
            "outputs" => {
                item.outputs = Some(
                    MapParser::new(&mut StringParser)
                        .parse_node(value)
                        .at(value),
                );
            }
            "timeout-minutes" => {
                let timeout: PossumNodeKind<f64> = value.extract_number().into();
                item.timeout_minutes = Some(timeout.at(value));
            }
            "continue-on-error" => {
                let coe: PossumNodeKind<bool> = value.extract_bool().into();
                item.continue_on_error = Some(coe.at(value));
            }
            "uses" => {
                let uses: PossumNodeKind<String> =
                    value.extract_str().map(ToOwned::to_owned).into();
                item.uses = Some(uses.at(value));
            }
            s => panic!("unknown key inside job {s}"), //self.annotate(UnexpectedKey::from(s).at(p)),
        }
    }
}
