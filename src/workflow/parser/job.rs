use crate::document::{Annotation, Annotations, AsDocumentPointer};
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::yaml::YamlKind;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::workflow::job::{self, Job};
use crate::workflow::parser::step::StepParser;
use std::marker::PhantomData;
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;
use yaml_peg::{Map as YamlMap, Yaml};

pub struct JobParser<'a, R>
where
    R: Repr + 'a,
{
    _x: PhantomData<R>,
    annotations: &'a mut Annotations,
}

impl<'a, R> Parser<'a, R, Job> for JobParser<'a, R>
where
    R: Repr + 'a,
{
    fn parse_node(mut self, root: &yaml_peg::Node<R>) -> PossumNodeKind<Job>
    where
        R: Repr,
    {
        match root.extract_map() {
            Ok(m) => PossumNodeKind::Value(self.parse(m)),
            Err(e) => PossumNodeKind::Invalid(e.to_string()),
        }
    }
}

impl<'a, R> JobParser<'a, R>
where
    R: Repr + 'a,
{
    pub fn new(a: &'a mut Annotations) -> JobParser<'a, R> {
        JobParser {
            _x: PhantomData,
            annotations: a,
        }
    }

    fn annotate<A>(&mut self, annotation: A)
    where
        A: Into<Annotation>,
    {
        self.annotations.add(annotation)
    }

    fn parse(&mut self, root: &YamlMap<R>) -> Job {
        let mut job = Job::default();

        for (key, value) in root.iter() {
            match key.extract_str() {
                Ok(s) => self.job_key(&mut job, s, value, key),
                Err(err) => self.annotate(err.at(key)),
            }
        }

        job
    }

    fn job_key<P>(&mut self, job: &mut Job, key: &str, value: &YamlNode<R>, p: &P)
    where
        P: AsDocumentPointer,
    {
        use PossumNodeKind::*;
        match key.to_lowercase().as_str() {
            "permissions" => {
                todo!();
            }
            "concurrency" => {
                todo!();
            }
            "env" => {
                todo!();
            }
            "with" => {
                todo!()
            }
            "steps" => {
                job.steps = Some({
                    match value.extract_seq() {
                        Err(u) => Invalid(u.to_string()),
                        Ok(seq) => Value(
                            seq.iter()
                                .map(|step| {
                                    StepParser::new(self.annotations).parse_node(step).at(step)
                                })
                                .collect(),
                        ),
                    }
                    .at(value)
                });
            }
            "environment" => {
                let environment = match value.yaml() {
                    Yaml::Str(s) => Value(job::Environment::Bare(s.to_owned())),
                    Yaml::Map(m) => {
                        let mut env_name = None;
                        let mut env_url = None;

                        for (key, value) in m.iter() {
                            match key.extract_str() {
                                Err(u) => self.annotate(u.at(key)),
                                Ok(s) => match s.to_lowercase().as_str() {
                                    "name" => env_name = Some(Value(s.to_owned()).at(value)),
                                    "url" => env_url = Some(Value(s.to_owned()).at(value)),
                                    _ => self.annotate(UnexpectedKey::at(&s.to_owned(), key)),
                                },
                            }
                        }

                        Value(job::Environment::Env {
                            name: env_name,
                            url: env_url,
                        })
                    }
                    unexpected @ _ => Invalid(
                        ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Map])
                            .but_found(unexpected)
                            .to_string(),
                    ),
                }
                .at(value);

                job.environment = Some(environment);
            }
            "name" => {
                let name: PossumNodeKind<String> =
                    value.extract_str().map(ToOwned::to_owned).into();
                job.name = Some(name.at(value));
            }
            "needs" => {
                let needs = match value.yaml() {
                    Yaml::Str(s) => Value(Value(s.to_owned()).at(value).into()),
                    Yaml::Seq(seq) => {
                        let needs = seq
                            .iter()
                            .map(|root| {
                                let need: PossumNodeKind<String> =
                                    root.extract_str().map(ToOwned::to_owned).into();
                                need.at(root)
                            })
                            .collect();
                        Value(needs)
                    }
                    unexpected @ _ => Invalid(
                        ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Seq])
                            .but_found(unexpected)
                            .to_string(),
                    ),
                }
                .at(value);

                job.needs = Some(needs);
            }
            "if" => {
                job.cond = Some(
                    value
                        .extract_str()
                        .map_or_else(
                            |unexpected| Invalid(unexpected.to_string()),
                            |v| Expr(v.to_owned()),
                        )
                        .at(value),
                );
            }
            "runs-on" => {
                let runs_on = match value.yaml() {
                    // this is just incomprehensible gibberish. to_owned value of value of into
                    // value at my butt what the fuck
                    Yaml::Str(s) => Value(Value(s.to_owned()).at(value).into()),
                    Yaml::Seq(seq) => Value(
                        seq.iter()
                            .map(|runner| {
                                let r: PossumNodeKind<String> =
                                    runner.extract_str().map(ToOwned::to_owned).into();
                                r.at(runner)
                            })
                            .collect(),
                    ),
                    unexpected @ _ => Invalid(
                        ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Seq])
                            .but_found(unexpected)
                            .to_string(),
                    ),
                }
                .at(value);

                job.runs_on = Some(runs_on);
            }
            "outputs" => {
                let outputs = match value.extract_map() {
                    Err(u) => Invalid(u.to_string()),
                    Ok(map) => Value(
                        map.iter()
                            .map(|(key, value)| {
                                let k: PossumNodeKind<String> =
                                    key.extract_str().map(ToOwned::to_owned).into();
                                let v: PossumNodeKind<String> =
                                    value.extract_str().map(ToOwned::to_owned).into();
                                (k.at(key), v.at(value))
                            })
                            .collect(),
                    ),
                }
                .at(value);

                job.outputs = Some(outputs);
            }
            "timeout-minutes" => {
                let timeout: PossumNodeKind<f64> = value.extract_number().into();
                job.timeout_minutes = Some(timeout.at(value));
            }
            "continue-on-error" => {
                let coe: PossumNodeKind<bool> = value.extract_bool().into();
                job.continue_on_error = Some(coe.at(value));
            }
            "uses" => {
                let uses: PossumNodeKind<String> =
                    value.extract_str().map(ToOwned::to_owned).into();
                job.uses = Some(uses.at(value));
            }
            s => self.annotate(UnexpectedKey::at(&s.to_owned(), p)),
        }
    }
}
