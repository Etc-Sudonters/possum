use crate::document::{Annotation, Annotations};
use crate::scavenge::ast::{PossumMap, PossumNodeKind};
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::yaml::YamlKind;
use crate::scavenge::Parser;
use crate::workflow::{Grant, Permission};
use std::marker::PhantomData;
use yaml_peg::repr::Repr;
use yaml_peg::Yaml;

pub struct PermissionParser<'a, R>
where
    R: Repr + 'a,
{
    _x: PhantomData<R>,
    annotations: &'a mut Annotations,
}

impl<'a, R> PermissionParser<'a, R>
where
    R: Repr + 'a,
{
    pub fn new(a: &'a mut Annotations) -> PermissionParser<'a, R> {
        PermissionParser {
            _x: PhantomData,
            annotations: a,
        }
    }

    fn annotate<A>(&mut self, a: A)
    where
        A: Into<Annotation>,
    {
        self.annotations.add(a)
    }
}

fn parse_individual_grant(grant: &str) -> PossumNodeKind<Grant> {
    use PossumNodeKind::{Invalid, Value};
    match grant.to_lowercase().as_str() {
        "read" => Value(Grant::Read),
        "write" => Value(Grant::Write),
        "none" => Value(Grant::Deny),
        u @ _ => Invalid("Expected read, write or none but found {u}".to_owned()),
    }
}

impl<'a, R> Parser<'a, R, Permission> for PermissionParser<'a, R>
where
    R: Repr + 'a,
{
    fn parse_node(&mut self, root: &yaml_peg::Node<R>) -> PossumNodeKind<Permission>
    where
        R: Repr,
    {
        use PossumNodeKind::*;
        match root.yaml() {
            Yaml::Str(all) => match all.to_lowercase().as_str() {
                "read-all" => Value(Permission::GlobalGrant(Grant::Read)),
                "write-all" => Value(Permission::GlobalGrant(Grant::Write)),
                g @ _ => Invalid("Expected read-all or write-all, but found {g}".to_owned()),
            },
            Yaml::Map(each) => {
                let mut perms = PossumMap::empty();

                for (key, value) in each.iter() {
                    match key.extract_str() {
                        Err(u) => self.annotate(u.at(key)),
                        Ok(k) => {
                            let grant: PossumNodeKind<Grant> = value
                                .extract_str()
                                .map_or_else(|u| Invalid(u.to_string()), parse_individual_grant);
                            perms.insert(Value(k.to_owned()).at(key), grant.at(value));
                        }
                    }
                }

                Value(Permission::IndividualGrants(perms))
            }
            Yaml::Null => todo!(),
            u @ _ => Invalid(
                ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Map])
                    .but_found(u)
                    .to_string(),
            ),
        }
    }
}
