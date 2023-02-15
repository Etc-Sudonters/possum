use super::yaml::YamlKind;
use std::fmt::Display;
use yaml_peg::repr::Repr;
use yaml_peg::{Map, Seq, Yaml};

pub struct UnexpectedYaml {
    expected: ExpectedYaml,
    unexpected: YamlKind,
}

impl Display for UnexpectedYaml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "expected {} but found {}",
            self.expected, self.unexpected
        )
    }
}

pub enum ExpectedYaml {
    Only(YamlKind),
    AnyOf(Vec<YamlKind>),
}

impl ExpectedYaml {
    pub fn but_found<Y>(self, n: Y) -> UnexpectedYaml
    where
        Y: Into<YamlKind>,
    {
        UnexpectedYaml {
            expected: self,
            unexpected: n.into(),
        }
    }
}

impl PartialEq<YamlKind> for ExpectedYaml {
    fn eq(&self, other: &YamlKind) -> bool {
        match self {
            Self::Only(y) => y == other,
            Self::AnyOf(ys) => ys.contains(other),
        }
    }
}

impl Display for ExpectedYaml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Only(o) => write!(f, "{}", o),
            Self::AnyOf(these) => write!(f, "{:?}", these),
        }
    }
}

pub type Extraction<T> = Result<T, UnexpectedYaml>;

pub trait Extract<'a, R>
where
    R: Repr,
{
    fn extract_map(&'a self) -> Extraction<&'a Map<R>>;
    fn extract_str(&'a self) -> Extraction<&'a str>;
    fn extract_seq(&'a self) -> Extraction<&'a Seq<R>>;
}

impl<'a, R> Extract<'a, R> for Yaml<R>
where
    R: Repr,
{
    fn extract_map(&'a self) -> Extraction<&'a Map<R>> {
        match self {
            Yaml::Map(m) => Ok(m),
            _ => Err(ExpectedYaml::Only(YamlKind::Map).but_found(YamlKind::from_yaml(self))),
        }
    }

    fn extract_str(&'a self) -> Extraction<&'a str> {
        match self {
            Yaml::Str(s) => Ok(s),
            _ => Err(ExpectedYaml::Only(YamlKind::Str).but_found(YamlKind::from_yaml(self))),
        }
    }

    fn extract_seq(&'a self) -> Extraction<&'a Seq<R>> {
        match self {
            Yaml::Seq(s) => Ok(s),
            _ => Err(ExpectedYaml::Only(YamlKind::Seq).but_found(YamlKind::from_yaml(self))),
        }
    }
}
