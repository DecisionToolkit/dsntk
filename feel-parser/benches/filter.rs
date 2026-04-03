#![feature(test)]

extern crate test;

use dsntk_feel::{FeelScope, scope};
use dsntk_feel_parser::parse_expression;
use test::Bencher;

#[bench]
fn feel_parser_filter_0001(b: &mut Bencher) {
  let scope = scope!();
  let input = r#""DeptTable[number=EmployeeTable[name=LastName].deptNum[1]].manager[1]""#;
  b.iter(|| parse_expression(&scope, input, false));
}
