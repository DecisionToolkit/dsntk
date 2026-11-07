//! Errors reported by workspace.

use dsntk_common::{DsntkError, ToErrorMessage};

/// Errors reported by workspace.
#[derive(ToErrorMessage)]
struct WorkspaceError(String);

pub fn err_invocable_not_found(invocable_path: &str) -> DsntkError {
    WorkspaceError(format!("invocable not found: '{invocable_path}'")).into()
}
