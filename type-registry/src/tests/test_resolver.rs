//! # Tests for type resolution and validation

use crate::front_matter::{BkmParameter, BkmSignature, DataTypeRef, DmnNode};
use crate::registry::{TypeEntry, TypeRegistry, TypeSource};
use crate::resolver::{resolve_data_type, resolve_field_chain, validate_allowed_value, validate_bkm_signature, validate_link_targets};
use dsntk_feel::{FeelType, Name};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

fn make_context_type() -> FeelType {
  let mut address_fields = BTreeMap::new();
  address_fields.insert(Name::from("street"), FeelType::String);
  address_fields.insert(Name::from("postcode"), FeelType::String);

  let mut fields = BTreeMap::new();
  fields.insert(Name::from("name"), FeelType::String);
  fields.insert(Name::from("age"), FeelType::Number);
  fields.insert(Name::from("address"), FeelType::Context(address_fields));

  FeelType::Context(fields)
}

// --- Enum/union constraint checking ---

#[test]
fn _0001() {
  // Cell value matches enum passes validation
  let entry = TypeEntry {
    name: "Status".to_string(),
    feel_type: FeelType::String,
    source: TypeSource::Primitive,
    allowed_values: Some(vec!["active".to_string(), "inactive".to_string()]),
    optional_fields: HashMap::new(),
  };
  assert!(validate_allowed_value("active", &entry).is_ok());
}

#[test]
fn _0002() {
  // Cell value violates enum fails validation
  let entry = TypeEntry {
    name: "Status".to_string(),
    feel_type: FeelType::String,
    source: TypeSource::Primitive,
    allowed_values: Some(vec!["active".to_string(), "inactive".to_string()]),
    optional_fields: HashMap::new(),
  };
  let result = validate_allowed_value("unknown", &entry);
  assert!(result.is_err());
  let err_msg = result.unwrap_err().to_string();
  assert!(err_msg.contains("not in allowed values"));
}

#[test]
fn _0003() {
  // No allowed values means any value passes
  let entry = TypeEntry {
    name: "Name".to_string(),
    feel_type: FeelType::String,
    source: TypeSource::Primitive,
    allowed_values: None,
    optional_fields: HashMap::new(),
  };
  assert!(validate_allowed_value("anything", &entry).is_ok());
}

// --- Nested context resolution ---

#[test]
fn _0004() {
  // Nested context resolves field chain
  let ctx = make_context_type();
  let result = resolve_field_chain(&ctx, &["address", "postcode"]);
  assert_eq!(result, Some(FeelType::String));
}

#[test]
fn _0005() {
  // Missing nested field returns None
  let ctx = make_context_type();
  let result = resolve_field_chain(&ctx, &["address", "city"]);
  assert!(result.is_none());
}

#[test]
fn _0006() {
  // Top-level field resolves
  let ctx = make_context_type();
  let result = resolve_field_chain(&ctx, &["name"]);
  assert_eq!(result, Some(FeelType::String));
}

#[test]
fn _0007() {
  // Empty chain returns the type itself
  let ctx = make_context_type();
  let result = resolve_field_chain(&ctx, &[]);
  assert_eq!(result, Some(ctx));
}

// --- BKM signature validation ---

#[test]
fn _0008() {
  // BKM missing return type fails
  let node = DmnNode {
    id: "BKM1".to_string(),
    node_type: "bkm".to_string(),
    name: "Risk Score".to_string(),
    data_type: None,
    signature: None,
    uri: None,
    owner: None,
    requires: None,
    governed_by: None,
    supported_by: None,
  };
  let result = validate_bkm_signature(&node);
  assert!(result.is_err());
  assert!(result.unwrap_err().to_string().contains("missing a return-type"));
}

// --- Link target validation ---

#[test]
fn _0009() {
  // governed-by pointing to knowledge-source is valid
  let decision = DmnNode {
    id: "D1".to_string(),
    node_type: "decision".to_string(),
    name: "Loan Approval".to_string(),
    data_type: None,
    signature: None,
    uri: None,
    owner: None,
    requires: None,
    governed_by: Some(vec!["lending_policy".to_string()]),
    supported_by: None,
  };
  let ks = DmnNode {
    id: "lending_policy".to_string(),
    node_type: "knowledge-source".to_string(),
    name: "Basel III".to_string(),
    data_type: None,
    signature: None,
    uri: None,
    owner: None,
    requires: None,
    governed_by: None,
    supported_by: None,
  };
  assert!(validate_link_targets(&decision, &[ks]).is_ok());
}

#[test]
fn _0010() {
  // governed-by pointing to non-knowledge-source is invalid
  let decision = DmnNode {
    id: "D1".to_string(),
    node_type: "decision".to_string(),
    name: "Loan Approval".to_string(),
    data_type: None,
    signature: None,
    uri: None,
    owner: None,
    requires: None,
    governed_by: Some(vec!["bad_target".to_string()]),
    supported_by: None,
  };
  let bkm = DmnNode {
    id: "bad_target".to_string(),
    node_type: "bkm".to_string(),
    name: "Some BKM".to_string(),
    data_type: None,
    signature: None,
    uri: None,
    owner: None,
    requires: None,
    governed_by: None,
    supported_by: None,
  };
  let result = validate_link_targets(&decision, &[bkm]);
  assert!(result.is_err());
}

// --- resolve_data_type with registry fallback ---

#[test]
fn _0011() {
  // Primitive resolves without registry or schema
  let data_type = DataTypeRef {
    type_ref: "number".to_string(),
    schema: None,
  };
  let registry = TypeRegistry::new();
  let entry = resolve_data_type(&data_type, Path::new("."), &registry).unwrap();
  assert_eq!(entry.feel_type, FeelType::Number);
}

#[test]
fn _0012() {
  // Named type resolves from pre-populated registry
  let mut registry = TypeRegistry::new();
  registry
    .insert(TypeEntry {
      name: "Applicant".to_string(),
      feel_type: FeelType::String,
      source: TypeSource::Primitive,
      allowed_values: None,
      optional_fields: HashMap::new(),
    })
    .unwrap();
  let data_type = DataTypeRef {
    type_ref: "Applicant".to_string(),
    schema: None,
  };
  let entry = resolve_data_type(&data_type, Path::new("."), &registry).unwrap();
  assert_eq!(entry.name, "Applicant");
  assert_eq!(entry.feel_type, FeelType::String);
}

#[test]
fn _0013() {
  // Unknown type with no schema errors from registry
  let registry = TypeRegistry::new();
  let data_type = DataTypeRef {
    type_ref: "Unknown".to_string(),
    schema: None,
  };
  let result = resolve_data_type(&data_type, Path::new("."), &registry);
  assert!(result.is_err());
  assert!(result.unwrap_err().to_string().contains("not found"));
}

// --- Insurance domain integration tests ---
//
// Models a simplified auto-insurance underwriting workflow:
//
//   [Applicant Info]  ──requires──▶  [Risk Assessment]  ──requires──▶  [Premium Decision]
//        (input-data)                    (bkm)                           (decision)
//                                                                            │
//                                   [State Regulations]  ◀──governed-by──────┘
//                                    (knowledge-source)
//
// Type registry contains:
//   - ClaimStatus  (enum: "open", "closed", "under review")
//   - CoverageType (enum: "liability", "collision", "comprehensive")
//   - Applicant    (context: name, age, driving_record { violations, years_licensed })
//   - PolicyQuote  (context: premium, coverage, deductible)

/// Builds the insurance-domain type registry used by integration tests.
fn make_insurance_registry() -> TypeRegistry {
  let mut registry = TypeRegistry::new();

  // ClaimStatus — constrained string enum
  registry
    .insert(TypeEntry {
      name: "ClaimStatus".to_string(),
      feel_type: FeelType::String,
      source: TypeSource::Primitive,
      allowed_values: Some(vec!["open".to_string(), "closed".to_string(), "under review".to_string()]),
      optional_fields: HashMap::new(),
    })
    .unwrap();

  // CoverageType — constrained string enum
  registry
    .insert(TypeEntry {
      name: "CoverageType".to_string(),
      feel_type: FeelType::String,
      source: TypeSource::Primitive,
      allowed_values: Some(vec!["liability".to_string(), "collision".to_string(), "comprehensive".to_string()]),
      optional_fields: HashMap::new(),
    })
    .unwrap();

  // Applicant — nested context with driving_record sub-context
  let mut driving_record = BTreeMap::new();
  driving_record.insert(Name::from("violations"), FeelType::Number);
  driving_record.insert(Name::from("years licensed"), FeelType::Number);

  let mut applicant_fields = BTreeMap::new();
  applicant_fields.insert(Name::from("name"), FeelType::String);
  applicant_fields.insert(Name::from("age"), FeelType::Number);
  applicant_fields.insert(Name::from("driving record"), FeelType::Context(driving_record));

  registry
    .insert(TypeEntry {
      name: "Applicant".to_string(),
      feel_type: FeelType::Context(applicant_fields),
      source: TypeSource::Primitive,
      allowed_values: None,
      optional_fields: HashMap::from([("driving record".to_string(), true)]),
    })
    .unwrap();

  // PolicyQuote — output context
  let mut quote_fields = BTreeMap::new();
  quote_fields.insert(Name::from("premium"), FeelType::Number);
  quote_fields.insert(Name::from("coverage"), FeelType::String);
  quote_fields.insert(Name::from("deductible"), FeelType::Number);

  registry
    .insert(TypeEntry {
      name: "PolicyQuote".to_string(),
      feel_type: FeelType::Context(quote_fields),
      source: TypeSource::Primitive,
      allowed_values: None,
      optional_fields: HashMap::new(),
    })
    .unwrap();

  registry
}

#[test]
fn _0014() {
  // Registry resolves all four insurance types
  let registry = make_insurance_registry();
  assert_eq!(registry.len(), 4);
  assert!(registry.get("ClaimStatus").is_some());
  assert!(registry.get("CoverageType").is_some());
  assert!(registry.get("Applicant").is_some());
  assert!(registry.get("PolicyQuote").is_some());
}

#[test]
fn _0015() {
  // resolve_data_type falls back to registry for domain types
  let registry = make_insurance_registry();
  let data_type = DataTypeRef {
    type_ref: "PolicyQuote".to_string(),
    schema: None,
  };
  let entry = resolve_data_type(&data_type, Path::new("."), &registry).unwrap();
  assert_eq!(entry.name, "PolicyQuote");
  assert!(matches!(entry.feel_type, FeelType::Context(_)));
}

#[test]
fn _0016() {
  // Navigate into Applicant → driving record → violations
  let registry = make_insurance_registry();
  let applicant = &registry.get("Applicant").unwrap().feel_type;
  let result = resolve_field_chain(applicant, &["driving record", "violations"]);
  assert_eq!(result, Some(FeelType::Number));
}

#[test]
fn _0017() {
  // Navigate into Applicant → nonexistent field returns None
  let registry = make_insurance_registry();
  let applicant = &registry.get("Applicant").unwrap().feel_type;
  assert!(resolve_field_chain(applicant, &["ssn"]).is_none());
}

#[test]
fn _0018() {
  // Navigate into PolicyQuote → premium
  let registry = make_insurance_registry();
  let quote = &registry.get("PolicyQuote").unwrap().feel_type;
  assert_eq!(resolve_field_chain(quote, &["premium"]), Some(FeelType::Number));
}

#[test]
fn _0019() {
  // ClaimStatus enum: valid values pass, invalid value fails
  let registry = make_insurance_registry();
  let claim_status = registry.get("ClaimStatus").unwrap();
  assert!(validate_allowed_value("open", claim_status).is_ok());
  assert!(validate_allowed_value("closed", claim_status).is_ok());
  assert!(validate_allowed_value("under review", claim_status).is_ok());
  let err = validate_allowed_value("denied", claim_status).unwrap_err();
  assert!(err.to_string().contains("not in allowed values"));
}

#[test]
fn _0020() {
  // CoverageType enum: valid values pass, invalid value fails
  let registry = make_insurance_registry();
  let coverage = registry.get("CoverageType").unwrap();
  assert!(validate_allowed_value("liability", coverage).is_ok());
  assert!(validate_allowed_value("collision", coverage).is_ok());
  assert!(validate_allowed_value("comprehensive", coverage).is_ok());
  let err = validate_allowed_value("flood", coverage).unwrap_err();
  assert!(err.to_string().contains("not in allowed values"));
}

#[test]
fn _0021() {
  // BKM "Risk Assessment" with valid signature passes validation
  let bkm = DmnNode {
    id: "risk_assessment".to_string(),
    node_type: "bkm".to_string(),
    name: "Risk Assessment".to_string(),
    data_type: Some(DataTypeRef {
      type_ref: "number".to_string(),
      schema: None,
    }),
    signature: Some(BkmSignature {
      parameters: vec![
        BkmParameter {
          name: "applicant".to_string(),
          param_type: "Applicant".to_string(),
        },
        BkmParameter {
          name: "coverage".to_string(),
          param_type: "CoverageType".to_string(),
        },
      ],
      return_type: Some("number".to_string()),
      schema: None,
    }),
    uri: None,
    owner: None,
    requires: None,
    governed_by: None,
    supported_by: None,
  };
  assert!(validate_bkm_signature(&bkm).is_ok());
}

#[test]
fn _0022() {
  // BKM "Risk Assessment" missing return type fails validation
  let bkm = DmnNode {
    id: "risk_assessment".to_string(),
    node_type: "bkm".to_string(),
    name: "Risk Assessment".to_string(),
    data_type: None,
    signature: Some(BkmSignature {
      parameters: vec![BkmParameter {
        name: "applicant".to_string(),
        param_type: "Applicant".to_string(),
      }],
      return_type: None,
      schema: None,
    }),
    uri: None,
    owner: None,
    requires: None,
    governed_by: None,
    supported_by: None,
  };
  let err = validate_bkm_signature(&bkm).unwrap_err();
  assert!(err.to_string().contains("missing a return-type"));
}

#[test]
fn _0023() {
  // Premium Decision governed-by State Regulations (knowledge-source) is valid
  let decision = DmnNode {
    id: "premium_decision".to_string(),
    node_type: "decision".to_string(),
    name: "Premium Decision".to_string(),
    data_type: Some(DataTypeRef {
      type_ref: "PolicyQuote".to_string(),
      schema: None,
    }),
    signature: None,
    uri: None,
    owner: None,
    requires: Some(vec!["risk_assessment".to_string()]),
    governed_by: Some(vec!["state_regulations".to_string()]),
    supported_by: None,
  };
  let regulations = DmnNode {
    id: "state_regulations".to_string(),
    node_type: "knowledge-source".to_string(),
    name: "State Insurance Regulations".to_string(),
    data_type: None,
    signature: None,
    uri: Some("https://insurance.gov/regulations".to_string()),
    owner: None,
    requires: None,
    governed_by: None,
    supported_by: None,
  };
  assert!(validate_link_targets(&decision, &[regulations]).is_ok());
}

#[test]
fn _0024() {
  // Premium Decision governed-by a BKM (wrong node type) fails
  let decision = DmnNode {
    id: "premium_decision".to_string(),
    node_type: "decision".to_string(),
    name: "Premium Decision".to_string(),
    data_type: Some(DataTypeRef {
      type_ref: "PolicyQuote".to_string(),
      schema: None,
    }),
    signature: None,
    uri: None,
    owner: None,
    requires: None,
    governed_by: Some(vec!["risk_assessment".to_string()]),
    supported_by: None,
  };
  let bkm = DmnNode {
    id: "risk_assessment".to_string(),
    node_type: "bkm".to_string(),
    name: "Risk Assessment".to_string(),
    data_type: None,
    signature: None,
    uri: None,
    owner: None,
    requires: None,
    governed_by: None,
    supported_by: None,
  };
  let result = validate_link_targets(&decision, &[bkm]);
  assert!(result.is_err());
}

#[test]
fn _0025() {
  // End-to-end: register types, resolve decision output type, drill into fields, validate enum
  let registry = make_insurance_registry();

  // 1. Resolve the decision's output type from the registry
  let data_type = DataTypeRef {
    type_ref: "PolicyQuote".to_string(),
    schema: None,
  };
  let quote_entry = resolve_data_type(&data_type, Path::new("."), &registry).unwrap();

  // 2. Drill into the resolved type to verify structure
  assert_eq!(resolve_field_chain(&quote_entry.feel_type, &["premium"]), Some(FeelType::Number));
  assert_eq!(resolve_field_chain(&quote_entry.feel_type, &["coverage"]), Some(FeelType::String));
  assert_eq!(resolve_field_chain(&quote_entry.feel_type, &["deductible"]), Some(FeelType::Number));

  // 3. The coverage field's value must be a valid CoverageType
  let coverage_entry = registry.get("CoverageType").unwrap();
  assert!(validate_allowed_value("comprehensive", coverage_entry).is_ok());
  assert!(validate_allowed_value("fire", coverage_entry).is_err());

  // 4. Resolve the BKM input type and navigate into nested fields
  let applicant_ref = DataTypeRef {
    type_ref: "Applicant".to_string(),
    schema: None,
  };
  let applicant_entry = resolve_data_type(&applicant_ref, Path::new("."), &registry).unwrap();
  assert_eq!(
    resolve_field_chain(&applicant_entry.feel_type, &["driving record", "years licensed"]),
    Some(FeelType::Number)
  );
}

#[test]
fn _0026() {
  // Merging a second registry of endorsement types into the insurance registry
  let mut registry = make_insurance_registry();
  let mut endorsement_registry = TypeRegistry::new();
  endorsement_registry
    .insert(TypeEntry {
      name: "EndorsementType".to_string(),
      feel_type: FeelType::String,
      source: TypeSource::Primitive,
      allowed_values: Some(vec!["rental car".to_string(), "roadside assistance".to_string(), "gap coverage".to_string()]),
      optional_fields: HashMap::new(),
    })
    .unwrap();

  registry.merge(endorsement_registry).unwrap();
  assert_eq!(registry.len(), 5);

  let endorsement = registry.get("EndorsementType").unwrap();
  assert!(validate_allowed_value("gap coverage", endorsement).is_ok());
  assert!(validate_allowed_value("pet insurance", endorsement).is_err());
}
