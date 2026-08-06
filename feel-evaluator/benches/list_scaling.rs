//! Evaluating a `for` expression, at three sizes.
//!
//! As in the parser benchmark of the same shape, read the **ratio between the sizes** rather
//! than any single time. Each size runs four times the iterations of the one before it, so a
//! linear evaluator answers in about four times the time.
//!
//! `for` binds the implicit variable `partial` to the results of all previous iterations. An
//! evaluator that copies that list on every iteration pays for every result collected so far,
//! which makes the whole expression quadratic in the number of iterations. It does so whether
//! or not the body ever reads `partial`, and neither of these bodies reads it.

#![feature(test)]

extern crate test;

use dsntk_feel::values::Value;
use dsntk_feel::{FeelScope, scope, value_number};
use dsntk_feel_evaluator::prepare;
use test::Bencher;

/// Returns an expression that runs `count` iterations and counts the results.
fn counted_for(count: usize) -> String {
  format!("count(for item in 1..{count} return item + 1)")
}

#[bench]
fn feel_evaluator_list_scaling_0001(b: &mut Bencher) {
  let scope = scope!();
  let input = counted_for(100);
  let node = dsntk_feel_parser::parse_expression(&scope, &input, false).unwrap();
  let evaluator = prepare(&node);
  assert_eq!(value_number!(100), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}

#[bench]
fn feel_evaluator_list_scaling_0002(b: &mut Bencher) {
  let scope = scope!();
  let input = counted_for(400);
  let node = dsntk_feel_parser::parse_expression(&scope, &input, false).unwrap();
  let evaluator = prepare(&node);
  assert_eq!(value_number!(400), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}

#[bench]
fn feel_evaluator_list_scaling_0003(b: &mut Bencher) {
  let scope = scope!();
  let input = counted_for(1600);
  let node = dsntk_feel_parser::parse_expression(&scope, &input, false).unwrap();
  let evaluator = prepare(&node);
  assert_eq!(value_number!(1600), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}

#[bench]
fn feel_evaluator_list_scaling_0004(b: &mut Bencher) {
  let scope = scope!();
  let input = counted_for(6400);
  let node = dsntk_feel_parser::parse_expression(&scope, &input, false).unwrap();
  let evaluator = prepare(&node);
  assert_eq!(value_number!(6400), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}
