#[derive(Debug)]
pub struct PossumNode<T> {
    location: usize,
    kind: NodeKind<T>,
}

#[derive(Debug)]
pub enum NodeKind<T> {
    Invalid(String),
    Expr(String),
    Value(T),
}

#[derive(Debug)]
pub struct Map<K, V> {
    keys: Seq<K>,
    values: Seq<V>,
}

#[derive(Debug)]
pub struct Seq<T> {
    entries: Vec<PossumNode<T>>,
}

//TODO(ANR): expand to include enums
macro_rules! possum_node {
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
            $field: Option<$crate::scavenge::ast::PossumNode<$t>>,
        )*
        }
    };
    // terminal
    () => {};
}
