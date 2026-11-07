//! # Error definitions for TCK handler

use dsntk_common::{DsntkError, ToErrorMessage};

/// Server errors for TCK handler.
#[derive(ToErrorMessage)]
struct TckServerError(String);

pub fn err_missing_attribute(name: &str) -> DsntkError {
    TckServerError(format!("missing attribute: {name}")).into()
}
