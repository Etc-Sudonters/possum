pub struct Node<T> {
    location: usize,
    kind: NodeKind<T>,
}

pub enum NodeKind<T> {
    Invalid(String),
    Expr(String),
    Value(T),
}

pub struct Map<K, V> {
    keys: Seq<K>,
    values: Seq<V>,
}

pub struct Seq<T> {
    entries: Vec<Node<T>>,
}

//TODO(ANR): expand to include enums
macro_rules! node {
    () => {};
    {
        $(#[$outer:meta])*
        struct $name:ident {
        $(
            $( #[$inner:ident $($args:tt)*])*
            $field:ident: $t:ty,
        )*
        }
    } => {
        $(#[$outer:meta])*
        pub struct $name {
        $(
            $(#[$inner $(args)*])*
            $field: $crate::scavenge::workflow::ast::Node<$t>,
        )*
        }
    };
}
