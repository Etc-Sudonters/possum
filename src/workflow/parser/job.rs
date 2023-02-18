use crate::document::{Annotation, Annotations, AsDocumentPointer};
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::Extract;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::workflow::job::Job;
use std::marker::PhantomData;
use yaml_peg::repr::Repr;
use yaml_peg::Map as YamlMap;
use yaml_peg::Node as YamlNode;

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
        self.annotations.add(annotation.into())
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

    fn job_key<P>(&mut self, event: &mut Job, key: &str, value: &YamlNode<R>, p: &P)
    where
        P: AsDocumentPointer,
    {
        match key.to_lowercase().as_str() {
            "name" => {}
            "permissions" => {}
            "needs" => {}
            "if" => {}
            "runs-on" => {}
            "environment" => {}
            "concurrency" => {}
            "outputs" => {}
            "env" => {}
            "steps" => {}
            "timeout-minutes" => {}
            "continue-on-error" => {}
            "uses" => {}
            "with" => {}
            s => self.annotate(UnexpectedKey::at(&s.to_owned(), p)),
        }
    }
}
