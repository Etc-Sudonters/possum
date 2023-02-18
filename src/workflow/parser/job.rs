use crate::document::Annotations;
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::Extract;
use crate::scavenge::Parser;
use crate::workflow::job::Job;
use std::marker::PhantomData;
use yaml_peg::repr::Repr;
use yaml_peg::Map as YamlMap;

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
            Ok(m) => self.parse(m),
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

    fn parse(&mut self, root: &YamlMap<R>) -> PossumNodeKind<Job> {
        PossumNodeKind::Empty
    }
}
