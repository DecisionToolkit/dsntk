//! # Tests for markdown decision tables with FEEL expressions
//!
//! This module contains tests specifically for markdown decision tables
//! that use FEEL (Friendly Enough Expression Language) expressions.
//! These tests verify actual DMN decision table parsing functionality
//! and evaluation with input data.

#[cfg(test)]
mod tests {
  use dsntk_feel::context::FeelContext;
  use dsntk_feel::values::Value;
  use dsntk_feel::{value_null, value_number, value_string, FeelNumber, FeelScope};
  use dsntk_model::DecisionTable as ModelDecisionTable;
  use dsntk_model_evaluator::build_decision_table_evaluator;
  use dsntk_recognizer::from_markdown;

  /// Creates a FEEL context from the specified input expression.
  fn context(input: &str) -> FeelContext {
    let scope = FeelScope::default();
    match dsntk_feel_parser::parse_context(&scope, input, false) {
      Ok(node) => {
        let evaluator = dsntk_feel_evaluator::prepare(&node);
        match evaluator(&scope) {
          Value::Context(ctx) => ctx,
          other => panic!("ERROR: expected context value, actual value is: {}", other as Value),
        }
      }
      Err(reason) => panic!("ERROR: parsing context failed with reason: {reason}"),
    }
  }

  /// Test that markdown decision tables can be parsed into DMN decision table structures
  #[test]
  fn test_markdown_decision_table_parsing() {
    let markdown = r#"| U | Customer | Order | Discount |
|:-:|:--------:|:-----:|:--------:|
|   |   `i`    |  `i`  |   `o`    |
| 1 | "Business" | <10  |   0.10   |
| 2 | "Business" | >=10 |   0.15   |
| 3 | "Private"  |   -  |   0.05   |"#;

    // Parse the decision table using DSNTK recognizer
    let decision_table = from_markdown(markdown, false).expect("Failed to parse markdown decision table");

    // Verify the parsed structure represents a valid DMN decision table
    assert!(!decision_table.input_clauses.is_empty(), "Decision table should have input clauses");
    assert!(!decision_table.output_clauses.is_empty(), "Decision table should have output clauses");
    assert!(!decision_table.rules.is_empty(), "Decision table should have rules");

    // Verify that the structure can be used for DMN evaluation
    assert_eq!(decision_table.input_clauses.len(), 2, "Should parse 2 input clauses");
    assert_eq!(decision_table.output_clauses.len(), 1, "Should parse 1 output clause");
    assert_eq!(decision_table.rules.len(), 3, "Should parse 3 decision rules");

    // Verify that FEEL expressions are preserved in the parsed structure
    assert!(decision_table.rules[0].input_entries[1].text.contains("<"), "Should preserve FEEL comparison operators");
    assert!(decision_table.rules[1].input_entries[1].text.contains(">="), "Should preserve FEEL comparison operators");
    assert!(decision_table.rules[2].input_entries[1].text.contains("-"), "Should preserve FEEL dash expressions");
  }

  /// Test decision table with complex FEEL expressions for age calculation
  #[test]
  fn test_age_calculation_decision_table() {
    let markdown = r#"| F | Date of Birth | Age Group | Notes |
|:-:|:-------------:|:---------:|:-----:|
|   |     `i`       |    `o`    | `ann` |
| 1 | (years and months duration(Date of Birth, today()).years) < 13  | "child"   | "Child: 0-12 years" |
| 2 | (years and months duration(Date of Birth, today()).years) < 18  | "teenager" | "Teenager: 13-17 years" |
| 3 | (years and months duration(Date of Birth, today()).years) < 65  | "adult"   | "Adult: 18-64 years" |
| 4 | (years and months duration(Date of Birth, today()).years) >= 65  | "senior"  | "Senior: 65+ years" |"#;

    // Parse the decision table using DSNTK recognizer
    let decision_table = from_markdown(markdown, false).expect("Failed to parse age calculation decision table");

    // Verify the parsed structure represents a valid DMN decision table
    assert!(!decision_table.input_clauses.is_empty(), "Decision table should have input clauses");
    assert!(!decision_table.output_clauses.is_empty(), "Decision table should have output clauses");
    assert!(!decision_table.rules.is_empty(), "Decision table should have rules");
    assert!(!decision_table.annotation_clauses.is_empty(), "Decision table should have annotation clauses");

    // Verify that the structure can be used for DMN evaluation
    assert_eq!(decision_table.input_clauses.len(), 1, "Should parse 1 input clause");
    assert_eq!(decision_table.output_clauses.len(), 1, "Should parse 1 output clause");
    assert_eq!(decision_table.annotation_clauses.len(), 1, "Should parse 1 annotation clause");
    assert_eq!(decision_table.rules.len(), 4, "Should parse 4 decision rules");

    // Verify hit policy is First (F)
    assert!(matches!(decision_table.hit_policy, dsntk_recognizer::HitPolicy::First), "Should have First hit policy");

    // Verify input expression
    assert_eq!(decision_table.input_clauses[0].input_expression, "Date of Birth");

    // Verify output component name
    assert_eq!(decision_table.output_clauses[0].output_component_name, Some("Age Group".to_string()));

    // Verify annotation clause name
    assert_eq!(decision_table.annotation_clauses[0].name, "Notes");

    // Verify that complex FEEL expressions are preserved in the parsed structure
    assert!(
      decision_table.rules[0].input_entries[0].text.contains("years and months duration"),
      "Should preserve FEEL duration function"
    );
    assert!(decision_table.rules[0].input_entries[0].text.contains("today()"), "Should preserve FEEL today function");
    assert!(decision_table.rules[0].input_entries[0].text.contains("< 13"), "Should preserve FEEL comparison operators");

    assert!(
      decision_table.rules[1].input_entries[0].text.contains("years and months duration"),
      "Should preserve FEEL duration function"
    );
    assert!(decision_table.rules[1].input_entries[0].text.contains("today()"), "Should preserve FEEL today function");
    assert!(decision_table.rules[1].input_entries[0].text.contains("< 18"), "Should preserve FEEL comparison operators");

    assert!(
      decision_table.rules[2].input_entries[0].text.contains("years and months duration"),
      "Should preserve FEEL duration function"
    );
    assert!(decision_table.rules[2].input_entries[0].text.contains("today()"), "Should preserve FEEL today function");
    assert!(decision_table.rules[2].input_entries[0].text.contains("< 65"), "Should preserve FEEL comparison operators");

    assert!(
      decision_table.rules[3].input_entries[0].text.contains("years and months duration"),
      "Should preserve FEEL duration function"
    );
    assert!(decision_table.rules[3].input_entries[0].text.contains("today()"), "Should preserve FEEL today function");
    assert!(decision_table.rules[3].input_entries[0].text.contains(">= 65"), "Should preserve FEEL comparison operators");

    // Verify output values
    assert_eq!(decision_table.rules[0].output_entries[0].text, "\"child\"");
    assert_eq!(decision_table.rules[1].output_entries[0].text, "\"teenager\"");
    assert_eq!(decision_table.rules[2].output_entries[0].text, "\"adult\"");
    assert_eq!(decision_table.rules[3].output_entries[0].text, "\"senior\"");

    // Verify annotation entries
    assert_eq!(decision_table.rules[0].annotation_entries[0].text, "\"Child: 0-12 years\"");
    assert_eq!(decision_table.rules[1].annotation_entries[0].text, "\"Teenager: 13-17 years\"");
    assert_eq!(decision_table.rules[2].annotation_entries[0].text, "\"Adult: 18-64 years\"");
    assert_eq!(decision_table.rules[3].annotation_entries[0].text, "\"Senior: 65+ years\"");
  }

  /// Test that markdown decision tables can be converted to DMN model decision tables
  #[test]
  fn test_markdown_to_dmn_conversion() {
    let markdown = r#"| U | Customer | Order | Discount |
|:-:|:--------:|:-----:|:--------:|
|   |   `i`    |  `i`  |   `o`    |
| 1 | "Business" | <10  |   0.10   |
| 2 | "Business" | >=10 |   0.15   |
| 3 | "Private"  |   -  |   0.05   |"#;

    // Parse the decision table using DSNTK recognizer
    let decision_table = from_markdown(markdown, false).expect("Failed to parse markdown decision table");

    // Convert to DMN model decision table
    let model_decision_table: ModelDecisionTable = decision_table.into();

    // Verify the converted structure represents a valid DMN decision table
    assert!(model_decision_table.input_clauses().len() > 0, "Model decision table should have input clauses");
    assert!(model_decision_table.output_clauses().len() > 0, "Model decision table should have output clauses");
    assert!(model_decision_table.rules().len() > 0, "Model decision table should have rules");

    // Verify that the structure can be used for DMN evaluation
    assert_eq!(model_decision_table.input_clauses().len(), 2, "Should convert 2 input clauses");
    assert_eq!(model_decision_table.output_clauses().len(), 1, "Should convert 1 output clause");
    assert_eq!(model_decision_table.rules().len(), 3, "Should convert 3 decision rules");
  }

  /// Test that complex FEEL expressions are preserved during parsing
  #[test]
  fn test_complex_feel_expressions_preserved() {
    let markdown = r#"| F | Date of Birth | Age Group | Notes |
|:-:|:-------------:|:---------:|:-----:|
|   |     `i`       |    `o`    | `ann` |
| 1 | (years and months duration(Date of Birth, today()).years) < 13  | "child"   | "Child: 0-12 years" |
| 2 | (years and months duration(Date of Birth, today()).years) < 18  | "teenager" | "Teenager: 13-17 years" |
| 3 | (years and months duration(Date of Birth, today()).years) < 65  | "adult"   | "Adult: 18-64 years" |
| 4 | (years and months duration(Date of Birth, today()).years) >= 65  | "senior"  | "Senior: 65+ years" |"#;

    // Parse the decision table using DSNTK recognizer
    let decision_table = from_markdown(markdown, false).expect("Failed to parse age calculation decision table");

    // Verify that complex FEEL expressions are preserved in the parsed structure
    assert!(
      decision_table.rules[0].input_entries[0].text.contains("years and months duration"),
      "Should preserve FEEL duration function"
    );
    assert!(decision_table.rules[0].input_entries[0].text.contains("today()"), "Should preserve FEEL today function");
    assert!(decision_table.rules[0].input_entries[0].text.contains("< 13"), "Should preserve FEEL comparison operators");

    assert!(
      decision_table.rules[1].input_entries[0].text.contains("years and months duration"),
      "Should preserve FEEL duration function"
    );
    assert!(decision_table.rules[1].input_entries[0].text.contains("today()"), "Should preserve FEEL today function");
    assert!(decision_table.rules[1].input_entries[0].text.contains("< 18"), "Should preserve FEEL comparison operators");

    assert!(
      decision_table.rules[2].input_entries[0].text.contains("years and months duration"),
      "Should preserve FEEL duration function"
    );
    assert!(decision_table.rules[2].input_entries[0].text.contains("today()"), "Should preserve FEEL today function");
    assert!(decision_table.rules[2].input_entries[0].text.contains("< 65"), "Should preserve FEEL comparison operators");

    assert!(
      decision_table.rules[3].input_entries[0].text.contains("years and months duration"),
      "Should preserve FEEL duration function"
    );
    assert!(decision_table.rules[3].input_entries[0].text.contains("today()"), "Should preserve FEEL today function");
    assert!(decision_table.rules[3].input_entries[0].text.contains(">= 65"), "Should preserve FEEL comparison operators");
  }

  /// Test different hit policies in markdown decision tables
  #[test]
  fn test_different_hit_policies() {
    // Test Unique hit policy (U)
    let markdown_unique = r#"| U | Input | Output |
|:-:|:-----:|:------:|
|   |  `i`  |  `o`   |
| 1 |   A   |   1    |
| 2 |   B   |   2    |"#;

    let decision_table_unique = from_markdown(markdown_unique, false).expect("Failed to parse unique hit policy decision table");
    assert!(
      matches!(decision_table_unique.hit_policy, dsntk_recognizer::HitPolicy::Unique),
      "Should have Unique hit policy"
    );

    // Test Any hit policy (A)
    let markdown_any = r#"| A | Input | Output |
|:-:|:-----:|:------:|
|   |  `i`  |  `o`   |
| 1 |   A   |   1    |
| 2 |   A   |   1    |"#;

    let decision_table_any = from_markdown(markdown_any, false).expect("Failed to parse any hit policy decision table");
    assert!(matches!(decision_table_any.hit_policy, dsntk_recognizer::HitPolicy::Any), "Should have Any hit policy");

    // Test First hit policy (F)
    let markdown_first = r#"| F | Input | Output |
|:-:|:-----:|:------:|
|   |  `i`  |  `o`   |
| 1 |   A   |   1    |
| 2 |   A   |   2    |"#;

    let decision_table_first = from_markdown(markdown_first, false).expect("Failed to parse first hit policy decision table");
    assert!(
      matches!(decision_table_first.hit_policy, dsntk_recognizer::HitPolicy::First),
      "Should have First hit policy"
    );
  }

  /// Test actual decision table evaluation with input data for the copy-test.md example
  #[test]
  fn test_copy_test_decision_table_evaluation() {
    // Load the copy-test.md decision table
    let markdown = r#"| U | Employment Type | Duration | Compliance Status |
|:-:|:---------------:|:---------:|:-----------------:|
|   |       `i`       |    `i`    |        `o`        |
| 1 |     "trial"     |     90    |    "Compliant"    |
| 2 |     "trial"     |     91    |  "Non-Compliant"  |
| 3 |   "probation"   |     90    |    "Compliant"    |"#;

    // Parse the decision table
    let decision_table = from_markdown(markdown, false).expect("Failed to parse markdown decision table");

    // Convert to DMN model decision table
    let model_decision_table: ModelDecisionTable = decision_table.into();

    // Test case 1: trial employment with 90 days duration (should be Compliant)
    let scope1 = context(r#"{Employment Type: "trial", Duration: 90}"#).into();
    let evaluator1 = build_decision_table_evaluator(&scope1, &model_decision_table).expect("Failed to build evaluator");
    let result1 = evaluator1(&scope1);
    assert_eq!(value_string!("Compliant"), result1, "trial employment with 90 days should be Compliant");

    // Test case 2: trial employment with 91 days duration (should be Non-Compliant)
    let scope2 = context(r#"{Employment Type: "trial", Duration: 91}"#).into();
    let evaluator2 = build_decision_table_evaluator(&scope2, &model_decision_table).expect("Failed to build evaluator");
    let result2 = evaluator2(&scope2);
    assert_eq!(value_string!("Non-Compliant"), result2, "trial employment with 91 days should be Non-Compliant");

    // Test case 3: probation employment with 90 days duration (should be Compliant)
    let scope3 = context(r#"{Employment Type: "probation", Duration: 90}"#).into();
    let evaluator3 = build_decision_table_evaluator(&scope3, &model_decision_table).expect("Failed to build evaluator");
    let result3 = evaluator3(&scope3);
    assert_eq!(value_string!("Compliant"), result3, "probation employment with 90 days should be Compliant");

    // Test case 4: trial employment with 89 days duration (no matching rule, should be null)
    let scope4 = context(r#"{Employment Type: "trial", Duration: 89}"#).into();
    let evaluator4 = build_decision_table_evaluator(&scope4, &model_decision_table).expect("Failed to build evaluator");
    let result4 = evaluator4(&scope4);
    assert_eq!(
      value_null!("no rules matched, no output value defined"),
      result4,
      "trial employment with 89 days should have no matching rule"
    );
  }

  /// Test decision table evaluation with complex FEEL expressions
  #[test]
  fn test_complex_decision_table_evaluation() {
    let markdown = r#"| U | Customer | Order | Discount |
|:-:|:--------:|:-----:|:--------:|
|   |   `i`    |  `i`  |   `o`    |
| 1 | "Business" | <10  |   0.10   |
| 2 | "Business" | >=10 |   0.15   |
| 3 | "Private"  |   -  |   0.05   |"#;

    // Parse the decision table
    let decision_table = from_markdown(markdown, false).expect("Failed to parse markdown decision table");
    let model_decision_table: ModelDecisionTable = decision_table.into();

    // Test case 1: Business customer with order < 10
    let scope1 = context(r#"{Customer: "Business", Order: 5}"#).into();
    let evaluator1 = build_decision_table_evaluator(&scope1, &model_decision_table).expect("Failed to build evaluator");
    let result1 = evaluator1(&scope1);
    assert_eq!(value_number!(10, 2), result1, "Business customer with order 5 should get 0.10 discount");

    // Test case 2: Business customer with order >= 10
    let scope2 = context(r#"{Customer: "Business", Order: 15}"#).into();
    let evaluator2 = build_decision_table_evaluator(&scope2, &model_decision_table).expect("Failed to build evaluator");
    let result2 = evaluator2(&scope2);
    assert_eq!(value_number!(15, 2), result2, "Business customer with order 15 should get 0.15 discount");

    // Test case 3: Private customer (any order)
    let scope3 = context(r#"{Customer: "Private", Order: 20}"#).into();
    let evaluator3 = build_decision_table_evaluator(&scope3, &model_decision_table).expect("Failed to build evaluator");
    let result3 = evaluator3(&scope3);
    assert_eq!(value_number!(5, 2), result3, "Private customer should get 0.05 discount regardless of order");

    // Test case 4: No matching customer type
    let scope4 = context(r#"{Customer: "Government", Order: 10}"#).into();
    let evaluator4 = build_decision_table_evaluator(&scope4, &model_decision_table).expect("Failed to build evaluator");
    let result4 = evaluator4(&scope4);
    assert_eq!(
      value_null!("no rules matched, no output value defined"),
      result4,
      "Government customer should have no matching rule"
    );
  }

  /// Test decision table evaluation with multiple outputs
  #[test]
  fn test_decision_table_evaluation_multiple_outputs() {
    let markdown = r#"| U | Age | Risk | Status | Points |
|:-:|:---:|:----:|:------:|:------:|
|   | `i` | `i`  |   `o`  |   `o`  |
| 1 | <18 | "Low" | "Approved" | 10 |
| 2 | <18 | "High" | "Rejected" | 0 |
| 3 | >=18 | "Low" | "Approved" | 20 |
| 4 | >=18 | "High" | "Pending" | 5 |"#;

    // Parse the decision table
    let decision_table = from_markdown(markdown, false).expect("Failed to parse markdown decision table");
    let model_decision_table: ModelDecisionTable = decision_table.into();

    // Test case 1: Under 18 with low risk
    let scope1 = context(r#"{Age: 16, Risk: "Low"}"#).into();
    let evaluator1 = build_decision_table_evaluator(&scope1, &model_decision_table).expect("Failed to build evaluator");
    let result1 = evaluator1(&scope1);

    // Multiple outputs should return a context
    if let Value::Context(ctx) = result1 {
      assert_eq!(value_string!("Approved"), ctx.get_entry(&"Status".into()).unwrap().clone(), "Status should be Approved");
      assert_eq!(Value::Number(10.into()), ctx.get_entry(&"Points".into()).unwrap().clone(), "Points should be 10");
    } else {
      panic!("Expected context value for multiple outputs, got: {}", result1);
    }

    // Test case 2: Over 18 with high risk
    let scope2 = context(r#"{Age: 25, Risk: "High"}"#).into();
    let evaluator2 = build_decision_table_evaluator(&scope2, &model_decision_table).expect("Failed to build evaluator");
    let result2 = evaluator2(&scope2);

    if let Value::Context(ctx) = result2 {
      assert_eq!(value_string!("Pending"), ctx.get_entry(&"Status".into()).unwrap().clone(), "Status should be Pending");
      assert_eq!(Value::Number(5.into()), ctx.get_entry(&"Points".into()).unwrap().clone(), "Points should be 5");
    } else {
      panic!("Expected context value for multiple outputs, got: {}", result2);
    }
  }
  /// Test-driven development: Discover decision table capabilities with date input data
  #[test]
  fn test_age_calculation_decision_table_evaluation_tdd() {
    let markdown = r#"| F | Date of Birth | Age Group | Notes |
|:-:|:-------------:|:---------:|:-----:|
|   |     `i`       |    `o`    | `ann` |
| 1 | (years and months duration(Date of Birth, today()).years) < 13  | "child"   | "Child: 0-12 years" |
| 2 | (years and months duration(Date of Birth, today()).years) < 18  | "teenager" | "Teenager: 13-17 years" |
| 3 | (years and months duration(Date of Birth, today()).years) < 65  | "adult"   | "Adult: 18-64 years" |
| 4 | (years and months duration(Date of Birth, today()).years) >= 65  | "senior"  | "Senior: 65+ years" |"#;

    // Parse the decision table using DSNTK recognizer
    let decision_table = from_markdown(markdown, false).expect("Failed to parse age calculation decision table");

    // Convert to DMN model decision table
    let model_decision_table: ModelDecisionTable = decision_table.into();

    // Test case 1: Person born in 1995 should be classified as "adult" (~30 years old)
    let scope_adult = context(r#"{Date of Birth: date("1995-08-20")}"#).into();
    let evaluator_adult = build_decision_table_evaluator(&scope_adult, &model_decision_table).expect("Failed to build evaluator");
    let result_adult = evaluator_adult(&scope_adult);

    // Assert that the result is not null and equals "adult"
    assert!(!result_adult.is_null(), "Decision table evaluation should not return null for person born in 1995");
    assert_eq!(value_string!("adult"), result_adult, "Person born in 1995 should be classified as 'adult'");

    // Test case 2: Person born in 2015 should be classified as "child" (~10 years old)
    let scope_child = context(r#"{Date of Birth: date("2015-03-15")}"#).into();
    let evaluator_child = build_decision_table_evaluator(&scope_child, &model_decision_table).expect("Failed to build evaluator");
    let result_child = evaluator_child(&scope_child);

    // Assert that the result is not null and equals "child"
    assert!(!result_child.is_null(), "Decision table evaluation should not return null for person born in 2015");
    assert_eq!(value_string!("child"), result_child, "Person born in 2015 should be classified as 'child'");

    // Test case 3: Person born in 1950 should be classified as "senior" (~75 years old)
    let scope_senior = context(r#"{Date of Birth: date("1950-01-01")}"#).into();
    let evaluator_senior = build_decision_table_evaluator(&scope_senior, &model_decision_table).expect("Failed to build evaluator");
    let result_senior = evaluator_senior(&scope_senior);

    // Assert that the result is not null and equals "senior"
    assert!(!result_senior.is_null(), "Decision table evaluation should not return null for person born in 1950");
    assert_eq!(value_string!("senior"), result_senior, "Person born in 1950 should be classified as 'senior'");

    // Additional test case: Person born in 2015 should be classified as "child" (~11 years old)
    let scope_child2 = context(r#"{Date of Birth: date("2015-06-10")}"#).into();
    let evaluator_child2 = build_decision_table_evaluator(&scope_child2, &model_decision_table).expect("Failed to build evaluator");
    let result_child2 = evaluator_child2(&scope_child2);

    // Assert that the result is not null and equals "child"
    assert!(!result_child2.is_null(), "Decision table evaluation should not return null for person born in 2015");
    assert_eq!(value_string!("child"), result_child2, "Person born in 2015 should be classified as 'child'");
  }

  /// Test decision table evaluation with current time comparisons
  #[test]
  fn test_event_time_decision_table_evaluation() {
    let markdown = r#"| F | Event | Status | Notes |
    |:-:|:-----:|:------:|:-----:|
    |   |  `i`  |  `o`   | `ann` |
    | 1 | Event < today()  | "past"   | "Event is in the past" |
    | 2 | Event >= today() | "future" | "Event is in the future or today" |"#;

    // Parse the decision table using DSNTK recognizer
    let decision_table = from_markdown(markdown, false).expect("Failed to parse event time decision table");

    // Convert to DMN model decision table
    let model_decision_table: ModelDecisionTable = decision_table.into();

    // Test case 1: Event in the past (one week ago)
    // We'll use a date that's definitely in the past
    let scope_past = context(r#"{Event: date("1995-08-20")}"#).into();
    let evaluator_past = build_decision_table_evaluator(&scope_past, &model_decision_table).expect("Failed to build evaluator");
    let result_past = evaluator_past(&scope_past);

    // Assert that the result is not null and equals "past"
    assert!(!result_past.is_null(), "Decision table evaluation should not return null for past event");
    assert_eq!(value_string!("past"), result_past, "Event from 1995 should be classified as 'past'");

    // Test case 2: Event in the future (one week from now)
    // We'll use a date that's definitely in the future
    let scope_future = context(r#"{Event: date("2030-01-01")}"#).into();
    let evaluator_future = build_decision_table_evaluator(&scope_future, &model_decision_table).expect("Failed to build evaluator");
    let result_future = evaluator_future(&scope_future);

    // Assert that the result is not null and equals "future"
    assert!(!result_future.is_null(), "Decision table evaluation should not return null for future event");
    assert_eq!(value_string!("future"), result_future, "Event from 2030 should be classified as 'future'");

    // Test case 3: Event today (using today's date)
    // This will help us understand if the today() function works correctly
    let scope_today = context(r#"{Event: today()}"#).into();
    let evaluator_today = build_decision_table_evaluator(&scope_today, &model_decision_table).expect("Failed to build evaluator");
    let result_today = evaluator_today(&scope_today);

    // This test helps us discover if today() function works in decision table evaluation
    println!("DISCOVERY: Testing today() function in decision table");
    println!("- Input: {{Event: today()}}");
    println!("- Expected: \"future\" (since Event >= today() should match rule 2)");
    println!("- Actual: {:?}", result_today);
    println!("- This helps us understand if the today() function works in FEEL expressions within decision tables");
  }
}
