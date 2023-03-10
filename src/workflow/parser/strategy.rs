use crate::document::{Annotations, AsDocumentPointer};
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::ExpectedYaml;
use crate::scavenge::parsers::{
    BoolParser, Builder, MapParser, MaybeExprParser, NumberParser, ObjectParser, OrParser,
    OrableParser, SeqParser, StringParser, TransformableParser,
};
use crate::scavenge::yaml::YamlKind;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::workflow::job::{self, MatrixInput};
use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

#[derive(Default)]
pub struct StrategyBuilder {
    strategy: job::Strategy,
}

impl Into<job::Strategy> for StrategyBuilder {
    fn into(self) -> job::Strategy {
        self.strategy
    }
}

#[allow(unused_variables)]
impl Builder<job::Strategy> for StrategyBuilder {
    fn build<'a, P, R>(
        &mut self,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer + 'a,
        R: Repr,
    {
        match key.to_lowercase().as_str() {
            "fail-fast" => {
                self.strategy.fail_fast = Some(BoolParser.parse_node(value).at(value));
            }
            "max-parallel" => {
                self.strategy.max_parallel = Some(NumberParser.parse_node(value).at(value));
            }
            "matrix" => {
                self.strategy.matrix = Some(
                    ObjectParser::new(
                        MatrixBuilder::default,
                        annotations,
                    )
                    .parse_node(value)
                    .at(value),
                );
            }
            s => annotations.add(UnexpectedKey::from(s).at(pointer)),
        }
    }
}

#[derive(Default)]
pub struct MatrixBuilder {
    matrix: job::Matrix,
}

impl Into<job::Matrix> for MatrixBuilder {
    fn into(self) -> job::Matrix {
        self.matrix
    }
}

impl Builder<job::Matrix> for MatrixBuilder {
    fn build<'a, P, R>(
        &mut self,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        _: &'a mut Annotations,
    ) where
        P: AsDocumentPointer + 'a,
        R: Repr,
    {
        let parser: OrParser<R, _, _, _, _> = StringParser
            .to(MatrixInput::Str)
            .or(NumberParser.to(MatrixInput::Number), |_| {
                PossumNodeKind::Empty
            })
            .or(BoolParser.to(MatrixInput::Bool), |r| {
                PossumNodeKind::invalid(
                    ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Number, YamlKind::Bool])
                        .but_found(r),
                )
            });
        match key.to_lowercase().as_str() {
            "include" => {
                let parser = MapParser::new(StringParser, parser);
                let parser = SeqParser::new(parser);
                let mut parser = MaybeExprParser::new(parser, |r| {
                    PossumNodeKind::invalid(
                        ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Number, YamlKind::Bool])
                            .but_found(r),
                    )
                });

                let parsed = parser.parse_node(value).at(value);

                self.matrix.include = Some(parsed);
            }
            "exclude" => {
                let parser = MapParser::new(StringParser, parser);
                let parser = SeqParser::new(parser);
                let mut parser =
                    MaybeExprParser::new(parser, |_| crate::scavenge::ast::PossumNodeKind::Empty);

                let parsed = parser.parse_node(value).at(value);

                self.matrix.include = Some(parsed);
            }
            s @ _ => {
                let parser = SeqParser::new(parser);
                let mut parser = MaybeExprParser::new(parser, |r| {
                    PossumNodeKind::invalid(
                        ExpectedYaml::SeqOf(Box::new(ExpectedYaml::AnyOf(vec![
                            YamlKind::Str,
                            YamlKind::Number,
                            YamlKind::Bool,
                        ])))
                        .but_found(r),
                    )
                });

                let parsed = parser.parse_node(value).at(value);

                self.matrix
                    .entries
                    .insert(PossumNodeKind::Value(s.to_owned()).at(pointer), parsed);
            }
        }
    }
}
