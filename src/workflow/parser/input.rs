use crate::document::Annotations;
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::ExpectedYaml;
use crate::scavenge::parser::{
    BoolParser, Builder, FlatMapParser, NumberParser, ObjectParser, OrParser, SeqParser,
    StringParser, TransformParser,
};
use crate::scavenge::yaml::YamlKind;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::workflow::on::{self, BadInputType};
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

pub struct InputParser<'a>(&'a mut Annotations);
struct InputBuilder;
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
        ObjectParser::new(InputBuilder, &on::WorkflowInput::default, &mut self.0).parse_node(root)
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
        FlatMapParser::new(
            &mut StringParser,
            &|s| match on::WorkflowInputType::fromstr(s.as_str()) {
                Ok(input_type) => PossumNodeKind::Value(input_type),
                Err(_) => PossumNodeKind::Invalid(BadInputType::Unknown(s).to_string()),
            },
        )
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
        let mut strs = StringParser;
        let mut bools = BoolParser;
        let mut nums = NumberParser;

        OrParser::new(
            &mut TransformParser::new(&mut strs, &|s| on::WorkflowInputDefault::Str(s)),
            &mut OrParser::new(
                &mut TransformParser::new(&mut bools, &|b| on::WorkflowInputDefault::Bool(b)),
                &mut TransformParser::new(&mut nums, &|n| on::WorkflowInputDefault::Number(n)),
                &|r| {
                    PossumNodeKind::Invalid(
                        ExpectedYaml::AnyOf(vec![YamlKind::Number, YamlKind::Str, YamlKind::Bool])
                            .but_found(r)
                            .to_string(),
                    )
                },
            ),
            &|r| {
                PossumNodeKind::Invalid(
                    ExpectedYaml::AnyOf(vec![YamlKind::Number, YamlKind::Str, YamlKind::Bool])
                        .but_found(r)
                        .to_string(),
                )
            },
        )
        .parse_node(root)
    }
}

impl Builder<on::WorkflowInput> for InputBuilder {
    fn build<'a, P, R>(
        &mut self,
        item: &mut on::WorkflowInput,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut crate::document::Annotations,
    ) where
        P: crate::document::AsDocumentPointer,
        R: Repr,
    {
        match key.to_lowercase().as_str() {
            "description" => {
                item.description = Some(StringParser.parse_node(value).at(value));
            }
            "default" => {
                item.default = Some(InputDefaultParser.parse_node(value).at(value));
            }
            "required" => {
                item.required = Some(BoolParser.parse_node(value).at(value));
            }
            "type" => {
                item.input_type = Some(InputTypeParser.parse_node(value).at(value));
            }
            "choices" => {
                item.choices = Some(
                    SeqParser::new(&mut StringParser)
                        .parse_node(value)
                        .at(value),
                );
            }
            s @ _ => annotations.add(UnexpectedKey::from(s).at(pointer)),
        }
    }
}
