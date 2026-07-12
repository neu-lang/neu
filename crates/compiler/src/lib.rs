#![forbid(unsafe_code)]
#![doc = "Core compiler crate for the new language."]

pub mod ast;
pub mod backend;
pub mod bootstrap;
pub mod borrow;
pub mod coroutine;
pub mod dependency;
pub mod driver;
pub mod hir;
pub mod lexer;
pub mod linker;
pub mod manifest;
pub mod mir;
pub mod module;
pub mod name_resolution;
pub mod ownership;
pub mod ownership_effects;
pub mod parser;
pub mod source;
pub mod symbol;
pub mod target_pack;
pub mod thread;
pub mod type_check;
pub mod types;
pub mod unsafe_boundary;
