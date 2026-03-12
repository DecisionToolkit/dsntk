//! # Loan pricing end-to-end integration tests
//!
//! Loads the L3 Affordability project from actual markdown and TypeScript
//! files on disk, assembles a type registry + DRD node graph, and validates
//! the full pipeline from filesystem artifacts to an intelligent decision engine.
//!
//! ## Fixture layout
//!
//! ```text
//! fixtures/loan_pricing/
//! ├── types/
//! │   ├── classifiers.ts          6 enum types (ProductType, BorrowerType, ...)
//! │   ├── applicant.ts            2 interface types (LoanApplication, ApplicantProfile)
//! │   └── affordability.ts        4 interface types (DtiCalculationOutput, ...)
//! ├── decisions/
//! │   ├── applicant_input.md              input-data  → LoanApplication
//! │   ├── applicant_profile_input.md      input-data  → ApplicantProfile
//! │   ├── dti_calculation.md              bkm         → DtiCalculationOutput (Table A)
//! │   ├── affordability_classification.md bkm         → AffordabilityClassificationOutput (Table B)
//! │   ├── income_verification_modifier.md bkm         → IncomeVerificationOutput (Table C)
//! │   └── affordability_result.md         decision    → AffordabilityResult
//! └── regulatory/
//!     └── credit_policy_framework.md      knowledge-source
//! ```

use crate::front_matter::{extract_body, parse_front_matter, DmnNode};
use crate::registry::TypeRegistry;
use crate::resolver::{resolve_data_type, resolve_field_chain, validate_allowed_value, validate_bkm_signature, validate_link_targets};
use crate::scanner;
use dsntk_feel::FeelType;
use std::path::{Path, PathBuf};

// ─── Fixture helpers ────────────────────────────────────────────────────────

/// Returns the absolute path to the loan pricing fixtures directory.
fn fixtures_dir() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).join("src/tests/fixtures/loan_pricing")
}

/// Returns the absolute path to the types/ directory within fixtures.
fn types_dir() -> PathBuf {
  fixtures_dir().join("types")
}

/// Returns the absolute path to the decisions/ directory within fixtures.
fn decisions_dir() -> PathBuf {
  fixtures_dir().join("decisions")
}

/// Loads a markdown file from the fixtures and parses its front matter.
fn load_node(dir: &str, filename: &str) -> (DmnNode, String) {
  let path = fixtures_dir().join(dir).join(filename);
  let content = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
  let fm = parse_front_matter(&content).unwrap_or_else(|e| panic!("failed to parse front matter in {}: {e}", path.display()));
  let body = extract_body(&content).unwrap_or("").to_string();
  (fm.dmn, body)
}

/// Loads all markdown decision/input/BKM nodes from decisions/ and regulatory/.
fn load_all_nodes() -> Vec<DmnNode> {
  let decision_files = [
    ("decisions", "applicant_input.md"),
    ("decisions", "applicant_profile_input.md"),
    ("decisions", "dti_calculation.md"),
    ("decisions", "affordability_classification.md"),
    ("decisions", "income_verification_modifier.md"),
    ("decisions", "affordability_result.md"),
    ("regulatory", "credit_policy_framework.md"),
  ];
  decision_files.iter().map(|(dir, file)| load_node(dir, file).0).collect()
}

// ═══════════════════════════════════════════════════════════════════════════
// Phase 1: TypeScript files load into a valid TypeRegistry
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn _0001() {
  // scan_folder loads all 13 exported types from the three .ts files:
  //   classifiers.ts:    7 (ProductType, BorrowerType, IncomeType, VerificationStatus,
  //                         AffordabilityBand, AffordabilityReasonCode, IncomeModifierCode)
  //   applicant.ts:      2 (LoanApplication, ApplicantProfile)
  //   affordability.ts:  4 (DtiCalculationOutput, AffordabilityClassificationOutput,
  //                         IncomeVerificationOutput, AffordabilityResult)
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  assert_eq!(registry.len(), 13);
}

#[test]
fn _0002() {
  // classifiers.ts: ProductType enum has exactly 4 allowed values
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  let pt = registry.get("ProductType").unwrap();
  assert!(pt.allowed_values.is_some());
  let allowed = pt.allowed_values.as_ref().unwrap();
  assert_eq!(allowed.len(), 4);
  assert!(validate_allowed_value("mortgage", pt).is_ok());
  assert!(validate_allowed_value("buy-to-let", pt).is_ok());
  assert!(validate_allowed_value("overdraft", pt).is_err());
}

#[test]
fn _0003() {
  // classifiers.ts: BorrowerType enum constrains to 3 values
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  let bt = registry.get("BorrowerType").unwrap();
  assert!(validate_allowed_value("individual", bt).is_ok());
  assert!(validate_allowed_value("sme", bt).is_ok());
  assert!(validate_allowed_value("corporate", bt).is_ok());
  assert!(validate_allowed_value("trust", bt).is_err());
}

#[test]
fn _0004() {
  // classifiers.ts: IncomeType enum constrains to 4 values
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  let it = registry.get("IncomeType").unwrap();
  for v in ["employed", "self-employed", "contract", "benefits"] {
    assert!(validate_allowed_value(v, it).is_ok(), "rejected valid income type: {v}");
  }
  assert!(validate_allowed_value("retired", it).is_err());
}

#[test]
fn _0005() {
  // classifiers.ts: VerificationStatus enum constrains to 9 values
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  let vs = registry.get("VerificationStatus").unwrap();
  let allowed = vs.allowed_values.as_ref().unwrap();
  assert_eq!(allowed.len(), 9);
  assert!(validate_allowed_value("fraud flag", vs).is_ok());
  assert!(validate_allowed_value("payslip verified", vs).is_ok());
  assert!(validate_allowed_value("self-certified", vs).is_err());
}

#[test]
fn _0006() {
  // classifiers.ts: AffordabilityBand enum constrains to Pass/Marginal/Refer/Fail
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  let ab = registry.get("AffordabilityBand").unwrap();
  for v in ["Pass", "Marginal", "Refer", "Fail"] {
    assert!(validate_allowed_value(v, ab).is_ok());
  }
  assert!(validate_allowed_value("pass", ab).is_err()); // case-sensitive
}

#[test]
fn _0007() {
  // classifiers.ts: AffordabilityReasonCode enum has 9 codes (AFC-01 through AFC-09)
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  let arc = registry.get("AffordabilityReasonCode").unwrap();
  let allowed = arc.allowed_values.as_ref().unwrap();
  assert_eq!(allowed.len(), 9);
  assert!(validate_allowed_value("AFC-01", arc).is_ok());
  assert!(validate_allowed_value("AFC-09", arc).is_ok());
  assert!(validate_allowed_value("AFC-10", arc).is_err());
}

#[test]
fn _0008() {
  // classifiers.ts: IncomeModifierCode enum has 12 codes
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  let imc = registry.get("IncomeModifierCode").unwrap();
  let allowed = imc.allowed_values.as_ref().unwrap();
  assert_eq!(allowed.len(), 12);
  assert!(validate_allowed_value("INC-99", imc).is_ok());
  assert!(validate_allowed_value("INC-00", imc).is_ok());
  assert!(validate_allowed_value("INC-11", imc).is_err());
}

#[test]
fn _0009() {
  // applicant.ts: LoanApplication has 8 fields, 2 optional
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  let la = registry.get("LoanApplication").unwrap();
  assert!(matches!(la.feel_type, FeelType::Context(_)));
  // Required fields
  assert_eq!(resolve_field_chain(&la.feel_type, &["grossAnnualIncome"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&la.feel_type, &["totalMonthlyDebt"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&la.feel_type, &["requestedMonthlyRepayment"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&la.feel_type, &["loanAmount"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&la.feel_type, &["termMonths"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&la.feel_type, &["purpose"]), Some(FeelType::String));
  // Optional fields
  assert_eq!(resolve_field_chain(&la.feel_type, &["propertyValue"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&la.feel_type, &["assets"]), Some(FeelType::Number));
  assert_eq!(la.optional_fields.get("propertyValue"), Some(&true));
  assert_eq!(la.optional_fields.get("assets"), Some(&true));
  // TS parser records required fields as false (not absent)
  assert_eq!(la.optional_fields.get("grossAnnualIncome"), Some(&false));
}

#[test]
fn _0010() {
  // applicant.ts: ApplicantProfile has 5 fields, 1 optional
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  let ap = registry.get("ApplicantProfile").unwrap();
  assert_eq!(resolve_field_chain(&ap.feel_type, &["bureauScore"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&ap.feel_type, &["incomeType"]), Some(FeelType::String));
  assert_eq!(resolve_field_chain(&ap.feel_type, &["verificationStatus"]), Some(FeelType::String));
  assert_eq!(resolve_field_chain(&ap.feel_type, &["existingCustomer"]), Some(FeelType::Boolean));
  assert_eq!(resolve_field_chain(&ap.feel_type, &["yearsAtAddress"]), Some(FeelType::Number));
  assert_eq!(ap.optional_fields.get("yearsAtAddress"), Some(&true));
}

#[test]
fn _0011() {
  // affordability.ts: DtiCalculationOutput has 4 fields (all required)
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  let dti = registry.get("DtiCalculationOutput").unwrap();
  assert_eq!(resolve_field_chain(&dti.feel_type, &["dtiRatio"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&dti.feel_type, &["stressedDtiRatio"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&dti.feel_type, &["stressedRepayment"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&dti.feel_type, &["calculationBasis"]), Some(FeelType::String));
  // All fields are required — TS parser records them as false
  assert!(dti.optional_fields.values().all(|v| !v));
}

#[test]
fn _0012() {
  // affordability.ts: AffordabilityResult has nested auditTrail context
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  let ar = registry.get("AffordabilityResult").unwrap();
  // Top-level fields
  assert_eq!(resolve_field_chain(&ar.feel_type, &["finalBand"]), Some(FeelType::String));
  assert_eq!(resolve_field_chain(&ar.feel_type, &["reasonCode"]), Some(FeelType::String));
  assert_eq!(resolve_field_chain(&ar.feel_type, &["modifierCode"]), Some(FeelType::String));
  assert_eq!(resolve_field_chain(&ar.feel_type, &["dtiRatio"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&ar.feel_type, &["stressedDtiRatio"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&ar.feel_type, &["calculationBasis"]), Some(FeelType::String));
  // Nested audit trail — 3 levels deep through the type system
  assert_eq!(resolve_field_chain(&ar.feel_type, &["auditTrail", "tableAOutput"]), Some(FeelType::String));
  assert_eq!(resolve_field_chain(&ar.feel_type, &["auditTrail", "tableBBand"]), Some(FeelType::String));
  assert_eq!(resolve_field_chain(&ar.feel_type, &["auditTrail", "tableBReason"]), Some(FeelType::String));
  assert_eq!(resolve_field_chain(&ar.feel_type, &["auditTrail", "tableCModifier"]), Some(FeelType::String));
  assert_eq!(resolve_field_chain(&ar.feel_type, &["auditTrail", "tableCOriginalBand"]), Some(FeelType::String));
}

#[test]
fn _0013() {
  // affordability.ts: nonexistent audit trail field returns None
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  let ar = &registry.get("AffordabilityResult").unwrap().feel_type;
  assert!(resolve_field_chain(ar, &["auditTrail", "gender"]).is_none());
  assert!(resolve_field_chain(ar, &["auditTrail", "ethnicity"]).is_none());
}

// ═══════════════════════════════════════════════════════════════════════════
// Phase 2: Markdown front matter loads into valid DmnNodes
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn _0014() {
  // All 7 markdown files parse successfully into DmnNodes
  let nodes = load_all_nodes();
  assert_eq!(nodes.len(), 7);
}

#[test]
fn _0015() {
  // Node types are correctly parsed from front matter
  let nodes = load_all_nodes();
  let by_id = |id: &str| nodes.iter().find(|n| n.id == id).unwrap();

  assert_eq!(by_id("loan_application_input").node_type, "input-data");
  assert_eq!(by_id("applicant_profile_input").node_type, "input-data");
  assert_eq!(by_id("dti_calculation").node_type, "bkm");
  assert_eq!(by_id("affordability_classification").node_type, "bkm");
  assert_eq!(by_id("income_verification_modifier").node_type, "bkm");
  assert_eq!(by_id("affordability_result").node_type, "decision");
  assert_eq!(by_id("credit_policy_framework").node_type, "knowledge-source");
}

#[test]
fn _0016() {
  // Input data nodes have correct data-type references
  let (app, _) = load_node("decisions", "applicant_input.md");
  assert_eq!(app.id, "loan_application_input");
  assert_eq!(app.name, "Loan Application");
  let dt = app.data_type.as_ref().unwrap();
  assert_eq!(dt.type_ref, "LoanApplication");
  assert_eq!(dt.schema.as_deref(), Some("../types/applicant.ts"));
}

#[test]
fn _0017() {
  // DTI Calculation BKM has correct signature (6 params, return type)
  let (dti, _) = load_node("decisions", "dti_calculation.md");
  assert_eq!(dti.node_type, "bkm");
  let sig = dti.signature.as_ref().unwrap();
  assert_eq!(sig.parameters.len(), 6);
  assert_eq!(sig.parameters[0].name, "grossAnnualIncome");
  assert_eq!(sig.parameters[0].param_type, "number");
  assert_eq!(sig.parameters[5].name, "productType");
  assert_eq!(sig.parameters[5].param_type, "ProductType");
  assert_eq!(sig.return_type.as_deref(), Some("DtiCalculationOutput"));
}

#[test]
fn _0018() {
  // Affordability Classification BKM has correct signature (4 params)
  let (cls, _) = load_node("decisions", "affordability_classification.md");
  let sig = cls.signature.as_ref().unwrap();
  assert_eq!(sig.parameters.len(), 4);
  assert_eq!(sig.parameters[0].param_type, "number"); // dtiRatio
  assert_eq!(sig.parameters[2].param_type, "ProductType"); // productType
  assert_eq!(sig.parameters[3].param_type, "BorrowerType"); // borrowerType
  assert_eq!(sig.return_type.as_deref(), Some("AffordabilityClassificationOutput"));
}

#[test]
fn _0019() {
  // Income Verification Modifier BKM has correct signature (3 params with enum types)
  let (ivm, _) = load_node("decisions", "income_verification_modifier.md");
  let sig = ivm.signature.as_ref().unwrap();
  assert_eq!(sig.parameters.len(), 3);
  assert_eq!(sig.parameters[0].param_type, "IncomeType");
  assert_eq!(sig.parameters[1].param_type, "VerificationStatus");
  assert_eq!(sig.parameters[2].param_type, "AffordabilityBand");
  assert_eq!(sig.return_type.as_deref(), Some("IncomeVerificationOutput"));
}

#[test]
fn _0020() {
  // Affordability Result decision declares requires and governed-by links
  let (result, _) = load_node("decisions", "affordability_result.md");
  assert_eq!(result.node_type, "decision");
  let requires = result.requires.as_ref().unwrap();
  assert_eq!(requires.len(), 3);
  assert!(requires.contains(&"dti_calculation".to_string()));
  assert!(requires.contains(&"affordability_classification".to_string()));
  assert!(requires.contains(&"income_verification_modifier".to_string()));
  let governed = result.governed_by.as_ref().unwrap();
  assert_eq!(governed, &["credit_policy_framework"]);
}

#[test]
fn _0021() {
  // Credit Policy Framework knowledge source has URI and owner
  let (cpf, _) = load_node("regulatory", "credit_policy_framework.md");
  assert_eq!(cpf.node_type, "knowledge-source");
  assert_eq!(cpf.uri.as_deref(), Some("urn:bank:credit-policy:2024"));
  assert_eq!(cpf.owner.as_deref(), Some("Chief Risk Officer"));
}

#[test]
fn _0022() {
  // Markdown body content is extracted correctly (not empty)
  let (_, body) = load_node("decisions", "dti_calculation.md");
  assert!(body.contains("Table A"));
  assert!(body.contains("Stressed repayment formula"));
  let (_, body2) = load_node("regulatory", "credit_policy_framework.md");
  assert!(body2.contains("FCA Consumer Duty"));
  assert!(body2.contains("Basel III"));
}

// ═══════════════════════════════════════════════════════════════════════════
// Phase 3: BKM signature validation on loaded nodes
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn _0023() {
  // All three BKMs pass signature validation (have return types)
  let nodes = load_all_nodes();
  for node in nodes.iter().filter(|n| n.node_type == "bkm") {
    let result = validate_bkm_signature(node);
    assert!(result.is_ok(), "BKM '{}' failed signature validation: {}", node.name, result.unwrap_err());
  }
}

#[test]
fn _0024() {
  // Non-BKM nodes pass signature validation trivially (not checked)
  let nodes = load_all_nodes();
  for node in nodes.iter().filter(|n| n.node_type != "bkm") {
    assert!(validate_bkm_signature(node).is_ok());
  }
}

// ═══════════════════════════════════════════════════════════════════════════
// Phase 4: DRD link validation on loaded nodes
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn _0025() {
  // Affordability Result governed-by Credit Policy Framework passes link validation
  let nodes = load_all_nodes();
  let result_node = nodes.iter().find(|n| n.id == "affordability_result").unwrap();
  assert!(validate_link_targets(result_node, &nodes).is_ok());
}

#[test]
fn _0026() {
  // All nodes pass link validation against the full node set
  let nodes = load_all_nodes();
  for node in &nodes {
    let result = validate_link_targets(node, &nodes);
    assert!(result.is_ok(), "link validation failed for '{}': {}", node.name, result.unwrap_err());
  }
}

// ═══════════════════════════════════════════════════════════════════════════
// Phase 5: Type resolution — connecting markdown nodes to TypeScript schemas
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn _0027() {
  // resolve_data_type resolves LoanApplication via schema path in front matter
  let (app, _) = load_node("decisions", "applicant_input.md");
  let dt = app.data_type.as_ref().unwrap();
  let registry = TypeRegistry::new(); // empty — schema path should suffice
  let entry = resolve_data_type(dt, &decisions_dir(), &registry).unwrap();
  assert_eq!(entry.name, "LoanApplication");
  assert_eq!(resolve_field_chain(&entry.feel_type, &["grossAnnualIncome"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&entry.feel_type, &["loanAmount"]), Some(FeelType::Number));
}

#[test]
fn _0028() {
  // resolve_data_type resolves ApplicantProfile via schema path
  let (profile, _) = load_node("decisions", "applicant_profile_input.md");
  let dt = profile.data_type.as_ref().unwrap();
  let registry = TypeRegistry::new();
  let entry = resolve_data_type(dt, &decisions_dir(), &registry).unwrap();
  assert_eq!(entry.name, "ApplicantProfile");
  assert_eq!(resolve_field_chain(&entry.feel_type, &["bureauScore"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&entry.feel_type, &["existingCustomer"]), Some(FeelType::Boolean));
}

#[test]
fn _0029() {
  // resolve_data_type resolves DtiCalculationOutput from affordability.ts schema
  let (dti, _) = load_node("decisions", "dti_calculation.md");
  let dt = dti.data_type.as_ref().unwrap();
  let registry = TypeRegistry::new();
  let entry = resolve_data_type(dt, &decisions_dir(), &registry).unwrap();
  assert_eq!(entry.name, "DtiCalculationOutput");
  assert_eq!(resolve_field_chain(&entry.feel_type, &["dtiRatio"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&entry.feel_type, &["calculationBasis"]), Some(FeelType::String));
}

#[test]
fn _0030() {
  // resolve_data_type resolves AffordabilityResult with nested auditTrail
  let (result, _) = load_node("decisions", "affordability_result.md");
  let dt = result.data_type.as_ref().unwrap();
  let registry = TypeRegistry::new();
  let entry = resolve_data_type(dt, &decisions_dir(), &registry).unwrap();
  assert_eq!(entry.name, "AffordabilityResult");
  // Verify the nested audit trail resolves through the loaded schema
  assert_eq!(resolve_field_chain(&entry.feel_type, &["auditTrail", "tableBBand"]), Some(FeelType::String));
  assert_eq!(resolve_field_chain(&entry.feel_type, &["auditTrail", "tableCModifier"]), Some(FeelType::String));
}

#[test]
fn _0031() {
  // Every node with a data-type + schema resolves successfully
  let nodes = load_all_nodes();
  for node in nodes.iter().filter(|n| n.data_type.is_some()) {
    let dt = node.data_type.as_ref().unwrap();
    if dt.schema.is_some() {
      // Determine the correct base_dir based on where the node was loaded from
      let base_dir = if node.id == "credit_policy_framework" {
        fixtures_dir().join("regulatory")
      } else {
        decisions_dir()
      };
      let registry = TypeRegistry::new();
      let result = resolve_data_type(dt, &base_dir, &registry);
      assert!(result.is_ok(), "node '{}' type '{}' failed to resolve: {}", node.name, dt.type_ref, result.unwrap_err());
    }
  }
}

// ═══════════════════════════════════════════════════════════════════════════
// Phase 6: BKM parameter types resolve through scanned registry
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn _0032() {
  // All BKM parameter types resolve (primitives via primitive resolver, domain types via scanned registry)
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  let nodes = load_all_nodes();
  for node in nodes.iter().filter(|n| n.node_type == "bkm") {
    let sig = node.signature.as_ref().unwrap();
    for param in &sig.parameters {
      let data_ref = crate::front_matter::DataTypeRef {
        type_ref: param.param_type.clone(),
        schema: None,
      };
      let result = resolve_data_type(&data_ref, &decisions_dir(), &registry);
      assert!(
        result.is_ok(),
        "BKM '{}' param '{}' type '{}' failed to resolve: {}",
        node.name,
        param.name,
        param.param_type,
        result.unwrap_err()
      );
    }
  }
}

#[test]
fn _0033() {
  // All BKM return types resolve via schema path
  let nodes = load_all_nodes();
  for node in nodes.iter().filter(|n| n.node_type == "bkm") {
    let sig = node.signature.as_ref().unwrap();
    if let Some(schema) = &sig.schema {
      let ret = sig.return_type.as_ref().unwrap();
      let schema_registry = scanner::resolve_schema(schema, &decisions_dir()).unwrap();
      let result = schema_registry.resolve(ret);
      assert!(
        result.is_ok(),
        "BKM '{}' return type '{}' not found in schema '{}': {}",
        node.name,
        ret,
        schema,
        result.unwrap_err()
      );
    }
  }
}

// ═══════════════════════════════════════════════════════════════════════════
// Phase 7: Full pipeline — from filesystem to validated decision engine
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn _0034() {
  // End-to-end: load TS types + MD nodes, resolve every type, validate every
  // link, verify the audit trail is complete, and confirm enum constraints
  // are enforced at every boundary.

  // Step 1: Load type registry from TypeScript files
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  assert_eq!(registry.len(), 13);

  // Step 2: Load all DRD nodes from markdown files
  let nodes = load_all_nodes();
  assert_eq!(nodes.len(), 7);

  // Step 3: Validate all BKM signatures
  for node in nodes.iter().filter(|n| n.node_type == "bkm") {
    assert!(validate_bkm_signature(node).is_ok(), "BKM '{}' failed", node.name);
  }

  // Step 4: Validate all DRD links
  for node in &nodes {
    assert!(validate_link_targets(node, &nodes).is_ok(), "links failed for '{}'", node.name);
  }

  // Step 5: Resolve every node's data-type through its schema
  for node in nodes.iter().filter(|n| n.data_type.is_some()) {
    let dt = node.data_type.as_ref().unwrap();
    if let Some(schema) = &dt.schema {
      let base_dir = if node.id == "credit_policy_framework" {
        fixtures_dir().join("regulatory")
      } else {
        decisions_dir()
      };
      let schema_registry = scanner::resolve_schema(schema, &base_dir).unwrap();
      let entry = schema_registry.resolve(&dt.type_ref);
      assert!(entry.is_ok(), "node '{}' type '{}' not in schema: {}", node.name, dt.type_ref, entry.unwrap_err());
    }
  }

  // Step 6: Resolve BKM parameter types through the unified registry
  for node in nodes.iter().filter(|n| n.node_type == "bkm") {
    let sig = node.signature.as_ref().unwrap();
    for param in &sig.parameters {
      let data_ref = crate::front_matter::DataTypeRef {
        type_ref: param.param_type.clone(),
        schema: None,
      };
      assert!(
        resolve_data_type(&data_ref, &decisions_dir(), &registry).is_ok(),
        "param '{}' type '{}' in BKM '{}' failed",
        param.name,
        param.param_type,
        node.name
      );
    }
  }

  // Step 7: Verify the AffordabilityResult audit trail is complete
  let result_node = nodes.iter().find(|n| n.id == "affordability_result").unwrap();
  let dt = result_node.data_type.as_ref().unwrap();
  let result_entry = resolve_data_type(dt, &decisions_dir(), &TypeRegistry::new()).unwrap();
  for audit_field in ["tableAOutput", "tableBBand", "tableBReason", "tableCModifier", "tableCOriginalBand"] {
    let resolved = resolve_field_chain(&result_entry.feel_type, &["auditTrail", audit_field]);
    assert!(resolved.is_some(), "audit trail missing '{audit_field}'");
    assert_eq!(resolved, Some(FeelType::String));
  }

  // Step 8: Verify enum constraints are enforced at decision boundaries
  // The Income Verification Modifier BKM takes IncomeType as input —
  // only the 4 allowed values should pass.
  let income_type = registry.get("IncomeType").unwrap();
  assert!(validate_allowed_value("self-employed", income_type).is_ok());
  assert!(validate_allowed_value("retired", income_type).is_err());

  // The AffordabilityBand enum constrains the output of Table B —
  // only Pass/Marginal/Refer/Fail are valid.
  let band = registry.get("AffordabilityBand").unwrap();
  assert!(validate_allowed_value("Refer", band).is_ok());
  assert!(validate_allowed_value("Approved", band).is_err());

  // The IncomeModifierCode enum constrains Table C output codes.
  let modifier = registry.get("IncomeModifierCode").unwrap();
  assert!(validate_allowed_value("INC-99", modifier).is_ok()); // fraud flag
  assert!(validate_allowed_value("INC-00", modifier).is_ok()); // catch-all
  assert!(validate_allowed_value("AFC-01", modifier).is_err()); // wrong code family
}

#[test]
fn _0035() {
  // The DRD requires-chain is consistent: affordability_result depends on all 3 BKMs,
  // and affordability_classification depends on dti_calculation.
  let nodes = load_all_nodes();
  let by_id = |id: &str| nodes.iter().find(|n| n.id == id).unwrap();

  // affordability_result → 3 BKMs
  let result = by_id("affordability_result");
  let requires = result.requires.as_ref().unwrap();
  assert!(requires.contains(&"dti_calculation".to_string()));
  assert!(requires.contains(&"affordability_classification".to_string()));
  assert!(requires.contains(&"income_verification_modifier".to_string()));

  // affordability_classification → dti_calculation
  let classification = by_id("affordability_classification");
  let cls_requires = classification.requires.as_ref().unwrap();
  assert!(cls_requires.contains(&"dti_calculation".to_string()));

  // income_verification_modifier → affordability_classification
  let modifier = by_id("income_verification_modifier");
  let mod_requires = modifier.requires.as_ref().unwrap();
  assert!(mod_requires.contains(&"affordability_classification".to_string()));

  // dti_calculation has no requires (it's the leaf BKM)
  let dti = by_id("dti_calculation");
  assert!(dti.requires.is_none());
}

#[test]
fn _0036() {
  // resolve_schema with a single .ts file path works (not just directories)
  let schema_reg = scanner::resolve_schema("../types/classifiers.ts", &decisions_dir()).unwrap();
  assert!(schema_reg.get("ProductType").is_some());
  assert!(schema_reg.get("IncomeModifierCode").is_some());
  // Types from other .ts files should NOT be present
  assert!(schema_reg.get("LoanApplication").is_none());
  assert!(schema_reg.get("DtiCalculationOutput").is_none());
}

#[test]
fn _0037() {
  // resolve_schema with the types/ directory loads all .ts files
  let schema_reg = scanner::resolve_schema("../types/", &decisions_dir()).unwrap();
  assert_eq!(schema_reg.len(), 13);
  // Types from all three files should be present
  assert!(schema_reg.get("ProductType").is_some());
  assert!(schema_reg.get("LoanApplication").is_some());
  assert!(schema_reg.get("AffordabilityResult").is_some());
}

#[test]
fn _0038() {
  // resolve_schema with nonexistent path returns clear error
  let result = scanner::resolve_schema("../types/nonexistent.ts", &decisions_dir());
  assert!(result.is_err());
  assert!(result.unwrap_err().to_string().contains("not found"));
}

// ═══════════════════════════════════════════════════════════════════════════
// Phase 8: DMN XML export of loaded types
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn _0039() {
  // Loaded types can be exported to DMN ItemDefinition XML
  let registry = scanner::scan_folder(&types_dir()).unwrap();

  // Export a simple enum type
  let product_type = registry.get("ProductType").unwrap();
  let xml = crate::exporter::type_entry_to_item_definition_xml(product_type, 0);
  assert!(xml.contains("itemDefinition"));
  assert!(xml.contains("ProductType"));
  assert!(xml.contains("allowedValues"));

  // Export a context type with nested fields
  let loan_app = registry.get("LoanApplication").unwrap();
  let xml2 = crate::exporter::type_entry_to_item_definition_xml(loan_app, 0);
  assert!(xml2.contains("LoanApplication"));
  assert!(xml2.contains("itemComponent"));

  // Export the AffordabilityResult with nested auditTrail
  let result = registry.get("AffordabilityResult").unwrap();
  let xml3 = crate::exporter::type_entry_to_item_definition_xml(result, 0);
  assert!(xml3.contains("AffordabilityResult"));
  assert!(xml3.contains("auditTrail"));
}

#[test]
fn _0040() {
  // Batch export of all loaded types produces valid XML fragments
  let registry = scanner::scan_folder(&types_dir()).unwrap();
  let entries: Vec<&_> = registry.iter().map(|(_, e)| e).collect();
  let xml = crate::exporter::registry_to_item_definitions_xml(&entries, 0);
  // All 12 types should appear in the export
  assert!(xml.contains("ProductType"));
  assert!(xml.contains("LoanApplication"));
  assert!(xml.contains("AffordabilityResult"));
  assert!(xml.contains("IncomeModifierCode"));
  // Should contain multiple itemDefinition elements
  let count = xml.matches("itemDefinition").count();
  assert!(count >= 13, "expected at least 13 itemDefinition occurrences, got {count}");
}
