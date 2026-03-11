//! Tests for age calculation using temporal functions within decision tables.

use crate::decision_table::build_decision_table_evaluator;
use crate::tests::context;
use dsntk_feel::values::Value;
use dsntk_model::DecisionTable;
use dsntk_recognizer::from_markdown;

/// Decision table that tests age calculation using temporal functions.
/// This demonstrates the power of expressions in unary tests with temporal functions.
const AGE_CALCULATION_TABLE: &str = r#"
┌───┬───────────────────────────────────────────────────────────────╥─────────────────────┐
│ F │ Age Calculation                                               ║ Age Category        │
╞═══╪═══════════════════════════════════════════════════════════════╬═════════════════════╡
│ 1 │ years and months duration(Birth Date, today()).years < 13     ║ "junior"            │
├───┼───────────────────────────────────────────────────────────────╫─────────────────────┤
│ 2 │ years and months duration(Birth Date, today()).years >= 13    ║ "teenager"          │
│   │ and years and months duration(Birth Date, today()).years < 18 ║                     │
├───┼───────────────────────────────────────────────────────────────╫─────────────────────┤
│ 3 │ years and months duration(Birth Date, today()).years >= 18    ║ "adult"             │
│   │ and years and months duration(Birth Date, today()).years < 65 ║                     │
├───┼───────────────────────────────────────────────────────────────╫─────────────────────┤
│ 4 │ years and months duration(Birth Date, today()).years >= 65    ║ "senior"            │
└───┴───────────────────────────────────────────────────────────────╨─────────────────────┘
"#;

#[test]
fn test_age_calculation_evaluation() {
  // Parse the decision table from unicode representation
  let decision_table: DecisionTable = dsntk_recognizer::from_unicode(AGE_CALCULATION_TABLE, false).unwrap().into();

  // Test 1: Junior (age < 13)
  // Birth date 5 years ago — always produces age 5 (junior)
  let scope_junior = context(r#"{Birth Date: today() - duration("P5Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_junior, &decision_table).unwrap();
  let result_junior = evaluator(&scope_junior);

  match result_junior {
    Value::String(s) => {
      assert_eq!(s, "junior", "Expected 'junior' for age < 13, got: {}", s);
    }
    _ => panic!("Expected string result for junior age, got: {}", result_junior),
  }

  // Test 2: Teenager (13 ≤ age < 18)
  // Birth date 15 years ago — always produces age 15 (teenager)
  let scope_teenager = context(r#"{Birth Date: today() - duration("P15Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_teenager, &decision_table).unwrap();
  let result_teenager = evaluator(&scope_teenager);

  match result_teenager {
    Value::String(s) => {
      assert_eq!(s, "teenager", "Expected 'teenager' for 13 ≤ age < 18, got: {}", s);
    }
    _ => panic!("Expected string result for teenager age, got: {}", result_teenager),
  }

  // Test 3: Adult (18 ≤ age < 65)
  // Birth date 30 years ago — always produces age 30 (adult)
  let scope_adult = context(r#"{Birth Date: today() - duration("P30Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_adult, &decision_table).unwrap();
  let result_adult = evaluator(&scope_adult);

  match result_adult {
    Value::String(s) => {
      assert_eq!(s, "adult", "Expected 'adult' for 18 ≤ age < 65, got: {}", s);
    }
    _ => panic!("Expected string result for adult age, got: {}", result_adult),
  }

  // Test 4: Senior (age ≥ 65)
  // Birth date 70 years ago — always produces age 70 (senior)
  let scope_senior = context(r#"{Birth Date: today() - duration("P70Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_senior, &decision_table).unwrap();
  let result_senior = evaluator(&scope_senior);

  match result_senior {
    Value::String(s) => {
      assert_eq!(s, "senior", "Expected 'senior' for age ≥ 65, got: {}", s);
    }
    _ => panic!("Expected string result for senior age, got: {}", result_senior),
  }
}

/// Markdown equivalent of AGE_CALCULATION_TABLE
const AGE_CALCULATION_TABLE_MARKDOWN: &str = r#"
| F | Age Calculation | Age Category |
|:-:|:---------------:|:------------:|
|   |       `i`       |     `o`      |
| 1 | years and months duration(Birth Date, today()).years < 13 | "junior" |
| 2 | years and months duration(Birth Date, today()).years >= 13 and years and months duration(Birth Date, today()).years < 18 | "teenager" |
| 3 | years and months duration(Birth Date, today()).years >= 18 and years and months duration(Birth Date, today()).years < 65 | "adult" |
| 4 | years and months duration(Birth Date, today()).years >= 65 | "senior" |
"#;

#[test]
fn test_age_calculation_evaluation_markdown() {
  // Parse the decision table from markdown representation
  let decision_table: DecisionTable = from_markdown(AGE_CALCULATION_TABLE_MARKDOWN, false).unwrap().into();

  // Test 1: Junior (age < 13)
  let scope_junior = context(r#"{Birth Date: today() - duration("P5Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_junior, &decision_table).unwrap();
  let result_junior = evaluator(&scope_junior);

  match result_junior {
    Value::String(s) => {
      assert_eq!(s, "junior", "Expected 'junior' for age < 13, got: {}", s);
    }
    _ => panic!("Expected string result for junior age, got: {}", result_junior),
  }

  // Test 2: Teenager (13 ≤ age < 18)
  let scope_teenager = context(r#"{Birth Date: today() - duration("P15Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_teenager, &decision_table).unwrap();
  let result_teenager = evaluator(&scope_teenager);

  match result_teenager {
    Value::String(s) => {
      assert_eq!(s, "teenager", "Expected 'teenager' for 13 ≤ age < 18, got: {}", s);
    }
    _ => panic!("Expected string result for teenager age, got: {}", result_teenager),
  }

  // Test 3: Adult (18 ≤ age < 65)
  let scope_adult = context(r#"{Birth Date: today() - duration("P30Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_adult, &decision_table).unwrap();
  let result_adult = evaluator(&scope_adult);

  match result_adult {
    Value::String(s) => {
      assert_eq!(s, "adult", "Expected 'adult' for 18 ≤ age < 65, got: {}", s);
    }
    _ => panic!("Expected string result for adult age, got: {}", result_adult),
  }

  // Test 4: Senior (age ≥ 65)
  let scope_senior = context(r#"{Birth Date: today() - duration("P70Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_senior, &decision_table).unwrap();
  let result_senior = evaluator(&scope_senior);

  match result_senior {
    Value::String(s) => {
      assert_eq!(s, "senior", "Expected 'senior' for age ≥ 65, got: {}", s);
    }
    _ => panic!("Expected string result for senior age, got: {}", result_senior),
  }
}
/// Markdown equivalent of AGE_EDGE_CASES_TABLE
const AGE_EDGE_CASES_TABLE_MARKDOWN: &str = r#"
| U | Age Calculation | Age Category |
|:-:|:---------------:|:------------:|
|   |       `i`       |     `o`      |
| 1 | years and months duration(Birth Date, today()).years < 13 | "junior" |
| 2 | years and months duration(Birth Date, today()).years >= 13 and years and months duration(Birth Date, today()).years < 18 | "teenager" |
| 3 | years and months duration(Birth Date, today()).years >= 18 and years and months duration(Birth Date, today()).years < 65 | "adult" |
| 4 | years and months duration(Birth Date, today()).years >= 65 | "senior" |
"#;

#[test]
fn test_age_edge_cases_evaluation_markdown() {
  // Parse the decision table from markdown representation
  let decision_table: DecisionTable = from_markdown(AGE_EDGE_CASES_TABLE_MARKDOWN, false).unwrap().into();

  // Test edge case: exactly 13 years old (should be teenager)
  let scope_13 = context(r#"{Birth Date: today() - duration("P13Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_13, &decision_table).unwrap();
  let result_13 = evaluator(&scope_13);

  match result_13 {
    Value::String(s) => {
      assert_eq!(s, "teenager", "Expected 'teenager' for age >= 13 and < 18, got: {}", s);
    }
    _ => panic!("Expected string result for age 13, got: {}", result_13),
  }

  // Test edge case: exactly 18 years old (should be adult)
  let scope_18 = context(r#"{Birth Date: today() - duration("P18Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_18, &decision_table).unwrap();
  let result_18 = evaluator(&scope_18);

  match result_18 {
    Value::String(s) => {
      assert_eq!(s, "adult", "Expected 'adult' for age >= 18 and < 65, got: {}", s);
    }
    _ => panic!("Expected string result for age 18, got: {}", result_18),
  }

  // Test edge case: exactly 65 years old (should be senior)
  let scope_65 = context(r#"{Birth Date: today() - duration("P65Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_65, &decision_table).unwrap();
  let result_65 = evaluator(&scope_65);

  match result_65 {
    Value::String(s) => {
      assert_eq!(s, "senior", "Expected 'senior' for age >= 65, got: {}", s);
    }
    _ => panic!("Expected string result for age 65, got: {}", result_65),
  }

  // Test: 5 years old (should be junior)
  let scope_junior = context(r#"{Birth Date: today() - duration("P5Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_junior, &decision_table).unwrap();
  let result_junior = evaluator(&scope_junior);

  match result_junior {
    Value::String(s) => {
      assert_eq!(s, "junior", "Expected 'junior' for age < 13, got: {}", s);
    }
    _ => panic!("Expected string result for age 5, got: {}", result_junior),
  }
}

/// Decision table that tests edge cases for age calculation.
/// This tests the boundaries between age categories.
const AGE_EDGE_CASES_TABLE: &str = r#"
┌───┬───────────────────────────────────────────────────────────────╥─────────────────────┐
│ U │ Age Calculation                                               ║ Age Category        │
╞═══╪═══════════════════════════════════════════════════════════════╬═════════════════════╡
│ 1 │ years and months duration(Birth Date, today()).years < 13     ║ "junior"            │
├───┼───────────────────────────────────────────────────────────────╫─────────────────────┤
│ 2 │ years and months duration(Birth Date, today()).years >= 13    ║ "teenager"          │
│   │ and years and months duration(Birth Date, today()).years < 18 ║                     │
├───┼───────────────────────────────────────────────────────────────╫─────────────────────┤
│ 3 │ years and months duration(Birth Date, today()).years >= 18    ║ "adult"             │
│   │ and years and months duration(Birth Date, today()).years < 65 ║                     │
├───┼───────────────────────────────────────────────────────────────╫─────────────────────┤
│ 4 │ years and months duration(Birth Date, today()).years >= 65    ║ "senior"            │
└───┴───────────────────────────────────────────────────────────────╨─────────────────────┘
"#;

#[test]
fn test_age_edge_cases_evaluation() {
  // Parse the decision table from unicode representation
  let decision_table: DecisionTable = dsntk_recognizer::from_unicode(AGE_EDGE_CASES_TABLE, false).unwrap().into();

  // Test edge case: exactly 13 years old (should be teenager)
  let scope_13 = context(r#"{Birth Date: today() - duration("P13Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_13, &decision_table).unwrap();
  let result_13 = evaluator(&scope_13);

  match result_13 {
    Value::String(s) => {
      assert_eq!(s, "teenager", "Expected 'teenager' for age >= 13 and < 18, got: {}", s);
    }
    _ => panic!("Expected string result for age 13, got: {}", result_13),
  }

  // Test edge case: exactly 18 years old (should be adult)
  let scope_18 = context(r#"{Birth Date: today() - duration("P18Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_18, &decision_table).unwrap();
  let result_18 = evaluator(&scope_18);

  match result_18 {
    Value::String(s) => {
      assert_eq!(s, "adult", "Expected 'adult' for age >= 18 and < 65, got: {}", s);
    }
    _ => panic!("Expected string result for age 18, got: {}", result_18),
  }

  // Test edge case: exactly 65 years old (should be senior)
  let scope_65 = context(r#"{Birth Date: today() - duration("P65Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_65, &decision_table).unwrap();
  let result_65 = evaluator(&scope_65);

  match result_65 {
    Value::String(s) => {
      assert_eq!(s, "senior", "Expected 'senior' for age >= 65, got: {}", s);
    }
    _ => panic!("Expected string result for age 65, got: {}", result_65),
  }

  // Test: 5 years old (should be junior)
  let scope_junior = context(r#"{Birth Date: today() - duration("P5Y")}"#).into();
  let evaluator = build_decision_table_evaluator(&scope_junior, &decision_table).unwrap();
  let result_junior = evaluator(&scope_junior);

  match result_junior {
    Value::String(s) => {
      assert_eq!(s, "junior", "Expected 'junior' for age < 13, got: {}", s);
    }
    _ => panic!("Expected string result for age 5, got: {}", result_junior),
  }
}
