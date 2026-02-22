//! Tests for progressive tax calculations using temporal functions within decision tables.

use crate::decision_table::build_decision_table_evaluator;
use crate::tests::context;
use dsntk_feel::values::Value;
use dsntk_model::DecisionTable;
use dsntk_recognizer::from_markdown;

/// Progressive tax decision table with C+ (Collect Sum) hit policy.
/// This represents New Zealand's marginal tax system where multiple rules apply simultaneously
/// and their results are summed.
/// Note: Ranges are overlapping to allow multiple brackets to apply to different portions of income.
const PROGRESSIVE_TAX_TABLE_MARKDOWN: &str = r#"
| C+ | Income       | Tax Calculation for Band                     |
|:-:|:-------------:|:--------------------------------------------:|
|   |  `i`          |           `o`                                |
| 1 | [0..)         | min(Income, 15600) * 0.105                   |
| 2 | [15600..)     | max(0, min(Income, 53500) - 15600) * 0.175   |
| 3 | [53500..)     | max(0, min(Income, 78100) - 53500) * 0.30    |
| 4 | [78100..)     | max(0, min(Income, 180000) - 78100) * 0.33   |
| 5 | [180000..)    | max(0, Income - 180000) * 0.39               |
"#;

#[test]
fn test_progressive_tax_calculation_markdown() {
    // Parse the decision table from markdown representation
    let decision_table: DecisionTable = from_markdown(PROGRESSIVE_TAX_TABLE_MARKDOWN, false).unwrap().into();
    
    // Test 1: Income below first threshold ($10,000)
    let scope_10k = context(r#"{Income: 10000}"#).into();
    let evaluator = build_decision_table_evaluator(&scope_10k, &decision_table).unwrap();
    let result_10k = evaluator(&scope_10k);
    
    // Expected: 10000 × 0.105 = 1050
    match result_10k {
        Value::Number(n) => {
            assert_eq!(n.to_string(), "1050", "Expected 1050 for $10,000 income, got: {}", n);
        }
        _ => panic!("Expected number result for $10,000 income, got: {}", result_10k),
    }
    
    // Test 2: Income at first bracket boundary ($14,000)
    let scope_14k = context(r#"{Income: 14000}"#).into();
    let evaluator = build_decision_table_evaluator(&scope_14k, &decision_table).unwrap();
    let result_14k = evaluator(&scope_14k);
    
    // Expected: 14000 × 0.105 = 1470
    match result_14k {
        Value::Number(n) => {
            assert_eq!(n.to_string(), "1470", "Expected 1470 for $14,000 income, got: {}", n);
        }
        _ => panic!("Expected number result for $14,000 income, got: {}", result_14k),
    }
    
    // Test 3: Income between brackets ($30,000)
    let scope_30k = context(r#"{Income: 30000}"#).into();
    let evaluator = build_decision_table_evaluator(&scope_30k, &decision_table).unwrap();
    let result_30k = evaluator(&scope_30k);
    
    // Expected:
    // - First bracket: 14000 × 0.105 = 1470
    // - Second bracket: (30000 - 14000) × 0.175 = 16000 × 0.175 = 2800
    // Total: 1470 + 2800 = 4270
    match result_30k {
        Value::Number(n) => {
            assert_eq!(n.to_string(), "4270", "Expected 4270 for $30,000 income, got: {}", n);
        }
        _ => panic!("Expected number result for $30,000 income, got: {}", result_30k),
    }
    
    // Test 4: Income at second bracket boundary ($48,000)
    let scope_48k = context(r#"{Income: 48000}"#).into();
    let evaluator = build_decision_table_evaluator(&scope_48k, &decision_table).unwrap();
    let result_48k = evaluator(&scope_48k);
    
    // Expected:
    // - First bracket: 14000 × 0.105 = 1470
    // - Second bracket: (48000 - 14000) × 0.175 = 34000 × 0.175 = 5950
    // Total: 1470 + 5950 = 7420
    match result_48k {
        Value::Number(n) => {
            assert_eq!(n.to_string(), "7420", "Expected 7420 for $48,000 income, got: {}", n);
        }
        _ => panic!("Expected number result for $48,000 income, got: {}", result_48k),
    }
    
    // Test 5: Income between brackets ($60,000)
    let scope_60k = context(r#"{Income: 60000}"#).into();
    let evaluator = build_decision_table_evaluator(&scope_60k, &decision_table).unwrap();
    let result_60k = evaluator(&scope_60k);
    
    // Expected:
    // - First bracket: 14000 × 0.105 = 1470
    // - Second bracket: (48000 - 14000) × 0.175 = 34000 × 0.175 = 5950
    // - Third bracket: (60000 - 48000) × 0.30 = 12000 × 0.30 = 3600
    // Total: 1470 + 5950 + 3600 = 11020
    match result_60k {
        Value::Number(n) => {
            assert_eq!(n.to_string(), "11020", "Expected 11020 for $60,000 income, got: {}", n);
        }
        _ => panic!("Expected number result for $60,000 income, got: {}", result_60k),
    }
    
    // Test 6: Income at third bracket boundary ($70,000)
    let scope_70k = context(r#"{Income: 70000}"#).into();
    let evaluator = build_decision_table_evaluator(&scope_70k, &decision_table).unwrap();
    let result_70k = evaluator(&scope_70k);
    
    // Expected:
    // - First bracket: 14000 × 0.105 = 1470
    // - Second bracket: (48000 - 14000) × 0.175 = 34000 × 0.175 = 5950
    // - Third bracket: (70000 - 48000) × 0.30 = 22000 × 0.30 = 6600
    // Total: 1470 + 5950 + 6600 = 14020
    match result_70k {
        Value::Number(n) => {
            assert_eq!(n.to_string(), "14020", "Expected 14020 for $70,000 income, got: {}", n);
        }
        _ => panic!("Expected number result for $70,000 income, got: {}", result_70k),
    }
    
    // Test 7: Income between brackets ($120,000)
    let scope_120k = context(r#"{Income: 120000}"#).into();
    let evaluator = build_decision_table_evaluator(&scope_120k, &decision_table).unwrap();
    let result_120k = evaluator(&scope_120k);
    
    // Expected:
    // - First bracket: 14000 × 0.105 = 1470
    // - Second bracket: (48000 - 14000) × 0.175 = 34000 × 0.175 = 5950
    // - Third bracket: (70000 - 48000) × 0.30 = 22000 × 0.30 = 6600
    // - Fourth bracket: (120000 - 70000) × 0.33 = 50000 × 0.33 = 16500
    // Total: 1470 + 5950 + 6600 + 16500 = 30520
    match result_120k {
        Value::Number(n) => {
            assert_eq!(n.to_string(), "30520", "Expected 30520 for $120,000 income, got: {}", n);
        }
        _ => panic!("Expected number result for $120,000 income, got: {}", result_120k),
    }
    
    // Test 8: Income at fourth bracket boundary ($180,000)
    let scope_180k = context(r#"{Income: 180000}"#).into();
    let evaluator = build_decision_table_evaluator(&scope_180k, &decision_table).unwrap();
    let result_180k = evaluator(&scope_180k);
    
    // Expected:
    // - First bracket: 14000 × 0.105 = 1470
    // - Second bracket: (48000 - 14000) × 0.175 = 34000 × 0.175 = 5950
    // - Third bracket: (70000 - 48000) × 0.30 = 22000 × 0.30 = 6600
    // - Fourth bracket: (180000 - 70000) × 0.33 = 110000 × 0.33 = 36300
    // Total: 1470 + 5950 + 6600 + 36300 = 50320
    match result_180k {
        Value::Number(n) => {
            assert_eq!(n.to_string(), "50320", "Expected 50320 for $180,000 income, got: {}", n);
        }
        _ => panic!("Expected number result for $180,000 income, got: {}", result_180k),
    }
    
    // Test 9: Income above highest bracket ($200,000)
    let scope_200k = context(r#"{Income: 200000}"#).into();
    let evaluator = build_decision_table_evaluator(&scope_200k, &decision_table).unwrap();
    let result_200k = evaluator(&scope_200k);
    
    // Expected:
    // - First bracket: 14000 × 0.105 = 1470
    // - Second bracket: (48000 - 14000) × 0.175 = 34000 × 0.175 = 5950
    // - Third bracket: (70000 - 48000) × 0.30 = 22000 × 0.30 = 6600
    // - Fourth bracket: (180000 - 70000) × 0.33 = 110000 × 0.33 = 36300
    // - Fifth bracket: (200000 - 180000) × 0.39 = 20000 × 0.39 = 7800
    // Total: 1470 + 5950 + 6600 + 36300 + 7800 = 58120
    match result_200k {
        Value::Number(n) => {
            assert_eq!(n.to_string(), "58120", "Expected 58120 for $200,000 income, got: {}", n);
        }
        _ => panic!("Expected number result for $200,000 income, got: {}", result_200k),
    }
    
    // Test 10: Zero income edge case
    let scope_zero = context(r#"{Income: 0}"#).into();
    let evaluator = build_decision_table_evaluator(&scope_zero, &decision_table).unwrap();
    let result_zero = evaluator(&scope_zero);
    
    // Expected: 0 × 0.105 = 0
    match result_zero {
        Value::Number(n) => {
            assert_eq!(n.to_string(), "0.00", "Expected 0 for zero income, got: {}", n);
        }
        _ => panic!("Expected number result for zero income, got: {}", result_zero),
    }
}

/// Unicode equivalent of PROGRESSIVE_TAX_TABLE_MARKDOWN
const PROGRESSIVE_TAX_TABLE: &str = r#"
┌───┬─────────────────────╥─────────────────────────────────────────────┐
│ C+│ Income              ║ Tax Calculation for Band                    │
╞═══╪═════════════════════╬═════════════════════════════════════════════╡
│   │ `i`                 ║ `o`                                         │
├───┼─────────────────────╫─────────────────────────────────────────────┤
│ 1 │ [0..)               ║ min(Income, 15600) * 0.105                  │
├───┼─────────────────────╫─────────────────────────────────────────────┤
│ 2 │ [15600..)           ║ max(0, min(Income, 53500) - 15600) * 0.175  │
├───┼─────────────────────╫─────────────────────────────────────────────┤
│ 3 │ [53500..)           ║ max(0, min(Income, 78100) - 53500) * 0.30   │
├───┼─────────────────────╫─────────────────────────────────────────────┤
│ 4 │ [78100..)           ║ max(0, min(Income, 180000) - 78100) * 0.33  │
├───┼─────────────────────╫─────────────────────────────────────────────┤
│ 5 │ [180000..)          ║ max(0, Income - 180000) * 0.39              │
└───┴─────────────────────╨─────────────────────────────────────────────┘
"#;

#[test]
fn test_progressive_tax_calculation_unicode() {
    // Parse the decision table from unicode representation
    let decision_table: DecisionTable = dsntk_recognizer::from_unicode(PROGRESSIVE_TAX_TABLE, false).unwrap().into();
    
    // Test a few key cases to verify unicode parsing works the same as markdown
    
    // Test: Income between brackets ($30,000)
    let scope_30k = context(r#"{Income: 30000}"#).into();
    let evaluator = build_decision_table_evaluator(&scope_30k, &decision_table).unwrap();
    let result_30k = evaluator(&scope_30k);
    
    // Expected: 1470 + 2800 = 4270
    match result_30k {
        Value::Number(n) => {
            assert_eq!(n.to_string(), "4270", "Expected 4270 for $30,000 income (unicode), got: {}", n);
        }
        _ => panic!("Expected number result for $30,000 income (unicode), got: {}", result_30k),
    }
    
    // Test: Income above highest bracket ($200,000)
    let scope_200k = context(r#"{Income: 200000}"#).into();
    let evaluator = build_decision_table_evaluator(&scope_200k, &decision_table).unwrap();
    let result_200k = evaluator(&scope_200k);
    
    // Expected: 58120
    match result_200k {
        Value::Number(n) => {
            assert_eq!(n.to_string(), "58120", "Expected 58120 for $200,000 income (unicode), got: {}", n);
        }
        _ => panic!("Expected number result for $200,000 income (unicode), got: {}", result_200k),
    }
}