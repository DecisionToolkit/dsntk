//! Tests for date and time comparison functions within decision tables.

use crate::decision_table::build_decision_table_evaluator;
use crate::tests::context;
use dsntk_feel::values::Value;
use dsntk_model::DecisionTable;
use dsntk_recognizer::from_markdown;

/// Decision table that tests date and time comparisons using temporal functions.
const DATE_TIME_COMPARISON_TABLE: &str = r#"
┌───┬─────────────────────╥─────────────────────┐
│ U │ Date/Time           ║ Comparison Result   │
╞═══╪═════════════════════╬═════════════════════╡
│ 1 │ date("2023-01-01")  ║ "before 2023"       │
├───┼─────────────────────╫─────────────────────┤
│ 2 │ date("2023-06-15")  ║ "mid 2023"          │
├───┼─────────────────────╫─────────────────────┤
│ 3 │ date("2023-12-31")  ║ "end 2023"          │
├───┼─────────────────────╫─────────────────────┤
│ 4 │ date("2024-01-01")  ║ "after 2023"        │
└───┴─────────────────────╨─────────────────────┘
"#;

#[test]
fn test_date_time_comparison_evaluation() {
  // Parse the decision table from unicode representation
  let decision_table: DecisionTable = dsntk_recognizer::from_unicode(DATE_TIME_COMPARISON_TABLE, false).unwrap().into();

  // Create a scope for evaluation with a specific date input
  let scope = context(r#"{Date/Time: date("2023-06-15")}"#).into();

  // Build the decision table evaluator
  let evaluator = build_decision_table_evaluator(&scope, &decision_table).unwrap();

  // Evaluate the decision table
  let result = evaluator(&scope);

  // The result should be "mid 2023" for the input date 2023-06-15
  match result {
    Value::String(s) => {
      assert_eq!(s, "mid 2023", "Expected 'mid 2023', got: {}", s);
    }
    _ => panic!("Expected string result, got: {}", result),
  }
}

/// Markdown equivalent of DATE_TIME_COMPARISON_TABLE
const DATE_TIME_COMPARISON_TABLE_MARKDOWN: &str = r#"
| U | Date/Time | Comparison Result |
|:-:|:---------:|:-----------------:|
|   |    `i`    |        `o`        |
| 1 | date("2023-01-01") | "before 2023" |
| 2 | date("2023-06-15") | "mid 2023" |
| 3 | date("2023-12-31") | "end 2023" |
| 4 | date("2024-01-01") | "after 2023" |
"#;

#[test]
fn test_date_time_comparison_evaluation_markdown() {
  // Parse the decision table from markdown representation
  let decision_table: DecisionTable = from_markdown(DATE_TIME_COMPARISON_TABLE_MARKDOWN, false).unwrap().into();

  // Create a scope for evaluation with a specific date input
  let scope = context(r#"{Date/Time: date("2023-06-15")}"#).into();

  // Build the decision table evaluator
  let evaluator = build_decision_table_evaluator(&scope, &decision_table).unwrap();

  // Evaluate the decision table
  let result = evaluator(&scope);

  // The result should be "mid 2023" for the input date 2023-06-15
  match result {
    Value::String(s) => {
      assert_eq!(s, "mid 2023", "Expected 'mid 2023', got: {}", s);
    }
    _ => panic!("Expected string result, got: {}", result),
  }
}

/// Decision table that tests time and date functions.
const CURRENT_TIME_TABLE: &str = r#"
┌───┬─────────────────────╥─────────────────────┐
│ U │ Time Function       ║ Result              │
╞═══╪═════════════════════╬═════════════════════╡
│ 1 │ time("10:30:00")    ║ "morning"           │
├───┼─────────────────────╫─────────────────────┤
│ 2 │ time("14:00:00")    ║ "afternoon"         │
├───┼─────────────────────╫─────────────────────┤
│ 3 │ time("20:00:00")    ║ "evening"           │
└───┴─────────────────────╨─────────────────────┘
"#;

#[test]
fn test_current_time_evaluation() {
  // Parse the decision table from unicode representation
  let decision_table: DecisionTable = dsntk_recognizer::from_unicode(CURRENT_TIME_TABLE, false).unwrap().into();

  // Create a scope for evaluation with a specific time input
  // We'll use a fixed time to make the test predictable
  let scope = context(r#"{Time Function: time("10:30:00")}"#).into();

  // Build the decision table evaluator
  let evaluator = build_decision_table_evaluator(&scope, &decision_table).unwrap();

  // Evaluate the decision table
  let result = evaluator(&scope);

  // The result should be "morning" since time("10:30:00") will match the first rule
  match result {
    Value::String(s) => {
      assert_eq!(s, "morning", "Expected 'morning', got: {}", s);
    }
    _ => panic!("Expected string result, got: {}", result),
  }
}

/// Markdown equivalent of CURRENT_TIME_TABLE
const CURRENT_TIME_TABLE_MARKDOWN: &str = r#"
| U | Time Function    | Result      |
|:-:|:----------------:|:-----------:|
|   |     `i`          |  `o`        |
| 1 | time("10:30:00") | "morning"   |
| 2 | time("14:00:00") | "afternoon" |
| 3 | time("20:00:00") | "evening"   |
"#;

#[test]
fn test_current_time_evaluation_markdown() {
  // Parse the decision table from markdown representation
  let decision_table: DecisionTable = from_markdown(CURRENT_TIME_TABLE_MARKDOWN, false).unwrap().into();

  // Create a scope for evaluation with a specific time input
  // We'll use a fixed time to make the test predictable
  let scope = context(r#"{Time Function: time("10:30:00")}"#).into();

  // Build the decision table evaluator
  let evaluator = build_decision_table_evaluator(&scope, &decision_table).unwrap();

  // Evaluate the decision table
  let result = evaluator(&scope);

  // The result should be "morning" since time("10:30:00") will match the first rule
  match result {
    Value::String(s) => {
      assert_eq!(s, "morning", "Expected 'morning', got: {}", s);
    }
    _ => panic!("Expected string result, got: {}", result),
  }
}
