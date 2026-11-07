//! # FEEL evaluator errors

use dsntk_common::{DsntkError, ToErrorMessage};

/// `FEEL` expressions evaluator errors.
#[derive(ToErrorMessage)]
struct FeelEvaluatorError(String);

pub fn err_not_a_context() -> DsntkError {
    FeelEvaluatorError("expected FEEL context on input".to_string()).into()
}

pub fn err_not_a_range() -> DsntkError {
    FeelEvaluatorError("expected FEEL range on input".to_string()).into()
}

pub fn err_invalid_interval_start(start_type: &str) -> DsntkError {
    FeelEvaluatorError(format!("invalid type of interval start: {start_type}")).into()
}

pub fn err_invalid_interval_end(expected: &str, end_type: &str) -> DsntkError {
    FeelEvaluatorError(format!(
        "invalid type of interval end, expected {expected}, actual: {end_type}"
    ))
    .into()
}

pub fn err_invalid_range_start(value_type: &str) -> DsntkError {
    FeelEvaluatorError(format!("invalid range start: {value_type}")).into()
}

pub fn err_invalid_range_end(value_type: &str) -> DsntkError {
    FeelEvaluatorError(format!("invalid range end: {value_type}")).into()
}

pub fn err_invalid_range() -> DsntkError {
    FeelEvaluatorError("range start must be less or equal then range end".to_string()).into()
}
