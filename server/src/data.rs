//! # Shared application data

use dsntk_workspace::Workspaces;
use std::sync::Arc;

/// Workspaces with decision model evaluators.
pub struct ApplicationData {
    pub workspaces: Arc<Workspaces>,
}
