use dsntk_feel::{scope, FeelScope};

/// Tests for expressions in unary tests - the new grammar extension
#[test]
fn _0001_date_less_than_today() {
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "Date < today()", false).unwrap();
  let expected = r#"
       ExpressionList
       └─ Lt
          ├─ Name
          │  └─ `Date`
          └─ FunctionInvocation
             ├─ Name
             │  └─ `today`
             └─ PositionalParameters
                └─ (empty)
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0002_time_greater_than_now() {
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "Time > now()", false).unwrap();
  let expected = r#"
       ExpressionList
       └─ Gt
          ├─ Name
          │  └─ `Time`
          └─ FunctionInvocation
             ├─ Name
             │  └─ `now`
             └─ PositionalParameters
                └─ (empty)
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0003_complex_expression_with_function() {
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "Amount <= calculate_total()", false).unwrap();
  let expected = r#"
       ExpressionList
       └─ Le
          ├─ Name
          │  └─ `Amount`
          └─ FunctionInvocation
             ├─ Name
             │  └─ `calculate_total`
             └─ PositionalParameters
                └─ (empty)
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0004_arithmetic_expression() {
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "Value > 100 + 50", false).unwrap();
  let expected = r#"
       ExpressionList
       └─ Gt
          ├─ Name
          │  └─ `Value`
          └─ Add
             ├─ Numeric
             │  └─ `100`
             └─ Numeric
                └─ `50`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0005_function_call_with_parameters() {
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "Score >= calculate_score(level, multiplier)", false).unwrap();
  let expected = r#"
       ExpressionList
       └─ Ge
          ├─ Name
          │  └─ `Score`
          └─ FunctionInvocation
             ├─ Name
             │  └─ `calculate_score`
             └─ PositionalParameters
                ├─ Name
                │  └─ `level`
                └─ Name
                   └─ `multiplier`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0006_backward_compatibility_simple_literals() {
  // Test that simple literals still work exactly as before
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "<5, <=10, >15, >=20, =25, !=30", false).unwrap();
  let expected = r#"
       ExpressionList
       ├─ UnaryLt
       │  └─ Numeric
       │     └─ `5`
       ├─ UnaryLe
       │  └─ Numeric
       │     └─ `10`
       ├─ UnaryGt
       │  └─ Numeric
       │     └─ `15`
       ├─ UnaryGe
       │  └─ Numeric
       │     └─ `20`
       ├─ UnaryEq
       │  └─ Numeric
       │     └─ `25`
       └─ UnaryNe
          └─ Numeric
             └─ `30`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0007_mixed_expressions_and_literals() {
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "Date < today(), <5, Amount > calculate_total()", false).unwrap();
  let expected = r#"
       ExpressionList
       ├─ Lt
       │  ├─ Name
       │  │  └─ `Date`
       │  └─ FunctionInvocation
       │     ├─ Name
       │     │  └─ `today`
       │     └─ PositionalParameters
       │        └─ (empty)
       ├─ UnaryLt
       │  └─ Numeric
       │     └─ `5`
       └─ Gt
          ├─ Name
          │  └─ `Amount`
          └─ FunctionInvocation
             ├─ Name
             │  └─ `calculate_total`
             └─ PositionalParameters
                └─ (empty)
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0008_negated_unary_tests_with_expressions() {
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "not(Date < today(), Amount > 100)", false).unwrap();
  let expected = r#"
       NegatedList
       ├─ Lt
       │  ├─ Name
       │  │  └─ `Date`
       │  └─ FunctionInvocation
       │     ├─ Name
       │     │  └─ `today`
       │     └─ PositionalParameters
       │        └─ (empty)
       └─ Gt
          ├─ Name
          │  └─ `Amount`
          └─ Numeric
             └─ `100`
    "#;
  assert_eq!(expected, node.trace());
}
