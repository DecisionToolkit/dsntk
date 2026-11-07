//! # FEEL regex errors

use dsntk_common::{DsntkError, ToErrorMessage};
use dsntk_macros::ToErrorMessage;

/// `FEEL` expressions evaluator errors.
#[derive(ToErrorMessage)]
struct FeelRegexError(String);

pub fn err_empty_flags() -> DsntkError {
    FeelRegexError("empty regex flags".to_string()).into()
}

pub fn err_invalid_flag(flag: char) -> DsntkError {
    FeelRegexError(format!("invalid regex flag: '{flag}'").to_string()).into()
}

pub fn err_invalid_pattern(pattern: &str) -> DsntkError {
    FeelRegexError(format!("invalid regex pattern: '{pattern}'").to_string()).into()
}
