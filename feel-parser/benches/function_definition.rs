#![feature(test)]

extern crate test;

use dsntk_feel::{FeelScope, scope};
use dsntk_feel_parser::{parse_context, parse_expression};
use test::Bencher;

#[bench]
fn feel_parser_function_definition_0001(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"{msg: function () "hello!" }"#;
  b.iter(|| parse_context(&scope, input, false));
}

#[bench]
fn feel_parser_function_definition_0002(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"function () "hello!""#;
  b.iter(|| parse_expression(&scope, input, false));
}
