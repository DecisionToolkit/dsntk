#![doc = include_str!("../docs/README.md")]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate dsntk_macros;

mod errors;
mod mapper;
mod model;
mod parser;
mod tests;
mod validators;
mod xml_utils;

pub use model::*;
pub use parser::parse;
