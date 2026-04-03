#![feature(test)]

extern crate test;

use dsntk_feel::{FeelScope, scope};
use dsntk_feel_parser::parse_context;
use test::Bencher;

#[bench]
fn feel_parser_context_0001(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"{}"#;
  b.iter(|| parse_context(&scope, input, false));
}

#[bench]
fn feel_parser_context_0002(b: &mut Bencher) {
  let scope = scope!();
  let input = " \n { \t } \r ";
  b.iter(|| parse_context(&scope, input, false));
}

#[bench]
fn feel_parser_context_0003(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"{age:49}"#;
  b.iter(|| parse_context(&scope, input, false));
}

#[bench]
fn feel_parser_context_0004(b: &mut Bencher) {
  let scope = scope!();
  let input = r#"{"name":"John","age":27,"car":{"model":"{Porsche}","production year":2021}}"#;
  b.iter(|| parse_context(&scope, input, false));
}
