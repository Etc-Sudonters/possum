use crate::document::{AsDocumentPointer, DocumentPointer};
use std::iter::{FromIterator, IntoIterator};

#[derive(Debug)]
pub struct PossumNode<T> {
    location: DocumentPointer,
    kind: PossumNodeKind<T>,
}

impl<T> PossumNode<T> {
    pub fn new(location: DocumentPointer, kind: PossumNodeKind<T>) -> PossumNode<T> {
        PossumNode { location, kind }
    }

    pub fn loc(&self) -> DocumentPointer {
        self.location.clone()
    }

    pub fn data(&self) -> &PossumNodeKind<T> {
        &self.kind
    }
}

#[derive(Debug)]
pub enum PossumNodeKind<T> {
    Invalid(String),
    Expr(String),
    Value(T),
    Empty,
}

impl<T> PossumNodeKind<T> {
    pub fn at<D>(self, location: D) -> PossumNode<T>
    where
        D: AsDocumentPointer,
    {
        PossumNode::new(location.as_document_pointer(), self)
    }

    pub fn invalid(&self) -> bool {
        match self {
            PossumNodeKind::Invalid(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct PossumMap<K, V> {
    keys: PossumSeq<K>,
    values: PossumSeq<V>,
}

impl<K, V> PossumMap<K, V> {
    pub fn empty() -> PossumMap<K, V> {
        PossumMap {
            keys: PossumSeq::empty(),
            values: PossumSeq::empty(),
        }
    }

    pub fn insert(&mut self, k: PossumNode<K>, v: PossumNode<V>) {
        self.keys.push(k);
        self.values.push(v);
    }
}

impl<K, V> FromIterator<(PossumNode<K>, PossumNode<V>)> for PossumMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (PossumNode<K>, PossumNode<V>)>>(iter: T) -> Self {
        let mut map = PossumMap::empty();

        for (k, v) in iter.into_iter() {
            map.insert(k, v)
        }

        map
    }
}

#[derive(Debug, Default)]
pub struct PossumSeq<T> {
    entries: Vec<PossumNode<T>>,
}

impl<T> IntoIterator for PossumSeq<T> {
    type Item = PossumNode<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<T> FromIterator<PossumNode<T>> for PossumSeq<T> {
    fn from_iter<I: IntoIterator<Item = PossumNode<T>>>(iter: I) -> Self {
        let v: Vec<PossumNode<T>> = iter.into_iter().collect();
        v.into()
    }
}

impl<T> Into<PossumSeq<T>> for Vec<PossumNode<T>> {
    fn into(self) -> PossumSeq<T> {
        PossumSeq { entries: self }
    }
}

impl<T> PossumSeq<T> {
    pub fn empty() -> PossumSeq<T> {
        PossumSeq {
            entries: Vec::new(),
        }
    }

    pub fn push(&mut self, t: PossumNode<T>) {
        self.entries.push(t)
    }
}

//TODO(ANR): expand to include enums
#[macro_export]
macro_rules! possum_node_type {
    // struct
    {
        $(#[$outer:meta])*
        struct $name:ident {
        $(
            $( #[$inner:ident $($args:tt)*])*
            $field:ident: $t:ty,
        )*
        }
    } => {
        $(#[$outer])*
        pub struct $name {
        $(
            $(#[$inner $(args)*])*
            pub $field: Option<$crate::scavenge::ast::PossumNode<$t>>,
        )*
        }
    };
    // terminal
    () => {};
}

pub(crate) use possum_node_type;
