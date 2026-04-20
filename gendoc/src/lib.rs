#![doc = include_str!("../docs/README.md")]
#![deny(rustdoc::broken_intra_doc_links)]

mod ascii_model;
mod auto_size;
mod defs;
mod generator;
mod horizontal_decision_table;
mod styles;
mod tests;

use crate::generator::HTMLGenerator;
pub use ascii_model::print_model;
use dsntk_model::{DecisionTable, Definitions};

/// Generates HTML documentation from decision model.
pub fn dmn_model_to_html(definitions: &Definitions) -> String {
  let mut html_generator = HTMLGenerator::new();
  html_generator.dmn_model_to_html(definitions)
}

/// Generates HTML documentation from decision table.
pub fn decision_table_to_html(decision_table: &DecisionTable) -> String {
  let html_generator = HTMLGenerator::new();
  html_generator.decision_table_to_html(decision_table)
}
