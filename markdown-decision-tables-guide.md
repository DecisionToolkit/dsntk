# Using dsntk-rs with Markdown Decision Tables

This guide provides comprehensive documentation for using dsntk-rs with markdown decision tables, including required dependencies, table formats, file structure requirements, and complete code examples.

## Table of Contents

1. [Required Crates and Dependencies](#required-crates-and-dependencies)
2. [Markdown Decision Table Format](#markdown-decision-table-format)
   - [Horizontal Decision Tables (Rules as Rows)](#horizontal-decision-tables-rules-as-rows)
   - [Vertical Decision Tables (Rules as Columns)](#vertical-decision-tables-rules-as-columns)
3. [File Structure Requirements](#file-structure-requirements)
4. [Complete Code Examples](#complete-code-examples)
   - [Basic Decision Table Processing](#basic-decision-table-processing)
   - [Advanced Decision Table with Multiple Outputs](#advanced-decision-table-with-multiple-outputs)
   - [Working with Hit Policies](#working-with-hit-policies)
5. [Working Example with Test Cases](#working-example-with-test-cases)

## Required Crates and Dependencies

To use dsntk-rs with markdown decision tables, add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
dsntk = "0.3.0-dev"
dsntk-recognizer = "0.3.0-dev"
dsntk-model = "0.3.0-dev"
dsntk-model-evaluator = "0.3.0-dev"
dsntk-feel = "0.3.0-dev"
dsntk-feel-parser = "0.3.0-dev"
dsntk-feel-evaluator = "0.3.0-dev"
```

These crates provide:
- **dsntk-recognizer**: Parses markdown decision tables
- **dsntk-model**: Defines decision table data structures
- **dsntk-model-evaluator**: Evaluates decision tables against input data
- **dsntk-feel**: FEEL (Friendly Enough Expression Language) support
- **dsntk-feel-parser**: Parses FEEL expressions
- **dsntk-feel-evaluator**: Evaluates FEEL expressions

## Markdown Decision Table Format

### Horizontal Decision Tables (Rules as Rows)

Horizontal decision tables organize rules as rows. Each row represents a complete rule with input conditions and output values.

#### Basic Structure

```markdown
| U | Customer | Order | Discount |
|:-:|:--------:|:-----:|:--------:|
|   |   `i`    |  `i`  |   `o`    |
| 1 | "Business" | <10  |   0.10   |
| 2 | "Business" | >=10 |   0.15   |
| 3 | "Private"  |   -  |   0.05   |
```

**Key Components:**
- **Hit Policy (U)**: First column indicates the hit policy
- **Input Clauses (`i`)**: Marked with `i`, `input`, `>>>`, or `>>`
- **Output Clauses (`o`)**: Marked with `o`, `output`, `<<<`, or `<<`
- **Annotation Clauses**: Marked with `a`, `annotation`, `###`, `##`, or `#`
- **Rules**: Numbered rows containing actual rule data

#### Supported Hit Policies

| Policy | Description |
|--------|-------------|
| `U` | Unique - Only one rule can match |
| `A` | Any - Any matching rule, all outputs must be equal |
| `P` | Priority - First rule by output priority |
| `F` | First - First rule in order |
| `C` | Collect - Collect all matching rules |
| `C+` | Collect with sum aggregation |
| `C#` | Collect with count aggregation |
| `C<` | Collect with min aggregation |
| `C>` | Collect with max aggregation |

### Vertical Decision Tables (Rules as Columns)

Vertical decision tables organize rules as columns. Each column represents a complete rule.

#### Basic Structure

```markdown
> # Discount calculation
> Discount percentage

| U                     |       |   1    |    2     |    3     |
|:----------------------|:-----:|:------:|:--------:|:--------:|
| Customer              | `in`  | "Business" | "Business" | "Private" |
| Order                 | `in`  |   <10   |   >=10   |    -     |
| Discount              | `out` |   0.10   |   0.15   |   0.05   |
| Description           | `ann` | Small business | Large business | Private customer |
```

**Key Components:**
- **Information Item Name**: `> # Discount calculation`
- **Output Label**: `> Discount percentage`
- **Hit Policy (U)**: First row indicates the hit policy
- **Input Clauses (`in`)**: Marked with `in`, `input`, `>>>`, or `>>`
- **Output Clauses (`out`)**: Marked with `out`, `output`, `<<<`, or `<<`
- **Annotation Clauses (`ann`)**: Marked with `ann`, `annotation`, `###`, `##`, or `#`
- **Rules**: Numbered columns containing actual rule data

## File Structure Requirements

### Single Table Per File

Each markdown file should contain **only one decision table**. The parser expects this structure and will only recognize the first table found in the file.

#### Valid Structure
```markdown
> # Decision table name
> Output label (optional)

| U | Input | Output |
|:-:|:-----:|:------:|
|   |  `i`  |  `o`   |
| 1 | value | result |
```

#### Invalid Structure (Multiple Tables)
```markdown
| U | Input | Output |
|:-:|:-----:|:------:|
|   |  `i`  |  `o`   |
| 1 | value | result |

| U | Input | Output |
|:-:|:-----:|:------:|
|   |  `i`  |  `o`   |
| 1 | value | result |
```

### File Naming Convention

Use descriptive names that reflect the decision table's purpose:
- `discount-calculation.md`
- `risk-assessment.md`
- `eligibility-check.md`

## Complete Code Examples

### Basic Decision Table Processing

```rust
use dsntk_recognizer::from_markdown;
use dsntk_model_evaluator::ModelEvaluator;
use dsntk_feel::values::Value;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the markdown decision table
    let markdown = r#"
        | U | Customer | Order | Discount |
        |:-:|:--------:|:-----:|:--------:|
        |   |   `i`    |  `i`  |   `o`    |
        | 1 | "Business" | <10  |   0.10   |
        | 2 | "Business" | >=10 |   0.15   |
        | 3 | "Private"  |   -  |   0.05   |
    "#;

    // Parse the decision table
    let decision_table = from_markdown(markdown, false)?;
    
    // Create input context
    let mut context = HashMap::new();
    context.insert("Customer".to_string(), Value::String("Business".to_string()));
    context.insert("Order".to_string(), Value::Number(8.into()));
    
    // Evaluate the decision table
    let evaluator = ModelEvaluator::new();
    let result = evaluator.evaluate_decision_table(&decision_table, &context)?;
    
    println!("Discount: {:?}", result);
    Ok(())
}
```

### Advanced Decision Table with Multiple Outputs

```rust
use dsntk_recognizer::from_markdown;
use dsntk_model_evaluator::ModelEvaluator;
use dsntk_feel::values::Value;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = r#"
        > # Risk Assessment
        > Risk level and premium

        | U | Age | Medical History | Risk Level | Premium |
        |:-:|:---:|:--------------:|:----------:|:-------:|
        |   | `i` |      `i`       |    `o`     |  `o`    |
        | 1 | <25 |    "good"      |   "Low"    |   100   |
        | 2 | <25 |     "bad"      |  "Medium"  |   200   |
        | 3 | [25..60] |   -    |  "Medium"  |   150   |
        | 4 | >60 |    "good"      |  "Medium"  |   180   |
        | 5 | >60 |     "bad"      |   "High"   |   300   |
    "#;

    let decision_table = from_markdown(markdown, false)?;
    
    let mut context = HashMap::new();
    context.insert("Age".to_string(), Value::Number(30.into()));
    context.insert("Medical History".to_string(), Value::String("good".to_string()));
    
    let evaluator = ModelEvaluator::new();
    let result = evaluator.evaluate_decision_table(&decision_table, &context)?;
    
    println!("Assessment result: {:?}", result);
    Ok(())
}
```

### Working with Hit Policies

```rust
use dsntk_recognizer::{from_markdown, HitPolicy};
use dsntk_model_evaluator::ModelEvaluator;
use dsntk_feel::values::Value;
use std::collections::HashMap;

fn process_different_hit_policies() -> Result<(), Box<dyn std::error::Error>> {
    // Unique hit policy (default)
    let unique_table = r#"
        | U | Score | Grade |
        |:-:|:-----:|:-----:|
        |   |  `i`  |  `o`  |
        | 1 | >=90  |  "A"  |
        | 2 | >=80  |  "B"  |
        | 3 | >=70  |  "C"  |
        | 4 |  <70  |  "F"  |
    "#;

    // Collect hit policy with sum aggregation
    let collect_table = r#"
        | C+ | Product | Quantity | Total |
        |:--:|:-------:|:--------:|:-----:|
        |    |   `i`   |    `i`   |  `o`  |
        | 1  | "A"     |    2     |  20   |
        | 2  | "B"     |    3     |  30   |
        | 3  | "A"     |    1     |  10   |
    "#;

    let dt_unique = from_markdown(unique_table, false)?;
    let dt_collect = from_markdown(collect_table, false)?;
    
    assert_eq!(dt_unique.hit_policy, HitPolicy::Unique);
    assert!(matches!(dt_collect.hit_policy, HitPolicy::Collect(_)));
    
    Ok(())
}
```

## Working Example with Test Cases

Here's a complete working example with comprehensive test cases:

```rust
use dsntk_recognizer::from_markdown;
use dsntk_model_evaluator::ModelEvaluator;
use dsntk_feel::values::Value;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    const DISCOUNT_TABLE: &str = r#"
        | U | Customer | Order | Discount |
        |:-:|:--------:|:-----:|:--------:|
        |   |   `i`    |  `i`  |   `o`    |
        | 1 | "Business" | <10  |   0.10   |
        | 2 | "Business" | >=10 |   0.15   |
        | 3 | "Private"  |   -  |   0.05   |
    "#;

    #[test]
    fn test_business_small_order() -> Result<(), Box<dyn std::error::Error>> {
        let decision_table = from_markdown(DISCOUNT_TABLE, false)?;
        let evaluator = ModelEvaluator::new();
        
        let mut context = HashMap::new();
        context.insert("Customer".to_string(), Value::String("Business".to_string()));
        context.insert("Order".to_string(), Value::Number(8.into()));
        
        let result = evaluator.evaluate_decision_table(&decision_table, &context)?;
        assert_eq!(result.get("Discount"), Some(&Value::Number(0.10.into())));
        Ok(())
    }

    #[test]
    fn test_business_large_order() -> Result<(), Box<dyn std::error::Error>> {
        let decision_table = from_markdown(DISCOUNT_TABLE, false)?;
        let evaluator = ModelEvaluator::new();
        
        let mut context = HashMap::new();
        context.insert("Customer".to_string(), Value::String("Business".to_string()));
        context.insert("Order".to_string(), Value::Number(15.into()));
        
        let result = evaluator.evaluate_decision_table(&decision_table, &context)?;
        assert_eq!(result.get("Discount"), Some(&Value::Number(0.15.into())));
        Ok(())
    }

    #[test]
    fn test_private_customer() -> Result<(), Box<dyn std::error::Error>> {
        let decision_table = from_markdown(DISCOUNT_TABLE, false)?;
        let evaluator = ModelEvaluator::new();
        
        let mut context = HashMap::new();
        context.insert("Customer".to_string(), Value::String("Private".to_string()));
        context.insert("Order".to_string(), Value::Number(5.into()));
        
        let result = evaluator.evaluate_decision_table(&decision_table, &context)?;
        assert_eq!(result.get("Discount"), Some(&Value::Number(0.05.into())));
        Ok(())
    }

    #[test]
    fn test_no_matching_rule() -> Result<(), Box<dyn std::error::Error>> {
        let decision_table = from_markdown(DISCOUNT_TABLE, false)?;
        let evaluator = ModelEvaluator::new();
        
        let mut context = HashMap::new();
        context.insert("Customer".to_string(), Value::String("Government".to_string()));
        context.insert("Order".to_string(), Value::Number(10.into()));
        
        let result = evaluator.evaluate_decision_table(&decision_table, &context)?;
        assert!(result.is_empty()); // No matching rules
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example usage
    let decision_table = from_markdown(DISCOUNT_TABLE, false)?;
    let evaluator = ModelEvaluator::new();
    
    let mut context = HashMap::new();
    context.insert("Customer".to_string(), Value::String("Business".to_string()));
    context.insert("Order".to_string(), Value::Number(12.into()));
    
    let result = evaluator.evaluate_decision_table(&decision_table, &context)?;
    
    if let Some(discount) = result.get("Discount") {
        println!("Applied discount: {}", discount);
    } else {
        println!("No discount applicable");
    }
    
    Ok(())
}
```

### Cargo.toml for the Example

```toml
[package]
name = "dsntk-example"
version = "0.1.0"
edition = "2021"

[dependencies]
dsntk-recognizer = "0.3.0-dev"
dsntk-model-evaluator = "0.3.0-dev"
dsntk-feel = "0.3.0-dev"
```

## Best Practices

1. **Single Responsibility**: Each decision table should represent one business decision
2. **Descriptive Names**: Use clear names for information items and output labels
3. **Consistent Formatting**: Maintain consistent column alignment and spacing
4. **Validation**: Always validate parsed decision tables before evaluation
5. **Error Handling**: Implement proper error handling for parsing and evaluation
6. **Testing**: Create comprehensive test cases covering all rule combinations

## Common Patterns

### Using Default Values

```markdown
| U | Status | Priority |
|:-:|:------:|:--------:|
|   |  `i`   |   `o`    |
|   |        | "Normal" |
| 1 | "High" | "Urgent" |
```

### Range Expressions

```markdown
| U | Age Group | Discount |
|:-:|:---------:|:--------:|
|   |    `i`    |   `o`    |
| 1 |   <18     |   0.20   |
| 2 |  [18..65] |   0.10   |
| 3 |   >65     |   0.15   |
```

### Multiple Conditions

```markdown
| U | Member | Premium | Discount |
|:-:|:------:|:-------:|:--------:|
|   |  `i`   |   `i`   |   `o`    |
| 1 |  true  |  >=1000 |   0.25   |
| 2 |  true  |   <1000 |   0.15   |
| 3 |  false |    -    |   0.05   |
```

This guide provides everything needed to start using dsntk-rs with markdown decision tables effectively. The examples can be copied and used immediately in your projects.

## Struct Recommendations for Payroll/Legal Decision Tables

When implementing payroll or legal decision tables, it's recommended to use a combination of existing dsntk structs and domain-specific structs for type safety and maintainability.

### Using Existing dsntk Structs

The dsntk-model crate provides core structs that should be used as the foundation:

```rust
use dsntk_model::{DecisionTable, InputClause, OutputClause};
use dsntk_feel::values::Value;
use std::collections::HashMap;

// Core decision table structs from dsntk-model
let decision_table: DecisionTable = from_markdown(markdown_content, false)?;
let input_clauses: &Vec<InputClause> = &decision_table.input;
let output_clauses: &Vec<OutputClause> = &decision_table.output;
```

### Creating Domain-Specific Payroll Structs

For payroll applications, create domain-specific structs that wrap the core dsntk structs:

```rust
use serde::{Deserialize, Serialize};

// Domain-specific input struct for payroll calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollInput {
    pub employee_type: String,
    pub annual_salary: f64,
    pub pay_frequency: String, // "weekly", "fortnightly", "monthly"
    pub tax_code: String,
    pub kiwi_saver_rate: f64,
    pub student_loan: bool,
    pub acc_levy_applicable: bool,
}

// Domain-specific output struct for payroll results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollOutput {
    pub income_tax: f64,
    pub acc_levy: f64,
    pub kiwi_saver: f64,
    pub student_loan_deduction: f64,
    pub net_pay: f64,
    pub paye_rate: f64,
}

// Conversion from domain struct to evaluation context
impl From<&PayrollInput> for HashMap<String, Value> {
    fn from(input: &PayrollInput) -> Self {
        let mut context = HashMap::new();
        context.insert("employee_type".to_string(), Value::String(input.employee_type.clone()));
        context.insert("annual_salary".to_string(), Value::Number(input.annual_salary.into()));
        context.insert("pay_frequency".to_string(), Value::String(input.pay_frequency.clone()));
        context.insert("tax_code".to_string(), Value::String(input.tax_code.clone()));
        context.insert("kiwi_saver_rate".to_string(), Value::Number(input.kiwi_saver_rate.into()));
        context.insert("student_loan".to_string(), Value::Boolean(input.student_loan));
        context.insert("acc_levy_applicable".to_string(), Value::Boolean(input.acc_levy_applicable));
        context
    }
}
```

### Code Examples for Payroll-Specific Structs

```rust
use dsntk_recognizer::from_markdown;
use dsntk_model_evaluator::ModelEvaluator;

pub struct PayrollDecisionEngine {
    income_tax_table: DecisionTable,
    acc_levy_table: DecisionTable,
    student_loan_table: DecisionTable,
    evaluator: ModelEvaluator,
}

impl PayrollDecisionEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let income_tax_table = from_markdown(include_str!("../tables/nz-income-tax.md"), false)?;
        let acc_levy_table = from_markdown(include_str!("../tables/acc-levy.md"), false)?;
        let student_loan_table = from_markdown(include_str!("../tables/student-loan.md"), false)?;
        
        Ok(Self {
            income_tax_table,
            acc_levy_table,
            student_loan_table,
            evaluator: ModelEvaluator::new(),
        })
    }
    
    pub fn calculate_payroll(&self, input: &PayrollInput) -> Result<PayrollOutput, Box<dyn std::error::Error>> {
        let context: HashMap<String, Value> = input.into();
        
        // Calculate income tax
        let tax_result = self.evaluator.evaluate_decision_table(&self.income_tax_table, &context)?;
        let income_tax = extract_number(&tax_result, "income_tax")?;
        
        // Calculate ACC levy
        let acc_result = self.evaluator.evaluate_decision_table(&self.acc_levy_table, &context)?;
        let acc_levy = extract_number(&acc_result, "acc_levy")?;
        
        // Calculate student loan
        let student_loan_deduction = if input.student_loan {
            let loan_result = self.evaluator.evaluate_decision_table(&self.student_loan_table, &context)?;
            extract_number(&loan_result, "student_loan_deduction")?
        } else {
            0.0
        };
        
        // Calculate KiwiSaver
        let kiwi_saver = input.annual_salary * input.kiwi_saver_rate / 52.0; // Weekly calculation
        
        // Calculate net pay
        let gross_pay = input.annual_salary / 52.0; // Weekly
        let net_pay = gross_pay - income_tax - acc_levy - student_loan_deduction - kiwi_saver;
        
        Ok(PayrollOutput {
            income_tax,
            acc_levy,
            kiwi_saver,
            student_loan_deduction,
            net_pay,
            paye_rate: income_tax / gross_pay,
        })
    }
}

fn extract_number(result: &HashMap<String, Value>, key: &str) -> Result<f64, Box<dyn std::error::Error>> {
    match result.get(key) {
        Some(Value::Number(n)) => Ok(n.to_f64()),
        _ => Ok(0.0),
    }
}
```

## File Organization Strategy

### Strong Recommendation: ONE TABLE PER FILE

**This is not just a best practice - it's a technical requirement due to parser limitations.**

#### Why One Table Per File is Necessary

1. **Parser Limitation**: The dsntk recognizer parser is designed to parse only one decision table per file. If multiple tables are present, only the first table will be recognized and parsed.

2. **Maintainability**: Each decision table represents a distinct business rule. Separating them makes:
   - Version control easier (individual table changes)
   - Testing more focused
   - Deployment more granular
   - Business rule management clearer

3. **Composition**: Multiple decision tables can be composed together in code, allowing complex business logic while maintaining simple, focused individual tables.

#### Proposed File Structure for Payroll/Legal Decision Tables

```
payroll-tables/
├── nz-income-tax.md
├── acc-levy.md
├── student-loan.md
├── kiwi-saver.md
├── esct-rates.md
└── holiday-pay.md

legal-tables/
├── employment-agreement.md
├── minimum-wage.md
├── leave-entitlements.md
├── termination-rules.md
└── health-safety.md
```

#### Example File Organization with Specific Paths

```rust
// Recommended project structure
src/
├── main.rs
├── payroll/
│   ├── mod.rs
│   ├── engine.rs
│   └── models.rs
└── tables/
    ├── payroll/
    │   ├── nz-income-tax.md
    │   ├── acc-levy.md
    │   └── student-loan.md
    └── legal/
        ├── minimum-wage.md
        └── leave-entitlements.md
```

**Usage in code:**
```rust
// Load tables using include_str! for compile-time inclusion
const INCOME_TAX_TABLE: &str = include_str!("../tables/payroll/nz-income-tax.md");
const ACC_LEVY_TABLE: &str = include_str!("../tables/payroll/acc-levy.md");

// Or load dynamically at runtime
fn load_table(path: &str) -> Result<DecisionTable, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    from_markdown(&content, false)
}
```

## Payroll-Specific Examples

### NZ Income Tax Calculation Table

**File: `tables/payroll/nz-income-tax.md`**

```markdown
> # NZ Income Tax Calculation
> Weekly PAYE amount

| U | Annual Salary | Tax Code | Income Tax |
|:-:|:-------------:|:--------:|:----------:|
|   |      `i`      |   `i`    |    `o`     |
| 1 |    <=14000    |   "M"    |     0      |
| 2 |    <=14000    |   "S"    |     0      |
| 3 |    <=14000    |   "SH"   |     0      |
| 4 | [14001..48000]|   "M"    |   (salary - 14000) * 0.105 / 52 |
| 5 | [14001..48000]|   "S"    |   (salary - 14000) * 0.105 / 52 |
| 6 | [14001..48000]|   "SH"   |   (salary - 14000) * 0.105 / 52 |
| 7 | [48001..70000]|   "M"    |   (3570 + (salary - 48000) * 0.175) / 52 |
| 8 | [48001..70000]|   "S"    |   (3570 + (salary - 48000) * 0.175) / 52 |
| 9 | [48001..70000]|   "SH"   |   (3570 + (salary - 48000) * 0.175) / 52 |
|10 |   >70000      |   "M"    |   (7420 + (salary - 70000) * 0.33) / 52 |
|11 |   >70000      |   "S"    |   (7420 + (salary - 70000) * 0.33) / 52 |
|12 |   >70000      |   "SH"   |   (7420 + (salary - 70000) * 0.33) / 52 |
```

### ACC Levy Calculation Table

**File: `tables/payroll/acc-levy.md`**

```markdown
> # ACC Levy Calculation
> Weekly ACC levy amount

| U | Annual Salary | ACC Levy Applicable | ACC Levy |
|:-:|:-------------:|:-------------------:|:--------:|
|   |      `i`      |         `i`         |   `o`    |
|   |               |                     |    0     |
| 1 |       -       |        true         | salary * 0.0139 / 52 |
```

### Processing Code for Multiple Payroll Decision Tables

```rust
use dsntk_recognizer::from_markdown;
use dsntk_model_evaluator::ModelEvaluator;
use std::collections::HashMap;

pub struct PayrollProcessor {
    tables: HashMap<String, DecisionTable>,
    evaluator: ModelEvaluator,
}

impl PayrollProcessor {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut tables = HashMap::new();
        
        // Load all payroll decision tables
        tables.insert("income_tax".to_string(), from_markdown(include_str!("../tables/payroll/nz-income-tax.md"), false)?);
        tables.insert("acc_levy".to_string(), from_markdown(include_str!("../tables/payroll/acc-levy.md"), false)?);
        tables.insert("student_loan".to_string(), from_markdown(include_str!("../tables/payroll/student-loan.md"), false)?);
        
        Ok(Self {
            tables,
            evaluator: ModelEvaluator::new(),
        })
    }
    
    pub fn process_payroll(&self, employee_data: &HashMap<String, Value>) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
        let mut results = HashMap::new();
        
        // Process each decision table sequentially
        for (table_name, table) in &self.tables {
            let table_result = self.evaluator.evaluate_decision_table(table, employee_data)?;
            results.extend(table_result);
        }
        
        // Calculate final net pay
        if let (Some(Value::Number(gross)), Some(Value::Number(tax)), Some(Value::Number(acc)), Some(Value::Number(student_loan))) = (
            employee_data.get("gross_pay"),
            results.get("income_tax"),
            results.get("acc_levy"),
            results.get("student_loan_deduction"),
        ) {
            let net_pay = gross.to_f64() - tax.to_f64() - acc.to_f64() - student_loan.to_f64();
            results.insert("net_pay".to_string(), Value::Number(net_pay.into()));
        }
        
        Ok(results)
    }
}
```

## Key Recommendations Summary

### 1. Use Existing Structs from dsntk-model
- Leverage [`DecisionTable`](model/src/lib.rs), [`InputClause`](model/src/lib.rs), and [`OutputClause`](model/src/lib.rs) from the dsntk-model crate
- These provide the foundation for all decision table operations
- Avoid reinventing core data structures

### 2. Separate Files for Each Decision Table
- **Mandatory**: One table per file due to parser limitations
- Use descriptive file names that reflect the business rule
- Organize files in logical directories (payroll/, legal/, etc.)
- Consider using `include_str!` for compile-time table inclusion

### 3. Domain-Specific Structs for Type Safety
- Create wrapper structs like `PayrollInput` and `PayrollOutput`
- Implement `From` traits for conversion to evaluation context
- Use Rust's type system to catch errors at compile time
- Add serialization/deserialization with serde for API compatibility

### 4. Composition of Multiple Tables
- Build complex business logic by composing simple decision tables
- Create engine classes that manage multiple related tables
- Implement sequential evaluation with result aggregation
- Maintain separation of concerns between different business rules

### 5. Error Handling and Validation
- Always validate parsed decision tables
- Implement proper error handling for table evaluation
- Use Rust's Result type for robust error propagation
- Add comprehensive logging for debugging complex table interactions

By following these recommendations, you can build robust, maintainable payroll and legal decision systems using dsntk-rs with clear separation of concerns and strong type safety.