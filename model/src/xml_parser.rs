//! # XML parser for DMN model

use crate::errors::*;
use crate::model::*;
use crate::validators::{validate_model, validate_schema};
use crate::xml_utils::*;
use dsntk_common::{gen_id, to_uri, HRef, Result, Uri};
use dsntk_feel::{Name, FEEL_TYPE_NAME_ANY};
use roxmltree::{Node, NodeType};

/// Parses the XML input document containing DMN model into [Definitions].
pub fn from_xml(input: &str) -> Result<Definitions> {
  // parse document
  match roxmltree::Document::parse(input) {
    Ok(document) => {
      // firstly validate the document against the XML Schema
      let node = validate_schema(&document)?;
      // initialize the model parser
      let mut model_parser = Parser::new();
      // parse the model into definitions
      let definitions = model_parser.parse_definitions(&node)?;
      // validate the final model against several rules defined in specification
      validate_model(definitions)
    }
    Err(reason) => Err(err_xml_parsing_model_failed(&reason.to_string())),
  }
}

/// DMN model parser from XML format.
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
  fn parse_definitions(&mut self, node: &Node) -> Result<Definitions> {
    self.namespace = required_uri(node, ATTR_NAMESPACE)?;
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
      expression_language: optional_uri(node, ATTR_EXPRESSION_LANGUAGE)?,
      type_language: optional_attribute(node, ATTR_TYPE_LANGUAGE),
      exporter: optional_attribute(node, ATTR_EXPORTER),
      exporter_version: optional_attribute(node, ATTR_EXPORTER_VERSION),
      item_definitions: self.parse_item_definition_nodes(node, NODE_ITEM_DEFINITION)?,
      drg_elements: self.parse_drg_elements(node)?,
      business_context_elements: self.parse_business_context_elements(node)?,
      imports: self.parse_imports(node)?,
      dmndi: self.parse_dmndi(node)?,
    };
    Ok(definitions)
  }

  /// Parses a collection of [ItemDefinition] nodes.
  fn parse_item_definition_nodes(&mut self, node: &Node, child_name: &str) -> Result<Vec<ItemDefinition>> {
    let mut items = vec![];
    for ref child_node in node.children().filter(name_eq(child_name)) {
      items.push(self.parse_item_definition(child_node)?);
    }
    Ok(items)
  }

  /// Parses a single [ItemDefinition] node.
  fn parse_item_definition(&mut self, node: &Node) -> Result<ItemDefinition> {
    let namespace = self.namespace.clone();
    let model_name = self.model_name.clone();
    let name = required_name(node)?;
    let feel_name = required_feel_name(node)?;
    let id = optional_id(node);
    let description = optional_description(node);
    let label = optional_label(node);
    let extension_elements = self.parse_extension_elements(node);
    let extension_attributes = self.parse_extension_attributes(node);
    let type_language = optional_attribute(node, ATTR_TYPE_LANGUAGE);
    let allowed_values = self.parse_unary_tests(node, NODE_ALLOWED_VALUES)?;
    let item_components = self.parse_item_definition_nodes(node, NODE_ITEM_COMPONENT)?;
    let is_collection = self.parse_boolean_attribute(node, ATTR_IS_COLLECTION, false);
    let function_item = self.parse_function_item(node)?;
    let mut type_ref = optional_child_required_content(node, NODE_TYPE_REF)?;
    if type_ref.is_none() && item_components.is_empty() && !is_collection && function_item.is_none() {
      type_ref = Some(FEEL_TYPE_NAME_ANY.to_string());
    }
    Ok(ItemDefinition {
      namespace,
      model_name,
      name,
      feel_name,
      id,
      description,
      label,
      extension_elements,
      extension_attributes,
      type_ref,
      type_language,
      feel_type: None,
      allowed_values,
      item_components,
      is_collection,
      function_item,
    })
  }

  /// Parses optional function item.
  fn parse_function_item(&self, node: &Node) -> Result<Option<FunctionItem>> {
    if let Some(ref n) = node.children().find(name_eq(NODE_FUNCTION_ITEM)) {
      Ok(Some(FunctionItem {
        output_type_ref: optional_attribute(n, ATTR_OUTPUT_TYPE_REF),
        parameters: self.parse_vec_information_item_child(n, NODE_PARAMETERS)?,
      }))
    } else {
      Ok(None)
    }
  }

  /// Parsers unary tests.
  fn parse_unary_tests(&self, node: &Node, child_name: &str) -> Result<Option<UnaryTests>> {
    if let Some(child_node) = node.children().find(name_eq(child_name)) {
      Ok(Some(UnaryTests {
        text: optional_child_required_content(&child_node, NODE_TEXT)?,
        expression_language: optional_attribute(&child_node, ATTR_EXPRESSION_LANGUAGE),
      }))
    } else {
      Ok(None)
    }
  }

  /// Parses DRG elements.
  fn parse_drg_elements(&mut self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut drg_elements = vec![];
    drg_elements.append(&mut self.parse_input_data_nodes(node)?);
    drg_elements.append(&mut self.parse_decision_nodes(node)?);
    drg_elements.append(&mut self.parse_business_knowledge_model_nodes(node)?);
    drg_elements.append(&mut self.parse_decision_services(node)?);
    drg_elements.append(&mut self.parse_knowledge_sources(node)?);
    Ok(drg_elements)
  }

  /// Parses `inputData` nodes.
  fn parse_input_data_nodes(&self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut input_data_items = vec![];
    for ref child_node in node.children().filter(name_eq(NODE_INPUT_DATA)) {
      let name = required_name(child_node)?;
      let feel_name = required_feel_name(child_node)?;
      let variable = self
        .parse_opt_information_item_child(child_node, NODE_VARIABLE)?
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

  /// Parses `decision` nodes.
  fn parse_decision_nodes(&mut self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut decision_items = vec![];
    for ref child_node in node.children().filter(name_eq(NODE_DECISION)) {
      let name = required_name(child_node)?;
      let feel_name = required_feel_name(child_node)?;
      let variable = self
        .parse_opt_information_item_child(child_node, NODE_VARIABLE)?
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
        question: optional_child_optional_content(child_node, NODE_QUESTION).map(|value| value.trim().to_string()),
        allowed_answers: optional_child_optional_content(child_node, NODE_ALLOWED_ANSWERS).map(|value| value.trim().to_string()),
        variable,
        decision_logic: self.parse_optional_child_expression_instance(child_node)?,
        information_requirements: self.parse_information_requirements(child_node, NODE_INFORMATION_REQUIREMENT)?,
        knowledge_requirements: self.parse_knowledge_requirements(child_node, NODE_KNOWLEDGE_REQUIREMENT)?,
        authority_requirements: self.parse_authority_requirements(child_node, NODE_AUTHORITY_REQUIREMENT)?,
      };
      decision_items.push(DrgElement::Decision(decision));
    }
    Ok(decision_items)
  }

  /// Parses `businessKnowledgeModel` nodes.
  fn parse_business_knowledge_model_nodes(&mut self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut parsed_items = vec![];
    for ref child_node in node.children().filter(name_eq(NODE_BUSINESS_KNOWLEDGE_MODEL)) {
      let name = required_name(child_node)?;
      let feel_name = required_feel_name(child_node)?;
      let variable = self
        .parse_opt_information_item_child(child_node, NODE_VARIABLE)?
        .unwrap_or(self.create_information_item(name.clone(), feel_name.clone())?);
      let business_knowledge_model = BusinessKnowledgeModel {
        namespace: self.namespace.clone(),
        model_name: self.model_name.clone(),
        name,
        feel_name,
        id: optional_id(child_node),
        description: optional_description(child_node),
        label: optional_label(child_node),
        extension_elements: self.parse_extension_elements(child_node),
        extension_attributes: self.parse_extension_attributes(child_node),
        variable,
        encapsulated_logic: self.parse_function_definition_child(child_node, NODE_ENCAPSULATED_LOGIC)?,
        knowledge_requirements: self.parse_knowledge_requirements(child_node, NODE_KNOWLEDGE_REQUIREMENT)?,
        authority_requirements: self.parse_authority_requirements(child_node, NODE_AUTHORITY_REQUIREMENT)?,
      };
      parsed_items.push(DrgElement::BusinessKnowledgeModel(business_knowledge_model));
    }
    Ok(parsed_items)
  }

  /// Parses decision services.
  fn parse_decision_services(&self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut drg_elements = vec![];
    for ref child_node in node.children().filter(name_eq(NODE_DECISION_SERVICE)) {
      let name = required_name(child_node)?;
      let feel_name = required_feel_name(child_node)?;
      let variable = self
        .parse_opt_information_item_child(child_node, NODE_VARIABLE)?
        .unwrap_or(self.create_information_item(name.clone(), feel_name.clone())?);
      let decision_service = DecisionService {
        namespace: self.namespace.clone(),
        model_name: self.model_name.clone(),
        name,
        feel_name,
        id: optional_id(child_node),
        description: optional_description(child_node),
        label: optional_label(child_node),
        extension_elements: self.parse_extension_elements(child_node),
        extension_attributes: self.parse_extension_attributes(child_node),
        variable,
        output_decisions: self.required_hrefs_in_child_nodes(child_node, NODE_OUTPUT_DECISION)?,
        encapsulated_decisions: self.required_hrefs_in_child_nodes(child_node, NODE_ENCAPSULATED_DECISION)?,
        input_decisions: self.required_hrefs_in_child_nodes(child_node, NODE_INPUT_DECISION)?,
        input_data: self.required_hrefs_in_child_nodes(child_node, NODE_INPUT_DATA)?,
      };
      drg_elements.push(DrgElement::DecisionService(decision_service));
    }
    Ok(drg_elements)
  }

  /// Parses knowledge sources.
  fn parse_knowledge_sources(&mut self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut drg_elements = vec![];
    for ref child_node in node.children().filter(name_eq(NODE_KNOWLEDGE_SOURCE)) {
      let knowledge_source = KnowledgeSource {
        namespace: self.namespace.clone(),
        model_name: self.model_name.clone(),
        id: optional_id(child_node),
        description: optional_description(child_node),
        label: optional_label(child_node),
        extension_elements: self.parse_extension_elements(child_node),
        extension_attributes: self.parse_extension_attributes(child_node),
        name: required_name(child_node)?,
        feel_name: required_feel_name(child_node)?,
        authority_requirements: self.parse_authority_requirements(child_node, NODE_AUTHORITY_REQUIREMENT)?,
      };
      drg_elements.push(DrgElement::KnowledgeSource(knowledge_source));
    }
    Ok(drg_elements)
  }

  fn required_hrefs_in_child_nodes(&self, node: &Node, child_name: &str) -> Result<Vec<HRef>> {
    let mut hrefs = vec![];
    for ref child_node in node.children().filter(name_eq(child_name)) {
      let text = required_attribute(child_node, ATTR_HREF)?;
      let href = HRef::try_from(text.as_str())?;
      hrefs.push(href);
    }
    Ok(hrefs)
  }

  fn parse_function_definition_child(&self, node: &Node, child_name: &str) -> Result<Option<FunctionDefinition>> {
    if let Some(child_node) = node.children().find(name_eq(child_name)) {
      Ok(Some(self.parse_function_definition(&child_node)?))
    } else {
      Ok(None)
    }
  }

  fn parse_optional_function_definition(&self, node: &Node) -> Result<Option<FunctionDefinition>> {
    if let Some(child_node) = node.children().find(name_eq(NODE_FUNCTION_DEFINITION)) {
      Ok(Some(self.parse_function_definition(&child_node)?))
    } else {
      Ok(None)
    }
  }

  fn parse_function_definition(&self, node: &Node) -> Result<FunctionDefinition> {
    Ok(FunctionDefinition {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      formal_parameters: self.parse_vec_information_item_child(node, NODE_FORMAL_PARAMETER)?,
      body: self.parse_optional_child_expression_instance(node)?,
      kind: self.parse_function_kind(node)?,
    })
  }

  fn parse_function_kind(&self, node: &Node) -> Result<FunctionKind> {
    if let Some(function_kind_text) = optional_attribute(node, ATTR_KIND) {
      match function_kind_text.trim() {
        "FEEL" => Ok(FunctionKind::Feel),
        "Java" => Ok(FunctionKind::Java),
        "PMML" => Ok(FunctionKind::Pmml),
        other => Err(err_invalid_function_kind(other)),
      }
    } else {
      Ok(FunctionKind::Feel)
    }
  }

  /// Parses business context elements like performance indicators and organisational units.
  fn parse_business_context_elements(&self, node: &Node) -> Result<Vec<BusinessContextElementInstance>> {
    let mut business_context_elements = vec![];
    for ref child_node in node.children().filter(name_eq(NODE_PERFORMANCE_INDICATOR)) {
      let performance_indicator = PerformanceIndicator {
        namespace: self.namespace.clone(),
        model_name: self.model_name.clone(),
        id: optional_id(child_node),
        description: optional_description(child_node),
        label: optional_label(child_node),
        extension_elements: self.parse_extension_elements(child_node),
        extension_attributes: self.parse_extension_attributes(child_node),
        name: required_name(child_node)?,
        feel_name: required_feel_name(child_node)?,
        uri: optional_attribute(child_node, ATTR_URI),
        impacting_decisions: optional_children_required_href(child_node, NODE_IMPACTING_DECISION)?,
      };
      business_context_elements.push(BusinessContextElementInstance::PerformanceIndicator(performance_indicator));
    }
    for ref child_node in node.children().filter(name_eq(NODE_ORGANISATION_UNIT)) {
      let organisation_unit = OrganizationUnit {
        namespace: self.namespace.clone(),
        model_name: self.model_name.clone(),
        id: optional_id(child_node),
        description: optional_description(child_node),
        label: optional_label(child_node),
        extension_elements: self.parse_extension_elements(child_node),
        extension_attributes: self.parse_extension_attributes(child_node),
        name: required_name(child_node)?,
        feel_name: required_feel_name(child_node)?,
        uri: optional_attribute(child_node, ATTR_URI),
        decisions_made: optional_children_required_href(child_node, NODE_DECISION_MADE)?,
        decisions_owned: optional_children_required_href(child_node, NODE_DECISION_OWNED)?,
      };
      business_context_elements.push(BusinessContextElementInstance::OrganizationUnit(organisation_unit));
    }
    Ok(business_context_elements)
  }

  /// Parses a collection of [Imports](Import).
  fn parse_imports(&self, node: &Node) -> Result<Vec<Import>> {
    let mut imports = vec![];
    for ref child_node in node.children().filter(name_eq(NODE_IMPORT)) {
      let import = Import {
        namespace: required_uri(child_node, ATTR_NAMESPACE)?,
        model_name: self.model_name.clone(),
        id: optional_id(child_node),
        description: optional_description(child_node),
        label: optional_label(child_node),
        extension_elements: self.parse_extension_elements(child_node),
        extension_attributes: self.parse_extension_attributes(child_node),
        name: required_name(child_node)?,
        feel_name: required_feel_name(child_node)?,
        import_type: required_attribute(child_node, ATTR_IMPORT_TYPE)?,
        location_uri: optional_attribute(child_node, ATTR_LOCATION_URI),
      };
      imports.push(import);
    }
    Ok(imports)
  }

  /// Parses a required [InformationItem], contained in a child node having the specified name.
  fn parse_req_information_item_child(&self, node: &Node, child_name: &str) -> Result<InformationItem> {
    if let Some(child_node) = node.children().find(name_eq(child_name)) {
      self.parse_information_item(&child_node)
    } else {
      Err(err_xml_expected_mandatory_child_node(&node_name_pos(node), child_name))
    }
  }

  /// Parses an optional [InformationItem], contained in a child node having the specified name.
  fn parse_opt_information_item_child(&self, node: &Node, child_name: &str) -> Result<Option<InformationItem>> {
    Ok(if let Some(child_node) = node.children().find(name_eq(child_name)) {
      Some(self.parse_information_item(&child_node)?)
    } else {
      None
    })
  }

  /// Parses a collection of [InformationItem]s contained in multiple child nodes having the specified name.
  fn parse_vec_information_item_child(&self, node: &Node, child_name: &str) -> Result<Vec<InformationItem>> {
    let mut information_items = vec![];
    for child_node in node.children().filter(name_eq(child_name)) {
      information_items.push(self.parse_information_item(&child_node)?);
    }
    Ok(information_items)
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
      type_ref: optional_attribute(node, ATTR_TYPE_REF).unwrap_or(FEEL_TYPE_NAME_ANY.to_string()),
      feel_type: None,
    })
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

  /// Parses information requirements.
  fn parse_information_requirements(&mut self, node: &Node, child_name: &str) -> Result<Vec<InformationRequirement>> {
    let mut information_requirement_items = vec![];
    for child_node in node.children().filter(name_eq(child_name)) {
      information_requirement_items.push(self.parse_information_requirement(&child_node)?);
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
      required_decision: optional_child_required_href(node, NODE_REQUIRED_DECISION)?,
      required_input: optional_child_required_href(node, NODE_REQUIRED_INPUT)?,
    };
    Ok(req)
  }

  /// Parses knowledge requirements.
  fn parse_knowledge_requirements(&mut self, node: &Node, child_name: &str) -> Result<Vec<KnowledgeRequirement>> {
    let mut knowledge_requirement_items = vec![];
    for child_node in node.children().filter(name_eq(child_name)) {
      knowledge_requirement_items.push(self.parse_knowledge_requirement(&child_node)?);
    }
    Ok(knowledge_requirement_items)
  }

  /// Parses single knowledge requirement.
  fn parse_knowledge_requirement(&mut self, node: &Node) -> Result<KnowledgeRequirement> {
    let req = KnowledgeRequirement {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      required_knowledge: required_child_required_href(node, NODE_REQUIRED_KNOWLEDGE)?,
    };
    Ok(req)
  }

  /// Parses authority requirements.
  fn parse_authority_requirements(&mut self, node: &Node, child_name: &str) -> Result<Vec<AuthorityRequirement>> {
    let mut authority_requirement_items = vec![];
    for child_node in node.children().filter(name_eq(child_name)) {
      authority_requirement_items.push(self.parse_authority_requirement(&child_node)?);
    }
    Ok(authority_requirement_items)
  }

  /// Parses single authority requirement.
  fn parse_authority_requirement(&mut self, node: &Node) -> Result<AuthorityRequirement> {
    let req = AuthorityRequirement {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      required_authority: optional_child_required_href(node, NODE_REQUIRED_AUTHORITY)?,
      required_decision: optional_child_required_href(node, NODE_REQUIRED_DECISION)?,
      required_input: optional_child_required_href(node, NODE_REQUIRED_INPUT)?,
    };
    Ok(req)
  }

  fn parse_required_child_expression_instance(&self, node: &Node) -> Result<ExpressionInstance> {
    self.parse_optional_child_expression_instance(node)?.ok_or_else(err_required_expression_instance_is_missing)
  }

  fn parse_optional_child_expression_instance(&self, node: &Node) -> Result<Option<ExpressionInstance>> {
    if let Some(context) = self.parse_optional_context(node)? {
      return Ok(Some(ExpressionInstance::Context(Box::new(context))));
    }
    if let Some(decision_table) = self.parse_optional_decision_table(node)? {
      return Ok(Some(ExpressionInstance::DecisionTable(Box::new(decision_table))));
    }
    if let Some(function_definition) = self.parse_optional_function_definition(node)? {
      return Ok(Some(ExpressionInstance::FunctionDefinition(Box::new(function_definition))));
    }
    if let Some(invocation) = self.parse_optional_invocation(node)? {
      return Ok(Some(ExpressionInstance::Invocation(Box::new(invocation))));
    }
    if let Some(list) = self.parse_optional_list(node)? {
      return Ok(Some(ExpressionInstance::List(Box::new(list))));
    }
    if let Some(literal_expression) = self.parse_optional_literal_expression(node) {
      return Ok(Some(ExpressionInstance::LiteralExpression(Box::new(literal_expression))));
    }
    if let Some(relation) = self.parse_optional_relation(node)? {
      return Ok(Some(ExpressionInstance::Relation(Box::new(relation))));
    }
    if let Some(conditional) = self.parse_optional_conditional(node)? {
      return Ok(Some(ExpressionInstance::Conditional(Box::new(conditional))));
    }
    if let Some(filter) = self.parse_optional_filter(node)? {
      return Ok(Some(ExpressionInstance::Filter(Box::new(filter))));
    }
    if let Some(r#for) = self.parse_optional_for(node)? {
      return Ok(Some(ExpressionInstance::For(Box::new(r#for))));
    }
    if let Some(every) = self.parse_optional_every(node)? {
      return Ok(Some(ExpressionInstance::Every(Box::new(every))));
    }
    if let Some(some) = self.parse_optional_some(node)? {
      return Ok(Some(ExpressionInstance::Some(Box::new(some))));
    }
    Ok(None)
  }

  fn parse_required_expression_instance(&self, node: &Node) -> Result<ExpressionInstance> {
    match node.tag_name().name() {
      NODE_CONTEXT => {
        let context = self.parse_context(node)?;
        Ok(ExpressionInstance::Context(Box::new(context)))
      }
      NODE_DECISION_TABLE => {
        let decision_table = self.parse_decision_table(node)?;
        Ok(ExpressionInstance::DecisionTable(Box::new(decision_table)))
      }
      NODE_FUNCTION_DEFINITION => {
        let function_definition = self.parse_function_definition(node)?;
        Ok(ExpressionInstance::FunctionDefinition(Box::new(function_definition)))
      }
      NODE_INVOCATION => {
        let invocation = self.parse_invocation(node)?;
        Ok(ExpressionInstance::Invocation(Box::new(invocation)))
      }
      NODE_LIST => {
        let list = self.parse_list(node)?;
        Ok(ExpressionInstance::List(Box::new(list)))
      }
      NODE_LITERAL_EXPRESSION => {
        let literal_expression = self.parse_literal_expression(node);
        Ok(ExpressionInstance::LiteralExpression(Box::new(literal_expression)))
      }
      NODE_RELATION => {
        let relation = self.parse_relation(node)?;
        Ok(ExpressionInstance::Relation(Box::new(relation)))
      }
      NODE_CONDITIONAL => {
        let conditional = self.parse_conditional(node)?;
        Ok(ExpressionInstance::Conditional(Box::new(conditional)))
      }
      _ => Err(err_required_expression_instance_is_missing()),
    }
  }

  fn parse_optional_decision_table(&self, node: &Node) -> Result<Option<DecisionTable>> {
    if let Some(ref child_node) = node.children().find(name_eq(NODE_DECISION_TABLE)) {
      return Ok(Some(self.parse_decision_table(child_node)?));
    }
    Ok(None)
  }

  fn parse_decision_table(&self, node: &Node) -> Result<DecisionTable> {
    Ok(DecisionTable {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      information_item_name: None,
      input_clauses: self.parse_decision_table_inputs(node)?,
      output_clauses: self.parse_decision_table_outputs(node)?,
      annotations: vec![], // TODO implement parsing annotations
      rules: self.parse_decision_table_rules(node)?,
      hit_policy: self.parse_hit_policy_attribute(node)?,
      aggregation: None,
      preferred_orientation: self.parse_preferred_orientation_attribute(node)?,
      output_label: optional_attribute(node, ATTR_OUTPUT_LABEL),
    })
  }

  fn parse_decision_table_inputs(&self, node: &Node) -> Result<Vec<InputClause>> {
    let mut input_clauses = vec![];
    for ref child_node in node.children().filter(name_eq(NODE_INPUT)) {
      input_clauses.push(self.parse_decision_table_input(child_node)?);
    }
    Ok(input_clauses)
  }

  fn parse_decision_table_input(&self, node: &Node) -> Result<InputClause> {
    let input_expression = if let Ok(ref child_node) = required_child(node, NODE_INPUT_EXPRESSION) {
      req_child_req_content(child_node, NODE_TEXT)?
    } else {
      return Err(err_required_input_expression_is_missing());
    };
    let input_values = if let Some(ref child_node) = optional_child(node, NODE_INPUT_VALUES) {
      optional_child_required_content(child_node, NODE_TEXT)?
    } else {
      None
    };
    Ok(InputClause {
      input_expression,
      allowed_input_values: input_values,
    })
  }

  fn parse_decision_table_outputs(&self, node: &Node) -> Result<Vec<OutputClause>> {
    let mut output_clauses = vec![];
    for ref child_node in node.children().filter(name_eq(NODE_OUTPUT)) {
      output_clauses.push(self.parse_decision_table_output(child_node)?);
    }
    Ok(output_clauses)
  }

  fn parse_decision_table_output(&self, node: &Node) -> Result<OutputClause> {
    let output_values = if let Some(ref child_node) = optional_child(node, NODE_OUTPUT_VALUES) {
      optional_child_required_content(child_node, NODE_TEXT)?
    } else {
      None
    };
    let default_output_entry = if let Some(ref child_node) = optional_child(node, NODE_DEFAULT_OUTPUT_ENTRY) {
      optional_child_required_content(child_node, NODE_TEXT)?
    } else {
      None
    };
    Ok(OutputClause {
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      name: optional_attribute(node, ATTR_NAME),
      allowed_output_values: output_values,
      default_output_entry,
    })
  }

  fn parse_decision_table_rules(&self, node: &Node) -> Result<Vec<DecisionRule>> {
    let mut rules = vec![];
    for ref child_node in node.children().filter(name_eq(NODE_RULE)) {
      rules.push(self.parse_decision_table_rule(child_node)?);
    }
    Ok(rules)
  }

  fn parse_decision_table_rule(&self, node: &Node) -> Result<DecisionRule> {
    Ok(DecisionRule {
      input_entries: self.parse_decision_table_input_entries(node)?,
      output_entries: self.parse_decision_table_output_entries(node)?,
      annotation_entries: vec![],
    })
  }

  fn parse_decision_table_input_entries(&self, node: &Node) -> Result<Vec<InputEntry>> {
    let mut input_entries = vec![];
    for ref child_node in node.children().filter(name_eq(NODE_INPUT_ENTRY)) {
      input_entries.push(self.parse_decision_table_input_entry(child_node)?);
    }
    Ok(input_entries)
  }

  /// Parses the content of the input entry in decision table.
  ///
  /// The content of the input entry is placed in `text` element which is required,
  /// but the string inside this element may be empty. Some tools, like Camunda,
  /// generate decision tables with empty string here, although in the graphical
  /// interface there is `-` character representing FEEL `irrelevant` operator.
  ///
  /// So, long story short, when the string in `text` element is empty,
  /// it is replaced by `irrelevant` operator.
  fn parse_decision_table_input_entry(&self, node: &Node) -> Result<InputEntry> {
    Ok(InputEntry {
      text: req_child_opt_content(node, NODE_TEXT)?.unwrap_or("-".to_string()),
    })
  }

  fn parse_decision_table_output_entries(&self, node: &Node) -> Result<Vec<OutputEntry>> {
    let mut output_entries = vec![];
    for ref child_node in node.children().filter(name_eq(NODE_OUTPUT_ENTRY)) {
      output_entries.push(self.parse_decision_table_output_entry(child_node)?);
    }
    Ok(output_entries)
  }

  fn parse_decision_table_output_entry(&self, node: &Node) -> Result<OutputEntry> {
    Ok(OutputEntry {
      text: req_child_req_content(node, NODE_TEXT)?,
    })
  }

  fn parse_optional_context(&self, node: &Node) -> Result<Option<Context>> {
    if let Some(ref child_node) = node.children().find(name_eq(NODE_CONTEXT)) {
      return Ok(Some(self.parse_context(child_node)?));
    }
    Ok(None)
  }

  fn parse_context(&self, node: &Node) -> Result<Context> {
    Ok(Context {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      context_entries: self.parse_context_entries(node)?,
    })
  }

  fn parse_context_entries(&self, node: &Node) -> Result<Vec<ContextEntry>> {
    let mut context_entries = vec![];
    for ref child_node in node.children().filter(name_eq(NODE_CONTEXT_ENTRY)) {
      context_entries.push(ContextEntry {
        variable: self.parse_opt_information_item_child(child_node, NODE_VARIABLE)?,
        value: self.parse_required_child_expression_instance(child_node)?,
      });
    }
    Ok(context_entries)
  }

  fn parse_optional_invocation(&self, node: &Node) -> Result<Option<Invocation>> {
    if let Some(ref child_node) = node.children().find(name_eq(NODE_INVOCATION)) {
      return Ok(Some(self.parse_invocation(child_node)?));
    }
    Ok(None)
  }

  fn parse_invocation(&self, node: &Node) -> Result<Invocation> {
    Ok(Invocation {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      called_function: self.parse_required_child_expression_instance(node)?,
      bindings: self.parse_bindings(node)?,
    })
  }

  fn parse_bindings(&self, node: &Node) -> Result<Vec<Binding>> {
    let mut bindings = vec![];
    for ref child_node in node.children().filter(name_eq(NODE_BINDING)) {
      bindings.push(Binding {
        parameter: self.parse_req_information_item_child(child_node, NODE_PARAMETER)?,
        binding_formula: self.parse_optional_child_expression_instance(child_node)?,
      });
    }
    Ok(bindings)
  }

  /// Searches for the first node named 'list' among children of the specified `node`.
  /// When such node is found, then parses a list and returns it, otherwise returns [None].
  fn parse_optional_list(&self, node: &Node) -> Result<Option<List>> {
    if let Some(ref child_node) = node.children().find(name_eq(NODE_LIST)) {
      return Ok(Some(self.parse_list(child_node)?));
    }
    Ok(None)
  }

  /// Parses [List] directly from the specified node.
  fn parse_list(&self, node: &Node) -> Result<List> {
    let mut elements = vec![];
    for child_node in node.children().filter(|n| {
      let name = n.tag_name().name();
      matches!(
        name,
        NODE_CONTEXT | NODE_DECISION_TABLE | NODE_FUNCTION_DEFINITION | NODE_INVOCATION | NODE_LIST | NODE_LITERAL_EXPRESSION | NODE_RELATION
      )
    }) {
      elements.push(self.parse_required_expression_instance(&child_node)?);
    }
    Ok(List {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      elements,
    })
  }

  /// Searches for the first node named 'literalExpression' among children of the specified `node`.
  /// When such node is found, then parses literal expression and returns it, otherwise returns [None].
  fn parse_optional_literal_expression(&self, node: &Node) -> Option<LiteralExpression> {
    if let Some(ref child_node) = node.children().find(name_eq(NODE_LITERAL_EXPRESSION)) {
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
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      text: optional_child_optional_content(node, NODE_TEXT),
      expression_language: optional_attribute(node, ATTR_EXPRESSION_LANGUAGE),
      imported_values: None,
    }
  }

  fn parse_optional_relation(&self, node: &Node) -> Result<Option<Relation>> {
    if let Some(ref child_node) = node.children().find(name_eq(NODE_RELATION)) {
      return Ok(Some(self.parse_relation(child_node)?));
    }
    Ok(None)
  }

  fn parse_relation(&self, node: &Node) -> Result<Relation> {
    let mut columns = vec![];
    for ref column_node in node.children().filter(name_eq(NODE_COLUMN)) {
      columns.push(self.parse_information_item(column_node)?);
    }
    let mut rows = vec![];
    for ref row_node in node.children().filter(name_eq(NODE_ROW)) {
      let mut elements = vec![];
      for ref expression_instance_node in row_node.children() {
        if expression_instance_node.tag_name().name() == NODE_LITERAL_EXPRESSION {
          let literal_expression = self.parse_literal_expression(expression_instance_node);
          elements.push(ExpressionInstance::LiteralExpression(Box::new(literal_expression)));
        }
      }
      if elements.len() != columns.len() {
        return Err(err_number_of_elements_in_row_differs_from_number_of_columns());
      }
      rows.push(List {
        namespace: self.namespace.clone(),
        model_name: self.model_name.clone(),
        id: optional_id(row_node),
        description: optional_description(row_node),
        label: optional_label(row_node),
        extension_elements: self.parse_extension_elements(row_node),
        extension_attributes: self.parse_extension_attributes(row_node),
        type_ref: optional_attribute(row_node, ATTR_TYPE_REF),
        elements,
      });
    }
    Ok(Relation {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      rows,
      columns,
    })
  }

  fn parse_optional_conditional(&self, node: &Node) -> Result<Option<Conditional>> {
    if let Some(ref child_node) = node.children().find(name_eq(NODE_CONDITIONAL)) {
      return Ok(Some(self.parse_conditional(child_node)?));
    }
    Ok(None)
  }

  fn parse_conditional(&self, node: &Node) -> Result<Conditional> {
    let node_if = required_child(node, NODE_IF)?;
    let node_then = required_child(node, NODE_THEN)?;
    let node_else = required_child(node, NODE_ELSE)?;
    Ok(Conditional {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      if_expression: self.parse_required_child_expression(&node_if)?,
      then_expression: self.parse_required_child_expression(&node_then)?,
      else_expression: self.parse_required_child_expression(&node_else)?,
    })
  }

  fn parse_optional_filter(&self, node: &Node) -> Result<Option<Filter>> {
    if let Some(ref child_node) = node.children().find(name_eq(NODE_FILTER)) {
      return Ok(Some(self.parse_filter(child_node)?));
    }
    Ok(None)
  }

  fn parse_filter(&self, node: &Node) -> Result<Filter> {
    let node_in = required_child(node, NODE_IN)?;
    let node_match = required_child(node, NODE_MATCH)?;
    Ok(Filter {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      in_expression: self.parse_required_child_expression(&node_in)?,
      match_expression: self.parse_required_child_expression(&node_match)?,
    })
  }

  fn parse_optional_for(&self, node: &Node) -> Result<Option<For>> {
    if let Some(ref child_node) = node.children().find(name_eq(NODE_FOR)) {
      return Ok(Some(self.parse_for(child_node)?));
    }
    Ok(None)
  }

  fn parse_for(&self, node: &Node) -> Result<For> {
    let node_in = required_child(node, NODE_IN)?;
    let node_return = required_child(node, NODE_RETURN)?;
    Ok(For {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      iterator_variable: required_attribute(node, ATTR_ITERATOR_VARIABLE)?,
      in_expression: self.parse_required_typed_child_expression(&node_in)?,
      return_expression: self.parse_required_child_expression(&node_return)?,
    })
  }

  fn parse_optional_every(&self, node: &Node) -> Result<Option<Every>> {
    if let Some(ref child_node) = node.children().find(name_eq(NODE_EVERY)) {
      return Ok(Some(self.parse_every(child_node)?));
    }
    Ok(None)
  }

  fn parse_every(&self, node: &Node) -> Result<Every> {
    let node_in = required_child(node, NODE_IN)?;
    let node_satisfies = required_child(node, NODE_SATISFIES)?;
    Ok(Every {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      iterator_variable: required_attribute(node, ATTR_ITERATOR_VARIABLE)?,
      in_expression: self.parse_required_typed_child_expression(&node_in)?,
      satisfies_expression: self.parse_required_child_expression(&node_satisfies)?,
    })
  }

  fn parse_optional_some(&self, node: &Node) -> Result<Option<Some>> {
    if let Some(ref child_node) = node.children().find(name_eq(NODE_SOME)) {
      return Ok(Some(self.parse_some(child_node)?));
    }
    Ok(None)
  }

  fn parse_some(&self, node: &Node) -> Result<Some> {
    let node_in = required_child(node, NODE_IN)?;
    let node_satisfies = required_child(node, NODE_SATISFIES)?;
    Ok(Some {
      namespace: self.namespace.clone(),
      model_name: self.model_name.clone(),
      id: optional_id(node),
      description: optional_description(node),
      label: optional_label(node),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      iterator_variable: required_attribute(node, ATTR_ITERATOR_VARIABLE)?,
      in_expression: self.parse_required_typed_child_expression(&node_in)?,
      satisfies_expression: self.parse_required_child_expression(&node_satisfies)?,
    })
  }

  fn parse_required_child_expression(&self, node: &Node) -> Result<ChildExpression> {
    let child_node = first_child_element(node)?;
    Ok(ChildExpression {
      id: optional_id(node),
      value: self.parse_required_expression_instance(&child_node)?,
    })
  }

  fn parse_required_typed_child_expression(&self, node: &Node) -> Result<TypedChildExpression> {
    let child_node = first_child_element(node)?;
    Ok(TypedChildExpression {
      id: optional_id(node),
      value: self.parse_required_expression_instance(&child_node)?,
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
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

  /// Returns boolean value of the specified attribute.
  fn parse_boolean_attribute(&self, node: &Node, attr_name: &str, default_value: bool) -> bool {
    if let Some(attr_value) = node.attribute(attr_name) {
      attr_value == "true"
    } else {
      default_value
    }
  }

  /// Returns the value of the hit policy attribute.
  fn parse_hit_policy_attribute(&self, node: &Node) -> Result<HitPolicy> {
    if let Some(hit_policy_text) = node.attribute(ATTR_HIT_POLICY) {
      match hit_policy_text.trim() {
        "UNIQUE" => Ok(HitPolicy::Unique),
        "ANY" => Ok(HitPolicy::Any),
        "PRIORITY" => Ok(HitPolicy::Priority),
        "FIRST" => Ok(HitPolicy::First),
        "RULE ORDER" => Ok(HitPolicy::RuleOrder),
        "OUTPUT ORDER" => Ok(HitPolicy::OutputOrder),
        "COLLECT" => Ok(HitPolicy::Collect(self.parse_aggregation_attribute(node)?)),
        other => Err(err_invalid_hit_policy(other)),
      }
    } else {
      Ok(HitPolicy::Unique)
    }
  }

  /// Returns the value of the aggregation attribute.
  fn parse_aggregation_attribute(&self, node: &Node) -> Result<BuiltinAggregator> {
    if let Some(aggregation_text) = node.attribute(ATTR_AGGREGATION) {
      match aggregation_text.trim() {
        "COUNT" => Ok(BuiltinAggregator::Count),
        "SUM" => Ok(BuiltinAggregator::Sum),
        "MIN" => Ok(BuiltinAggregator::Min),
        "MAX" => Ok(BuiltinAggregator::Max),
        other => Err(err_invalid_aggregation(other)),
      }
    } else {
      Ok(BuiltinAggregator::List)
    }
  }

  /// Returns the value of the preferred decision table orientation attribute.
  fn parse_preferred_orientation_attribute(&self, node: &Node) -> Result<DecisionTableOrientation> {
    if let Some(attr_value) = node.attribute(ATTR_PREFERRED_ORIENTATION) {
      DecisionTableOrientation::try_from(attr_value)
    } else {
      Ok(DecisionTableOrientation::RuleAsRow)
    }
  }

  /// Parse DMNDI part of the diagram definitions.
  fn parse_dmndi(&self, node: &Node) -> Result<Option<Dmndi>> {
    if let Some(child_node) = node.children().find(name_eq(NODE_DMNDI)) {
      let dmndi = Dmndi {
        styles: self.parse_styles(&child_node)?,
        diagrams: self.parse_diagrams(&child_node)?,
      };
      return Ok(Some(dmndi));
    }
    Ok(None)
  }

  /// Parses shared styles defined in diagram.
  fn parse_styles(&self, node: &Node) -> Result<Vec<DmnStyle>> {
    let mut styles = vec![];
    for child_node in node.children().filter(name_eq(NODE_DMNDI_STYLE)) {
      styles.push(self.parse_style(&child_node)?);
    }
    Ok(styles)
  }

  /// Parses single style.
  fn parse_style(&self, node: &Node) -> Result<DmnStyle> {
    Ok(DmnStyle {
      id: required_attribute(node, ATTR_ID)?,
      fill_color: self.parse_optional_color(node, NODE_DMNDI_FILL_COLOR)?,
      stroke_color: self.parse_optional_color(node, NODE_DMNDI_STROKE_COLOR)?,
      font_color: self.parse_optional_color(node, NODE_DMNDI_FONT_COLOR)?,
      font_family: optional_attribute(node, ATTR_FONT_FAMILY),
      font_size: optional_double(node, ATTR_FONT_SIZE),
      font_italic: optional_bool(node, ATTR_FONT_ITALIC),
      font_bold: optional_bool(node, ATTR_FONT_BOLD),
      font_underline: optional_bool(node, ATTR_FONT_UNDERLINE),
      font_strike_through: optional_bool(node, ATTR_FONT_STRIKE_THROUGH),
      label_horizontal_alignment: self.parse_alignment_kind(node, NODE_DMNDI_LABEL_HORIZONTAL_ALIGNMENT),
      label_vertical_alignment: self.parse_alignment_kind(node, NODE_DMNDI_LABEL_VERTICAL_ALIGNMENT),
    })
  }

  /// Parses the color definition.
  fn parse_optional_color(&self, node: &Node, child_name: &str) -> Result<Option<DcColor>> {
    if let Some(color_node) = node.children().find(name_eq(child_name)) {
      Ok(Some(DcColor {
        red: required_color_part(&color_node, ATTR_RED)?,
        green: required_color_part(&color_node, ATTR_GREEN)?,
        blue: required_color_part(&color_node, ATTR_BLUE)?,
      }))
    } else {
      Ok(None)
    }
  }

  /// Parses the alignment.
  fn parse_alignment_kind(&self, node: &Node, attr_name: &str) -> Option<DcAlignmentKind> {
    match node.attribute(attr_name) {
      Some("start") => Some(DcAlignmentKind::Start),
      Some("center") => Some(DcAlignmentKind::Center),
      Some("end") => Some(DcAlignmentKind::End),
      _ => None,
    }
  }

  /// Parses diagrams defined in [Dmndi].
  fn parse_diagrams(&self, node: &Node) -> Result<Vec<DmnDiagram>> {
    let mut diagrams = vec![];
    for child_node in node.children().filter(name_eq(NODE_DMNDI_DMN_DIAGRAM)) {
      diagrams.push(self.parse_diagram(&child_node)?);
    }
    Ok(diagrams)
  }

  /// Parses a single diagram.
  fn parse_diagram(&self, node: &Node) -> Result<DmnDiagram> {
    Ok(DmnDiagram {
      id: optional_attribute(node, ATTR_ID),
      name: optional_attribute(node, ATTR_NAME),
      documentation: "".to_string(),
      resolution: optional_double(node, ATTR_RESOLUTION).unwrap_or(300.0),
      diagram_elements: self.parse_diagram_elements(node)?,
      shared_style: optional_attribute(node, ATTR_SHARED_STYLE),
      local_style: None, //TODO Implement or remove when the official specification is ready.
      size: self.parse_dimension(node)?,
    })
  }

  /// Parses dimension.
  fn parse_dimension(&self, size_node: &Node) -> Result<Option<DcDimension>> {
    if let Some(node) = size_node.children().find(name_eq(NODE_DMNDI_SIZE)) {
      Ok(Some(DcDimension {
        width: required_double(&node, ATTR_WIDTH)?,
        height: required_double(&node, ATTR_HEIGHT)?,
      }))
    } else {
      Ok(None)
    }
  }

  /// Parses diagram elements
  fn parse_diagram_elements(&self, size_node: &Node) -> Result<Vec<DmnDiagramElement>> {
    let mut diagram_element = vec![];
    for child_node in size_node.children().filter(name_eq(NODE_DMNDI_DMN_SHAPE)) {
      diagram_element.push(self.parse_shape(&child_node)?);
    }
    for child_node in size_node.children().filter(name_eq(NODE_DMNDI_DMN_EDGE)) {
      diagram_element.push(self.parse_edge(&child_node)?);
    }
    Ok(diagram_element)
  }

  /// Parses shape.
  fn parse_shape(&self, node: &Node) -> Result<DmnDiagramElement> {
    Ok(DmnDiagramElement::DmnShape(DmnShape {
      id: optional_attribute(node, ATTR_ID),
      bounds: self.parse_bounds(node)?,
      dmn_element_ref: optional_attribute(node, ATTR_DMN_ELEMENT_REF),
      is_listed_input_data: false,
      decision_service_divider_line: self.parse_divider_line(node)?,
      is_collapsed: optional_bool(node, ATTR_IS_COLLAPSED).unwrap_or(false),
      shared_style: optional_attribute(node, ATTR_SHARED_STYLE),
      local_style: None, //TODO Implement or remove when the official specification is ready.
      label: self.parse_label(node)?,
    }))
  }

  /// Parses bounds.
  fn parse_bounds(&self, node: &Node) -> Result<DcBounds> {
    match self.parse_optional_bounds(node) {
      Ok(Some(n)) => Ok(n),
      _ => Err(err_required_child_node_is_missing(node.tag_name().name(), NODE_DMNDI_BOUNDS)),
    }
  }

  /// Parses bounds.
  fn parse_optional_bounds(&self, node: &Node) -> Result<Option<DcBounds>> {
    if let Some(child_node) = node.children().find(name_eq(NODE_DMNDI_BOUNDS)) {
      Ok(Some(DcBounds {
        x: required_double(&child_node, ATTR_X)?,
        y: required_double(&child_node, ATTR_Y)?,
        width: required_double(&child_node, ATTR_WIDTH)?,
        height: required_double(&child_node, ATTR_HEIGHT)?,
      }))
    } else {
      Ok(None)
    }
  }

  /// Parses divider line.
  fn parse_divider_line(&self, node: &Node) -> Result<Option<DmnDecisionServiceDividerLine>> {
    if let Some(child_node) = node.children().find(name_eq(NODE_DMNDI_DECISION_SERVICE_DIVIDER_LINE)) {
      Ok(Some(DmnDecisionServiceDividerLine {
        id: optional_attribute(&child_node, ATTR_ID),
        way_points: self.parse_way_points(&child_node)?,
        shared_style: optional_attribute(node, ATTR_SHARED_STYLE),
        local_style: None, //TODO Implement or remove when the official specification is ready.
      }))
    } else {
      Ok(None)
    }
  }

  /// Parses edge.
  fn parse_edge(&self, node: &Node) -> Result<DmnDiagramElement> {
    Ok(DmnDiagramElement::DmnEdge(DmnEdge {
      id: optional_attribute(node, ATTR_ID),
      way_points: self.parse_way_points(node)?,
      dmn_element_ref: optional_attribute(node, ATTR_DMN_ELEMENT_REF),
      source_element: None,
      target_element: None,
      shared_style: optional_attribute(node, ATTR_SHARED_STYLE),
      local_style: None, //TODO Implement or remove when the official specification is ready.
      label: self.parse_label(node)?,
    }))
  }

  /// Parses way points.
  fn parse_way_points(&self, node: &Node) -> Result<Vec<DcPoint>> {
    let mut way_points = vec![];
    for child_node in node.children().filter(name_eq(NODE_DMNDI_WAYPOINT)) {
      way_points.push(self.parse_point(&child_node)?)
    }
    Ok(way_points)
  }

  /// Parses the point coordinates.
  fn parse_point(&self, node: &Node) -> Result<DcPoint> {
    Ok(DcPoint {
      x: required_double(node, ATTR_X)?,
      y: required_double(node, ATTR_Y)?,
    })
  }

  /// Parses the label of the element.
  fn parse_label(&self, node: &Node) -> Result<Option<DmnLabel>> {
    if let Some(child_node) = node.children().find(name_eq(NODE_DMNDI_LABEL)) {
      Ok(Some(DmnLabel {
        bounds: self.parse_optional_bounds(&child_node)?,
        text: optional_child_optional_content(&child_node, ATTR_LABEL_TEXT),
        shared_style: optional_attribute(&child_node, ATTR_SHARED_STYLE),
      }))
    } else {
      Ok(None)
    }
  }
}

/// Returns the first child element of the specified node.
fn first_child_element<'a>(node: &'a Node) -> Result<Node<'a, 'a>> {
  node
    .children()
    .filter(|n| matches!(n.node_type(), NodeType::Element))
    .take(1)
    .next()
    .ok_or(err_node_has_no_children(node.tag_name().name()))
}

/// Returns required name attribute for specified node.
fn required_name(node: &Node) -> Result<String> {
  required_attribute(node, ATTR_NAME)
}

/// Returns optional identifier provided in model or generates a new one.
fn optional_id(node: &Node) -> DmnId {
  optional_attribute(node, ATTR_ID).map(DmnId::Provided).unwrap_or(DmnId::Generated(gen_id()))
}

/// Returns FEEL name for specified node.
fn required_feel_name(node: &Node) -> Result<Name> {
  let input = required_name(node)?;
  Ok(dsntk_feel_parser::parse_longest_name(&input).unwrap_or(input.into()))
}

/// Returns the required `href` attribute.
pub fn required_href(node: &Node) -> Result<HRef> {
  HRef::try_from(required_attribute(node, ATTR_HREF)?.as_str())
}

/// Returns the required URI attribute.
pub fn required_uri(node: &Node, attr_name: &str) -> Result<Uri> {
  to_uri(required_attribute(node, attr_name)?.as_str())
}

/// Returns an optional URI attribute.
pub fn optional_uri(node: &Node, attr_name: &str) -> Result<Option<Uri>> {
  if let Some(value) = optional_attribute(node, attr_name) {
    Ok(Some(to_uri(value.as_str())?))
  } else {
    Ok(None)
  }
}

/// Returns the required `href` attribute taken from required child node.
pub fn required_child_required_href(node: &Node, child_name: &str) -> Result<HRef> {
  let child_node = required_child(node, child_name)?;
  HRef::try_from(required_attribute(&child_node, ATTR_HREF)?.as_str())
}

/// Returns the required `href` attribute of the optional child node.
pub fn optional_child_required_href(node: &Node, child_name: &str) -> Result<Option<HRef>> {
  if let Some(child_node) = optional_child(node, child_name) {
    Ok(Some(HRef::try_from(required_attribute(&child_node, ATTR_HREF)?.as_str())?))
  } else {
    Ok(None)
  }
}

/// Returns required `href` attributes collected from optional child nodes.
pub fn optional_children_required_href(node: &Node, child_name: &str) -> Result<Vec<HRef>> {
  let mut hrefs = vec![];
  for ref child_node in node.children().filter(name_eq(child_name)) {
    hrefs.push(required_href(child_node)?);
  }
  Ok(hrefs)
}
