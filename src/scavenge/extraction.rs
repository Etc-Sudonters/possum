use crate::document::{Annotation, AsDocumentPointer};

use super::ast::PossumNodeKind;
use super::yaml::YamlKind;
use std::fmt::Display;
use std::num::ParseIntError;
use yaml_peg::repr::Repr;
use yaml_peg::{Map, Node as YamlNode, Seq, Yaml};

#[derive(Debug)]
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

impl UnexpectedYaml {
    pub fn at<P>(self, at: &P) -> Annotation
    where
        P: AsDocumentPointer,
    {
        Annotation::fatal(at, &self)
    }
}

pub trait IntoYamlKind {
    fn into_yaml_kind(&self) -> YamlKind;
}

impl<R> IntoYamlKind for &Yaml<R>
where
    R: Repr,
{
    fn into_yaml_kind(&self) -> YamlKind {
        use yaml_peg::Yaml::*;

        match self {
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

impl<R> IntoYamlKind for YamlNode<R>
where
    R: Repr,
{
    fn into_yaml_kind(&self) -> YamlKind {
        self.yaml().into_yaml_kind()
    }
}
impl<R> IntoYamlKind for &YamlNode<R>
where
    R: Repr,
{
    fn into_yaml_kind(&self) -> YamlKind {
        self.yaml().into_yaml_kind()
    }
}

impl IntoYamlKind for YamlKind {
    fn into_yaml_kind(&self) -> YamlKind {
        self.clone()
    }
}

#[derive(Debug)]
pub enum ExpectedYaml {
    Only(YamlKind),
    AnyOf(Vec<YamlKind>),
}

impl ExpectedYaml {
    pub fn but_found<Y>(self, n: Y) -> UnexpectedYaml
    where
        Y: IntoYamlKind,
    {
        UnexpectedYaml {
            expected: self,
            unexpected: n.into_yaml_kind(),
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
    fn extract_bool(&'a self) -> Extraction<bool>;
    fn extract_number(&'a self) -> Extraction<f64>;
}

impl<'a, R> Extract<'a, R> for YamlNode<R>
where
    R: Repr,
{
    fn extract_map(&'a self) -> Extraction<&'a Map<R>> {
        self.yaml().extract_map()
    }

    fn extract_str(&'a self) -> Extraction<&'a str> {
        self.yaml().extract_str()
    }

    fn extract_seq(&'a self) -> Extraction<&'a Seq<R>> {
        self.yaml().extract_seq()
    }

    fn extract_bool(&'a self) -> Extraction<bool> {
        self.yaml().extract_bool()
    }

    fn extract_number(&'a self) -> Extraction<f64> {
        self.as_number()
            .map_err(|_| ExpectedYaml::Only(YamlKind::Number).but_found(self))
    }
}

impl<'a, R> Extract<'a, R> for Yaml<R>
where
    R: Repr,
{
    fn extract_map(&'a self) -> Extraction<&'a Map<R>> {
        match self {
            Yaml::Map(m) => Ok(m),
            _ => Err(ExpectedYaml::Only(YamlKind::Map).but_found(self)),
        }
    }

    fn extract_str(&'a self) -> Extraction<&'a str> {
        match self {
            Yaml::Str(s) => Ok(s),
            _ => Err(ExpectedYaml::Only(YamlKind::Str).but_found(self)),
        }
    }

    fn extract_seq(&'a self) -> Extraction<&'a Seq<R>> {
        match self {
            Yaml::Seq(s) => Ok(s),
            _ => Err(ExpectedYaml::Only(YamlKind::Seq).but_found(self)),
        }
    }

    fn extract_bool(&'a self) -> Extraction<bool> {
        match self {
            Yaml::Bool(true) => Ok(true),
            Yaml::Bool(false) => Ok(false),
            _ => Err(ExpectedYaml::Only(YamlKind::Bool).but_found(self)),
        }
    }

    fn extract_number(&'a self) -> Extraction<f64> {
        match self {
            Yaml::Int(raw) => match i64_from_yaml(raw) {
                Ok(i) => Ok(i as f64),
                Err(_) => Err(ExpectedYaml::Only(YamlKind::Number).but_found(YamlKind::Str)),
            },
            Yaml::Float(raw) => raw
                .parse()
                .map_err(|_| ExpectedYaml::Only(YamlKind::Number).but_found(YamlKind::Str)),
            u @ _ => Err(ExpectedYaml::Only(YamlKind::Number).but_found(u)),
        }
    }
}

fn i64_from_yaml(raw: &str) -> Result<i64, ParseIntError> {
    if raw.starts_with("0x") {
        return i64::from_str_radix(&raw.replace("0x", ""), 16);
    } else if raw.starts_with("0o") {
        return i64::from_str_radix(&raw.replace("0o", ""), 8);
    }
    raw.parse()
}

impl<T> Into<PossumNodeKind<T>> for Extraction<T> {
    fn into(self) -> PossumNodeKind<T> {
        match self {
            Ok(t) => PossumNodeKind::Value(t),
            Err(u) => PossumNodeKind::Invalid(u.to_string()),
        }
    }
}
