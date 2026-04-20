#![doc = include_str!("../docs/README.md")]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate dsntk_macros;

mod errors;
mod href;
mod idents;
mod jsonify;
mod namespace;
mod uri;

pub use errors::{DsntkError, Result, ToErrorMessage};
pub use href::HRef;
pub use idents::gen_id;
pub use jsonify::Jsonify;
pub use namespace::to_rdnn;
pub use uri::{Uri, encode_segments, to_uri};
