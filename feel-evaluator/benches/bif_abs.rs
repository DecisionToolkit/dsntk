#![feature(test)]

extern crate test;

use dsntk_feel::values::Value;
use dsntk_feel::{FeelScope, scope, value_number};
use dsntk_feel_evaluator::prepare;
use test::Bencher;

#[bench]
fn feel_evaluator_bif_abs_0001(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"abs(-1)"#;
  let node = dsntk_feel_parser::parse_textual_expression(&scope, input, false).unwrap();
  let evaluator = prepare(&node);
  assert_eq!(value_number!(1), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}
