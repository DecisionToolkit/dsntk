#![doc = include_str!("../docs/README.md")]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate dsntk_macros;

mod errors;
mod test_files;

pub use dsntk_feel_evaluator::{evaluate, evaluate_context, evaluate_equals, evaluate_max, evaluate_min, evaluate_sum};
pub use dsntk_model_evaluator::{ModelEvaluator, build_decision_table_evaluator};
pub use test_files::prepare_test_cases;
