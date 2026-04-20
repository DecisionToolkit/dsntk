#![doc = include_str!("../docs/README.md")]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]

#[macro_use]
extern crate dsntk_macros;

mod builder;
mod canvas;
mod errors;
mod markdown;
mod model;
mod plane;
mod point;
mod recognizer;
mod rect;
mod tests;
mod utils;

pub use builder::from_unicode;
pub use markdown::from_markdown;

pub use model::{
  AnnotationClause, AnnotationEntry, BuiltinAggregator, DecisionRule, DecisionTable, DecisionTableOrientation, HitPolicy, InputClause, InputEntry, OutputClause, OutputEntry,
};
