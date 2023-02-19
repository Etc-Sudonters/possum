use crate::document::{Annotation, Annotations, AsDocumentPointer};
use crate::scavenge::ast::{PossumNodeKind, PossumSeq};
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::yaml::YamlKind;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::workflow::job::Job;
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
            "name" => {
                job.name = Some(
                    value
                        .extract_str()
                        .map_or_else(
                            |unexpected| Invalid(unexpected.to_string()),
                            |v| Value(v.to_owned()),
                        )
                        .at(value),
                );
            }
            "permissions" => {}
            "needs" => {
                let needs = match value.yaml() {
                    Yaml::Str(s) => {
                        // this sucks and is too difficult
                        let need = Value(s.to_owned()).at(value);
                        let mut needs = PossumSeq::empty();
                        needs.push(need);
                        Value(needs)
                    }
                    Yaml::Seq(seq) => {
                        let needs = seq
                            .iter()
                            .map(|need| {
                                match need.extract_str() {
                                    Ok(s) => Value(s.to_owned()),
                                    Err(u) => Invalid(u.to_string()),
                                }
                                .at(need)
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
                    Yaml::Str(s) => {
                        // this sucks and is too difficult
                        let runner = Value(s.to_owned()).at(value);
                        let mut runners = PossumSeq::empty();
                        runners.push(runner);
                        Value(runners)
                    }
                    Yaml::Seq(seq) => {
                        let runners = seq
                            .iter()
                            .map(|runner| {
                                match runner.extract_str() {
                                    Ok(s) => Value(s.to_owned()),
                                    Err(u) => Invalid(u.to_string()),
                                }
                                .at(runner)
                            })
                            .collect();
                        Value(runners)
                    }
                    unexpected @ _ => Invalid(
                        ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Seq])
                            .but_found(unexpected)
                            .to_string(),
                    ),
                }
                .at(value);

                job.runs_on = Some(runs_on);
            }
            "environment" => {}
            "concurrency" => {}
            "outputs" => {
                let outputs = match value.extract_map() {
                    Err(u) => Invalid(u.to_string()),
                    Ok(map) => Value(
                        map.iter()
                            .map(|(k, v)| {
                                let k = k
                                    .extract_str()
                                    .map_or_else(
                                        |u| Invalid(u.to_string()),
                                        |v| Value(v.to_owned()),
                                    )
                                    .at(k);
                                let v = v
                                    .extract_str()
                                    .map_or_else(
                                        |u| Invalid(u.to_string()),
                                        |v| Value(v.to_owned()),
                                    )
                                    .at(v);
                                (k, v)
                            })
                            .collect(),
                    ),
                }
                .at(value);

                job.outputs = Some(outputs);
            }
            "env" => {}
            "steps" => {}
            "timeout-minutes" => {
                job.timeout_minutes = Some(
                    value
                        .extract_number()
                        .map_or_else(|u| Invalid(u.to_string()), |v| Value(v.clone()))
                        .at(value),
                );
            }
            "continue-on-error" => {
                job.continue_on_error = Some(
                    value
                        .extract_bool()
                        .map_or_else(|u| Invalid(u.to_string()), |v| Value(v.to_owned()))
                        .at(value),
                );
            }
            "uses" => {
                job.uses = Some(
                    value
                        .extract_str()
                        .map_or_else(
                            |unexpected| Invalid(unexpected.to_string()),
                            |v| Value(v.to_owned()),
                        )
                        .at(value),
                );
            }
            "with" => {}
            s => self.annotate(UnexpectedKey::at(&s.to_owned(), p)),
        }
    }
}
