use crate::errors::*;
use dsntk_common::Result;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::{Evaluator, FeelScope};
use dsntk_feel_parser::AstNode;

/// Evaluates a [Value] from given [AstNode].
pub fn evaluate(scope: &FeelScope, node: &AstNode) -> Value {
  crate::builders::build_evaluator(node)(scope)
}

/// Prepares an evaluator for given [AstNode].
pub fn prepare(node: &AstNode) -> Evaluator {
  crate::builders::build_evaluator(node)
}

/// Evaluates the sum of specified values.
pub fn evaluate_sum(values: Vec<Value>) -> Value {
  crate::bifs::core::sum(&values)
}

/// Evaluates the minimum value from specified values.
pub fn evaluate_min(values: Vec<Value>) -> Value {
  crate::bifs::core::min(&values)
}

/// Evaluates the maximum value from specified values.
pub fn evaluate_max(values: Vec<Value>) -> Value {
  crate::bifs::core::max(&values)
}

/// Compares two values and returns `true` when the two `FEEL` values are equal.
pub fn evaluate_equals(left: &Value, right: &Value) -> bool {
  crate::builders::eval_ternary_equality(left, right).unwrap_or(false)
}

/// Evaluates a context from text containing `FEEL` expression.
pub fn evaluate_context(scope: &FeelScope, input: &str) -> Result<FeelContext> {
  let node = dsntk_feel_parser::parse_context(scope, input, false)?;
  let evaluator = crate::builders::build_evaluator(&node);
  // `Evaluator` has no lifetime parameter, so an evaluator can never borrow the
  // parsed syntax tree. The tree is dead here. For a large data literal, it is
  // the biggest of the three input representations. Free it before the evaluator
  // builds the value tree.
  drop(node);
  if let Value::Context(context) = evaluator(scope) {
    Ok(context)
  } else {
    Err(err_not_a_context())
  }
}

/// Evaluates a context from AST node.
pub fn evaluate_context_node(scope: &FeelScope, node: &AstNode) -> Result<FeelContext> {
  let evaluator = crate::builders::build_evaluator(node);
  if let Value::Context(context) = evaluator(scope) {
    Ok(context)
  } else {
    Err(err_not_a_context())
  }
}

/// Evaluates a range value from range literal expression.
pub fn evaluate_range_literal(scope: &FeelScope, input: &str) -> Result<Value> {
  let node = &dsntk_feel_parser::parse_range_literal(scope, input, false)?;
  evaluate_range_literal_node(scope, node)
}

/// Evaluates a range value from AST node.
pub fn evaluate_range_literal_node(scope: &FeelScope, node: &AstNode) -> Result<Value> {
  let evaluator = crate::builders::build_evaluator(node);
  if let value @ Value::Range(_, _, _, _) = evaluator(scope) {
    Ok(value)
  } else {
    Err(err_not_a_range())
  }
}
