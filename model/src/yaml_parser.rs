//! # YAML parser for DMN model

use super::errors::*;
use crate::yaml_utils::*;
use crate::{
  Decision, Definitions, DmnId, DrgElement, ExpressionInstance, ExtensionAttribute, ExtensionElement, InformationItem, InformationRequirement, InputData, LiteralExpression,
};
use dsntk_common::{gen_id, Result};
use dsntk_feel::{Name, FEEL_TYPE_NAME_ANY};
use yaml_rust::{Yaml, YamlLoader};

/// Parses the YAML input document containing DMN model into [Definitions].
pub fn from_yaml(input: &str) -> Result<Definitions> {
  match YamlLoader::load_from_str(input) {
    Ok(docs) => match docs.len() {
      0 => Err(err_yaml_parsing_model_failed("empty YAML model")),
      1 => {
        let mut parser = Parser::new();
        parser.parse_definitions(&docs[0])
      }
      other => Err(err_yaml_parsing_model_failed(&format!("expected only one document in YAML model, actual is {}", other))),
    },
    Err(reason) => Err(err_yaml_parsing_model_failed(&reason.to_string())),
  }
}

/// DMN model parser from YAML format.
struct Parser {
  /// Model namespace used in parsed definitions.
  namespace: String,
  /// Model name used in parsed definitions.
  model_name: String,
}

impl Parser {
  /// Creates new model parser.
  fn new() -> Self {
    Self {
      namespace: "".to_string(),
      model_name: "".to_string(),
    }
  }

  /// Parses model [Definitions].
  fn parse_definitions(&mut self, yaml: &Yaml) -> Result<Definitions> {
    self.namespace = required_uri(yaml, YAML_NAMESPACE)?;
    self.model_name = required_name(yaml)?;
    let definitions = Definitions {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      name: self.model_name.clone(),
      feel_name: required_feel_name(yaml)?,
      id: optional_id(yaml),
      description: optional_description(yaml),
      label: optional_label(yaml),
      extension_elements: self.parse_extension_elements(yaml),
      extension_attributes: self.parse_extension_attributes(yaml),
      expression_language: optional_uri(yaml, YAML_EXPRESSION_LANGUAGE)?,
      type_language: optional_attribute(yaml, YAML_TYPE_LANGUAGE),
      exporter: None,
      exporter_version: None,
      item_definitions: vec![],
      drg_elements: self.parse_drg_elements(yaml)?,
      business_context_elements: vec![],
      imports: vec![],
      dmndi: None,
    };
    Ok(definitions)
  }

  /// Parses DRG elements.
  fn parse_drg_elements(&mut self, yaml: &Yaml) -> Result<Vec<DrgElement>> {
    let mut drg_elements = vec![];
    let mut input_data_items = vec![];
    let mut decision_items = vec![];
    if let Some(definitions) = yaml[YAML_DEFINITIONS].as_vec() {
      for item in definitions {
        if let Some(input_data_yaml) = scalar(item, YAML_INPUT_DATA) {
          input_data_items.push(self.parse_input_data(input_data_yaml)?);
        }
        if let Some(decision_yaml) = scalar(item, YAML_DECISION) {
          decision_items.push(self.parse_decision(decision_yaml)?);
        }
      }
    }
    drg_elements.append(&mut input_data_items);
    drg_elements.append(&mut decision_items);
    // drg_elements.append(&mut self.parse_business_knowledge_model_nodes(node)?);
    // drg_elements.append(&mut self.parse_decision_services(node)?);
    // drg_elements.append(&mut self.parse_knowledge_sources(node)?);
    Ok(drg_elements)
  }

  /// Parses [InputData].
  fn parse_input_data(&self, yaml: &Yaml) -> Result<DrgElement> {
    let name = required_name(yaml)?;
    let feel_name = required_feel_name(yaml)?;
    let variable = self
      .parse_opt_information_item_child(yaml, YAML_VARIABLE)?
      .unwrap_or(self.create_information_item(name.clone(), feel_name.clone())?);
    let input_data = InputData {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(yaml),
      description: optional_description(yaml),
      label: optional_label(yaml),
      extension_elements: self.parse_extension_elements(yaml),
      extension_attributes: self.parse_extension_attributes(yaml),
      name,
      feel_name,
      variable,
    };
    Ok(DrgElement::InputData(input_data))
  }

  /// Parses [Decision].
  fn parse_decision(&mut self, yaml: &Yaml) -> Result<DrgElement> {
    let name = required_name(yaml)?;
    let feel_name = required_feel_name(yaml)?;
    let variable = self
      .parse_opt_information_item_child(yaml, YAML_VARIABLE)?
      .unwrap_or(self.create_information_item(name.clone(), feel_name.clone())?);
    let decision = Decision {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      name,
      feel_name,
      id: optional_id(yaml),
      description: optional_description(yaml),
      label: optional_label(yaml),
      extension_elements: self.parse_extension_elements(yaml),
      extension_attributes: self.parse_extension_attributes(yaml),
      question: optional_attribute(yaml, YAML_QUESTION).map(|value| value.trim().to_string()),
      allowed_answers: optional_attribute(yaml, YAML_ALLOWED_ANSWERS).map(|value| value.trim().to_string()),
      variable,
      decision_logic: self.parse_optional_child_expression_instance(yaml)?,
      information_requirements: self.parse_information_requirements(yaml, YAML_INFORMATION_REQUIREMENT)?,
      knowledge_requirements: vec![], // self.parse_knowledge_requirements(child_node, NODE_KNOWLEDGE_REQUIREMENT)?,
      authority_requirements: vec![], // self.parse_authority_requirements(child_node, NODE_AUTHORITY_REQUIREMENT)?,
    };
    Ok(DrgElement::Decision(decision))
  }

  fn parse_optional_child_expression_instance(&self, yaml: &Yaml) -> Result<Option<ExpressionInstance>> {
    // if let Some(context) = self.parse_optional_context(node)? {
    //   return Ok(Some(ExpressionInstance::Context(Box::new(context))));
    // }
    // if let Some(decision_table) = self.parse_optional_decision_table(node)? {
    //   return Ok(Some(ExpressionInstance::DecisionTable(Box::new(decision_table))));
    // }
    // if let Some(function_definition) = self.parse_optional_function_definition(node)? {
    //   return Ok(Some(ExpressionInstance::FunctionDefinition(Box::new(function_definition))));
    // }
    // if let Some(invocation) = self.parse_optional_invocation(node)? {
    //   return Ok(Some(ExpressionInstance::Invocation(Box::new(invocation))));
    // }
    // if let Some(list) = self.parse_optional_list(node)? {
    //   return Ok(Some(ExpressionInstance::List(Box::new(list))));
    // }
    if let Some(literal_expression) = self.parse_optional_literal_expression(yaml) {
      return Ok(Some(ExpressionInstance::LiteralExpression(Box::new(literal_expression))));
    }
    // if let Some(relation) = self.parse_optional_relation(node)? {
    //   return Ok(Some(ExpressionInstance::Relation(Box::new(relation))));
    // }
    // if let Some(conditional) = self.parse_optional_conditional(node)? {
    //   return Ok(Some(ExpressionInstance::Conditional(Box::new(conditional))));
    // }
    // if let Some(filter) = self.parse_optional_filter(node)? {
    //   return Ok(Some(ExpressionInstance::Filter(Box::new(filter))));
    // }
    // if let Some(r#for) = self.parse_optional_for(node)? {
    //   return Ok(Some(ExpressionInstance::For(Box::new(r#for))));
    // }
    // if let Some(every) = self.parse_optional_every(node)? {
    //   return Ok(Some(ExpressionInstance::Every(Box::new(every))));
    // }
    // if let Some(some) = self.parse_optional_some(node)? {
    //   return Ok(Some(ExpressionInstance::Some(Box::new(some))));
    // }
    Ok(None)
  }

  /// Searches for the literal expression attribute.
  /// If found, then parses literal expression and returns it, otherwise returns [None].
  fn parse_optional_literal_expression(&self, yaml: &Yaml) -> Option<LiteralExpression> {
    scalar(yaml, YAML_LITERAL_EXPRESSION).map(|literal_expression_yaml| self.parse_literal_expression(literal_expression_yaml))
  }

  /// Parses [LiteralExpression].
  fn parse_literal_expression(&self, yaml: &Yaml) -> LiteralExpression {
    LiteralExpression {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(yaml),
      description: optional_description(yaml),
      label: optional_label(yaml),
      extension_elements: self.parse_extension_elements(yaml),
      extension_attributes: self.parse_extension_attributes(yaml),
      type_ref: optional_attribute(yaml, YAML_TYPE_REF),
      text: optional_attribute(yaml, YAML_TEXT),
      expression_language: optional_attribute(yaml, YAML_EXPRESSION_LANGUAGE),
      imported_values: None,
    }
  }

  /// Parses an optional [InformationItem].
  fn parse_opt_information_item_child(&self, yaml: &Yaml, attr_name: &str) -> Result<Option<InformationItem>> {
    Ok(if let Some(information_item_yaml) = scalar(yaml, attr_name) {
      Some(self.parse_information_item(information_item_yaml)?)
    } else {
      None
    })
  }

  /// Parses [InformationItem].
  fn parse_information_item(&self, yaml: &Yaml) -> Result<InformationItem> {
    Ok(InformationItem {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(yaml),
      description: optional_description(yaml),
      label: optional_label(yaml),
      extension_elements: self.parse_extension_elements(yaml),
      extension_attributes: self.parse_extension_attributes(yaml),
      name: required_name(yaml)?,
      feel_name: required_feel_name(yaml)?,
      type_ref: optional_attribute(yaml, YAML_TYPE_REF).unwrap_or(FEEL_TYPE_NAME_ANY.to_string()),
      feel_type: None,
    })
  }

  /// Creates a new [InformationItem].
  fn create_information_item(&self, name: String, feel_name: Name) -> Result<InformationItem> {
    Ok(InformationItem {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: DmnId::Generated(gen_id()),
      description: None,
      label: None,
      extension_elements: vec![],
      extension_attributes: vec![],
      name,
      feel_name,
      type_ref: FEEL_TYPE_NAME_ANY.to_string(),
      feel_type: None,
    })
  }

  /// Parses a list of [InformationRequirement].
  fn parse_information_requirements(&mut self, node: &Yaml, child_name: &str) -> Result<Vec<InformationRequirement>> {
    let mut information_requirements = vec![];
    if let Some(items) = node[child_name].as_vec() {
      for item in items {
        information_requirements.push(self.parse_information_requirement(item)?);
      }
    }
    Ok(information_requirements)
  }

  /// Parses [InformationRequirement].
  fn parse_information_requirement(&mut self, yaml: &Yaml) -> Result<InformationRequirement> {
    let req = InformationRequirement {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(yaml),
      description: optional_description(yaml),
      label: optional_label(yaml),
      extension_elements: self.parse_extension_elements(yaml),
      extension_attributes: self.parse_extension_attributes(yaml),
      required_decision: optional_attribute_required_href(yaml, YAML_REQUIRED_DECISION)?,
      required_input: optional_attribute_required_href(yaml, YAML_REQUIRED_INPUT)?,
    };
    Ok(req)
  }

  /// Parses extension elements.
  fn parse_extension_elements(&self, _yaml: &Yaml) -> Vec<ExtensionElement> {
    // Currently ignored. Ready for future development when needed.
    vec![]
  }

  /// Parses extension attributes.
  fn parse_extension_attributes(&self, _yaml: &Yaml) -> Vec<ExtensionAttribute> {
    // Currently ignored. Ready for future development when needed.
    vec![]
  }
}
