use crate::xml_utils::node_name_pos;
use dsntk_common::{DsntkError, ToErrorMessage};
use roxmltree::Node;

/// Errors related to the DMN model.
#[derive(ToErrorMessage)]
struct ModelError(String);

pub fn err_invalid_decision_table_orientation(orientation: &str) -> DsntkError {
  ModelError(format!("invalid decision table orientation: {orientation}")).into()
}

pub fn err_invalid_decision_table_hit_policy(hit_policy: &str) -> DsntkError {
  ModelError(format!("invalid decision table hit policy: {hit_policy}")).into()
}

/// Errors related with parsing the decision model.
#[derive(ToErrorMessage)]
struct ModelParserError(String);

/// Raised when parsed text is not a valid function kind, accepted values are:
/// `FEEL`, `Java` or `PMML`.
pub fn err_invalid_function_kind(s: &str) -> DsntkError {
  ModelParserError(format!("'{s}' is not a valid function kind, accepted values are: 'FEEL', 'Java', 'PMML'")).into()
}

/// Raised when parsed text is not a valid hit policy, accepted values are:
/// `UNIQUE`, `FIRST`, `PRIORITY`, `ANY`, `COLLECT`, `RULE ORDER` or `OUTPUT ORDER`.
pub fn err_invalid_hit_policy(s: &str) -> DsntkError {
  ModelParserError(format!(
    "'{s}' is not a valid hit policy, allowed values are: 'UNIQUE', 'FIRST', 'PRIORITY', 'ANY', 'COLLECT', 'RULE ORDER', 'OUTPUT ORDER'"
  ))
  .into()
}

/// Raised when parsed text is not a valid aggregation for hit policy, accepted values are:
/// `COUNT`, `SUM`, `MIN`, or `MAX`.
pub fn err_invalid_aggregation(s: &str) -> DsntkError {
  ModelParserError(format!("'{s}' is not a valid aggregation, allowed values are: 'COUNT', 'SUM', 'MIN', 'MAX'")).into()
}

/// Invalid value for a color.
pub fn err_invalid_color_value(s: &str) -> DsntkError {
  ModelParserError(format!("conversion to valid color value failed with reason: {s}")).into()
}

/// Invalid value for a double.
pub fn err_invalid_double_value(reason: &str) -> DsntkError {
  ModelParserError(format!("conversion to valid double value failed with reason: {reason}")).into()
}

/// Raised when required child node is missing.
pub fn err_required_child_node_is_missing(s1: &str, s2: &str) -> DsntkError {
  ModelParserError(format!("required child node '{s2}' in parent node '{s1}' is missing")).into()
}

/// Raised when required `inputExpression` node is missing.
pub fn err_required_input_expression_is_missing() -> DsntkError {
  ModelParserError("required input expression in decision table's input clause is missing".to_string()).into()
}

/// Raised when required expression instance is missing.
pub fn err_required_expression_instance_is_missing() -> DsntkError {
  ModelParserError("required expression instance is missing".to_string()).into()
}

/// Raised when the number of elements in a row differs from the number of columns in relation.
pub fn err_number_of_elements_in_row_differs_from_number_of_columns() -> DsntkError {
  ModelParserError("number of elements in a row differs from the number of columns defined in a relation".to_string()).into()
}

pub fn err_xml_parsing_model_failed(s: &str) -> DsntkError {
  ModelParserError(format!("parsing model from XML failed with reason: {s}")).into()
}

/// Raised when specified node has no child nodes.
pub fn err_node_has_no_children(s: &str) -> DsntkError {
  ModelParserError(format!("node {s} has no children nodes")).into()
}

pub fn err_xml_unexpected_node(s1: &str, s2: &str) -> DsntkError {
  ModelParserError(format!("unexpected XML node, expected: {s1}, actual: {s2}")).into()
}

pub fn err_xml_expected_mandatory_attribute(s1: &str, s2: &str) -> DsntkError {
  ModelParserError(format!("expected value for mandatory attribute '{s2}' in node {s1}")).into()
}

pub fn err_xml_expected_mandatory_child_node(s1: &str, s2: &str) -> DsntkError {
  ModelParserError(format!("expected mandatory child node '{s2}' in parent node {s1}")).into()
}

pub fn err_xml_expected_mandatory_text_content(s: &str) -> DsntkError {
  ModelParserError(format!("expected mandatory text content in node '{s}'")).into()
}

pub fn err_yaml_parsing_model_failed(s: &str) -> DsntkError {
  ModelParserError(format!("parsing model from YAML failed with reason: {s}")).into()
}

pub fn err_yaml_expected_mandatory_attribute(attr_name: &str) -> DsntkError {
  ModelParserError(format!("expected value for mandatory attribute '{attr_name}'")).into()
}

/// Errors related with validating the decision model.
#[derive(ToErrorMessage)]
struct ModelValidatorError(String);

pub fn err_item_definitions_cycle() -> DsntkError {
  ModelValidatorError("cyclic dependency between item definitions".to_string()).into()
}

pub fn err_no_supported_namespace() -> DsntkError {
  ModelError("no supported namespace found".to_string()).into()
}

pub fn err_not_allowed_attribute(namespace: &str, name: &str, node: &Node) -> DsntkError {
  let namespace = if namespace.is_empty() { "" } else { &format!("{}:", namespace) };
  ModelError(format!("not allowed attribute: '{}{}' in node {}", namespace, name, node_name_pos(node))).into()
}

pub fn err_not_allowed_child_node(child_node_name: &str, node: &Node) -> DsntkError {
  ModelError(format!("not allowed child node: '{}' in node {}", child_node_name, node_name_pos(node))).into()
}
