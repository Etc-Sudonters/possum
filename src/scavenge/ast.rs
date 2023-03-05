use crate::document::{AsDocumentPointer, DocumentPointer};
use std::iter::{FromIterator, IntoIterator, Zip};
use std::slice::Iter;

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

    pub fn kind(&self) -> &PossumNodeKind<T> {
        &self.kind
    }

    pub fn value(&self) -> Option<&T> {
        match self.kind() {
            PossumNodeKind::Value(t) => Some(t),
            _ => None,
        }
    }
}

impl<T> AsDocumentPointer for &PossumNode<T> {
    fn as_document_pointer(&self) -> DocumentPointer {
        self.loc()
    }
}

impl<T> AsDocumentPointer for PossumNode<T> {
    fn as_document_pointer(&self) -> DocumentPointer {
        AsDocumentPointer::as_document_pointer(&self)
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

    pub fn map<U>(self, f: impl Fn(T) -> U) -> PossumNodeKind<U> {
        use PossumNodeKind::*;
        match self {
            Value(t) => Value(f(t)),
            Expr(s) => Expr(s),
            Invalid(s) => Invalid(s),
            Empty => Empty,
        }
    }

    pub fn flatmap<U>(self, f: impl Fn(T) -> PossumNodeKind<U>) -> PossumNodeKind<U> {
        use PossumNodeKind::*;
        match self {
            Value(t) => f(t),
            Expr(s) => Expr(s),
            Invalid(s) => Invalid(s),
            Empty => Empty,
        }
    }

    pub fn recover(self, mut f: impl FnMut() -> PossumNodeKind<T>) -> PossumNodeKind<T> {
        use PossumNodeKind::*;
        match self {
            Value(t) => Value(t),
            Expr(s) => Expr(s),
            Empty => Empty,
            Invalid(_) => f(),
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

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn insert(&mut self, k: PossumNode<K>, v: PossumNode<V>) {
        self.keys.push(k);
        self.values.push(v);
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn iter(&self) -> Zip<Iter<'_, PossumNode<K>>, Iter<'_, PossumNode<V>>> {
        self.keys.iter().zip(self.values.iter()).into_iter()
    }
}

#[derive(Debug, Default)]
pub struct PossumSeq<T> {
    entries: Vec<PossumNode<T>>,
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

    pub fn new<I>(nodes: I) -> PossumSeq<T>
    where
        I: IntoIterator<Item = PossumNode<T>>,
    {
        nodes.into_iter().collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> Iter<'_, PossumNode<T>> {
        self.entries.iter()
    }
}

impl<T> Into<PossumSeq<T>> for PossumNode<T> {
    fn into(self) -> PossumSeq<T> {
        PossumSeq {
            entries: vec![self],
        }
    }
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
