//! # Tests for front matter parsing

use crate::front_matter::{extract_body, parse_front_matter, DmnNodeType};
use std::str::FromStr;

#[test]
fn _0001() {
  // Parses primitive data type
  let content = r#"---
dmn:
  id: I1
  type: input-data
  name: Applicant Age
  data-type:
    ref: number
---
"#;
  let fm = parse_front_matter(content).unwrap();
  assert_eq!(fm.dmn.id, "I1");
  assert_eq!(fm.dmn.node_type, "input-data");
  assert_eq!(fm.dmn.name, "Applicant Age");
  let dt = fm.dmn.data_type.unwrap();
  assert_eq!(dt.type_ref, "number");
  assert!(dt.schema.is_none());
}

#[test]
fn _0002() {
  // Parses file schema reference
  let content = r#"---
dmn:
  id: I2
  type: input-data
  name: Employment Status
  data-type:
    ref: EmploymentStatus
    schema: ../types/applicant.ts
---
"#;
  let fm = parse_front_matter(content).unwrap();
  let dt = fm.dmn.data_type.unwrap();
  assert_eq!(dt.type_ref, "EmploymentStatus");
  assert_eq!(dt.schema.as_deref(), Some("../types/applicant.ts"));
}

#[test]
fn _0003() {
  // Parses folder schema reference
  let content = r#"---
dmn:
  id: I3
  type: input-data
  name: Applicant
  data-type:
    ref: Applicant
    schema: ../types/
---
"#;
  let fm = parse_front_matter(content).unwrap();
  let dt = fm.dmn.data_type.unwrap();
  assert_eq!(dt.type_ref, "Applicant");
  assert_eq!(dt.schema.as_deref(), Some("../types/"));
}

#[test]
fn _0004() {
  // Missing opening delimiter errors
  let content = "no front matter here";
  let result = parse_front_matter(content);
  assert!(result.is_err());
}

#[test]
fn _0005() {
  // Missing closing delimiter errors
  let content = "---\ndmn:\n  id: X\n";
  let result = parse_front_matter(content);
  assert!(result.is_err());
}

#[test]
fn _0006() {
  // BKM signature parsing
  let content = r#"---
dmn:
  id: BKM1
  type: bkm
  name: Risk Score
  signature:
    parameters:
      - name: age
        type: number
      - name: income
        type: number
    return-type: string
---
"#;
  let fm = parse_front_matter(content).unwrap();
  let sig = fm.dmn.signature.unwrap();
  assert_eq!(sig.parameters.len(), 2);
  assert_eq!(sig.parameters[0].name, "age");
  assert_eq!(sig.parameters[0].param_type, "number");
  assert_eq!(sig.return_type.as_deref(), Some("string"));
}

#[test]
fn _0007() {
  // Knowledge source parsing
  let content = r#"---
dmn:
  id: KS1
  type: knowledge-source
  name: Basel III Lending Policy
  uri: https://internal.wiki/lending-policy-v4
  owner: Credit Risk Team
---
"#;
  let fm = parse_front_matter(content).unwrap();
  assert_eq!(fm.dmn.node_type, "knowledge-source");
  assert_eq!(fm.dmn.uri.as_deref(), Some("https://internal.wiki/lending-policy-v4"));
  assert_eq!(fm.dmn.owner.as_deref(), Some("Credit Risk Team"));
}

#[test]
fn _0008() {
  // Decision with link keys
  let content = r#"---
dmn:
  id: D1
  type: decision
  name: Loan Approval
  requires:
    - ../inputs/credit_score.md
  governed-by:
    - ../knowledge/lending_policy.md
  supported-by:
    - ../knowledge/risk_score_fn.md
---
"#;
  let fm = parse_front_matter(content).unwrap();
  assert_eq!(fm.dmn.requires.as_ref().unwrap().len(), 1);
  assert_eq!(fm.dmn.governed_by.as_ref().unwrap().len(), 1);
  assert_eq!(fm.dmn.supported_by.as_ref().unwrap().len(), 1);
}

#[test]
fn _0009() {
  // Extract body content
  let content = "---\ndmn:\n  id: I1\n  type: input-data\n  name: X\n---\nBody content here";
  let body = extract_body(content).unwrap();
  assert_eq!(body, "Body content here");
}

#[test]
fn _0010() {
  // DmnNodeType parsing
  assert_eq!(DmnNodeType::from_str("input-data").unwrap(), DmnNodeType::InputData);
  assert_eq!(DmnNodeType::from_str("decision").unwrap(), DmnNodeType::Decision);
  assert_eq!(DmnNodeType::from_str("bkm").unwrap(), DmnNodeType::Bkm);
  assert_eq!(DmnNodeType::from_str("knowledge-source").unwrap(), DmnNodeType::KnowledgeSource);
  assert!(DmnNodeType::from_str("unknown").is_err());
}

#[test]
fn _0011() {
  // Parses feel-expression and output-name fields
  let content = r#"---
dmn:
  id: BKM2
  type: bkm
  name: LTV Calculation
  feel-expression: Requested Amount / Valuation Amount
  output-name: LTV
  requires:
    - loan_request
    - property_data
---
"#;
  let fm = parse_front_matter(content).unwrap();
  assert_eq!(fm.dmn.feel_expression.as_deref(), Some("Requested Amount / Valuation Amount"));
  assert_eq!(fm.dmn.output_name.as_deref(), Some("LTV"));
  assert_eq!(fm.dmn.requires.as_ref().unwrap().len(), 2);
}

#[test]
fn _0012() {
  // BKM without feel-expression still parses (fields are optional)
  let content = r#"---
dmn:
  id: BKM3
  type: bkm
  name: Risk Table
  signature:
    parameters:
      - name: score
        type: number
---
"#;
  let fm = parse_front_matter(content).unwrap();
  assert!(fm.dmn.feel_expression.is_none());
  assert!(fm.dmn.output_name.is_none());
  assert!(fm.dmn.signature.is_some());
}
