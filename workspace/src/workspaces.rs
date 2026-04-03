//! # Container for decision model evaluators

use crate::builder::WorkspaceBuilder;
use crate::errors::*;
use antex::ColorMode;
use dsntk_common::Result;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_model_evaluator::ModelEvaluator;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// Container for decision model evaluators.
pub struct Workspaces {
  /// Map: invocable path -> (workspace name, model namespace, model name, invocable name)
  pub(crate) invocables: HashMap<String, (String, String, String, String)>,
  /// Map: workspace name -> model evaluator
  pub(crate) evaluators: HashMap<String, Arc<ModelEvaluator>>,
}

impl Workspaces {
  /// Creates a new [Workspaces] and loads decision models from specified directory.
  pub fn new(dirs: &[PathBuf], colors: ColorMode, verbose: bool) -> Self {
    let builder = WorkspaceBuilder::new(dirs, colors, verbose);
    Self {
      invocables: builder.invocables,
      evaluators: builder.evaluators,
    }
  }

  /// Evaluates invocable identified by invocable path.
  pub fn evaluate(&self, invocable_path: &str, input_data: &FeelContext) -> Result<Value> {
    if let Some((workspace_name, model_namespace, model_name, invocable_name)) = self.invocables.get(invocable_path)
      && let Some(evaluator) = self.evaluators.get(workspace_name)
    {
      return Ok(evaluator.evaluate_invocable(model_namespace, model_name, invocable_name, input_data));
    }
    Err(err_invocable_not_found(invocable_path))
  }
}
