#[macro_use]
pub mod ast;
pub mod parser;

pub use self::parser::{parse, ParseFailure, Parser};
