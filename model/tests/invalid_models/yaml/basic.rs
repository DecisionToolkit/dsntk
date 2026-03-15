use dsntk_model::from_yaml;

#[test]
fn _0001() {
  // Parsing an invalid file should fail.
  let yaml = r#"key: one\n key: one"#;
  assert_eq!(
    "<ModelParserError> parsing model from YAML failed with reason: mapping values are not allowed in this context at line 1 column 15",
    from_yaml(yaml).unwrap_err().to_string()
  );
}

#[test]
fn _0002() {
  // Parsing an empty file should fail.
  let yaml = r#""#;
  assert_eq!(
    "<ModelParserError> parsing model from YAML failed with reason: empty YAML model",
    from_yaml(yaml).unwrap_err().to_string()
  );
}

#[test]
fn _0003() {
  // Parsing multiple documents in one file should fail.
  let yaml = r#"
key: "1st document"
---
key: "2nd document"
---
key: "3rd document"
  "#;
  assert_eq!(
    "<ModelParserError> parsing model from YAML failed with reason: expected only one document in YAML model, actual is 3",
    from_yaml(yaml).unwrap_err().to_string()
  );
}
