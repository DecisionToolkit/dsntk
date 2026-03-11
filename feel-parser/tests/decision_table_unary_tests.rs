use dsntk_feel::{scope, FeelScope};

/// Tests for decision table input entries with expressions in unary tests
#[test]
fn _0001_decision_table_input_with_function_calls() {
  // Test that decision table input entries can contain expressions with function calls
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "Date < today()", false).unwrap();

  // Verify the AST structure is correct for decision table input evaluation
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
fn _0002_decision_table_input_with_complex_expressions() {
  // Test that decision table input entries can contain complex expressions
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "Amount > calculate_total() + 100", false).unwrap();

  let expected = r#"
       ExpressionList
       └─ Gt
          ├─ Name
          │  └─ `Amount`
          └─ Add
             ├─ FunctionInvocation
             │  ├─ Name
             │  │  └─ `calculate_total`
             │  └─ PositionalParameters
             │     └─ (empty)
             └─ Numeric
                └─ `100`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0003_decision_table_input_with_temporal_functions() {
  // Test that decision table input entries can contain temporal function calls
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
fn _0004_decision_table_input_with_multiple_expressions() {
  // Test that decision table input entries can contain multiple expressions
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "Date < today(), Amount > 1000, Status = \"active\"", false).unwrap();

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
       ├─ Gt
       │  ├─ Name
       │  │  └─ `Amount`
       │  └─ Numeric
       │     └─ `1000`
       └─ Eq
          ├─ Name
          │  └─ `Status`
          └─ String
             └─ `active`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0005_decision_table_input_with_backward_compatibility() {
  // Test that traditional unary test syntax still works
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "<5, >10, =15", false).unwrap();

  let expected = r#"
       ExpressionList
       ├─ UnaryLt
       │  └─ Numeric
       │     └─ `5`
       ├─ UnaryGt
       │  └─ Numeric
       │     └─ `10`
       └─ UnaryEq
          └─ Numeric
             └─ `15`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0006_decision_table_input_with_age_calculation() {
  // Test that decision table input entries can contain age calculation expressions
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "years and months duration(Birth Date, today()).years < 13", false).unwrap();

  let expected = r#"
       ExpressionList
       └─ Lt
          ├─ Path
          │  ├─ FunctionInvocation
          │  │  ├─ Name
          │  │  │  └─ `years and months duration`
          │  │  └─ PositionalParameters
          │  │     ├─ Name
          │  │     │  └─ `Birth Date`
          │  │     └─ FunctionInvocation
          │  │        ├─ Name
          │  │        │  └─ `today`
          │  │        └─ PositionalParameters
          │  │           └─ (empty)
          │  └─ Name
          │     └─ `years`
          └─ Numeric
             └─ `13`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0007_decision_table_input_with_complex_age_conditions() {
  // Test that decision table input entries can contain complex age conditions with multiple comparisons
  let node = dsntk_feel_parser::parse_unary_tests(
    &scope!(),
    "years and months duration(Birth Date, today()).years >= 13 and years and months duration(Birth Date, today()).years < 18",
    false,
  )
  .unwrap();

  let expected = r#"
       ExpressionList
       └─ And
          ├─ Ge
          │  ├─ Path
          │  │  ├─ FunctionInvocation
          │  │  │  ├─ Name
          │  │  │  │  └─ `years and months duration`
          │  │  │  └─ PositionalParameters
          │  │  │     ├─ Name
          │  │  │     │  └─ `Birth Date`
          │  │  │     └─ FunctionInvocation
          │  │  │        ├─ Name
          │  │  │        │  └─ `today`
          │  │  │        └─ PositionalParameters
          │  │  │           └─ (empty)
          │  │  └─ Name
          │  │     └─ `years`
          │  └─ Numeric
          │     └─ `13`
          └─ Lt
             ├─ Path
             │  ├─ FunctionInvocation
             │  │  ├─ Name
             │  │  │  └─ `years and months duration`
             │  │  └─ PositionalParameters
             │  │     ├─ Name
             │  │     │  └─ `Birth Date`
             │  │     └─ FunctionInvocation
             │  │        ├─ Name
             │  │        │  └─ `today`
             │  │        └─ PositionalParameters
             │  │           └─ (empty)
             │  └─ Name
             │     └─ `years`
             └─ Numeric
                └─ `18`
    "#;
  assert_eq!(expected, node.trace());
}

#[test]
fn _0008_decision_table_input_with_age_boundary_conditions() {
  // Test that decision table input entries can contain age boundary conditions
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "years and months duration(Birth Date, today()).years = 13", false).unwrap();

  let expected = r#"
       ExpressionList
       └─ Eq
          ├─ Path
          │  ├─ FunctionInvocation
          │  │  ├─ Name
          │  │  │  └─ `years and months duration`
          │  │  └─ PositionalParameters
          │  │     ├─ Name
          │  │     │  └─ `Birth Date`
          │  │     └─ FunctionInvocation
          │  │        ├─ Name
          │  │        │  └─ `today`
          │  │        └─ PositionalParameters
          │  │           └─ (empty)
          │  └─ Name
          │     └─ `years`
          └─ Numeric
             └─ `13`
    "#;
  assert_eq!(expected, node.trace());
}
