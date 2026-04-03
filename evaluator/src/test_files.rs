//! # Implementation of the evaluator for test files
//!
//! Test files (files containing test cases) are provided for automating tests.
//!
//! Test files are used in evaluation tests for:
//! - `DMN` models (see `tdm` subcommand),
//! - decision tables (see `tdt` subcommand),
//! - `FEEL` expressions (see `tfe` subcommand).
//!
//! Single test case has the following structure:
//! ```text
//! separator input_data , expected_result
//! ```
//! where:
//! - **separator** is one or more characters at the beginning of the file followed by whitespace,
//! - **input_data** is a valid `FEEL` context containing input data for a test case,
//! - **,** is literally the comma character,
//! - **expected_result** is a valid `FEEL` value that is expected as a result in test case.
//!
//! Test file may contain one or more test cases.
//!
//! # Example
//!
//! An example of a test file may look like this:
//!
//! ```text
//! % { Customer:"Business",   Order:  -3.23 }, 0.10
//! % { Customer:"Business",   Order:   9.00 }, 0.10
//! % { Customer:"Business",   Order:  10.00 }, 0.15
//! % { Customer:"Business",   Order: 120.00 }, 0.15
//! % { Customer:"Private",    Order:  -2.34 }, 0.05
//! % { Customer:"Private",    Order:  10.00 }, 0.05
//! % { Customer:"Private",    Order: 101.00 }, 0.05
//! % { Customer:"Government", Order:  10.00 }, null
//! ```

use crate::errors::{err_expected_expression_list, err_expected_two_elements_in_expression_list};
use dsntk_common::Result;
use dsntk_feel::FeelScope;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel_parser::AstNode;

/// Prepares test cases loaded from the specified input.
pub fn prepare_test_cases(input: &str) -> Result<Vec<(FeelContext, Value)>> {
  let mut test_cases = vec![];
  if let Some(separator) = detect_separator(input) {
    let scope = FeelScope::default();
    for unary_tests in split_test_cases(input, &separator) {
      match dsntk_feel_parser::parse_unary_tests(&scope, unary_tests, false) {
        Ok(ast_node) => match ast_node {
          AstNode::ExpressionList(nodes) => {
            if nodes.len() == 2 {
              match dsntk_feel_evaluator::evaluate_context_node(&scope, &nodes[0]) {
                Ok(input_data) => {
                  let expected_result = dsntk_feel_evaluator::evaluate(&scope, &nodes[1]);
                  test_cases.push((input_data, expected_result));
                }
                Err(reason) => return Err(reason),
              }
            } else {
              return Err(err_expected_two_elements_in_expression_list(nodes.len()));
            }
          }
          other => return Err(err_expected_expression_list(&other)),
        },
        Err(reason) => return Err(reason),
      }
    }
  }
  Ok(test_cases)
}

/// Splits test cases from input test file using specified separator.
fn split_test_cases<'a>(input: &'a str, separator: &'a str) -> Vec<&'a str> {
  let split = input.split(&separator);
  split
    .filter_map(|s| {
      let trimmed = s.trim();
      if !trimmed.is_empty() { Some(trimmed) } else { None }
    })
    .collect()
}

/// Detects the separator used in test file.
fn detect_separator(input: &str) -> Option<String> {
  enum State {
    BeforeSeparator,
    InsideSeparator,
  }
  let mut separator = String::new();
  let mut state = State::BeforeSeparator;
  for ch in input.chars() {
    match state {
      State::BeforeSeparator => {
        if !ch.is_whitespace() {
          separator.push(ch);
          state = State::InsideSeparator;
        }
      }
      State::InsideSeparator => {
        if ch.is_whitespace() {
          break;
        } else {
          separator.push(ch);
        }
      }
    }
  }
  if separator.is_empty() { None } else { Some(separator) }
}

#[cfg(test)]
mod tests {
  use super::detect_separator;

  #[test]
  fn test_detect_separator() {
    assert_eq!(None, detect_separator(""));
    assert_eq!("{", detect_separator("   {  ").unwrap());
    assert_eq!("{", detect_separator(" \n\n  {  ").unwrap());
    assert_eq!("~", detect_separator("~").unwrap());
    assert_eq!("~", detect_separator(" \n\n  ~").unwrap());
    assert_eq!("!", detect_separator("!").unwrap());
    assert_eq!("@", detect_separator("@").unwrap());
    assert_eq!("#", detect_separator("#").unwrap());
    assert_eq!("$", detect_separator("$").unwrap());
    assert_eq!("%", detect_separator("%").unwrap());
    assert_eq!("^", detect_separator("^").unwrap());
    assert_eq!("&", detect_separator("&").unwrap());
    assert_eq!("|", detect_separator("|").unwrap());
    assert_eq!("%%", detect_separator("   %%   ").unwrap());
    assert_eq!("%%{", detect_separator(" \n\n\n \t  %%{   ").unwrap());
    assert_eq!("%%", detect_separator(" \n %%\n").unwrap());
    assert_eq!("~!@#$%^&|", detect_separator(" ~!@#$%^&| ").unwrap());
    assert_eq!("😀", detect_separator(" 😀 ").unwrap());
  }
}
