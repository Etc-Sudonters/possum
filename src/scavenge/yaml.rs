use yaml_peg::repr::Repr;
use yaml_peg::{Node as YamlNode, Yaml};

#[derive(Debug, strum::Display, PartialEq, Eq)]
pub enum YamlKind {
    Map,
    Seq,
    Str,
    Bool,
    Number,
    Alias,
    Null,
}

impl YamlKind {
    pub fn from_yaml_node<R>(n: &YamlNode<R>) -> YamlKind
    where
        R: Repr,
    {
        Self::from_yaml(n.yaml())
    }
    pub fn from_yaml<R>(n: &Yaml<R>) -> YamlKind
    where
        R: Repr,
    {
        use yaml_peg::Yaml::*;

        match n {
            Null => YamlKind::Null,
            Map(_) => YamlKind::Map,
            Seq(_) => YamlKind::Seq,
            Bool(_) => YamlKind::Bool,
            Int(_) | Float(_) => YamlKind::Number,
            Str(_) => YamlKind::Str,
            Alias(_) => YamlKind::Alias,
        }
    }
}

impl<R> Into<YamlKind> for YamlNode<R>
where
    R: Repr,
{
    fn into(self) -> YamlKind {
        YamlKind::from_yaml_node(&self)
    }
}
