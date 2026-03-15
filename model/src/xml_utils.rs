//! # XML utilities

use crate::errors::*;
use dsntk_common::{trim_multiline, Result};
use roxmltree::Node;
use std::str::FromStr;

pub const NAMESPACE_DMN_13: &str = "https://www.omg.org/spec/DMN/20191111/MODEL/";
pub const NAMESPACE_DMN_14: &str = "https://www.omg.org/spec/DMN/20211108/MODEL/";
pub const NAMESPACE_DMN_15: &str = "https://www.omg.org/spec/DMN/20230324/MODEL/";
pub const NAMESPACE_DMNDI_13: &str = "https://www.omg.org/spec/DMN/20191111/DMNDI/";
pub const NAMESPACE_DMNDI_15: &str = "https://www.omg.org/spec/DMN/20230324/DMNDI/";
pub const NAMESPACE_DC_13: &str = "http://www.omg.org/spec/DMN/20180521/DC/";
pub const NAMESPACE_DI_13: &str = "http://www.omg.org/spec/DMN/20180521/DI/";

// XML node names
pub const NODE_ALLOWED_ANSWERS: &str = "allowedAnswers";
pub const NODE_ALLOWED_VALUES: &str = "allowedValues";
pub const NODE_ASSOCIATION: &str = "association";
pub const NODE_AUTHORITY_REQUIREMENT: &str = "authorityRequirement";
pub const NODE_BINDING: &str = "binding";
pub const NODE_BUSINESS_KNOWLEDGE_MODEL: &str = "businessKnowledgeModel";
pub const NODE_COLUMN: &str = "column";
pub const NODE_CONDITIONAL: &str = "conditional";
pub const NODE_CONTEXT: &str = "context";
pub const NODE_CONTEXT_ENTRY: &str = "contextEntry";
pub const NODE_DEFAULT_OUTPUT_ENTRY: &str = "defaultOutputEntry";
pub const NODE_DEFINITIONS: &str = "definitions";
pub const NODE_DECISION: &str = "decision";
pub const NODE_DECISION_MADE: &str = "decisionMade";
pub const NODE_DECISION_MAKER: &str = "decisionMaker";
pub const NODE_DECISION_OWNED: &str = "decisionOwned";
pub const NODE_DECISION_OWNER: &str = "decisionOwner";
pub const NODE_DECISION_TABLE: &str = "decisionTable";
pub const NODE_DECISION_SERVICE: &str = "decisionService";
pub const NODE_DMNDI: &str = "DMNDI";
pub const NODE_DMNDI_DMN_DIAGRAM: &str = "DMNDiagram";
pub const NODE_DMNDI_SIZE: &str = "Size";
pub const NODE_DMNDI_STYLE: &str = "DMNStyle";
pub const NODE_DMNDI_DMN_SHAPE: &str = "DMNShape";
pub const NODE_DMNDI_BOUNDS: &str = "Bounds";
pub const NODE_DMNDI_DMN_EDGE: &str = "DMNEdge";
pub const NODE_DMNDI_WAYPOINT: &str = "waypoint";
pub const NODE_DMNDI_FILL_COLOR: &str = "FillColor";
pub const NODE_DMNDI_STROKE_COLOR: &str = "StrokeColor";
pub const NODE_DMNDI_FONT_COLOR: &str = "FontColor";
pub const NODE_DMNDI_LABEL_HORIZONTAL_ALIGNMENT: &str = "labelHorizontalAlignment";
pub const NODE_DMNDI_LABEL_VERTICAL_ALIGNMENT: &str = "labelVerticalAlignment";
pub const NODE_DMNDI_LABEL: &str = "DMNLabel";
pub const NODE_DMNDI_DECISION_SERVICE_DIVIDER_LINE: &str = "DMNDecisionServiceDividerLine";
pub const NODE_DESCRIPTION: &str = "description";
pub const NODE_ELSE: &str = "else";
pub const NODE_ENCAPSULATED_DECISION: &str = "encapsulatedDecision";
pub const NODE_ENCAPSULATED_LOGIC: &str = "encapsulatedLogic";
pub const NODE_EVERY: &str = "every";
pub const NODE_ELEMENT_COLLECTION: &str = "elementCollection";
pub const NODE_EXTENSION_ELEMENTS: &str = "extensionElements";
pub const NODE_GROUP: &str = "group";
pub const NODE_FILTER: &str = "filter";
pub const NODE_FOR: &str = "for";
pub const NODE_FORMAL_PARAMETER: &str = "formalParameter";
pub const NODE_FUNCTION_DEFINITION: &str = "functionDefinition";
pub const NODE_FUNCTION_ITEM: &str = "functionItem";
pub const NODE_IF: &str = "if";
pub const NODE_IMPACTED_PERFORMANCE_INDICATOR: &str = "impactedPerformanceIndicator";
pub const NODE_IMPACTING_DECISION: &str = "impactingDecision";
pub const NODE_IMPORT: &str = "import";
pub const NODE_IN: &str = "in";
pub const NODE_INFORMATION_REQUIREMENT: &str = "informationRequirement";
pub const NODE_INPUT_DATA: &str = "inputData";
pub const NODE_INPUT: &str = "input";
pub const NODE_INPUT_DECISION: &str = "inputDecision";
pub const NODE_INPUT_ENTRY: &str = "inputEntry";
pub const NODE_INPUT_EXPRESSION: &str = "inputExpression";
pub const NODE_INPUT_VALUES: &str = "inputValues";
pub const NODE_INVOCATION: &str = "invocation";
pub const NODE_ITEM_DEFINITION: &str = "itemDefinition";
pub const NODE_ITEM_COMPONENT: &str = "itemComponent";
pub const NODE_KNOWLEDGE_REQUIREMENT: &str = "knowledgeRequirement";
pub const NODE_KNOWLEDGE_SOURCE: &str = "knowledgeSource";
pub const NODE_LIST: &str = "list";
pub const NODE_LITERAL_EXPRESSION: &str = "literalExpression";
pub const NODE_MATCH: &str = "match";
pub const NODE_OUTPUT: &str = "output";
pub const NODE_OUTPUT_DECISION: &str = "outputDecision";
pub const NODE_OUTPUT_ENTRY: &str = "outputEntry";
pub const NODE_OUTPUT_VALUES: &str = "outputValues";
pub const NODE_PARAMETER: &str = "parameter";
pub const NODE_PARAMETERS: &str = "parameters";
pub const NODE_PERFORMANCE_INDICATOR: &str = "performanceIndicator";
pub const NODE_ORGANISATION_UNIT: &str = "organizationUnit";
pub const NODE_QUESTION: &str = "question";
pub const NODE_RELATION: &str = "relation";
pub const NODE_REQUIRED_AUTHORITY: &str = "requiredAuthority";
pub const NODE_REQUIRED_DECISION: &str = "requiredDecision";
pub const NODE_REQUIRED_KNOWLEDGE: &str = "requiredKnowledge";
pub const NODE_REQUIRED_INPUT: &str = "requiredInput";
pub const NODE_RETURN: &str = "return";
pub const NODE_ROW: &str = "row";
pub const NODE_RULE: &str = "rule";
pub const NODE_SATISFIES: &str = "satisfies";
pub const NODE_SOME: &str = "some";
pub const NODE_SUPPORTED_OBJECTIVE: &str = "supportedObjective";
pub const NODE_TEXT: &str = "text";
pub const NODE_TEXT_ANNOTATION: &str = "textAnnotation";
pub const NODE_THEN: &str = "then";
pub const NODE_TYPE_REF: &str = "typeRef";
pub const NODE_USING_PROCESS: &str = "usingProcess";
pub const NODE_USING_TASK: &str = "usingTask";
pub const NODE_VARIABLE: &str = "variable";

// XML attribute names
pub const ATTR_BLUE: &str = "blue";
pub const ATTR_DMN_ELEMENT_REF: &str = "dmnElementRef";
pub const ATTR_EXPORTER: &str = "exporter";
pub const ATTR_EXPORTER_VERSION: &str = "exporterVersion";
pub const ATTR_EXPRESSION_LANGUAGE: &str = "expressionLanguage";
pub const ATTR_FONT_BOLD: &str = "fontBold";
pub const ATTR_FONT_FAMILY: &str = "fontFamily";
pub const ATTR_FONT_ITALIC: &str = "fontItalic";
pub const ATTR_FONT_SIZE: &str = "fontSize";
pub const ATTR_FONT_STRIKE_THROUGH: &str = "fontStrikeThrough";
pub const ATTR_FONT_UNDERLINE: &str = "fontUnderline";
pub const ATTR_GREEN: &str = "green";
pub const ATTR_HIT_POLICY: &str = "hitPolicy";
pub const ATTR_AGGREGATION: &str = "aggregation";
pub const ATTR_PREFERRED_ORIENTATION: &str = "preferredOrientation";
pub const ATTR_HEIGHT: &str = "height";
pub const ATTR_HREF: &str = "href";
pub const ATTR_ID: &str = "id";
pub const ATTR_IMPORT_TYPE: &str = "importType";
pub const ATTR_IS_COLLAPSED: &str = "isCollapsed";
pub const ATTR_IS_COLLECTION: &str = "isCollection";
pub const ATTR_ITERATOR_VARIABLE: &str = "iteratorVariable";
pub const ATTR_KIND: &str = "kind";
pub const ATTR_LABEL: &str = "label";
pub const ATTR_LABEL_TEXT: &str = "Text";
pub const ATTR_LOCATION_URI: &str = "locationURI";
pub const ATTR_NAME: &str = "name";
pub const ATTR_NAMESPACE: &str = "namespace";
pub const ATTR_OUTPUT_LABEL: &str = "outputLabel";
pub const ATTR_OUTPUT_TYPE_REF: &str = "outputTypeRef";
pub const ATTR_RED: &str = "red";
pub const ATTR_RESOLUTION: &str = "resolution";
pub const ATTR_SHARED_STYLE: &str = "sharedStyle";
pub const ATTR_TYPE_LANGUAGE: &str = "typeLanguage";
pub const ATTR_TYPE_REF: &str = "typeRef";
pub const ATTR_URI: &str = "URI";
pub const ATTR_WIDTH: &str = "width";
pub const ATTR_X: &str = "x";
pub const ATTR_Y: &str = "y";

/// Returns the value of the required attribute.
pub fn required_attribute(node: &Node, attr_name: &str) -> Result<String> {
  if let Some(attr_value) = node.attribute(attr_name) {
    Ok(attr_value.to_owned())
  } else {
    Err(err_xml_expected_mandatory_attribute(&node_name_pos(node), attr_name))
  }
}

/// Returns the value of the mandatory color attribute.
pub fn required_color_part(node: &Node, attr_name: &str) -> Result<u8> {
  u8::from_str(&required_attribute(node, attr_name)?).map_err(|e| err_invalid_color_value(&e.to_string()))
}

/// Returns the value of the mandatory double value.
pub fn required_double(node: &Node, attr_name: &str) -> Result<f64> {
  f64::from_str(&required_attribute(node, attr_name)?).map_err(|e| err_invalid_double_value(&e.to_string()))
}

/// Returns the value of the optional attribute.
pub fn optional_attribute(node: &Node, attr_name: &str) -> Option<String> {
  node.attribute(attr_name).map(|attr_value| attr_value.to_owned())
}

/// Returns the value of the optional double attribute or the default value, when this attribute is not defined.
pub fn optional_double(node: &Node, attr_name: &str) -> Option<f64> {
  optional_attribute(node, attr_name).and_then(|value| f64::from_str(&value).ok())
}

/// Returns the value of the optional bool attribute.
pub fn optional_bool(node: &Node, attr_name: &str) -> Option<bool> {
  optional_attribute(node, attr_name).and_then(|value| bool::from_str(&value).ok())
}

/// Returns required textual content of the node.
pub fn required_content(node: &Node) -> Result<String> {
  if let Some(text) = node.text() {
    Ok(text.to_owned())
  } else {
    Err(err_xml_expected_mandatory_text_content(node.tag_name().name()))
  }
}

/// Returns optional text content of the node with specified name.
pub fn opt_content(node: &Node) -> Option<String> {
  node.text().map(|text| text.to_owned())
}

/// Returns required child node or raises an error when there is no child with given name.
pub fn required_child<'a>(node: &'a Node, child_name: &str) -> Result<Node<'a, 'a>> {
  if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
    Ok(child_node)
  } else {
    Err(err_required_child_node_is_missing(&node_name_pos(node), child_name))
  }
}

/// Returns child node when there is a child with the given name.
pub fn optional_child<'a>(node: &'a Node, child_name: &str) -> Option<Node<'a, 'a>> {
  node.children().find(|n| n.tag_name().name() == child_name)
}

/// Returns the required text contained in the required child node with specified name.
pub fn req_child_req_content(node: &Node, child_name: &str) -> Result<String> {
  if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
    required_content(&child_node)
  } else {
    Err(err_xml_expected_mandatory_child_node(&node_name_pos(node), child_name))
  }
}

/// Returns the optional text contained in the required child node with specified name.
pub fn req_child_opt_content(node: &Node, child_name: &str) -> Result<Option<String>> {
  if let Some(child_node) = node.children().find(name_eq(child_name)) {
    Ok(opt_content(&child_node))
  } else {
    Err(err_xml_expected_mandatory_child_node(&node_name_pos(node), child_name))
  }
}

/// Returns the required content of the optional child node.
pub fn optional_child_required_content(node: &Node, child_name: &str) -> Result<Option<String>> {
  Ok(if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
    Some(required_content(&child_node)?)
  } else {
    None
  })
}

/// Returns the optional content of the optional child node.
pub fn optional_child_optional_content(node: &Node, child_name: &str) -> Option<String> {
  if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
    opt_content(&child_node)
  } else {
    None
  }
}

/// Utility function that returns the node's name with its position in the original document.
pub fn node_name_pos(node: &Node) -> String {
  format!("'{}' at [{}]", node.tag_name().name(), node.document().text_pos_at(node.range().start))
}

/// Returns a closure that checks if the node's name is equal with specified value.
pub fn name_eq(name: &str) -> impl Fn(&Node) -> bool + '_ {
  move |node: &Node| node.tag_name().name() == name
}

/// Returns the optional description.
pub fn optional_description(node: &Node) -> Option<String> {
  optional_child_optional_content(node, NODE_DESCRIPTION).map(trim_multiline)
}

/// Returns the optional label.
pub fn optional_label(node: &Node) -> Option<String> {
  optional_attribute(node, ATTR_LABEL).map(|value| value.trim().to_string())
}
