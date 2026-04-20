#![doc = include_str!("../docs/README.md")]
#![deny(rustdoc::broken_intra_doc_links)]

mod errors;
mod impl_regex;
mod unicode_blocks;
mod utils;

pub use impl_regex::{is_match, is_match_with_flags};
