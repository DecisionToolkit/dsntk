use crate::errors::*;
use crate::idml_utils::*;
use crate::{
  Decision, DecisionTable, Definitions, DmnId, DrgElement, ExpressionInstance, ExtensionAttribute, ExtensionElement, InformationItem, InformationRequirement, InputData,
  LiteralExpression,
};
use dsntk_common::{gen_id, Result};
use dsntk_feel::{Name, FEEL_TYPE_NAME_ANY};
use idml::Node;

/// Parses the IDML input document containing DMN model into [Definitions].
pub fn from_idml(input: &str) -> Result<Definitions> {
  // Parse the IDML document.
  match idml::parse(input) {
    Ok(document) => {
      // Initialize model parser.
      let mut model_parser = ModelParser::new();
      // Parse model and convert into definitions.
      model_parser.parse_definitions(&document)
    }
    Err(reason) => Err(err_idml_parsing_model_failed(&reason.to_string())),
  }
}

/// IDML parser for DMN model.
struct ModelParser {
  /// Model namespace used in parsed definitions.
  namespace: String,
  /// Model name used in parsed definitions.
  model_name: String,
}

impl ModelParser {
  /// Creates new model parser.
  fn new() -> Self {
    Self {
      namespace: "".to_string(),
      model_name: "".to_string(),
    }
  }

  /// Parses [Definitions].
  fn parse_definitions(&mut self, node: &Node) -> Result<Definitions> {
    self.namespace = required_uri(node, IDML_NAMESPACE)?;
    self.model_name = required_name(node)?;
    let definitions = Definitions {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      name: self.model_name.clone(),
      feel_name: required_feel_name(node)?,
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      expression_language: optional_value(node, IDML_EXPRESSION_LANGUAGE),
      type_language: optional_value(node, IDML_TYPE_LANGUAGE),
      exporter: optional_value(node, IDML_EXPORTER),
      exporter_version: optional_value(node, IDML_EXPORTER_VERSION),
      item_definitions: vec![],
      drg_elements: self.parse_drg_elements(node)?,
      business_context_elements: vec![],
      imports: vec![],
      dmndi: None,
    };
    Ok(definitions)
  }

  /// Parses [DrgElement] nodes.
  fn parse_drg_elements(&mut self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut drg_elements = vec![];
    drg_elements.append(&mut self.parse_input_data_nodes(node)?);
    drg_elements.append(&mut self.parse_decision_nodes(node)?);
    // drg_elements.append(&mut self.parse_business_knowledge_model_nodes(node)?);
    // drg_elements.append(&mut self.parse_decision_services(node)?);
    // drg_elements.append(&mut self.parse_knowledge_sources(node)?);
    Ok(drg_elements)
  }

  /// Parses [InputData] nodes.
  fn parse_input_data_nodes(&self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut input_data_items = vec![];
    for child_node in node.children().filter(filter_name(IDML_INPUT_DATA)) {
      let name = required_name(child_node)?;
      let feel_name = required_feel_name(child_node)?;
      let variable = self
        .parse_opt_information_item_child(child_node, IDML_VARIABLE)?
        .unwrap_or(self.create_information_item(name.clone(), feel_name.clone())?);
      let input_data = InputData {
        namespace: self.namespace.clone(),
        model_name: self.model_name.clone(),
        id: optional_id(child_node),
        description: optional_description(child_node),
        label: optional_label(child_node),
        extension_elements: self.parse_extension_elements(child_node),
        extension_attributes: self.parse_extension_attributes(child_node),
        name,
        feel_name,
        variable,
      };
      input_data_items.push(DrgElement::InputData(input_data));
    }
    Ok(input_data_items)
  }

  /// Parses [Decision] nodes.
  fn parse_decision_nodes(&mut self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut decision_items = vec![];
    for child_node in node.children().filter(filter_name(IDML_DECISION)) {
      let name = required_name(child_node)?;
      let feel_name = required_feel_name(child_node)?;
      let variable = self
        .parse_opt_information_item_child(child_node, IDML_VARIABLE)?
        .unwrap_or(self.create_information_item(name.clone(), feel_name.clone())?);
      let decision = Decision {
        namespace: self.namespace.clone(),
        model_name: self.model_name.clone(),
        name,
        feel_name,
        id: optional_id(child_node),
        description: optional_description(child_node),
        label: optional_label(child_node),
        extension_elements: self.parse_extension_elements(child_node),
        extension_attributes: self.parse_extension_attributes(child_node),
        question: optional_value(child_node, IDML_QUESTION),
        allowed_answers: optional_value(child_node, IDML_ALLOWED_ANSWERS),
        variable,
        decision_logic: self.parse_optional_child_expression_instance(child_node)?,
        information_requirements: self.parse_information_requirements(child_node, IDML_INFORMATION_REQUIREMENT)?,
        knowledge_requirements: vec![],
        authority_requirements: vec![],
      };
      decision_items.push(DrgElement::Decision(decision));
    }
    Ok(decision_items)
  }

  /// Creates a new [InformationItem] element populated with name.
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

  /// Parses an optional [InformationItem], contained in a child node having the specified name.
  fn parse_opt_information_item_child(&self, node: &Node, child_name: &str) -> Result<Option<InformationItem>> {
    Ok(if let Some(child_node) = node.children().find(filter_name(child_name)) {
      Some(self.parse_information_item(child_node)?)
    } else {
      None
    })
  }

  /// Parses the attributes of the [InformationItem] element.
  fn parse_information_item(&self, node: &Node) -> Result<InformationItem> {
    Ok(InformationItem {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      name: required_name(node)?,
      feel_name: required_feel_name(node)?,
      type_ref: optional_value(node, IDML_TYPE_REF).unwrap_or(FEEL_TYPE_NAME_ANY.to_string()),
      feel_type: None,
    })
  }

  /// Parses extension elements.
  fn parse_extension_elements(&self, _node: &Node) -> Vec<ExtensionElement> {
    // Currently ignored. Ready for future development when needed.
    vec![]
  }

  /// Parses extension attributes.
  fn parse_extension_attributes(&self, _node: &Node) -> Vec<ExtensionAttribute> {
    // Currently ignored. Ready for future development when needed.
    vec![]
  }

  fn parse_optional_child_expression_instance(&self, node: &Node) -> Result<Option<ExpressionInstance>> {
    // if let Some(context) = self.parse_optional_context(node)? {
    //   return Ok(Some(ExpressionInstance::Context(Box::new(context))));
    // }
    if let Some(decision_table) = self.parse_optional_decision_table(node)? {
      return Ok(Some(ExpressionInstance::DecisionTable(Box::new(decision_table))));
    }
    // if let Some(function_definition) = self.parse_optional_function_definition(node)? {
    //   return Ok(Some(ExpressionInstance::FunctionDefinition(Box::new(function_definition))));
    // }
    // if let Some(invocation) = self.parse_optional_invocation(node)? {
    //   return Ok(Some(ExpressionInstance::Invocation(Box::new(invocation))));
    // }
    // if let Some(list) = self.parse_optional_list(node)? {
    //   return Ok(Some(ExpressionInstance::List(Box::new(list))));
    // }
    if let Some(literal_expression) = self.parse_optional_literal_expression(node) {
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

  /// Searches for the first node named 'literalExpression' among children of the specified `node`.
  /// When such node is found, then parses literal expression and returns it, otherwise returns [None].
  fn parse_optional_literal_expression(&self, node: &Node) -> Option<LiteralExpression> {
    if let Some(child_node) = node.children().find(filter_name(IDML_LITERAL_EXPRESSION)) {
      return Some(self.parse_literal_expression(child_node));
    }
    None
  }

  /// Parses [LiteralExpression] directly from the specified node.
  /// The `literal_expression_node` must be a node named `literalExpression`.
  fn parse_literal_expression(&self, node: &Node) -> LiteralExpression {
    LiteralExpression {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      type_ref: optional_value(node, IDML_TYPE_REF),
      text: optional_value(node, IDML_TEXT),
      expression_language: optional_value(node, IDML_EXPRESSION_LANGUAGE),
      imported_values: None,
    }
  }

  /// Parses information requirements.
  fn parse_information_requirements(&mut self, node: &Node, child_name: &str) -> Result<Vec<InformationRequirement>> {
    let mut information_requirement_items = vec![];
    for child_node in node.children().filter(filter_name(child_name)) {
      information_requirement_items.push(self.parse_information_requirement(child_node)?);
    }
    Ok(information_requirement_items)
  }

  /// Parses single information requirement.
  fn parse_information_requirement(&mut self, node: &Node) -> Result<InformationRequirement> {
    let req = InformationRequirement {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      required_decision: optional_child_required_href(node, IDML_REQUIRED_DECISION)?,
      required_input: optional_child_required_href(node, IDML_REQUIRED_INPUT)?,
    };
    Ok(req)
  }

  fn parse_optional_decision_table(&self, node: &Node) -> Result<Option<DecisionTable>> {
    if let Some(child_node) = node.children().find(filter_name(IDML_DECISION_TABLE)) {
      return Ok(Some(self.parse_decision_table(child_node)?));
    }
    Ok(None)
  }

  fn parse_decision_table(&self, node: &Node) -> Result<DecisionTable> {
    let content = node.text();
    let mut decision_table: DecisionTable = dsntk_recognizer::from_unicode(content, false)?.into();
    decision_table.namespace = self.namespace.clone();
    decision_table.model_name = self.model_name.clone();
    decision_table.id = optional_id(node);
    //decision_table.output_label = optional_value(node, IDML_OUTPUT_LABEL);
    Ok(decision_table)

    // Ok(DecisionTable {
    //   namespace: self.namespace.clone(),
    //   model_name: self.model_name.clone(),
    //   id: optional_id(node),
    //   description: optional_description(node),
    //   label: optional_label(node),
    //   extension_elements: self.parse_extension_elements(node),
    //   extension_attributes: self.parse_extension_attributes(node),
    //   type_ref: optional_value(node, IDML_TYPE_REF),
    //   information_item_name: None,
    //   input_clauses: self.parse_decision_table_inputs(node)?,
    //   output_clauses: self.parse_decision_table_outputs(node)?,
    //   annotations: vec![], // TODO implement parsing annotations
    //   rules: self.parse_decision_table_rules(node)?,
    //   hit_policy: self.parse_hit_policy_attribute(node)?,
    //   aggregation: None,
    //   preferred_orientation: self.parse_preferred_orientation_attribute(node)?,
    //   output_label: optional_value(node, IDML_OUTPUT_LABEL),
    // })
  }
}
