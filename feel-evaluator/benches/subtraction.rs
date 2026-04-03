#![feature(test)]

extern crate test;

use dsntk_feel::values::Value;
use dsntk_feel::{FeelScope, scope, value_null, value_number};
use dsntk_feel_evaluator::prepare;
use test::Bencher;

#[bench]
fn feel_evaluator_subtraction_0001(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"2-1"#;
  let node = dsntk_feel_parser::parse_textual_expression(&scope, input, false).unwrap();
  let evaluator = prepare(&node);
  assert_eq!(value_number!(1), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}

#[bench]
fn feel_evaluator_subtraction_0002(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"5-2-1"#;
  let node = dsntk_feel_parser::parse_textual_expression(&scope, input, false).unwrap();
  let evaluator = prepare(&node);
  assert_eq!(value_number!(2), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}

#[bench]
fn feel_evaluator_subtraction_0003(b: &mut Bencher) {
  let scope = scope!();
  scope.set_value(&"a".into(), value_null!());
  scope.set_value(&"b".into(), value_null!());
  let input = r#"a-b"#;
  let node = dsntk_feel_parser::parse_textual_expression(&scope, input, false).unwrap();
  let evaluator = prepare(&node);
  scope.set_value(&"a".into(), value_number!(18));
  scope.set_value(&"b".into(), value_number!(3));
  assert_eq!(value_number!(15), evaluator(&scope));
  b.iter(|| evaluator(&scope));
}
