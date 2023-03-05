use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

use crate::{
    document::{Annotations, AsDocumentPointer},
    scavenge::{ast::PossumNodeKind, extraction::Extract, Parser},
};

pub struct ObjectParser<'a, B, F, T>
where
    B: Builder<T>,
    F: Fn() -> T,
{
    builder: B,
    default: F,
    annotations: &'a mut Annotations,
}

pub trait Builder<T> {
    fn build<'a, P, R>(
        &mut self,
        item: &mut T,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer,
        R: Repr;
}

impl<'a, B, F, T> ObjectParser<'a, B, F, T>
where
    B: Builder<T>,
    F: Fn() -> T,
{
    pub fn new(
        builder: B,
        default: F,
        annotations: &'a mut Annotations,
    ) -> ObjectParser<'a, B, F, T> {
        ObjectParser {
            builder,
            default,
            annotations,
        }
    }
}

impl<'a, B, F, R, T> Parser<R, T> for ObjectParser<'a, B, F, T>
where
    R: Repr,
    B: Builder<T>,
    F: Fn() -> T,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<T>
    where
        R: Repr,
    {
        use PossumNodeKind::*;
        match root.extract_map() {
            Err(u) => Invalid(u.to_string()),
            Ok(m) => {
                let mut item = (self.default)();

                for (key, value) in m.iter() {
                    match key.extract_str() {
                        Err(u) => self.annotations.add(u.at(key)),
                        Ok(s) => self
                            .builder
                            .build(&mut item, s, value, key, self.annotations),
                    }
                }

                Value(item)
            }
        }
    }
}
