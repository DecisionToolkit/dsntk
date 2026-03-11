//! Tests for duration and period functions within decision tables.

use crate::decision_table::build_decision_table_evaluator;
use crate::tests::context;
use dsntk_feel::values::Value;
use dsntk_model::DecisionTable;
use dsntk_recognizer::from_markdown;

/// Decision table that tests duration and period functions.
const DURATION_TABLE: &str = r#"
┌───┬─────────────────────╥─────────────────────┐
│ U │ Duration            ║ Description         │
╞═══╪═════════════════════╬═════════════════════╡
│ 1 │ duration("P1D")     ║ "one day"           │
├───┼─────────────────────╫─────────────────────┤
│ 2 │ duration("P1W")     ║ "one week"          │
├───┼─────────────────────╫─────────────────────┤
│ 3 │ duration("P1M")     ║ "one month"         │
├───┼─────────────────────╫─────────────────────┤
│ 4 │ duration("P1Y")     ║ "one year"          │
└───┴─────────────────────╨─────────────────────┘
"#;

#[test]
fn test_duration_evaluation() {
  // Parse the decision table from unicode representation
  let decision_table: DecisionTable = dsntk_recognizer::from_unicode(DURATION_TABLE, false).unwrap().into();

  // Create a scope for evaluation with a specific duration input
  let scope = context(r#"{Duration: duration("P1D")}"#).into();

  // Build the decision table evaluator
  let evaluator = build_decision_table_evaluator(&scope, &decision_table).unwrap();

  // Evaluate the decision table
  let result = evaluator(&scope);

  // The result should be "one day" since duration("P1D") matches the first rule
  match result {
    Value::String(s) => {
      assert_eq!(s, "one day", "Expected 'one day', got: {}", s);
    }
    _ => panic!("Expected string result, got: {}", result),
  }
}

/// Markdown equivalent of DURATION_TABLE
const DURATION_TABLE_MARKDOWN: &str = r#"
| U | Duration        | Description |
|:-:|:---------------:|:-----------:|
|   |   `i`           |     `o`     |
| 1 | duration("P1D") | "one day"   |
| 2 | duration("P1W") | "one week"  |
| 3 | duration("P1M") | "one month" |
| 4 | duration("P1Y") | "one year"  |
"#;

#[test]
fn test_duration_evaluation_markdown() {
  // Parse the decision table from markdown representation
  let decision_table: DecisionTable = from_markdown(DURATION_TABLE_MARKDOWN, false).unwrap().into();

  // Create a scope for evaluation with a specific duration input
  let scope = context(r#"{Duration: duration("P1D")}"#).into();

  // Build the decision table evaluator
  let evaluator = build_decision_table_evaluator(&scope, &decision_table).unwrap();

  // Evaluate the decision table
  let result = evaluator(&scope);

  // The result should be "one day" since duration("P1D") matches the first rule
  match result {
    Value::String(s) => {
      assert_eq!(s, "one day", "Expected 'one day', got: {}", s);
    }
    _ => panic!("Expected string result, got: {}", result),
  }
}
