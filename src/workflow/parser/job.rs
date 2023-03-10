use super::concurrency::ConcurrencyParser;
use super::permissions::PermissionParser;
use crate::document::{Annotations, AsDocumentPointer};
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::parsers::{
    BoolParser, Builder, NumberParser, ObjectParser, OrableParser, PluralizableParser, SeqParser,
    StringMapParser, StringParser, TransformableParser,
};
use crate::scavenge::yaml::YamlKind;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::workflow::job::{self, Job};
use crate::workflow::parser::step::StepParser;
use crate::workflow::parser::strategy::StrategyBuilder;
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
        ObjectParser::new(JobBuilder::default, &mut self.annotations)
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
        StringParser.to(job::Environment::Bare).parse_node(root)
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

#[derive(Default)]
struct JobBuilder {
    job: job::Job,
}

impl Into<job::Job> for JobBuilder {
    fn into(self) -> job::Job {
       self.job
    }
}

impl Builder<job::Job> for JobBuilder {
    fn build<'a, P, R>(
        &mut self,
        key: &str,
        value: &YamlNode<R>,
        p: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer + 'a,
        R: Repr,
    {
        use PossumNodeKind::*;
        match key {
            "permissions" => {
                self.job.permissions = Some(
                    PermissionParser::new(annotations)
                        .parse_node(value)
                        .at(value),
                );
            }
            "env" => {
                self.job.env = Some(StringMapParser::new().parse_node(value).at(value));
            }
            "with" => {
                self.job.with = Some(StringMapParser::new().parse_node(value).at(value));
            }
            "concurrency" => {
                self.job.concurrency = Some(
                    ConcurrencyParser::new(annotations)
                        .parse_node(value)
                        .at(value),
                );
            }
            "steps" => {
                self.job.steps = Some(
                    SeqParser::new(StepParser::new(annotations))
                        .parse_node(value)
                        .at(value),
                );
            }
            "environment" => {
                self.job.environment = Some(
                    EnvStringParser
                        .or(EnvMapParser::new(annotations), |r| {
                            PossumNodeKind::invalid(
                                ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Map])
                                    .but_found(r),
                            )
                        })
                        .parse_node(value)
                        .at(value),
                );
            }
            "name" => {
                self.job.name = Some(StringParser.parse_node(value).at(value));
            }
            "needs" => {
                self.job.needs = Some(
                    StringParser
                        .pluralize()
                        .or(SeqParser::new(StringParser), |unexpected| {
                            Invalid(
                                ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Seq])
                                    .but_found(unexpected)
                                    .to_string(),
                            )
                        })
                        .parse_node(value)
                        .at(value),
                )
            }
            "if" => {
                self.job.cond = Some(StringParser.parse_node(value).at(value));
            }
            "runs-on" => {
                self.job.runs_on = Some(
                    StringParser
                        .pluralize()
                        .or(SeqParser::new(StringParser), |unexpected| {
                            Invalid(
                                ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Seq])
                                    .but_found(unexpected)
                                    .to_string(),
                            )
                        })
                        .parse_node(value)
                        .at(value),
                );
            }
            "outputs" => {
                self.job.outputs = Some(StringMapParser::new().parse_node(value).at(value));
            }
            "timeout-minutes" => {
                self.job.timeout_minutes = Some(NumberParser.parse_node(value).at(value));
            }
            "continue-on-error" => {
                self.job.continue_on_error = Some(BoolParser.parse_node(value).at(value));
            }
            "uses" => {
                self.job.uses = Some(StringParser.parse_node(value).at(value));
            }
            "strategy" => {
                self.job.strategy = Some(ObjectParser::new(StrategyBuilder::default, annotations).parse_node(value).at(value));
            }
            s => annotations.add(UnexpectedKey::from(s).at(p)),
        }
    }
}
