use crate::document::Annotations;
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::ExpectedYaml;
use crate::scavenge::parsers::{
    BoolParser, Builder, FlatMappableParser, NumberParser, ObjectParser, OrableParser, SeqParser,
    StringParser, TransformableParser,
};
use crate::scavenge::yaml::YamlKind;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::workflow::on::{self, BadInputType};
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

pub struct InputParser<'a>(&'a mut Annotations);
struct InputDefaultParser;
struct InputTypeParser;

impl<'a> InputParser<'a> {
    pub fn new(annotations: &'a mut Annotations) -> InputParser<'a> {
        InputParser(annotations)
    }
}

impl<'a, R> Parser<R, on::WorkflowInput> for InputParser<'a>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::WorkflowInput>
    where
        R: Repr,
    {
        ObjectParser::new(InputBuilder::default, &mut self.0).parse_node(root)
    }
}

impl<R> Parser<R, on::WorkflowInputType> for InputTypeParser
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::WorkflowInputType>
    where
        R: Repr,
    {
        StringParser
            .flatten(|s| match on::WorkflowInputType::fromstr(s.as_str()) {
                Ok(input_type) => PossumNodeKind::Value(input_type),
                Err(_) => PossumNodeKind::Invalid(BadInputType::Unknown(s).to_string()),
            })
            .parse_node(root)
    }
}

impl<R> Parser<R, on::WorkflowInputDefault> for InputDefaultParser
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<on::WorkflowInputDefault>
    where
        R: Repr,
    {
        StringParser
            .to(on::WorkflowInputDefault::Str)
            .or(BoolParser.to(on::WorkflowInputDefault::Bool), |r| {
                PossumNodeKind::invalid(
                    ExpectedYaml::AnyOf(vec![YamlKind::Number, YamlKind::Str, YamlKind::Bool])
                        .but_found(r),
                )
            })
            .or(NumberParser.to(on::WorkflowInputDefault::Number), |r| {
                PossumNodeKind::invalid(
                    ExpectedYaml::AnyOf(vec![YamlKind::Number, YamlKind::Str, YamlKind::Bool])
                        .but_found(r),
                )
            })
            .parse_node(root)
    }
}

#[derive(Default)]
struct InputBuilder {
    input: on::WorkflowInput
}

impl Into<on::WorkflowInput> for InputBuilder {
    fn into(self) -> on::WorkflowInput {
       self.input 
    }
}

impl Builder<on::WorkflowInput> for InputBuilder {
    fn build<'a, P, R>(
        &mut self,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut crate::document::Annotations,
    ) where
        P: crate::document::AsDocumentPointer + 'a,
        R: Repr,
    {
        match key.to_lowercase().as_str() {
            "description" => {
                self.input.description = Some(StringParser.parse_node(value).at(value));
            }
            "default" => {
                self.input.default = Some(InputDefaultParser.parse_node(value).at(value));
            }
            "required" => {
                self.input.required = Some(BoolParser.parse_node(value).at(value));
            }
            "type" => {
                self.input.input_type = Some(InputTypeParser.parse_node(value).at(value));
            }
            "choices" => {
                self.input.choices = Some(SeqParser::new(StringParser).parse_node(value).at(value));
            }
            s @ _ => annotations.add(UnexpectedKey::from(s).at(pointer)),
        }
    }
}
