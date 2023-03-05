use yaml_peg::repr::Repr;
use yaml_peg::Node as YamlNode;

use super::concrete::StringParser;
use crate::scavenge::ast::PossumNodeKind;
use crate::scavenge::ast::{PossumMap, PossumSeq};
use crate::scavenge::extraction::Extract;
use crate::scavenge::Parser;
use std::marker::PhantomData;

pub struct StringMapParser<R>(MapParser<R, String, String, StringParser, StringParser>)
where
    R: Repr;

impl<R> StringMapParser<R>
where
    R: Repr,
{
    pub fn new() -> StringMapParser<R> {
        StringMapParser(MapParser::new(StringParser, StringParser))
    }
}

impl<R> Parser<R, PossumMap<String, String>> for StringMapParser<R>
where
    R: Repr,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<PossumMap<String, String>>
    where
        R: Repr,
    {
        self.0.parse_node(root)
    }
}


pub struct MapParser<R, K, V, KP, VP>
where
    R: Repr,
    KP: Parser<R, K>,
    VP: Parser<R, V>,
{
    keys: KP,
    values: VP,
    _r: PhantomData<R>,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}
impl<R, K, V, KP, VP> MapParser<R, K, V, KP, VP>
where
    R: Repr,
    KP: Parser<R, K>,
    VP: Parser<R, V>,
{
    pub fn new(keys: KP, values: VP) -> MapParser<R, K, V, KP, VP> {
        MapParser {
            keys,
            values,
            _r: PhantomData,
            _k: PhantomData,
            _v: PhantomData,
        }
    }
}

impl<R, K, V, KP, VP> Parser<R, PossumMap<K, V>> for MapParser<R, K, V, KP, VP>
where
    R: Repr,
    KP: Parser<R, K>,
    VP: Parser<R, V>,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<PossumMap<K, V>>
    where
        R: Repr,
    {
        use PossumNodeKind::{Invalid, Value};
        match root.extract_map() {
            Err(u) => Invalid(u.to_string()),
            Ok(m) => {
                let mut map = PossumMap::empty();

                for (key, value) in m.iter() {
                    let k: PossumNodeKind<K> = self.keys.parse_node(key);
                    let v: PossumNodeKind<V> = self.values.parse_node(value);

                    map.insert(k.at(key), v.at(value));
                }

                Value(map)
            }
        }
    }
}

pub struct SeqParser<R, T, P>
where
    R: Repr,
    P: Parser<R, T> {
        inner: P,
        _r: PhantomData<R>,
        _t: PhantomData<T>,
    }

impl< R, T, P> SeqParser< R, T, P>
where
    R: Repr,
    P: Parser<R, T>,
{
    pub fn new(inner: P) -> SeqParser<R, T, P> {
        SeqParser {
            inner,
            _r: PhantomData,
            _t: PhantomData,
        }
    }
}

impl< R, T, P> Parser<R, PossumSeq<T>> for SeqParser<R, T, P>
where
    R: Repr,
    P: Parser<R, T>,
{
    fn parse_node(&mut self, root: &YamlNode<R>) -> PossumNodeKind<PossumSeq<T>>
    where
        R: Repr,
    {
        use PossumNodeKind::{Invalid, Value};
        match root.extract_seq() {
            Err(u) => Invalid(u.to_string()),
            Ok(seq) => Value(
                seq.iter()
                    .map(|elm| self.inner.parse_node(elm).at(elm))
                    .collect(),
            ),
        }
    }
}
