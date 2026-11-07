use dsntk_common::{DsntkError, ToErrorMessage};

/// `FEEL` number errors.
#[derive(ToErrorMessage)]
struct FeelNumberError(String);

/// Creates invalid number literal error.
pub fn err_invalid_number_literal(s: &str) -> DsntkError {
    FeelNumberError(format!("invalid number literal '{s}'")).into()
}

/// Creates number conversion error.
pub fn err_number_conversion_failed() -> DsntkError {
    FeelNumberError("number conversion failed".to_string()).into()
}

/// Creates an error describing invalid scale.
pub fn err_invalid_scale(min: i32, max: i32, scale: i32) -> DsntkError {
    FeelNumberError(format!(
        "invalid scale, allowed range is {min}..{max}, actual is {scale}"
    ))
    .into()
}
