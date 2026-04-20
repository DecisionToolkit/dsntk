#![doc = include_str!("../docs/README.md")]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate dsntk_macros;

mod builder;
mod errors;
mod workspaces;

pub use workspaces::Workspaces;
