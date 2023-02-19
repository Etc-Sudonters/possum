use crate::document::{Annotation, Annotations};
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::yaml::YamlKind;
use crate::scavenge::{Parser, UnexpectedKey};
use crate::workflow::Concurrency;
use std::marker::PhantomData;
use yaml_peg::repr::Repr;
use yaml_peg::Yaml;

pub struct ConcurrencyParser<'a, R>
where
    R: Repr + 'a,
{
    _x: PhantomData<R>,
    annotations: &'a mut Annotations,
}
impl<'a, R> ConcurrencyParser<'a, R>
where
    R: Repr + 'a,
{
    pub fn new(a: &'a mut Annotations) -> ConcurrencyParser<'a, R> {
        ConcurrencyParser {
            _x: PhantomData,
            annotations: a,
        }
    }

    fn annotate<A>(&mut self, a: A)
    where
        A: Into<Annotation>,
    {
        self.annotations.add(a)
    }
}

impl<'a, R> Parser<'a, R, Concurrency> for ConcurrencyParser<'a, R>
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &yaml_peg::Node<R>) -> PossumNodeKind<Concurrency>
    where
        R: Repr,
    {
        use PossumNodeKind::*;
        match root.yaml() {
            Yaml::Str(group) => Value(Concurrency::Concurrency(group.to_owned())),
            Yaml::Map(concur) => {
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

                Value(Concurrency::Group {
                    group,
                    cancel_in_progress,
                })
            }
            u @ _ => Invalid(
                ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Map])
                    .but_found(u)
                    .to_string(),
            ),
        }
    }
}
