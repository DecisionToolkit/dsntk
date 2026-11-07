//! # Evaluator error definitions

use dsntk_common::{DsntkError, ToErrorMessage};
use dsntk_feel_parser::AstNode;

/// Evaluator error.
#[derive(ToErrorMessage)]
struct EvaluatorError(String);

/// Error reported when the expected node is not an expression list.
pub fn err_expected_expression_list(other: &AstNode) -> DsntkError {
    EvaluatorError(format!("expected expression list, but found '{other}'")).into()
}

/// Error reported when the expression list has not exactly 2 elements.
pub fn err_expected_two_elements_in_expression_list(count: usize) -> DsntkError {
    EvaluatorError(format!(
        "expression list must have exactly 2 elements, found {count}"
    ))
    .into()
}
