use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::Extract;
use crate::scavenge::Parser;
use crate::workflow::on;
use std::marker::PhantomData;
use yaml_peg::repr::Repr;

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
        let mut input = on::WorkflowInput::default();

        for (key, value) in root.extract_map().unwrap().iter() {
            match key.extract_str() {
                Ok(s) => match s.to_lowercase().as_str() {
                    "description" => {
                        input.description = Some(PossumNodeKind::Empty.at(value.pos()));
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
                        input.choices = Some(PossumNodeKind::Empty.at(value.pos()));
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
}
