use crate::document::{Annotations, AsDocumentPointer};
use crate::scavenge::extraction::ExpectedYaml;
use crate::scavenge::yaml::YamlKind;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::scavenge::parsers::{Builder, NumberParser, BoolParser, ObjectParser, SeqParser, OrParser, StringParser,  MaybeExprParser, MapParser, TransformableParser};
use crate::workflow::job::{self, MatrixInput};
use crate::scavenge::ast::PossumNodeKind;
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

pub struct StrategyBuilder;

#[allow(unused_variables)]
impl Builder<job::Strategy> for StrategyBuilder {
    fn build<'a, P, R>(
        &mut self,
        item: &mut job::Strategy,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer,
        R: Repr,
    {
        match key.to_lowercase().as_str() {
            "fail-fast" => {
                item.fail_fast = Some(BoolParser.parse_node(value).at(value));
            },
            "max-parallel" => {
                item.max_parallel = Some(NumberParser.parse_node(value).at(value));
            },
            "matrix" => {
                item.matrix = Some(ObjectParser::new(MatrixBuilder, || job::Matrix::default(), annotations).parse_node(value).at(value));
            },
            s => annotations.add(UnexpectedKey::from(s).at(pointer)),
        }
    }
}

pub struct MatrixBuilder;

#[allow(unused_variables, unreachable_code)]
impl Builder<job::Matrix> for MatrixBuilder {
    fn build<'a, P, R>(
        &mut self,
        item: &mut job::Matrix,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer,
        R: Repr,
    {
        match key.to_lowercase().as_str() {
            "include" => {
                let str_parse = StringParser.map(MatrixInput::Str);
                let numb_parse = NumberParser.map(MatrixInput::Number);
                let bool_parse = BoolParser.map(MatrixInput::Bool);
                let parser = OrParser::new(numb_parse, bool_parse, |_| crate::scavenge::ast::PossumNodeKind::Empty);
                let parser = OrParser::new(
                    str_parse, parser,
                    |r| PossumNodeKind::Invalid(ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Number, YamlKind::Bool]).but_found(r).to_string()));

                let parser = MapParser::new(StringParser, parser);
                let parser = SeqParser::new(parser);
                let mut parser = MaybeExprParser::new(parser, |_| crate::scavenge::ast::PossumNodeKind::Empty);

                let parsed = parser.parse_node(value);

                item.include = Some(todo!());

            },
            "exclude" => {},
            s @ _ => {
            }
        }
    }
}
