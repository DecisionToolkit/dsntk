use crate::from_xml;
use crate::tests::parser::input_files::*;

#[test]
fn _0001() {
  let definitions = from_xml(T_DMN_0001);
  assert!(definitions.is_err());
  assert_eq!(
    r#"<ModelParserError> 'Python' is not a valid function kind, accepted values are: 'FEEL', 'Java', 'PMML'"#,
    format!("{}", definitions.err().unwrap())
  )
}

#[test]
fn _0002() {
  let definitions = from_xml(T_DMN_0002);
  assert!(definitions.is_err());
  assert_eq!(
    r#"<ModelParserError> 'LAST' is not a valid hit policy, allowed values are: 'UNIQUE', 'FIRST', 'PRIORITY', 'ANY', 'COLLECT', 'RULE ORDER', 'OUTPUT ORDER'"#,
    format!("{}", definitions.err().unwrap())
  )
}

#[test]
fn _0003() {
  let definitions = from_xml(T_DMN_0003);
  assert!(definitions.is_err());
  assert_eq!(
    r#"<ModelParserError> 'AVG' is not a valid aggregation, allowed values are: 'COUNT', 'SUM', 'MIN', 'MAX'"#,
    format!("{}", definitions.err().unwrap())
  )
}

#[test]
fn _0004() {
  let definitions = from_xml(T_DMN_0004);
  assert!(definitions.is_err());
  assert_eq!(
    r#"<ModelParserError> required input expression in decision table's input clause is missing"#,
    format!("{}", definitions.err().unwrap())
  )
}

#[test]
fn _0005() {
  let definitions = from_xml(T_DMN_0005);
  assert!(definitions.is_err());
  assert_eq!(r#"<ModelParserError> required expression instance is missing"#, format!("{}", definitions.err().unwrap()))
}

#[test]
fn _0006() {
  let definitions = from_xml(T_DMN_0006);
  assert!(definitions.is_err());
  assert_eq!(
    r#"<ModelParserError> number of elements in a row differs from the number of columns defined in a relation"#,
    format!("{}", definitions.err().unwrap())
  )
}

#[test]
fn _0007() {
  let definitions = from_xml(T_DMN_0007);
  assert!(definitions.is_err());
  assert_eq!(
    r#"<ModelParserError> parsing model from XML failed with reason: the root node was opened but never closed"#,
    format!("{}", definitions.err().unwrap())
  )
}

#[test]
fn _0008() {
  let definitions = from_xml(T_DMN_0008);
  assert!(definitions.is_err());
  assert_eq!(
    r#"<ModelParserError> unexpected XML node, expected: definitions, actual: definition"#,
    format!("{}", definitions.err().unwrap())
  )
}

#[test]
fn _0009() {
  let definitions = from_xml(T_DMN_0009);
  assert!(definitions.is_err());
  assert_eq!(
    r#"<ModelParserError> expected value for mandatory attribute 'namespace' in node 'definitions' at [2:1]"#,
    format!("{}", definitions.err().unwrap())
  )
}

#[test]
fn _0010() {
  let definitions = from_xml(T_DMN_0010);
  assert!(definitions.is_err());
  assert_eq!(
    r#"<ModelParserError> expected value for mandatory attribute 'name' in node 'decision' at [11:5]"#,
    format!("{}", definitions.err().unwrap())
  )
}

#[test]
fn _0011() {
  let definitions = from_xml(T_DMN_0011);
  assert!(definitions.is_err());
  assert_eq!(
    r#"<ModelParserError> expected mandatory text content in node 'text'"#,
    format!("{}", definitions.err().unwrap())
  )
}

#[test]
fn _0012() {
  let definitions = from_xml(T_DMN_0012);
  assert!(definitions.is_err());
  assert_eq!(
    r#"<ModelParserError> conversion to valid color value failed with reason: number too large to fit in target type"#,
    format!("{}", definitions.err().unwrap())
  )
}

#[test]
fn _0013() {
  let definitions = from_xml(T_DMN_0013);
  assert!(definitions.is_err());
  assert_eq!(
    r#"<ModelParserError> conversion to valid double value failed with reason: invalid float literal"#,
    format!("{}", definitions.err().unwrap())
  )
}

#[test]
fn _0014() {
  let definitions = from_xml(T_DMN_0014);
  assert!(definitions.is_err());
  assert_eq!(
    r#"<ModelParserError> expected mandatory child node 'text' in parent node 'outputEntry' at [31:17]"#,
    format!("{}", definitions.err().unwrap())
  )
}

#[test]
fn _0015() {
  let definitions = from_xml(T_DMN_0015);
  assert!(definitions.is_ok());
}

#[test]
fn _0016() {
  let definitions = from_xml(T_DMN_0016);
  assert!(definitions.is_err());
  assert_eq!(
    r#"<ModelParserError> required child node 'Bounds' in parent node 'DMNShape' is missing"#,
    format!("{}", definitions.err().unwrap())
  )
}
