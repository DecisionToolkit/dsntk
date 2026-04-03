mod ast;
mod bifs;
mod closure;
mod context;
mod expr;

/// Creates a parsing scope.
macro_rules! scope {
  () => {{
    use crate::tests::ParsingScope;
    ParsingScope::default()
  }};
}

macro_rules! s {
  ($l:literal) => {
    $l.to_string()
  };
  () => {
    "".to_string()
  };
}

macro_rules! _num {
  ($a:expr, $b:expr) => {
    AstNode::Numeric($a.to_string(), $b.to_string(), '+', "".to_string())
  };
  ($a:expr) => {
    AstNode::Numeric($a.to_string(), "".to_string(), '+', "".to_string())
  };
}

macro_rules! b_num {
  ($a:expr, $b:expr) => {
    Box::new(AstNode::Numeric($a.to_string(), $b.to_string(), '+', "".to_string()))
  };
  ($a:expr) => {
    Box::new(AstNode::Numeric($a.to_string(), "".to_string(), '+', "".to_string()))
  };
}

macro_rules! __name {
  ($a:tt) => {
    stringify!($a).into()
  };
}

macro_rules! _name {
  ($a:tt) => {
    AstNode::Name(stringify!($a).into())
  };
}

macro_rules! b_name {
  ($a:tt) => {
    Box::new(AstNode::Name(stringify!($a).into()))
  };
}

macro_rules! b_bool {
  ($a:literal) => {
    Box::new(AstNode::Boolean($a))
  };
}

use crate::lalr::TokenType;
use crate::lalr::TokenType::StartExpression;
use crate::parser::Parser;
use crate::{AstNode, ParsingScope};
pub(crate) use __name;
pub(crate) use _name;
pub(crate) use _num;
pub(crate) use b_bool;
pub(crate) use b_name;
pub(crate) use b_num;
use difference::Changeset;
use dsntk_feel::Name;
pub(crate) use s;
pub(crate) use scope;

/// Parses the input text and compared the result with expected value.
fn accept(scope: &ParsingScope, start_token_type: TokenType, input: &str, expected: &str, trace: bool) {
  let node = Parser::new(scope, start_token_type, input, trace).parse().unwrap();
  let actual = node.trace();
  if actual != expected {
    println!("EXPECTED:\n------------------------------------------------------------{expected}\n");
    println!("ACTUAL:\n------------------------------------------------------------{actual}\n");
    println!(
      "DIFF:\n------------------------------------------------------------{}\n",
      Changeset::new(expected, &actual, "")
    );
  }
  assert_eq!(expected, actual);
}

/// Utility function for comparing AST trees in debug mode.
fn eqd(expected: &str, node: &AstNode) {
  assert_eq!(expected, format!("{node:?}"));
}

/// Utility function for comparing AST trees in display mode.
fn eqs(expected: &str, node: &AstNode) {
  assert_eq!(expected, node.trace());
}

#[test]
fn test_parse_textual_expression() {
  let scope = dsntk_feel::FeelScope::default();
  let node = crate::parse_textual_expression(&scope, "1+2", false).unwrap();
  eqs(
    r#"
       Add
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `2`
    "#,
    &node,
  );
}

#[test]
fn test_parse_textual_expressions() {
  let scope = dsntk_feel::FeelScope::default();
  let node = crate::parse_textual_expressions(&scope, "1+2,2+3,3*4", false).unwrap();
  eqs(
    r#"
       ExpressionList
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `1`
       │  └─ Numeric
       │     └─ `2`
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `2`
       │  └─ Numeric
       │     └─ `3`
       └─ Mul
          ├─ Numeric
          │  └─ `3`
          └─ Numeric
             └─ `4`
    "#,
    &node,
  );
}

#[test]
fn test_parse_unary_tests() {
  let scope = dsntk_feel::FeelScope::default();
  let node = crate::parse_unary_tests(&scope, "1,2,3,4", false).unwrap();
  eqs(
    r#"
       ExpressionList
       ├─ Numeric
       │  └─ `1`
       ├─ Numeric
       │  └─ `2`
       ├─ Numeric
       │  └─ `3`
       └─ Numeric
          └─ `4`
    "#,
    &node,
  );
}

#[test]
fn test_parse_boxed_expression() {
  let scope = dsntk_feel::FeelScope::default();
  let node = crate::parse_boxed_expression(&scope, "[1,2,3,4]", false).unwrap();
  eqs(
    r#"
       List
       ├─ Numeric
       │  └─ `1`
       ├─ Numeric
       │  └─ `2`
       ├─ Numeric
       │  └─ `3`
       └─ Numeric
          └─ `4`
    "#,
    &node,
  );
}

#[test]
fn test_parse_context() {
  let scope = dsntk_feel::FeelScope::default();
  let node = crate::parse_context(&scope, "{age: 50}", false).unwrap();
  eqs(
    r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `age`
          └─ Numeric
             └─ `50`
    "#,
    &node,
  );
}

#[test]
fn test_parse_name() {
  let name_a: Name = Name::new(&["Full", "House"]);
  let scope = dsntk_feel::FeelScope::default();
  assert_eq!(name_a, crate::parse_name(&scope, "Full House", false).unwrap());
}

#[test]
fn test_parse_longest_name() {
  let name_a: Name = Name::new(&["Full", "House", "With", "A", "Cat"]);
  assert_eq!(name_a, crate::parse_longest_name(" Full House  With  \t A \n\n Cat    ").unwrap());
}

/// Covers the case when function `accept` reports an error,
/// which means that the test result differs from expected value.
#[test]
#[should_panic]
fn test_not_accept() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "1+2",
    r#"
        Add
          Numeric 1.
          Numeric 3.
      "#,
    false,
  );
}
