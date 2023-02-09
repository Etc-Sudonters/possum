use yaml_peg::repr::Repr;
use yaml_peg::{Map, Node, Seq, Yaml};

pub trait ParserVisitor<R: Repr> {
    fn visit_on(&mut self, on: &Node<R>) {
        match on.yaml() {
            Yaml::Str(on) => self.visit_on_str(on),
            Yaml::Seq(on) => self.visit_on_seq(on),
            Yaml::Map(on) => self.visit_on_map(on),
            _ => self.visit_on_invalid(on),
        }
    }

    fn visit_on_str(&mut self, on: &str);
    fn visit_on_seq(&mut self, on: &Seq<R>);
    fn visit_on_map(&mut self, on: &Map<R>);
    fn visit_on_invalid(&mut self, on: &Node<R>) {}

    fn visit_name(&mut self, name: &str);
    fn visit_invalid_name(&mut self, name: &Node<R>) {}

    fn visit_run_name(&mut self, run_name: &str);
    fn visit_invalid_run_name(&mut self, run_name: &Node<R>) {}
}
