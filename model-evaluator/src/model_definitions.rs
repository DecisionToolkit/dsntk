//! # Model definitions needed for building model evaluator

use crate::errors::err_invalid_item_definition_type;
use crate::type_ref::type_ref_to_feel_type;
use dsntk_common::{HRef, Result};
use dsntk_feel::{Name, QualifiedName};
use dsntk_model::*;
use std::collections::HashMap;
use std::fmt;

/// Types of invocables in DMN model.
#[derive(Clone)]
pub enum InvocableType {
  /// Decision invocable.
  Decision(
    /// A key uniquely identifying a decision, see [DefKey] for details.
    DefKey,
  ),
  /// Business knowledge model invocable.
  BusinessKnowledgeModel(
    /// A key uniquely identifying a business knowledge model, see [DefKey] for details.
    DefKey,
    /// Name of the output variable.
    Name,
  ),
  /// Decision service invocable.
  DecisionService(
    /// A key uniquely identifying a decision service, see [DefKey] for details.
    DefKey,
  ),
}

#[derive(Default)]
pub struct Invocables {
  /// (model namespace, model name, invocable name) -> invocable type
  items: HashMap<(String, String, String), InvocableType>,
}

impl Invocables {
  pub fn add_decision(&mut self, namespace: String, model_name: String, invocable_name: String, def_key: DefKey) {
    let invocable_type = InvocableType::Decision(def_key);
    self.items.insert((namespace, model_name, invocable_name), invocable_type);
  }

  pub fn add_bkm(&mut self, namespace: String, model_name: String, invocable_name: String, def_key: DefKey, output_variable_name: Name) {
    let invocable_type = InvocableType::BusinessKnowledgeModel(def_key, output_variable_name);
    self.items.insert((namespace, model_name, invocable_name), invocable_type);
  }

  pub fn add_decision_service(&mut self, namespace: String, model_name: String, invocable_name: String, def_key: DefKey) {
    let invocable_type = InvocableType::DecisionService(def_key);
    self.items.insert((namespace, model_name, invocable_name), invocable_type);
  }

  pub fn by_name(&self, namespace: &str, model_name: &str, invocable_name: &str) -> Option<&InvocableType> {
    self.items.get(&(namespace.to_string(), model_name.to_string(), invocable_name.to_string()))
  }

  pub fn list(&self) -> Vec<(String, String, String)> {
    let mut items = vec![];
    for (model_namespace, model_name, invocable_name) in self.items.keys().cloned() {
      items.push((model_namespace, model_name, invocable_name));
    }
    items.sort();
    items
  }

  pub fn len(&self) -> usize {
    self.items.len()
  }
}

/// The key in hash maps for indexing definition artefacts
/// by model namespace and artefact unique identifier.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DefKey {
  /// Model namespace.
  namespace: String,
  /// Artefact unique identifier.
  id: String,
}

impl DefKey {
  /// Creates new key based on namespace and identifier.
  pub fn new(namespace: &str, id: &str) -> Self {
    Self {
      namespace: namespace.to_string(),
      id: id.to_string(),
    }
  }
}

impl fmt::Display for DefKey {
  /// Converts [DefKey] to string.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}#{}", self.namespace, self.id)
  }
}

impl From<&DefHRef> for DefKey {
  /// Converts [DefKey] from reference to [DefHRef].
  fn from(value: &DefHRef) -> Self {
    Self::new(value.namespace(), value.id())
  }
}

impl DefKey {
  pub fn namespace(&self) -> &str {
    &self.namespace
  }
  pub fn id(&self) -> &str {
    &self.id
  }
}

/// Information item definition (variable properties).
pub struct DefInformationItem {
  /// Variable namespace.
  namespace: String,
  /// Variable name.
  name: Name,
  /// Variable type reference.
  type_ref: String,
}

impl DefInformationItem {
  /// Creates [DefInformationItem] from [InformationItem].
  pub fn new(information_item: &InformationItem, imports: &[DefImport]) -> Self {
    let type_ref_name = Name::from(information_item.type_ref().clone());
    let qname = QualifiedName::from(type_ref_name);
    if qname.len() == 2 {
      // type reference is prefixed with the import name
      let import_name = qname.first().unwrap(); // unwrap is safe
      let namespace = get_import_namespace(import_name, imports).unwrap_or(information_item.namespace().to_string());
      let name = information_item.feel_name().clone();
      let type_ref = qname.get(1).unwrap().to_string(); // unwrap is safe
      Self { namespace, name, type_ref }
    } else {
      // type reference has no import prefix
      let namespace = information_item.namespace().to_string();
      let type_ref = information_item.type_ref().clone();
      let name = information_item.feel_name().clone();
      Self { namespace, name, type_ref }
    }
  }
}

impl DefInformationItem {
  /// Returns information item's namespace.
  pub fn namespace(&self) -> &str {
    &self.namespace
  }

  /// Returns information item's name.
  pub fn name(&self) -> &Name {
    &self.name
  }

  /// Returns a reference to optional type reference.
  pub fn type_ref(&self) -> &String {
    &self.type_ref
  }
}

pub struct DefInputData {
  namespace: String,
  id: String,
  name: String,
  variable: DefInformationItem,
}

impl DefInputData {
  /// Creates [DefInputData] from [InputData].
  pub fn new(input_data: &InputData, imports: &[DefImport]) -> Self {
    Self {
      namespace: input_data.namespace().to_string(),
      id: input_data.id().to_string(),
      name: input_data.name().to_string(),
      variable: DefInformationItem::new(input_data.variable(), imports),
    }
  }
}

impl DefInputData {
  /// Returns the namespace.
  pub fn namespace(&self) -> &str {
    &self.namespace
  }

  /// Returns a reference to identifier.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Returns a reference to name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns reference to a variable.
  pub fn variable(&self) -> &DefInformationItem {
    &self.variable
  }
}

pub struct DefItemDefinition {
  namespace: String,
  id: String,
  name: String,
  feel_name: Name,
  type_ref: Option<String>,
  allowed_values: Option<UnaryTests>,
  item_components: Vec<DefItemDefinition>,
  function_item: Option<FunctionItem>,
  is_collection: bool,
}

impl DefItemDefinition {
  pub fn new(item_definition: &ItemDefinition) -> Self {
    Self {
      namespace: item_definition.namespace().to_string(),
      id: item_definition.id().to_string(),
      name: item_definition.name().to_string(),
      feel_name: item_definition.feel_name().clone(),
      type_ref: item_definition.type_ref().clone(),
      allowed_values: item_definition.allowed_values().clone(),
      item_components: item_definition.item_components().iter().map(DefItemDefinition::new).collect(),
      function_item: item_definition.function_item().clone(),
      is_collection: item_definition.is_collection(),
    }
  }
}

impl DefItemDefinition {
  /// Returns the namespace.
  pub fn namespace(&self) -> &str {
    &self.namespace
  }

  /// Returns the identifier.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Returns a name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns a FEEL name.
  pub fn feel_name(&self) -> &Name {
    &self.feel_name
  }

  /// Returns type reference.
  pub fn type_ref(&self) -> &Option<String> {
    &self.type_ref
  }

  /// Returns reference to possible values or ranges of values
  /// in the base type that are allowed in this [ItemDefinition].
  pub fn allowed_values(&self) -> &Option<UnaryTests> {
    &self.allowed_values
  }

  /// Returns reference to nested [ItemDefinitions](ItemDefinition) that compose this [ItemDefinition].
  pub fn item_components(&self) -> &Vec<DefItemDefinition> {
    &self.item_components
  }

  /// Returns a reference to an optional [FunctionItem] that compose this [ItemDefinition].
  pub fn function_item(&self) -> &Option<FunctionItem> {
    &self.function_item
  }

  /// Returns flag indicating if the actual values are collections of allowed values.
  pub fn is_collection(&self) -> bool {
    self.is_collection
  }

  /// Returns the item definition type.
  pub fn item_definition_type(&self) -> Result<ItemDefinitionType> {
    let simple_type_ref = if let Some(type_ref) = self.type_ref() { type_ref_to_feel_type(type_ref) } else { None };
    let condition = (
      self.type_ref().is_some(),
      simple_type_ref.is_some(),
      !self.item_components().is_empty(),
      self.is_collection(),
      self.function_item().is_some(),
    );
    match condition {
      (_, true, false, false, false) => Ok(ItemDefinitionType::SimpleType(simple_type_ref.unwrap().clone())),
      (true, false, false, false, false) => Ok(ItemDefinitionType::ReferencedType(self.namespace.clone(), self.type_ref().as_ref().unwrap().clone())),
      (false, false, true, false, false) => Ok(ItemDefinitionType::ComponentType),
      (_, true, false, true, false) => Ok(ItemDefinitionType::CollectionOfSimpleType(simple_type_ref.unwrap().clone())),
      (false, false, true, true, false) => Ok(ItemDefinitionType::CollectionOfComponentType),
      (true, false, false, true, false) => Ok(ItemDefinitionType::CollectionOfReferencedType(
        self.namespace.clone(),
        self.type_ref().as_ref().unwrap().clone(),
      )),
      (false, false, false, false, true) => Ok(ItemDefinitionType::FunctionType),
      _ => Err(err_invalid_item_definition_type(self.name())),
    }
  }
}

pub struct DefBusinessKnowledgeModel {
  namespace: String,
  model_name: String,
  id: String,
  name: String,
  variable: DefInformationItem,
  encapsulated_logic: Option<FunctionDefinition>,
  knowledge_requirements: Vec<DefKnowledgeRequirement>,
}

impl DefBusinessKnowledgeModel {
  /// Create [DefBusinessKnowledgeModel] from [BusinessKnowledgeModel].
  pub fn new(business_knowledge_model: &BusinessKnowledgeModel, imports: &[DefImport]) -> Self {
    Self {
      namespace: business_knowledge_model.namespace().to_string(),
      model_name: business_knowledge_model.model_name().to_string(),
      id: business_knowledge_model.id().to_string(),
      name: business_knowledge_model.name().to_string(),
      variable: DefInformationItem::new(business_knowledge_model.variable(), imports),
      encapsulated_logic: business_knowledge_model.encapsulated_logic().clone(),
      knowledge_requirements: business_knowledge_model
        .knowledge_requirements()
        .iter()
        .map(|knowledge_requirements| DefKnowledgeRequirement::new(knowledge_requirements, imports))
        .collect(),
    }
  }
}

impl DefBusinessKnowledgeModel {
  /// Returns the namespace.
  pub fn namespace(&self) -> &str {
    &self.namespace
  }

  /// Returns the model name.
  pub fn model_name(&self) -> &str {
    &self.model_name
  }

  /// Returns an identifier.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Returns a name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns an output variable.
  pub fn variable(&self) -> &DefInformationItem {
    &self.variable
  }

  /// Returns reference to a variable for this [BusinessKnowledgeModel].
  pub fn encapsulated_logic(&self) -> &Option<FunctionDefinition> {
    &self.encapsulated_logic
  }

  /// Returns reference to the collection of instances of [KnowledgeRequirement] that compose this [BusinessKnowledgeModel].
  pub fn knowledge_requirements(&self) -> &Vec<DefKnowledgeRequirement> {
    &self.knowledge_requirements
  }
}

/// Definitions for reference.
pub struct DefHRef {
  /// Namespace of the reference, derived from definition or set explicitly.
  namespace: String,
  /// Identifier of the referenced element.
  id: String,
  /// Name of the import associated with the namespace.
  import_name: Option<Name>,
}

impl DefHRef {
  pub fn new(namespace: &str, href: &HRef, imports: &[DefImport]) -> Self {
    let namespace = href.namespace().cloned().unwrap_or(namespace.to_string());
    let import_name = get_import_name(&namespace, imports);
    Self {
      namespace,
      id: href.id().to_string(),
      import_name,
    }
  }

  pub fn namespace(&self) -> &str {
    &self.namespace
  }

  pub fn id(&self) -> &str {
    &self.id
  }

  pub fn import_name(&self) -> Option<&Name> {
    self.import_name.as_ref()
  }
}

/// Definitions for information requirement.
pub struct DefInformationRequirement {
  required_decision: Option<DefHRef>,
  required_input: Option<DefHRef>,
}

impl DefInformationRequirement {
  pub fn new(information_requirement: &InformationRequirement, imports: &[DefImport]) -> Self {
    Self {
      required_decision: information_requirement
        .required_decision()
        .as_ref()
        .map(|href| DefHRef::new(information_requirement.namespace(), href, imports)),
      required_input: information_requirement
        .required_input()
        .as_ref()
        .map(|href| DefHRef::new(information_requirement.namespace(), href, imports)),
    }
  }

  pub fn required_decision(&self) -> Option<&DefHRef> {
    self.required_decision.as_ref()
  }

  pub fn required_input(&self) -> Option<&DefHRef> {
    self.required_input.as_ref()
  }
}

/// Definitions for knowledge requirement.
pub struct DefKnowledgeRequirement {
  required_knowledge: DefHRef,
}

impl DefKnowledgeRequirement {
  pub fn new(knowledge_requirement: &KnowledgeRequirement, imports: &[DefImport]) -> Self {
    Self {
      required_knowledge: DefHRef::new(knowledge_requirement.namespace(), knowledge_requirement.required_knowledge(), imports),
    }
  }

  pub fn required_knowledge(&self) -> &DefHRef {
    &self.required_knowledge
  }
}

/// Definitions for decision.
pub struct DefDecision {
  namespace: String,
  model_name: String,
  id: String,
  name: String,
  variable: DefInformationItem,
  decision_logic: Option<ExpressionInstance>,
  information_requirements: Vec<DefInformationRequirement>,
  knowledge_requirements: Vec<DefKnowledgeRequirement>,
}

impl DefDecision {
  /// Create [DefDecision] from [Decision].
  pub fn new(decision: &Decision, imports: &[DefImport]) -> Self {
    Self {
      namespace: decision.namespace().to_string(),
      model_name: decision.model_name().to_string(),
      id: decision.id().to_string(),
      name: decision.name().to_string(),
      variable: DefInformationItem::new(decision.variable(), imports),
      decision_logic: decision.decision_logic().clone(),
      information_requirements: decision
        .information_requirements()
        .iter()
        .map(|knowledge_requirements| DefInformationRequirement::new(knowledge_requirements, imports))
        .collect(),
      knowledge_requirements: decision
        .knowledge_requirements()
        .iter()
        .map(|knowledge_requirements| DefKnowledgeRequirement::new(knowledge_requirements, imports))
        .collect(),
    }
  }
}

impl DefDecision {
  /// Returns the model namespace this decision was defined in.
  pub fn namespace(&self) -> &str {
    &self.namespace
  }

  /// Returns the model name this decision was defined in.
  pub fn model_name(&self) -> &str {
    &self.model_name
  }

  /// Returns an identifier.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Returns a name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns output variable.
  pub fn variable(&self) -> &DefInformationItem {
    &self.variable
  }

  /// Returns a reference to optional [Expression].
  pub fn decision_logic(&self) -> &Option<ExpressionInstance> {
    &self.decision_logic
  }

  /// Returns a reference to collection of [InformationRequirement].
  pub fn information_requirements(&self) -> &Vec<DefInformationRequirement> {
    &self.information_requirements
  }

  /// Returns reference to the collection of instances of [KnowledgeRequirement] that compose this [BusinessKnowledgeModel].
  pub fn knowledge_requirements(&self) -> &Vec<DefKnowledgeRequirement> {
    &self.knowledge_requirements
  }
}

/// Definitions for decision service.
pub struct DefDecisionService {
  namespace: String,
  model_name: String,
  id: String,
  name: String,
  variable: DefInformationItem,
  input_decisions: Vec<DefHRef>,
  output_decisions: Vec<DefHRef>,
  encapsulated_decisions: Vec<DefHRef>,
  input_data: Vec<DefHRef>,
}

impl DefDecisionService {
  /// Creates [DefDecisionService] from [DecisionService].
  pub fn new(decision_service: &DecisionService, imports: &[DefImport]) -> Self {
    let model_namespace = decision_service.namespace();
    let model_name = decision_service.model_name();
    Self {
      namespace: model_namespace.to_string(),
      model_name: model_name.to_string(),
      id: decision_service.id().to_string(),
      name: decision_service.name().to_string(),
      variable: DefInformationItem::new(decision_service.variable(), imports),
      input_decisions: decision_service.input_decisions().iter().map(|href| DefHRef::new(model_namespace, href, imports)).collect(),
      output_decisions: decision_service
        .output_decisions()
        .iter()
        .map(|href| DefHRef::new(model_namespace, href, imports))
        .collect(),
      encapsulated_decisions: decision_service
        .encapsulated_decisions()
        .iter()
        .map(|href| DefHRef::new(model_namespace, href, imports))
        .collect(),
      input_data: decision_service.input_data().iter().map(|href| DefHRef::new(model_namespace, href, imports)).collect(),
    }
  }
}

impl DefDecisionService {
  /// Returns the namespace.
  pub fn namespace(&self) -> &str {
    &self.namespace
  }

  /// Returns the model name.
  pub fn model_name(&self) -> &str {
    &self.model_name
  }

  /// Returns a reference to identifier.
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Returns a reference to name.
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Returns reference to a variable.
  pub fn variable(&self) -> &DefInformationItem {
    &self.variable
  }

  /// Returns a collection of input [Decision]s for this [DecisionService].
  pub fn input_decisions(&self) -> &Vec<DefHRef> {
    &self.input_decisions
  }

  /// Returns a collection of encapsulated [Decision]s for this [DecisionService].
  pub fn encapsulated_decisions(&self) -> &Vec<DefHRef> {
    &self.encapsulated_decisions
  }
  /// Returns a collection of output [Decision]s for this [DecisionService].
  pub fn output_decisions(&self) -> &Vec<DefHRef> {
    &self.output_decisions
  }

  /// Returns a collection of [InputData] for this [DecisionService].
  pub fn input_data(&self) -> &Vec<DefHRef> {
    &self.input_data
  }
}

#[derive(Default)]
pub struct DefImport {
  namespace: String,
  name: Name,
}

impl DefImport {
  /// Creates [DefImport] from [Import].
  pub fn new(import: &Import) -> Self {
    Self {
      namespace: import.namespace().to_string(),
      name: import.feel_name().clone(),
    }
  }
}

/// All definitions needed to build complete model evaluator from DMN model.
#[derive(Default)]
pub struct DefDefinitions {
  /// List of imports.
  imports: Vec<DefImport>,
  /// Item definitions.
  item_definitions: Vec<DefItemDefinition>,
  /// Input data definitions.
  input_data: HashMap<DefKey, DefInputData>,
  /// Business knowledge models.
  business_knowledge_models: HashMap<DefKey, DefBusinessKnowledgeModel>,
  /// Decisions.
  decisions: HashMap<DefKey, DefDecision>,
  /// Decision services.
  decision_services: HashMap<DefKey, DefDecisionService>,
}

impl DefDefinitions {
  /// Adds definitions from specified model.
  pub fn add_model(&mut self, definitions: &Definitions) {
    self.imports.append(&mut definitions.imports().iter().map(DefImport::new).collect());
    self
      .item_definitions
      .append(&mut definitions.item_definitions().iter().map(DefItemDefinition::new).collect());
    for drg_element in definitions.drg_elements() {
      match drg_element {
        DrgElement::InputData(inner) => {
          self.input_data.insert(DefKey::new(inner.namespace(), inner.id()), DefInputData::new(inner, &self.imports));
        }
        DrgElement::BusinessKnowledgeModel(inner) => {
          self
            .business_knowledge_models
            .insert(DefKey::new(inner.namespace(), inner.id()), DefBusinessKnowledgeModel::new(inner, &self.imports));
        }
        DrgElement::Decision(inner) => {
          self.decisions.insert(DefKey::new(inner.namespace(), inner.id()), DefDecision::new(inner, &self.imports));
        }
        DrgElement::DecisionService(inner) => {
          self
            .decision_services
            .insert(DefKey::new(inner.namespace(), inner.id()), DefDecisionService::new(inner, &self.imports));
        }
        _ => {}
      }
    }
  }

  /// Returns item definitions.
  pub fn item_definitions(&self) -> &Vec<DefItemDefinition> {
    &self.item_definitions
  }

  /// Returns references to decisions.
  pub fn decisions(&self) -> Vec<&DefDecision> {
    self.decisions.values().collect()
  }

  /// Returns an optional reference to [Decision] with specified identifier
  /// or [None] when such [Decision] was not found among instances of [DrgElement].
  pub fn decision_by_key(&self, namespace: &str, id: &str) -> Option<&DefDecision> {
    self.decisions.get(&DefKey::new(namespace, id))
  }

  /// Returns references to business knowledge models.
  pub fn business_knowledge_models(&self) -> Vec<&DefBusinessKnowledgeModel> {
    self.business_knowledge_models.values().collect()
  }

  /// Returns an optional reference to [BusinessKnowledgeModel] with specified identifier
  /// or [None] when such [BusinessKnowledgeModel] was not found among instances of [DrgElement].
  pub fn business_knowledge_model_by_key(&self, namespace: &str, id: &str) -> Option<&DefBusinessKnowledgeModel> {
    self.business_knowledge_models.get(&DefKey::new(namespace, id))
  }

  /// Returns references to decision services.
  pub fn decision_services(&self) -> Vec<&DefDecisionService> {
    self.decision_services.values().collect()
  }

  /// Returns an optional reference to [DecisionService] with specified identifier
  /// or [None] when such [DecisionService] was not found among instances of [DrgElement].
  pub fn decision_service_by_id(&self, namespace: &str, id: &str) -> Option<&DefDecisionService> {
    self.decision_services.get(&DefKey::new(namespace, id))
  }

  /// Returns references to input data instances.
  pub fn input_data(&self) -> Vec<&DefInputData> {
    self.input_data.values().collect()
  }

  /// Returns an optional reference to [InputData] with specified identifier
  /// or [None] when such [InputData] was not found among
  /// instances of [DrgElement]).
  pub fn input_data_by_key(&self, namespace: &str, id: &str) -> Option<&DefInputData> {
    self.input_data.get(&DefKey::new(namespace, id))
  }

  /// Extracts the DRG topology (nodes and edges) from this set of definitions.
  pub fn to_trace_graph(&self) -> crate::trace::TraceGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    // Create InputData nodes.
    for (key, input_data) in &self.input_data {
      nodes.push(crate::trace::TraceNode::InputData {
        id: key.to_string(),
        name: input_data.name().to_string(),
        value: None,
      });
    }
    // Helper: extract decision table details from an ExpressionInstance.
    let extract_dt = |expr: &ExpressionInstance| -> Option<(String, Vec<String>, Vec<String>, Vec<crate::trace::TraceRule>)> {
      if let ExpressionInstance::DecisionTable(dt) = expr {
        let hp = format!("{}", dt.hit_policy());
        let inputs: Vec<String> = dt.input_clauses().map(|ic| ic.input_expression.clone()).collect();
        let outputs: Vec<String> = dt.output_clauses().map(|oc| oc.name.clone().unwrap_or_default()).collect();
        let rs: Vec<crate::trace::TraceRule> = dt
          .rules()
          .enumerate()
          .map(|(i, rule)| crate::trace::TraceRule {
            index: i,
            input_entries: rule.input_entries.iter().map(|ie| ie.text.clone()).collect(),
            output_entries: rule.output_entries.iter().map(|oe| oe.text.clone()).collect(),
          })
          .collect();
        Some((hp, inputs, outputs, rs))
      } else {
        None
      }
    };

    // Helper: find a decision table inside a BKM's encapsulated logic.
    let extract_dt_from_bkm = |bkm: &DefBusinessKnowledgeModel| -> Option<(String, Vec<String>, Vec<String>, Vec<crate::trace::TraceRule>)> {
      bkm.encapsulated_logic().as_ref().and_then(|fd| fd.body().as_ref()).and_then(|body| extract_dt(body))
    };

    // Create DecisionTable nodes and edges from information requirements.
    for (key, decision) in &self.decisions {
      // Try direct decision table first, then resolve BKM invocations.
      let (hit_policy, input_columns, output_columns, rules) = decision
        .decision_logic()
        .as_ref()
        .and_then(|expr| {
          // Direct decision table.
          extract_dt(expr).or_else(|| {
            // Invocation — look up the called BKM's decision table.
            if let ExpressionInstance::Invocation(inv) = expr {
              if let ExpressionInstance::LiteralExpression(le) = inv.called_function() {
                if let Some(bkm_name) = le.text() {
                  for bkm in self.business_knowledge_models.values() {
                    if bkm.name() == bkm_name {
                      return extract_dt_from_bkm(bkm);
                    }
                  }
                }
              }
            }
            None
          })
        })
        .unwrap_or_else(|| (String::new(), vec![], vec![], vec![]));
      nodes.push(crate::trace::TraceNode::DecisionTable {
        id: key.to_string(),
        name: decision.name().to_string(),
        hit_policy,
        input_columns,
        output_columns,
        rules,
      });
      // Create edges from information requirements.
      for req in decision.information_requirements() {
        if let Some(href) = req.required_decision() {
          let source_key = DefKey::from(href);
          let source_name = self.decisions.get(&source_key).map(|d| d.name().to_string()).unwrap_or_default();
          edges.push(crate::trace::TraceEdge {
            source: source_key.to_string(),
            target: key.to_string(),
            label: source_name,
          });
        }
        if let Some(href) = req.required_input() {
          let source_key = DefKey::from(href);
          let source_name = self.input_data.get(&source_key).map(|d| d.name().to_string()).unwrap_or_default();
          edges.push(crate::trace::TraceEdge {
            source: source_key.to_string(),
            target: key.to_string(),
            label: source_name,
          });
        }
      }
    }
    crate::trace::TraceGraph { nodes, edges }
  }
}

/// Returns a name of the import for specified namespace.
fn get_import_name(namespace: &str, imports: &[DefImport]) -> Option<Name> {
  imports
    .iter()
    .filter_map(|def_import| if def_import.namespace == namespace { Some(def_import.name.clone()) } else { None })
    .collect::<Vec<Name>>()
    .first()
    .cloned()
}

/// Returns a namespace of the import for specified name.
fn get_import_namespace(name: &Name, imports: &[DefImport]) -> Option<String> {
  imports
    .iter()
    .filter_map(|def_import| if &def_import.name == name { Some(def_import.namespace.clone()) } else { None })
    .collect::<Vec<String>>()
    .first()
    .cloned()
}
