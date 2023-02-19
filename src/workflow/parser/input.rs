use crate::document::AsDocumentPointer;
use crate::scavenge::ast::{PossumNodeKind, PossumSeq};
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::yaml::YamlKind;
use crate::scavenge::Parser;
use crate::workflow::on;
use std::marker::PhantomData;
use std::str::FromStr;
use yaml_peg::repr::Repr;
use yaml_peg::{Node as YamlNode, Seq as YamlSeq, Yaml};

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
    fn parse_node(&mut self, root: &yaml_peg::Node<R>) -> PossumNodeKind<on::WorkflowInput>
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
                                .map_or_else(
                                    |unexpected| Invalid(unexpected.to_string()),
                                    |description| Value(description.to_owned()),
                                )
                                .at(value),
                        );
                    }
                    "default" => {
                        input.default = Some(Self::default_value(value).at(value));
                    }
                    "required" => {
                        input.required = Some(
                            value
                                .extract_bool()
                                .map_or_else(|unexpected| Invalid(unexpected.to_string()), Value)
                                .at(value),
                        );
                    }
                    "type" => {
                        input.input_type = Some(
                            value
                                .extract_str()
                                .map_or_else(
                                    |unexpected| Invalid(unexpected.to_string()),
                                    |maybe_type| {
                                        on::WorkflowInputType::from_str(maybe_type).map_or_else(
                                            |_| {
                                                Invalid(format!("unknown input type: {maybe_type}"))
                                            },
                                            Value,
                                        )
                                    },
                                )
                                .at(value),
                        );
                    }
                    "choices" => {
                        input.choices = Some(
                            value
                                .extract_seq()
                                .map_or_else(
                                    |unexpected| Invalid(unexpected.to_string()),
                                    |choices| Value(Self::choices(choices)),
                                )
                                .at(value),
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
        root.into_iter()
            .map(|n| (n.extract_str(), n.as_document_pointer()))
            .map(|(node, pos)| {
                node.map_or_else(
                    |unexpected| PossumNodeKind::Invalid(unexpected.to_string()),
                    |choice| PossumNodeKind::Value(choice.to_string()),
                )
                .at(pos)
            })
            .collect()
    }

    fn default_value(root: &YamlNode<R>) -> PossumNodeKind<on::WorkflowInputDefault> {
        use on::WorkflowInputDefault::*;
        use PossumNodeKind::{Invalid, Value};

        match root.yaml() {
            Yaml::Str(s) => Value(Str(s.to_owned())),
            Yaml::Bool(b) => Value(Bool(b.clone())),
            Yaml::Int(i) => Value(Number(i.clone())),
            _ => Invalid(
                ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Number, YamlKind::Bool])
                    .but_found(root)
                    .to_string(),
            ),
        }
    }
}
