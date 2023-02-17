use crate::scavenge::ast::{PossumNodeKind, PossumSeq};
use crate::scavenge::extraction::Extract;
use crate::scavenge::Parser;
use crate::workflow::on;
use std::marker::PhantomData;
use yaml_peg::repr::Repr;
use yaml_peg::Seq as YamlSeq;

pub struct InputParser<'a, R>
where
    R: Repr + 'a,
{
    _x: PhantomData<&'a R>,
}

impl<'a, R> Parser<'a, R, on::WorkflowInput> for InputParser<'a, R>
where
    R: Repr + 'a,
{
    fn parse_node(self, root: &yaml_peg::Node<R>) -> PossumNodeKind<on::WorkflowInput>
    where
        R: yaml_peg::repr::Repr,
    {
        use PossumNodeKind::*;
        let mut input = on::WorkflowInput::default();

        for (key, value) in root.extract_map().unwrap().iter() {
            match key.extract_str() {
                Ok(s) => match s.to_lowercase().as_str() {
                    "description" => {
                        input.description = Some(
                            value
                                .extract_str()
                                .map_or_else(|u| Invalid(u.to_string()), |s| Value(s.to_owned()))
                                .at(value.pos()),
                        );
                    }
                    "default" => {
                        input.default = Some(PossumNodeKind::Empty.at(value.pos()));
                    }
                    "required" => {
                        input.required = Some(PossumNodeKind::Empty.at(value.pos()));
                    }
                    "type" => {
                        input.input_type = Some(PossumNodeKind::Empty.at(value.pos()));
                    }
                    "choices" => {
                        input.choices = Some(
                            value
                                .extract_seq()
                                .map_or_else(
                                    |unexpected| Invalid(unexpected.to_string()),
                                    |choices| Value(Self::choices(choices)),
                                )
                                .at(value.pos()),
                        );
                    }
                    _ => {}
                },
                Err(e) => {}
            }
        }

        PossumNodeKind::Value(input)
    }
}

impl<'a, R> InputParser<'a, R>
where
    R: Repr + 'a,
{
    pub fn new() -> InputParser<'a, R> {
        InputParser { _x: PhantomData }
    }

    fn choices(root: &YamlSeq<R>) -> PossumSeq<String> {
        todo!()
    }
}
