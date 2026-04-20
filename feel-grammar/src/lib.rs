#![doc = include_str!("../docs/README.md")]
#![deny(rustdoc::broken_intra_doc_links)]

mod extractor;
mod generator;

pub use generator::lalr_rust_tables;
