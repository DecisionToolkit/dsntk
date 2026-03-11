use dsntk_feel::{scope, FeelScope};

#[test]
fn _0001() {
  let node = dsntk_feel_parser::parse_expression(&scope!(), "now()", false).unwrap();
  let expected = r#"
       FunctionInvocation
       ├─ Name
       │  └─ `now`
       └─ PositionalParameters
          └─ (empty)
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0002() {
  let node = dsntk_feel_parser::parse_expression(&scope!(), "today()", false).unwrap();
  let expected = r#"
       FunctionInvocation
       ├─ Name
       │  └─ `today`
       └─ PositionalParameters
          └─ (empty)
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0003() {
  let node = dsntk_feel_parser::parse_expression(&scope!(), "date(\"2024-01-01\")", false).unwrap();
  let expected = r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date`
       └─ PositionalParameters
          └─ String
             └─ `2024-01-01`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0004() {
  let node = dsntk_feel_parser::parse_expression(&scope!(), "date and time(\"2024-01-01T10:00:00\")", false).unwrap();
  let expected = r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date and time`
       └─ PositionalParameters
          └─ String
             └─ `2024-01-01T10:00:00`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0005() {
  let node = dsntk_feel_parser::parse_expression(&scope!(), "duration(\"P30D\")", false).unwrap();
  let expected = r#"
       FunctionInvocation
       ├─ Name
       │  └─ `duration`
       └─ PositionalParameters
          └─ String
             └─ `P30D`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0006() {
  let node = dsntk_feel_parser::parse_expression(&scope!(), "years and months duration(date(\"2024-01-01\"), date(\"2024-12-31\"))", false).unwrap();
  let expected = r#"
       FunctionInvocation
       ├─ Name
       │  └─ `years and months duration`
       └─ PositionalParameters
          ├─ FunctionInvocation
          │  ├─ Name
          │  │  └─ `date`
          │  └─ PositionalParameters
          │     └─ String
          │        └─ `2024-01-01`
          └─ FunctionInvocation
             ├─ Name
             │  └─ `date`
             └─ PositionalParameters
                └─ String
                   └─ `2024-12-31`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0007() {
  let node = dsntk_feel_parser::parse_expression(&scope!(), "now() + duration(\"P1D\")", false).unwrap();
  let expected = r#"
       Add
       ├─ FunctionInvocation
       │  ├─ Name
       │  │  └─ `now`
       │  └─ PositionalParameters
       │     └─ (empty)
       └─ FunctionInvocation
          ├─ Name
          │  └─ `duration`
          └─ PositionalParameters
             └─ String
                └─ `P1D`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0008() {
  let node = dsntk_feel_parser::parse_expression(&scope!(), "date and time(\"2024-01-01T10:00:00\") - date and time(\"2024-01-01T09:00:00\")", false).unwrap();
  let expected = r#"
       Sub
       ├─ FunctionInvocation
       │  ├─ Name
       │  │  └─ `date and time`
       │  └─ PositionalParameters
       │     └─ String
       │        └─ `2024-01-01T10:00:00`
       └─ FunctionInvocation
          ├─ Name
          │  └─ `date and time`
          └─ PositionalParameters
             └─ String
                └─ `2024-01-01T09:00:00`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0009() {
  let node = dsntk_feel_parser::parse_expression(&scope!(), "time(\"10:30:00\")", false).unwrap();
  let expected = r#"
       FunctionInvocation
       ├─ Name
       │  └─ `time`
       └─ PositionalParameters
          └─ String
             └─ `10:30:00`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0010() {
  let node = dsntk_feel_parser::parse_expression(&scope!(), "duration(\"PT2H30M\")", false).unwrap();
  let expected = r#"
       FunctionInvocation
       ├─ Name
       │  └─ `duration`
       └─ PositionalParameters
          └─ String
             └─ `PT2H30M`
    "#;
  assert_eq!(expected, node.trace());
}
