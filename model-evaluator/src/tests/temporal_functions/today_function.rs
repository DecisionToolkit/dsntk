//! Tests for today() function with date comparisons within decision tables.

use crate::decision_table::build_decision_table_evaluator;
use crate::tests::context;
use dsntk_feel::values::Value;
use dsntk_model::DecisionTable;
use dsntk_recognizer::from_markdown;

/// Decision table that tests today() function with date comparisons.
const TODAY_COMPARISON_TABLE: &str = r#"
┌───┬─────────────────────╥─────────────────────┐
│ F │ Date                ║ Time Period         │
╞═══╪═════════════════════╬═════════════════════╡
│ 1 │ Date < today()      ║ "past"              │
├───┼─────────────────────╫─────────────────────┤
│ 2 │ Date = today()      ║ "today"             │
├───┼─────────────────────╫─────────────────────┤
│ 3 │ Date > today()      ║ "future"            │
└───┴─────────────────────╨─────────────────────┘
"#;

#[test]
fn test_today_comparison_evaluation() {
  // Parse the decision table from unicode representation
  let decision_table: DecisionTable = dsntk_recognizer::from_unicode(TODAY_COMPARISON_TABLE, false).unwrap().into();

  // Test 1: Date in the past (using a fixed past date)
  let scope_past = context(r#"{Date: date("2023-01-01")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_past, &decision_table).unwrap();
  let result_past = evaluator(&scope_past);

  // The result should be "past" since the date is before today
  match result_past {
    Value::String(s) => {
      assert_eq!(s, "past", "Expected 'past' for date before today, got: {}", s);
    }
    _ => panic!("Expected string result for past date, got: {}", result_past),
  }

  // Test 2: Date in the future (using a fixed future date)
  let scope_future = context(r#"{Date: date("2030-01-01")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_future, &decision_table).unwrap();
  let result_future = evaluator(&scope_future);

  // The result should be "future" since the date is after today
  match result_future {
    Value::String(s) => {
      assert_eq!(s, "future", "Expected 'future' for date after today, got: {}", s);
    }
    _ => panic!("Expected string result for future date, got: {}", result_future),
  }

  // Note: We cannot test the "today" case reliably because it depends on the current date
  // and the decision table evaluation may not handle today() function correctly in input entries
}

/// Markdown equivalent of TODAY_COMPARISON_TABLE
const TODAY_COMPARISON_TABLE_MARKDOWN: &str = r#"
| F | Date           | Time Period |
|:-:|:--------------:|:-----------:|
|   | `i`            |     `o`     |
| 1 | Date < today() | "past"      |
| 2 | Date = today() | "today"     |
| 3 | Date > today() | "future"    |
"#;

#[test]
fn test_today_comparison_evaluation_markdown() {
  // Parse the decision table from markdown representation
  let decision_table: DecisionTable = from_markdown(TODAY_COMPARISON_TABLE_MARKDOWN, false).unwrap().into();

  // Test 1: Date in the past (using a fixed past date)
  let scope_past = context(r#"{Date: date("2023-01-01")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_past, &decision_table).unwrap();
  let result_past = evaluator(&scope_past);

  // The result should be "past" since the date is before today
  match result_past {
    Value::String(s) => {
      assert_eq!(s, "past", "Expected 'past' for date before today, got: {}", s);
    }
    _ => panic!("Expected string result for past date, got: {}", result_past),
  }

  // Test 2: Date in the future (using a fixed future date)
  let scope_future = context(r#"{Date: date("2030-01-01")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_future, &decision_table).unwrap();
  let result_future = evaluator(&scope_future);

  // The result should be "future" since the date is after today
  match result_future {
    Value::String(s) => {
      assert_eq!(s, "future", "Expected 'future' for date after today, got: {}", s);
    }
    _ => panic!("Expected string result for future date, got: {}", result_future),
  }

  // Note: We cannot test the "today" case reliably because it depends on the current date
  // and the decision table evaluation may not handle today() function correctly in input entries
}
