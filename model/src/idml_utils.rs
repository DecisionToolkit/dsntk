use crate::errors::*;
use crate::DmnId;
use dsntk_common::{gen_id, to_uri, HRef, Result, Uri};
use dsntk_feel::Name;
use idml::Node;

pub const IDML_ALLOWED_ANSWERS: &str = "ALLOWED-ANSWERS";
pub const IDML_DECISION: &str = "DECISION";
pub const IDML_DECISION_TABLE: &str = "DECISION-TABLE";
pub const IDML_DESCRIPTION: &str = "DESCRIPTION";
pub const IDML_EXPORTER: &str = "EXPORTER";
pub const IDML_EXPORTER_VERSION: &str = "EXPORTER-VERSION";
pub const IDML_EXPRESSION_LANGUAGE: &str = "EXPRESSION-LANGUAGE";
pub const IDML_HREF: &str = "HREF";
pub const IDML_ID: &str = "ID";
pub const IDML_INFORMATION_REQUIREMENT: &str = "INFORMATION-REQUIREMENT";
pub const IDML_INPUT_DATA: &str = "INPUT-DATA";
pub const IDML_LABEL: &str = "LABEL";
pub const IDML_LITERAL_EXPRESSION: &str = "LITERAL-EXPRESSION";
pub const IDML_NAME: &str = "NAME";
pub const IDML_NAMESPACE: &str = "NAMESPACE";
//pub const IDML_OUTPUT_LABEL: &str = "OUTPUT-LABEL";
pub const IDML_QUESTION: &str = "QUESTION";
pub const IDML_REQUIRED_DECISION: &str = "REQUIRED-DECISION";
pub const IDML_REQUIRED_INPUT: &str = "REQUIRED-INPUT";
pub const IDML_TEXT: &str = "TEXT";
pub const IDML_TYPE_LANGUAGE: &str = "TYPE-LANGUAGE";
pub const IDML_TYPE_REF: &str = "TYPE-REF";
pub const IDML_VARIABLE: &str = "VARIABLE";

/// Returns the required string value of a child node with specified name.
pub fn required_value(node: &Node, child_name: &str) -> Result<String> {
  if let Some(value) = node.children().find_map(|child| if child.name() == child_name { Some(child.text()) } else { None }) {
    Ok(value.to_string())
  } else {
    Err(err_idml_expected_mandatory_value(node.name(), child_name))
  }
}

/// Returns the required URI value.
pub fn required_uri(node: &Node, child_name: &str) -> Result<Uri> {
  to_uri(required_value(node, child_name)?.as_str())
}

/// Returns the required name value.
pub fn required_name(node: &Node) -> Result<String> {
  required_value(node, IDML_NAME)
}

/// Returns FEEL name for specified node.
pub fn required_feel_name(node: &Node) -> Result<Name> {
  let input = required_name(node)?;
  Ok(dsntk_feel_parser::parse_longest_name(&input).unwrap_or(input.into()))
}

/// Returns the optional value of the child node.
pub fn optional_value(node: &Node, child_name: &str) -> Option<String> {
  if let Some(value) = node.children().find_map(|child| if child.name() == child_name { Some(child.text()) } else { None }) {
    if value.is_empty() {
      None
    } else {
      Some(value.to_string())
    }
  } else {
    None
  }
}

/// Returns optional identifier provided in model or generates a new one.
pub fn optional_id(node: &Node) -> DmnId {
  optional_value(node, IDML_ID).map(DmnId::Provided).unwrap_or(DmnId::Generated(gen_id()))
}

/// Returns optional description.
pub fn optional_description(node: &Node) -> Option<String> {
  optional_value(node, IDML_DESCRIPTION)
}

/// Returns optional label.
pub fn optional_label(node: &Node) -> Option<String> {
  optional_value(node, IDML_LABEL)
}

/// Returns the required `href` attribute of the optional child node.
pub fn optional_child_required_href(node: &Node, child_name: &str) -> Result<Option<HRef>> {
  if let Some(child_node) = optional_child(node, child_name) {
    Ok(Some(HRef::try_from(required_value(child_node, IDML_HREF)?.as_str())?))
  } else {
    Ok(None)
  }
}

/// Returns child node when there is a child with the given name.
pub fn optional_child<'a>(node: &'a Node, child_name: &str) -> Option<&'a Node> {
  node.children().find(|n| n.name() == child_name)
}

/// Returns a closure that checks if the name of the node is equal to provided one.
pub fn filter_name(name: &str) -> impl Fn(&&Node) -> bool + '_ {
  move |n| n.name() == name
}
