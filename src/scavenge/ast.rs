use crate::document::DocumentPointer;
use std::iter::FromIterator;

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
}

impl<T> PossumNodeKind<T> {
    pub fn at(self, location: DocumentPointer) -> PossumNode<T> {
        PossumNode::new(location, self)
    }
}

#[derive(Debug)]
pub struct PossumMap<K, V> {
    keys: PossumSeq<K>,
    values: PossumSeq<V>,
}

impl<K, V> PossumMap<K, V> {
    pub fn new() -> PossumMap<K, V> {
        PossumMap {
            keys: PossumSeq::new(),
            values: PossumSeq::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct PossumSeq<T> {
    entries: Vec<PossumNode<T>>,
}

impl<T> Into<PossumSeq<T>> for Vec<PossumNode<T>> {
    fn into(self) -> PossumSeq<T> {
        PossumSeq { entries: self }
    }
}

impl<T> PossumSeq<T> {
    pub fn new() -> PossumSeq<T> {
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
