use super::super::*;
use crate::lalr::TokenType::StartUnaryTests;
use dsntk_common::DsntkError;

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#"-"#,
    r#"
       Irrelevant
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#"1"#,
    r#"
       ExpressionList
       └─ Numeric
          └─ `1`
    "#,
    false,
  );
}

#[test]
fn _0003() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#"1,2"#,
    r#"
       ExpressionList
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `2`
    "#,
    false,
  );
}

#[test]
fn _0004() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#" true , false "#,
    r#"
       ExpressionList
       ├─ Boolean
       │  └─ `true`
       └─ Boolean
          └─ `false`
    "#,
    false,
  );
}

#[test]
fn _0005() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#" date("2021-10-01") , time("15:23") "#,
    r#"
       ExpressionList
       ├─ FunctionInvocation
       │  ├─ Name
       │  │  └─ `date`
       │  └─ PositionalParameters
       │     └─ String
       │        └─ `2021-10-01`
       └─ FunctionInvocation
          ├─ Name
          │  └─ `time`
          └─ PositionalParameters
             └─ String
                └─ `15:23`
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#"1,2,3,4"#,
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
    false,
  );
}

#[test]
fn _0007() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#"not (1)"#,
    r#"
       NegatedList
       └─ Numeric
          └─ `1`
    "#,
    false,
  );
}

#[test]
fn _0008() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#"not(1,2)"#,
    r#"
       NegatedList
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `2`
    "#,
    false,
  );
}

#[test]
fn _0009() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#" not ( 1 , 2 , 3 , 4 ) "#,
    r#"
       NegatedList
       ├─ Numeric
       │  └─ `1`
       ├─ Numeric
       │  └─ `2`
       ├─ Numeric
       │  └─ `3`
       └─ Numeric
          └─ `4`
    "#,
    false,
  );
}

#[test]
fn _0010() {
  let scope = scope!();
  assert_eq!(
    Err(DsntkError::new(r#"ParserError"#, r#"syntax error: (1,2,3,4)"#)),
    Parser::new(&scope, StartUnaryTests, "(1,2,3,4)", false).parse()
  );
}

#[test]
fn _0011() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#" < today() "#,
    r#"
       ExpressionList
       └─ UnaryLt
          └─ FunctionInvocation
             ├─ Name
             │  └─ `today`
             └─ PositionalParameters
                └─ (empty)
    "#,
    false,
  );
}

#[test]
fn _0012() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#" = today() "#,
    r#"
       ExpressionList
       └─ UnaryEq
          └─ FunctionInvocation
             ├─ Name
             │  └─ `today`
             └─ PositionalParameters
                └─ (empty)
    "#,
    false,
  );
}

#[test]
fn _0013() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#" > today() "#,
    r#"
       ExpressionList
       └─ UnaryGt
          └─ FunctionInvocation
             ├─ Name
             │  └─ `today`
             └─ PositionalParameters
                └─ (empty)
    "#,
    false,
  );
}

#[test]
fn _0014() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#" != today() "#,
    r#"
       ExpressionList
       └─ UnaryNe
          └─ FunctionInvocation
             ├─ Name
             │  └─ `today`
             └─ PositionalParameters
                └─ (empty)
    "#,
    false,
  );
}

#[test]
fn _0015() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#" >= today() "#,
    r#"
       ExpressionList
       └─ UnaryGe
          └─ FunctionInvocation
             ├─ Name
             │  └─ `today`
             └─ PositionalParameters
                └─ (empty)
    "#,
    false,
  );
}

#[test]
fn _0016() {
  let scope = scope!();
  accept(
    &scope,
    StartUnaryTests,
    r#" <= today() "#,
    r#"
       ExpressionList
       └─ UnaryLe
          └─ FunctionInvocation
             ├─ Name
             │  └─ `today`
             └─ PositionalParameters
                └─ (empty)
    "#,
    false,
  );
}
