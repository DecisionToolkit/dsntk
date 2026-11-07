//! # Error definitions

use dsntk_common::{DsntkError, ToErrorMessage};

/// Command-line action error.
#[derive(ToErrorMessage)]
struct ActionError(String);

/// Error related to creating directory.
pub fn err_create_directory(path: &str, reason: &str) -> DsntkError {
    ActionError(format!(
        "creating directory '{path}' failed with reason: {reason}"
    ))
    .into()
}

/// Error related to saving a file.
pub fn err_save_file(path: &str, reason: &str) -> DsntkError {
    ActionError(format!("saving file '{path}' failed with reason: {reason}")).into()
}
