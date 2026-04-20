#![doc = include_str!("../docs/README.md")]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate dsntk_macros;

mod bifs;
mod builders;
mod errors;
mod evaluator_java;
mod evaluator_pmml;
mod evaluators;
mod filters;
mod iterations;
mod macros;
mod tests;

pub use crate::evaluators::*;
pub use crate::filters::FilterExpressionEvaluator;
pub use crate::iterations::{EveryExpressionEvaluator, ForExpressionEvaluator, SomeExpressionEvaluator};
