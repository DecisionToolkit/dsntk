//! Parsing a list literal, at three sizes.
//!
//! The interesting figure is not any single time, which depends on the machine, but the
//! **ratio between the sizes**. Each size holds four times the items of the one before it.
//! Parsing that scales linearly answers in about four times the time. A parser that builds
//! the list by prepending each item into the front of a vector answers in about sixteen,
//! because every item moves all the items already collected.

#![feature(test)]

extern crate test;

use dsntk_feel::{FeelScope, scope};
use dsntk_feel_parser::parse_expression;
use test::Bencher;

/// Returns a list literal holding `count` numbers, like `[0,1,2]`.
fn list_literal(count: usize) -> String {
  let mut input = String::with_capacity(count * 5 + 2);
  input.push('[');
  for index in 0..count {
    if index > 0 {
      input.push(',');
    }
    input.push_str(&index.to_string());
  }
  input.push(']');
  input
}

#[bench]
fn feel_parser_list_literal_0001(b: &mut Bencher) {
  let scope = scope!();
  let input = list_literal(100);
  assert!(parse_expression(&scope, &input, false).is_ok());
  b.iter(|| parse_expression(&scope, &input, false));
}

#[bench]
fn feel_parser_list_literal_0002(b: &mut Bencher) {
  let scope = scope!();
  let input = list_literal(400);
  assert!(parse_expression(&scope, &input, false).is_ok());
  b.iter(|| parse_expression(&scope, &input, false));
}

#[bench]
fn feel_parser_list_literal_0003(b: &mut Bencher) {
  let scope = scope!();
  let input = list_literal(1600);
  assert!(parse_expression(&scope, &input, false).is_ok());
  b.iter(|| parse_expression(&scope, &input, false));
}

#[bench]
fn feel_parser_list_literal_0004(b: &mut Bencher) {
  let scope = scope!();
  let input = list_literal(6400);
  assert!(parse_expression(&scope, &input, false).is_ok());
  b.iter(|| parse_expression(&scope, &input, false));
}
