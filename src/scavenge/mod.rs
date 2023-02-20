#[macro_use]
pub mod ast;
pub mod extraction;
pub mod parser;
pub mod yaml;

pub use self::parser::{parse_single_document, MapParser, ParseFailure, Parser, UnexpectedKey};

pub enum Fallible<T> {
    Success,
    Failure(T),
}
