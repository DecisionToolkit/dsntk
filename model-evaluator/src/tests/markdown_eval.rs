//! Regression tests for evaluating decision tables parsed from markdown.
//!
//! These tests verify that `evaluate_decision_table_with_trace` correctly
//! handles various hit policies and FEEL name patterns (including multi-word
//! names) when the table is recognized from markdown format.

use crate::evaluate_decision_table_with_trace;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::{FeelScope, Name};

/// Helper: parse a markdown decision table body and evaluate it with the given scope.
fn eval_markdown(md_body: &str, scope: &FeelScope) -> crate::DecisionTableEvalResult {
  let recognized = dsntk_recognizer::from_markdown(md_body, false).expect("failed to parse markdown table");
  let model_dt: dsntk_model::DecisionTable = recognized.into();
  evaluate_decision_table_with_trace(scope, &model_dt).expect("evaluation failed")
}

/// Helper: build a FeelScope with entries from a list of (name, value) pairs.
fn scope_with(entries: &[(&str, Value)]) -> FeelScope {
  let scope = FeelScope::default();
  let mut ctx = FeelContext::default();
  for (name, value) in entries {
    ctx.set_entry(&Name::new(&[name]), value.clone());
  }
  scope.push(ctx);
  scope
}

/// Helper: parse a string as a FeelNumber.
fn num(s: &str) -> Value {
  Value::Number(s.parse().expect("invalid number"))
}

/// Helper: string value.
fn str_val(s: &str) -> Value {
  Value::String(s.to_string())
}

// ---------------------------------------------------------------------------
// FIRST hit policy with multi-word FEEL names
// ---------------------------------------------------------------------------

const CREDIT_GRADE_TABLE: &str = r#"
> # Credit Score Classification
> Credit Grade

| F  | Bureau Score | Years Trading | Credit Grade                |
|:--:|:------------:|:-------------:|:---------------------------:|
|    |              |               | "A","B","C","D","Decline"   |
|    | `in`         | `in`          | `out`                       |
|  1 | <30          | -             | "Decline"                   |
|  2 | >=80         | >=5           | "A"                         |
|  3 | [65..80)     | >=3           | "B"                         |
|  4 | [50..65)     | >=2           | "C"                         |
|  5 | -            | -             | "D"                         |
"#;

/// FIRST hit policy: high score, long trading → "A".
#[test]
fn _0001() {
  let scope = scope_with(&[("Bureau Score", num("92")), ("Years Trading", num("12"))]);
  let result = eval_markdown(CREDIT_GRADE_TABLE, &scope);
  assert_eq!(result.value, str_val("A"));
  assert!(result.matched_rules.contains(&1), "rule index 1 should match");
}

/// FIRST hit policy: low score → "Decline".
#[test]
fn _0002() {
  let scope = scope_with(&[("Bureau Score", num("20")), ("Years Trading", num("5"))]);
  let result = eval_markdown(CREDIT_GRADE_TABLE, &scope);
  assert_eq!(result.value, str_val("Decline"));
  assert!(result.matched_rules.contains(&0), "rule index 0 should match");
}

/// FIRST hit policy: catch-all fires when no specific rule matches.
#[test]
fn _0003() {
  let scope = scope_with(&[("Bureau Score", num("45")), ("Years Trading", num("1"))]);
  let result = eval_markdown(CREDIT_GRADE_TABLE, &scope);
  assert_eq!(result.value, str_val("D"));
  assert!(result.matched_rules.contains(&4), "catch-all rule index 4 should match");
}

// ---------------------------------------------------------------------------
// COLLECT hit policy with compound outputs
// ---------------------------------------------------------------------------

const RATE_ADJUSTMENTS_TABLE: &str = r#"
> # Rate Adjustments
> Rate Adjustment

| C  | Sector         | Has Guarantee | Adjustment | Reason Code |
|:--:|:--------------:|:------------:|:----------:|:-----------:|
|    | `in`           | `in`         | `out`      | `out`       |
|  1 | "Construction" | -            | 0.50       | "SEC-01"    |
|  2 | "Hospitality"  | -            | 0.40       | "SEC-02"    |
|  3 | -              | true         | -0.25      | "GTR-01"    |
"#;

/// COLLECT: multiple rules fire, compound outputs returned as list.
#[test]
fn _0004() {
  let scope = scope_with(&[("Sector", str_val("Construction")), ("Has Guarantee", Value::Boolean(true))]);
  let result = eval_markdown(RATE_ADJUSTMENTS_TABLE, &scope);
  // Both SEC-01 and GTR-01 should match.
  assert_eq!(result.matched_rules.len(), 2);
  assert!(result.matched_rules.contains(&0));
  assert!(result.matched_rules.contains(&2));
  // Output should be a list of contexts.
  match &result.value {
    Value::List(items) => assert_eq!(items.len(), 2),
    other => panic!("expected list, got: {other}"),
  }
}

/// COLLECT: no rules match → null.
#[test]
fn _0005() {
  let scope = scope_with(&[("Sector", str_val("Technology")), ("Has Guarantee", Value::Boolean(false))]);
  let result = eval_markdown(RATE_ADJUSTMENTS_TABLE, &scope);
  assert!(result.matched_rules.is_empty());
}

// ---------------------------------------------------------------------------
// Single-word output name (no spaces) works in FEEL evaluation
// ---------------------------------------------------------------------------

const SINGLE_WORD_OUTPUT_TABLE: &str = r#"
> # Sector Risk
> SectorRisk

| F  | Sector       | SectorRisk |
|:--:|:------------:|:----------:|
|    | `in`         | `out`      |
|  1 | "Technology" | "Low"      |
|  2 | "Retail"     | "Medium"   |
|  3 | -            | "High"     |
"#;

/// Single-word output name evaluates without parse errors.
#[test]
fn _0006() {
  let scope = scope_with(&[("Sector", str_val("Technology"))]);
  let result = eval_markdown(SINGLE_WORD_OUTPUT_TABLE, &scope);
  assert_eq!(result.value, str_val("Low"));
}

// ---------------------------------------------------------------------------
// Numeric comparisons in FEEL (ranges, inequalities)
// ---------------------------------------------------------------------------

const LEVERAGE_TABLE: &str = r#"
> # Leverage Check
> Risk

| F  | Leverage Ratio | Risk     |
|:--:|:--------------:|:--------:|
|    | `in`           | `out`    |
|  1 | <=0.50         | "Low"    |
|  2 | (0.50..0.75]   | "Medium" |
|  3 | >0.75          | "High"   |
"#;

/// Numeric range: low leverage.
#[test]
fn _0007() {
  let scope = scope_with(&[("Leverage Ratio", num("0.089"))]);
  let result = eval_markdown(LEVERAGE_TABLE, &scope);
  assert_eq!(result.value, str_val("Low"));
  assert!(result.matched_rules.contains(&0));
}

/// Numeric range: medium leverage.
#[test]
fn _0008() {
  let scope = scope_with(&[("Leverage Ratio", num("0.65"))]);
  let result = eval_markdown(LEVERAGE_TABLE, &scope);
  assert_eq!(result.value, str_val("Medium"));
}

/// Numeric range: high leverage.
#[test]
fn _0009() {
  let scope = scope_with(&[("Leverage Ratio", num("1.5"))]);
  let result = eval_markdown(LEVERAGE_TABLE, &scope);
  assert_eq!(result.value, str_val("High"));
}

// ---------------------------------------------------------------------------
// PRIORITY hit policy with allowed output values
// ---------------------------------------------------------------------------

const PRIORITY_TABLE: &str = r#"
> # Pricing
> Tier

| P  | Grade | Tier                 |
|:--:|:-----:|:--------------------:|
|    |       | "T1","T2","T3","T4"  |
|    | `in`  | `out`                |
|  1 | "A"   | "T1"                 |
|  2 | "B"   | "T2"                 |
|  3 | "A"   | "T2"                 |
|  4 | -     | "T4"                 |
"#;

/// PRIORITY: "A" matches rules 1 and 3; priority selects "T1" (earlier in allowed values).
#[test]
fn _0010() {
  let scope = scope_with(&[("Grade", str_val("A"))]);
  let result = eval_markdown(PRIORITY_TABLE, &scope);
  assert_eq!(result.value, str_val("T1"));
}

// ---------------------------------------------------------------------------
// FEEL expression evaluation (not decision tables)
// ---------------------------------------------------------------------------

/// FEEL expression: simple division with multi-word names in scope.
#[test]
fn _0011() {
  let scope = scope_with(&[("Requested Amount", num("350000")), ("Valuation Amount", num("750000"))]);
  let ast = dsntk_feel_parser::parse_expression(&scope, "Requested Amount / Valuation Amount", false).expect("failed to parse expression");
  let evaluator = dsntk_feel_evaluator::prepare(&ast);
  let result = evaluator(&scope);
  // 350000 / 750000 = 0.466666...
  match &result {
    Value::Number(n) => {
      let expected: dsntk_feel::FeelNumber = "0.4666666666666666666666666666666667".parse().unwrap();
      assert_eq!(*n, expected);
    }
    other => panic!("expected number, got: {other}"),
  }
}

/// FEEL expression: DSCR formula (net profit + depreciation + interest) / debt service.
#[test]
fn _0012() {
  let scope = scope_with(&[
    ("Net Profit", num("680000")),
    ("Depreciation", num("45000")),
    ("Interest Expense", num("32000")),
    ("Annual Debt Service", num("70000")),
  ]);
  let ast = dsntk_feel_parser::parse_expression(&scope, "(Net Profit + Depreciation + Interest Expense) / Annual Debt Service", false).expect("failed to parse DSCR expression");
  let evaluator = dsntk_feel_evaluator::prepare(&ast);
  let result = evaluator(&scope);
  // (680000 + 45000 + 32000) / 70000 = 10.81428...
  match &result {
    Value::Number(_) => {} // Just verify it's a number, not null
    other => panic!("expected number, got: {other}"),
  }
}

/// FEEL expression: division by zero returns null.
#[test]
fn _0013() {
  let scope = scope_with(&[("Requested Amount", num("350000")), ("Valuation Amount", num("0"))]);
  let ast = dsntk_feel_parser::parse_expression(&scope, "Requested Amount / Valuation Amount", false).expect("failed to parse expression");
  let evaluator = dsntk_feel_evaluator::prepare(&ast);
  let result = evaluator(&scope);
  match &result {
    Value::Null(_) => {}
    other => panic!("expected null for division by zero, got: {other}"),
  }
}

/// FEEL expression: missing name in scope produces null.
#[test]
fn _0014() {
  let scope = scope_with(&[("Requested Amount", num("350000"))]);
  // "Valuation Amount" is NOT in scope — parser may fail or evaluator returns null
  let ast = dsntk_feel_parser::parse_expression(&scope, "Requested Amount / Valuation Amount", false);
  match ast {
    Ok(node) => {
      let evaluator = dsntk_feel_evaluator::prepare(&node);
      let result = evaluator(&scope);
      match &result {
        Value::Null(_) => {}
        other => panic!("expected null for missing name, got: {other}"),
      }
    }
    Err(_) => {} // Parser rejection is also acceptable
  }
}

// ---------------------------------------------------------------------------
// Downstream decision consuming expression BKM output
// ---------------------------------------------------------------------------

const COLLATERAL_ADEQUACY_TABLE: &str = r#"
> # Collateral Adequacy
> Collateral Grade

| F  | LTV          | Property Risk    | Purpose            | Collateral Grade                                 |
|:--:|:------------:|:----------------:|:------------------:|:------------------------------------------------:|
|    |              |                  |                    | "Strong","Adequate","Marginal","Weak","Decline"  |
|    | `in`         | `in`             | `in`               | `out`                                            |
|  1 | -            | "Unacceptable"   | -                  | "Decline"                                        |
|  2 | >0.85        | -                | -                  | "Decline"                                        |
|  3 | >0.65        | -                | "Development"      | "Decline"                                        |
|  4 | <=0.50       | "Low"            | -                  | "Strong"                                         |
|  5 | (0.50..0.65] | "Low"            | -                  | "Adequate"                                       |
|  6 | (0.65..0.75] | "Low"            | -                  | "Marginal"                                       |
|  7 | (0.75..0.85] | "Low"            | -                  | "Weak"                                           |
|  8 | <=0.50       | "Medium"         | -                  | "Adequate"                                       |
|  9 | (0.50..0.65] | "Medium"         | -                  | "Marginal"                                       |
| 10 | (0.65..0.75] | "Medium"         | -                  | "Weak"                                           |
| 11 | (0.75..0.85] | "Medium"         | -                  | "Decline"                                        |
| 12 | <=0.50       | "High"           | -                  | "Marginal"                                       |
| 13 | (0.50..0.65] | "High"           | -                  | "Weak"                                           |
| 14 | >0.65        | "High"           | -                  | "Decline"                                        |
"#;

/// Downstream: LTV ~0.47 + Low risk + Purchase → "Strong" (rule 4).
#[test]
fn _0015() {
  let scope = scope_with(&[("LTV", num("0.47")), ("Property Risk", str_val("Low")), ("Purpose", str_val("Purchase"))]);
  let result = eval_markdown(COLLATERAL_ADEQUACY_TABLE, &scope);
  assert_eq!(result.value, str_val("Strong"));
  assert!(result.matched_rules.contains(&3), "rule index 3 (row 4) should match");
}

/// Downstream: LTV 0.71 + Medium risk → "Weak" (rule 10).
#[test]
fn _0016() {
  let scope = scope_with(&[("LTV", num("0.71")), ("Property Risk", str_val("Medium")), ("Purpose", str_val("Refinance"))]);
  let result = eval_markdown(COLLATERAL_ADEQUACY_TABLE, &scope);
  assert_eq!(result.value, str_val("Weak"));
  assert!(result.matched_rules.contains(&9), "rule index 9 (row 10) should match");
}

/// FIRST hit policy: Unacceptable property always declines regardless of LTV (rule 1 wins).
#[test]
fn _0017() {
  let scope = scope_with(&[("LTV", num("0.30")), ("Property Risk", str_val("Unacceptable")), ("Purpose", str_val("Purchase"))]);
  let result = eval_markdown(COLLATERAL_ADEQUACY_TABLE, &scope);
  assert_eq!(result.value, str_val("Decline"));
  assert!(result.matched_rules.contains(&0), "rule index 0 (row 1) should match first");
}

/// FIRST hit policy: LTV > 0.85 always declines (rule 2 wins over lower rules).
#[test]
fn _0018() {
  let scope = scope_with(&[("LTV", num("0.90")), ("Property Risk", str_val("Low")), ("Purpose", str_val("Purchase"))]);
  let result = eval_markdown(COLLATERAL_ADEQUACY_TABLE, &scope);
  assert_eq!(result.value, str_val("Decline"));
}

/// Development purpose with LTV > 0.65 declines (rule 3).
#[test]
fn _0019() {
  let scope = scope_with(&[("LTV", num("0.70")), ("Property Risk", str_val("Low")), ("Purpose", str_val("Development"))]);
  let result = eval_markdown(COLLATERAL_ADEQUACY_TABLE, &scope);
  assert_eq!(result.value, str_val("Decline"));
}
