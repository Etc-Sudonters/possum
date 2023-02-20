use crate::document::{Annotation, Annotations};
use crate::scavenge::ast::{PossumMap, PossumNodeKind};
use crate::scavenge::extraction::{ExpectedYaml, Extract};
use crate::scavenge::yaml::YamlKind;
use crate::scavenge::Parser;
use crate::workflow::{Grant, Permission};
use yaml_peg::repr::Repr;
use yaml_peg::Yaml;

pub struct PermissionParser<'a> {
    annotations: &'a mut Annotations,
}

impl<'a> PermissionParser<'a> {
    pub fn new(a: &'a mut Annotations) -> PermissionParser<'a> {
        PermissionParser { annotations: a }
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
        u @ _ => Invalid(format!("Expected read, write or none but found {u}")),
    }
}

impl<'a, R> Parser<R, Permission> for PermissionParser<'a>
where
    R: Repr,
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
                g @ _ => Invalid(format!("Expected read-all or write-all, but found {g}")),
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
            u @ _ => Invalid(
                ExpectedYaml::AnyOf(vec![YamlKind::Str, YamlKind::Map])
                    .but_found(u)
                    .to_string(),
            ),
        }
    }
}
