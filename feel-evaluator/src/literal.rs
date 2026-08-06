//! # Direct value construction for input that is pure data
//!
//! Evaluating a `FEEL` expression takes three steps: parse the text into a syntax tree, build a
//! tree of boxed closures from it, then run the closures to produce a value. Input data files
//! are not expressions. They are data: nested contexts and lists whose leaves are strings,
//! numbers, booleans and nulls. For such input the closure tree is a whole intermediate
//! representation built and thrown away for nothing.
//!
//! [literal_value] reads an accepted syntax tree straight into a [Value] and skips that step.
//!
//! # What is accepted
//!
//! Only [AstNode::Boolean], [AstNode::Null], [AstNode::Numeric], [AstNode::String],
//! [AstNode::Neg], [AstNode::List] and [AstNode::Context] whose entries are all
//! [AstNode::ContextEntry] with an [AstNode::ContextEntryKey] key. Every other node kind
//! rejects the whole tree, which then takes the normal path. The test is purely syntactic: it
//! never looks at a name, a scope or a value.
//!
//! # Why the syntax tree is read and not consumed
//!
//! Consuming the tree frees each record as soon as its value exists, which lowers the live byte
//! count. Measurement showed that it also raises peak RSS by 13%: freeing the tree a record at a
//! time, while the value tree grows, spreads `mimalloc`'s committed arena slices instead of
//! releasing them in one block. Reading the tree and freeing it whole afterwards keeps the
//! allocation pattern the previous code had, and the peak was never in this phase anyway. The
//! parse itself, with the lexer buffer and the parser stacks live, sets the high-water mark.
//!
//! # Why skipping the scope is safe
//!
//! [crate::builders] pushes a context onto the scope while it evaluates context entries, so that
//! a later entry can read an earlier one by name. A `FEEL` expression reads that context only
//! through an [AstNode::Name] or an [AstNode::Path], and both reject the tree. No accepted node
//! reads the scope at all, so this module needs no scope.
//!
//! # Where the results must stay identical
//!
//! The value produced here must equal the value the closure tree produces, byte for byte,
//! including error text. Four places carry that risk, and each one copies the corresponding
//! builder rather than reimplementing it:
//!
//! - number text is reassembled and parsed exactly as `build_number` does it,
//! - a duplicated key yields the same null and the same message as `build_context`,
//! - negation reproduces every arm of `build_neg`, including its error text,
//! - [AstNode::At] is not accepted, because it needs a temporal parse.
//!
//! Nothing here can panic on an unexpected node. Every function returns [None] for a node it
//! does not accept, and the caller then takes the normal path, so a gap between [is_data_literal]
//! and [build_value] costs a fast path and never a wrong answer.

use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::{FeelNumber, value_null};
use dsntk_feel_parser::AstNode;
use std::fmt::Write;

/// Reads a syntax tree that is pure data into a [Value].
///
/// Returns [None] when any node in the tree is not a data literal.
pub fn literal_value(node: &AstNode) -> Option<Value> {
  // The check runs first and allocates nothing, so a tree that is rejected at its last record
  // costs one pointer walk instead of a whole value tree that is then thrown away.
  if !is_data_literal(node) {
    return None;
  }
  build_value(node)
}

/// Returns `true` when every node in the subtree is a data literal.
fn is_data_literal(node: &AstNode) -> bool {
  match node {
    AstNode::Boolean(_) | AstNode::Null | AstNode::Numeric(..) | AstNode::String(_) => true,
    AstNode::Neg(value) => is_data_literal(value),
    AstNode::List(items) => items.iter().all(is_data_literal),
    AstNode::Context(entries) => entries.iter().all(is_data_literal_entry),
    _ => false,
  }
}

/// Returns `true` when the node is a context entry with a literal key and a literal value.
fn is_data_literal_entry(node: &AstNode) -> bool {
  match node {
    AstNode::ContextEntry(key, value) => matches!(key.as_ref(), AstNode::ContextEntryKey(_)) && is_data_literal(value),
    _ => false,
  }
}

/// Builds the value of a data literal subtree, or [None] for any other node.
fn build_value(node: &AstNode) -> Option<Value> {
  Some(match node {
    AstNode::Boolean(value) => Value::Boolean(*value),
    AstNode::Null => Value::Null(None),
    AstNode::String(text) => Value::String(text.clone()),
    AstNode::Numeric(before, after, sign, exponent) => number_value(before, after, *sign, exponent),
    AstNode::Neg(value) => negated_value(build_value(value)?),
    AstNode::List(items) => {
      let mut values = Vec::with_capacity(items.len());
      for item in items {
        values.push(build_value(item)?);
      }
      Value::List(values)
    }
    AstNode::Context(entries) => context_value(entries)?,
    _ => return None,
  })
}

/// Builds a context value from the entries of an [AstNode::Context].
///
/// This is `build_context` without the scope. The duplicated key check, its message and the
/// order of the entries are the same, so both paths return the same value.
fn context_value(entries: &[AstNode]) -> Option<Value> {
  let mut evaluated_ctx = FeelContext::default();
  for entry in entries {
    let AstNode::ContextEntry(key, value) = entry else {
      return None;
    };
    let AstNode::ContextEntryKey(name) = key.as_ref() else {
      return None;
    };
    if evaluated_ctx.contains_entry(name) {
      // duplicated context entry keys are not allowed
      return Some(value_null!("duplicated context entry key: {}", name));
    }
    evaluated_ctx.set_entry(name, build_value(value)?);
  }
  Some(Value::Context(evaluated_ctx))
}

/// Builds a number value from the four parts of an [AstNode::Numeric].
///
/// This is the body of `build_number`. The text is reassembled the same way and parsed by the
/// same implementation, so a number reaches the same last digit on both paths.
fn number_value(before: &str, after: &str, sign: char, exponent: &str) -> Value {
  let mut text = String::new();
  let _ = write!(&mut text, "{before}");
  if !after.is_empty() {
    let _ = write!(&mut text, ".{after}");
  }
  if !exponent.is_empty() {
    let _ = write!(&mut text, "e{sign}{exponent}");
  }
  if let Ok(num) = text.parse::<FeelNumber>() {
    Value::Number(num)
  } else {
    value_null!("failed to convert text '{}' into number", text)
  }
}

/// Negates a value.
///
/// This is the body of `build_neg`. Only the number arm and the error arm are reachable from a
/// data literal, but every arm is kept so that the two paths cannot drift apart.
fn negated_value(value: Value) -> Value {
  match value {
    Value::Number(lh) => Value::Number(-lh),
    Value::DaysAndTimeDuration(lh) => Value::DaysAndTimeDuration(-lh),
    Value::YearsAndMonthsDuration(lh) => Value::YearsAndMonthsDuration(-lh),
    other => value_null!("unexpected type in arithmetic negation: {}", other.type_of()),
  }
}

#[cfg(test)]
mod tests {
  use super::{is_data_literal, literal_value};
  use dsntk_feel::FeelScope;
  use dsntk_feel_parser::AstNode;

  /// Parses a context definition and returns the syntax tree.
  fn parse(input: &str) -> AstNode {
    dsntk_feel_parser::parse_context(&FeelScope::default(), input, false).unwrap()
  }

  /// Returns the value both paths must agree on, built by the closure tree.
  fn slow_path(input: &str) -> String {
    let node = parse(input);
    format!("{}", crate::builders::build_evaluator(&node)(&FeelScope::default()))
  }

  /// Asserts that the tree is accepted and that both paths agree.
  fn accepted(input: &str) {
    let Some(value) = literal_value(&parse(input)) else {
      panic!("data literal path rejected `{input}`");
    };
    assert_eq!(slow_path(input), format!("{value}"), "paths disagree on `{input}`");
  }

  /// Asserts that the tree is rejected.
  fn rejected(input: &str) {
    if let Some(value) = literal_value(&parse(input)) {
      panic!("data literal path accepted `{input}` and returned `{value}`");
    }
  }

  #[test]
  fn accepted_trees_agree_with_the_closure_tree() {
    accepted("{}");
    accepted("{a: 1}");
    accepted("{a: []}");
    accepted("{a: [[], [[]]]}");
    accepted("{a: {}}");
    accepted(r#"{a: null, b: true, c: false, d: "x", e: 1, f: -1, g: -0.5}"#);
    accepted("{a: 1.10, b: 1e3, c: 1.5e-3, d: 0.000000000000000000000000000001}");
    accepted("{a: 123456789012345678901234567890123456}");
    accepted("{a: 1e999999999}");
    accepted("{a: -1e999999999}");
    accepted("{a: - -1}");
    accepted(r#"{"a b": 1, "": 2}"#);
    accepted("{a: 1, a: 2}");
    accepted("{a: {b: 1, b: 2}, c: 3}");
    accepted("{a: [{b: 1, b: 2}]}");
    accepted(r#"{a: [{b: [{c: {d: [1, "x", null, true]}}]}]}"#);
    accepted(r#"{a: "A\n\"", b: "🙂"}"#);
  }

  #[test]
  fn trees_that_are_not_data_are_rejected() {
    rejected(r#"{a: date and time("2026-01-05T10:00:00Z")}"#);
    rejected(r#"{a: @"2026-01-05"}"#);
    rejected("{a: 1, b: a}");
    rejected("{a: 1 + 1}");
    rejected("{a: -(1 + 1)}");
    rejected("{a: [1, 2][item > 1]}");
    rejected("{a: for x in [1, 2] return x}");
    rejected("{a: if true then 1 else 2}");
    rejected("{a: {b: 1}.b}");
    rejected(r#"{a: [1, 2, duration("P1D")]}"#);
    rejected("{a: {b: 1, c: b}}");
    rejected("{a: function() 1}");
    rejected("{a: 1 in [1..2]}");
    rejected("{a: some x in [1] satisfies x = 1}");
  }

  #[test]
  fn the_rejection_test_never_looks_below_a_rejected_node() {
    // The predicate is syntactic, so a rejected node rejects every tree that contains it,
    // however deeply it is buried.
    assert!(!is_data_literal(&parse(r#"{a: [{b: [{c: {d: [1, x]}}]}]}"#)));
    assert!(is_data_literal(&parse(r#"{a: [{b: [{c: {d: [1, 2]}}]}]}"#)));
  }
}
