use super::*;
use crate::{AstNode, ClosureBuilder, parse_context, parse_textual_expression, parse_unary_tests};
use dsntk_feel::{FeelScope, scope};

#[test]
fn _0001() {
  let node = parse_context(&scope!(), r#" { x: 10, a: function(a) function(b) a * b + x } "#, false).unwrap();
  assert_eq!("[x]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0002() {
  let node = parse_context(&scope!(), r#" { x: 10, a: function(a) function(b) a * b + x } "#, false).unwrap();
  assert_eq!(r#"Closure({QualifiedName([Name("x")])})"#, format!("{:?}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0003() {
  let node = parse_textual_expression(&scope!(), r#" 10 between 2 and 34 "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0004() {
  let node = parse_textual_expression(&scope!(), r#" 1 + 2 "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0005() {
  let node = parse_textual_expression(&scope!(), r#" 1 < 2"#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0006() {
  let node = parse_textual_expression(&scope!(), r#" 1 <= 2"#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0007() {
  let node = parse_textual_expression(&scope!(), r#" 1 > 2"#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0008() {
  let node = parse_textual_expression(&scope!(), r#" 1 >= 2"#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0009() {
  let node = parse_textual_expression(&scope!(), r#" @"2022-02-08" "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0010() {
  let node = parse_textual_expression(&scope!(), r#" @"2022-02-08T10:11:12" "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0011() {
  let node = parse_textual_expression(&scope!(), r#" @"P1D" "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0012() {
  let node = parse_textual_expression(&scope!(), r#" @"P1Y" "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0013() {
  let node = parse_textual_expression(&scope!(), r#" true and false "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0014() {
  let node = parse_textual_expression(&scope!(), r#" [1..10] "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0015() {
  let node = AstNode::CommaList(vec![_num!(s!("1"), s!("2"))]);
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0016() {
  let node = AstNode::NegatedList(vec![_num!(s!("1"), s!("2"))]);
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0017() {
  let node = parse_textual_expression(&scope!(), r#" "a" instance of function<string,string>->string "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0018() {
  let node = parse_textual_expression(&scope!(), r#" ["a","b","c"] instance of list<string> "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0019() {
  let node = parse_textual_expression(&scope!(), r#" [1..10] instance of range<number> "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0020() {
  let node = AstNode::Irrelevant;
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0021() {
  let node = AstNode::Out(b_num!(s!("1"), s!("2")), b_num!(s!("1"), s!("2")));
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0022() {
  let scope = FeelScope::default();
  let node = parse_context(&scope, r#" { a: { b: 1, c: "heute" } } "#, false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0023() {
  let node = parse_unary_tests(&scope!(), "<5", false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0024() {
  let node = parse_unary_tests(&scope!(), "<=5", false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0025() {
  let node = parse_unary_tests(&scope!(), ">5", false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0026() {
  let node = parse_unary_tests(&scope!(), "=>5", false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0027() {
  let node = parse_unary_tests(&scope!(), "=5", false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}

#[test]
fn _0028() {
  let node = parse_unary_tests(&scope!(), "!=5", false).unwrap();
  assert_eq!("[]", format!("{}", ClosureBuilder::from_node(&node)));
}
