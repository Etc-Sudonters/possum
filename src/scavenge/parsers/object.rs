use std::marker::PhantomData;

use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

use crate::{
    document::{Annotations, AsDocumentPointer},
    scavenge::{ast::PossumNodeKind, extraction::Extract, Parser},
};

pub struct ObjectParser<'a, B, F, T>
where
    B: Builder<T>,
    F: Fn() -> B
{
    builder: F,
    annotations: &'a mut Annotations,
    _t: PhantomData<T>,
}

pub trait Builder<T> : Into<T> {
    fn build<'a, P, R>(
        &mut self,
        key: &str,
        value: &YamlNode<R>,
        pointer: &P,
        annotations: &'a mut Annotations,
    ) where
        P: AsDocumentPointer + 'a,
        R: Repr;
}

impl<'a, B, F, T> ObjectParser<'a, B, F, T>
where
    B: Builder<T>,
    F: Fn() -> B
{
    pub fn new(
        builder: F,
        annotations: &'a mut Annotations,
    ) -> ObjectParser<'a, B, F, T> {
        ObjectParser {
            builder,
            annotations,
            _t: PhantomData,
        }
    }
}

impl<'a, B, F, R, T> Parser<R, T> for ObjectParser<'a, B, F, T>
where
    R: Repr,
    B: Builder<T>,
    F: Fn() -> B
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<T>
    where
        R: Repr,
    {
        use PossumNodeKind::*;
        match root.extract_map() {
            Err(u) => Invalid(u.to_string()),
            Ok(m) => {
                let mut builder = (self.builder)();
                for (key, value) in m.iter() {
                    match key.extract_str() {
                        Err(u) => self.annotations.add(u.at(key)),
                        Ok(s) => builder .build( s, value, key, self.annotations),
                    }
                }

                Value(builder.into())
            }
        }
    }
}
