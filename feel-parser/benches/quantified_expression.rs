#![feature(test)]

extern crate test;

use dsntk_feel::values::Value;
use dsntk_feel::{FeelScope, scope, value_null};
use dsntk_feel_parser::parse_expression;
use test::Bencher;

#[bench]
fn feel_parser_quantified_expression_0001(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"some n in [1,2,3] satisfies n > 1.5"#;
  b.iter(|| parse_expression(&scope, input, false));
}

#[bench]
fn feel_parser_quantified_expression_0002(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"some n in [1,2,3], m in <= 100 satisfies n > 1.5 * m"#;
  b.iter(|| parse_expression(&scope, input, false));
}

#[bench]
fn feel_parser_quantified_expression_0003(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"every n in [1,2,3] satisfies n > 1.5"#;
  b.iter(|| parse_expression(&scope, input, false));
}

#[bench]
fn feel_parser_quantified_expression_0004(b: &mut Bencher) {
  let scope = scope!();
  scope.set_value(&"n".into(), value_null!());
  scope.set_value(&"m".into(), value_null!());
  let input = r#"every n in [1,2,3], m in <= 100 satisfies n > 1.5 * m"#;
  b.iter(|| parse_expression(&scope, input, false));
}
