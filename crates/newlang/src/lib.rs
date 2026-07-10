#![forbid(unsafe_code)]
#![doc = "Core compiler crate for the new language."]

pub mod ast;
pub mod lexer;
pub mod module;
pub mod parser;
pub mod source;
