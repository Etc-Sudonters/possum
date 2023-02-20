use crate::document::{Annotation, Annotations};
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::parser::{OrParser, StringParser, TransformParser};
use crate::scavenge::yaml::YamlKind;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::workflow::Concurrency;
use yaml_peg::repr::Repr;

pub struct ConcurrencyParser<'a>(&'a mut Annotations);

impl<'a> ConcurrencyParser<'a> {
    pub fn new(annotations: &'a mut Annotations) -> ConcurrencyParser<'a> {
        ConcurrencyParser(annotations)
    }
}

impl<'a, R> Parser<R, Concurrency> for ConcurrencyParser<'a>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &yaml_peg::Node<R>) -> PossumNodeKind<Concurrency>
    where
        R: Repr,
    {
        OrParser::new(
            &mut ConcurrencyStringParser,
            &mut ConcurrencyMapParser(self.0),
            &|root| {
                PossumNodeKind::Invalid(
                    ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Map])
                        .but_found(root)
                        .to_string(),
                )
            },
        )
        .parse_node(root)
    }
}

struct ConcurrencyStringParser;
struct ConcurrencyMapParser<'a>(&'a mut Annotations);

impl<'a> ConcurrencyMapParser<'a> {
    fn annotate<A>(&mut self, a: A)
    where
        A: Into<Annotation>,
    {
        self.0.add(a)
    }
}

impl<'a, R> Parser<R, Concurrency> for ConcurrencyStringParser
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &yaml_peg::Node<R>) -> PossumNodeKind<Concurrency>
    where
        R: Repr,
    {
        TransformParser::new(&mut StringParser, &|s| Concurrency::Concurrency(s)).parse_node(root)
    }
}

impl<'a, R> Parser<R, Concurrency> for ConcurrencyMapParser<'a>
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &yaml_peg::Node<R>) -> PossumNodeKind<Concurrency>
    where
        R: Repr,
    {
        match root.extract_map() {
            Err(u) => PossumNodeKind::Invalid(u.to_string()),
            Ok(concur) => {
                let mut group = None;
                let mut cancel_in_progress = None;

                for (key, value) in concur.iter() {
                    match key.extract_str() {
                        Err(u) => self.annotate(u.at(key)),
                        Ok(k) => match k.to_lowercase().as_str() {
                            "group" => {
                                group = Some({
                                    let g: PossumNodeKind<String> =
                                        value.extract_str().map(ToOwned::to_owned).into();
                                    g.at(value)
                                })
                            }

                            "cancel-in-progress" => {
                                cancel_in_progress = Some({
                                    let c: PossumNodeKind<bool> = value.extract_bool().into();
                                    c.at(value)
                                })
                            }

                            u @ _ => self.annotate(UnexpectedKey::at(&u.to_owned(), value)),
                        },
                    }
                }

                PossumNodeKind::Value(Concurrency::Group {
                    group,
                    cancel_in_progress,
                })
            }
        }
    }
}
