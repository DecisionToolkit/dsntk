//! # DMN model
//!
//! Model for Decision Requirements Graph (DRG)
//! depicted in one or more Decision Requirements Diagrams (DRD).

use crate::errors::*;
use dsntk_common::{gen_id, DsntkError, HRef, Result, Uri};
use dsntk_feel::{FeelType, Name};
use std::fmt;
use std::fmt::Display;
use std::slice::Iter;

pub enum DmnVersion {
  V13,
  V14,
  V15,
}

/// [DmnId] defines possible types of unique identifiers in model.
///
/// Specification defines this identifier as optional, so when this identifier
/// is not provided in the model, a new unique UUID identifier is generated.
/// This SHALL not be conflicting with any other identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DmnId {
  /// Identifier was provided in model.
  Provided(String),
  /// Identifier was generated during parsing (not provided in model).
  Generated(String),
}

/// [DmnElement] is the abstract superclass for the Decision Model elements.
/// It provides the optional attributes `id`, `description` and `label`,
/// which other elements will inherit.
pub trait DmnElement {
  /// Namespace the element belongs to.
  fn namespace(&self) -> &str;
  /// Name of the model the element was defined in.
  fn model_name(&self) -> &str;
  /// Returns a reference to identifier for this [DmnElement].
  /// This identifier SHALL be unique within its containing [Definitions] element.
  fn id(&self) -> &String;
  /// Returns a reference to optional identifier for this [DmnElement].
  fn opt_id(&self) -> Option<&String>;
  /// Returns reference to optional description of this [DmnElement].
  fn description(&self) -> &Option<String>;
  /// Returns reference to optional alternative short description of this [DmnElement].
  fn label(&self) -> &Option<String>;
  /// Returns reference to attached additional elements to any [DmnElement].
  fn extension_elements(&self) -> &Vec<ExtensionElement>;
  /// Returns reference to attached named extended attributes and model associations to any [DmnElement].
  fn extension_attributes(&self) -> &Vec<ExtensionAttribute>;
}

/// [NamedElement] adds attribute `name` to [DmnElement].
/// `name` attribute is required for [NamedElement].
pub trait NamedElement: DmnElement {
  /// Returns the name of this [NamedElement].
  fn name(&self) -> &str;
  /// Returns the `FEEL` name for this element.
  fn feel_name(&self) -> &Name;
}

/// [Expression] is an abstract class that describes the logic
/// by which a modeled decision shall be made, or pieces of that logic.
pub trait Expression {
  /// Optional namespace-prefixed name of the base type of this [Expression].
  fn type_ref(&self) -> &Option<String>;
}

/// [FeelTypedElement] adds the `FEEL` type attributes to element.
pub trait FeelTypedElement {
  /// Returns the optional `FEEL` type for this element.
  fn feel_type(&self) -> &Option<FeelType>;
  /// Sets `FEEL` type for this element.
  fn set_feel_type(&mut self, feel_type: FeelType);
}

/// [RequiredTypeRef] adds the required type reference to element.
pub trait RequiredTypeRef {
  /// Namespace-prefixed name of the base type of the implementor.
  fn type_ref(&self) -> &str;
}

/// [RequiredVariable] adds the required reference to [InformationItem].
pub trait RequiredVariable {
  /// Returns the reference to [InformationItem].
  fn variable(&self) -> &InformationItem;
}

/// `Invocable` is used to model the inputs of a decision whose values
/// are defined outside the decision model.
pub trait Invocable: DmnElement + NamedElement + RequiredVariable {}

/// The abstract class [BusinessContextElement].
///
/// The abstract class [BusinessContextElement], and its concrete specializations
/// [PerformanceIndicator] and [OrganizationUnit] are placeholders,
/// anticipating a definition to be adopted from other OMG metamodels,
/// such as OMG OSM when it is further developed.
pub trait BusinessContextElement: NamedElement {
  /// The URI of this [BusinessContextElement].
  fn uri(&self) -> &Option<String>;
}

/// The [ExtensionElement] contains element from other
/// metamodels inside any [DmnElement].
///
/// Not used, prepared for further development.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionElement;

/// The [ExtensionAttribute] element.
///
/// The [ExtensionAttribute] element contains an [ExtensionElement]
/// or a reference to an [ExtensionElement] from another metamodel.
/// An [ExtensionAttribute] also has a name
/// to define the role or purpose of the associated element.
///
/// Not used, prepared for further development.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionAttribute;

/// Enumeration of concrete instances of [BusinessContextElement].
#[derive(Debug, Clone, PartialEq)]
pub enum BusinessContextElementInstance {
  PerformanceIndicator(PerformanceIndicator),
  OrganizationUnit(OrganizationUnit),
}

/// [PerformanceIndicator] is a placeholder, anticipating a definition to be
/// adopted from other OMG metamodels, such as OMG OSM when it is further developed.
#[named_element]
#[dmn_element]
#[business_context_element]
#[derive(Debug, Clone, PartialEq)]
pub struct PerformanceIndicator {
  /// Collection of [Decision] that impact this [PerformanceIndicator].
  /// This attribute stores references
  pub(crate) impacting_decisions: Vec<HRef>,
}

impl PerformanceIndicator {
  pub fn impacting_decisions(&self) -> &Vec<HRef> {
    &self.impacting_decisions
  }
}

/// [OrganizationUnit] is a placeholder, anticipating a definition to be
/// adopted from other OMG metamodels, such as OMG OSM when it is further developed.
#[named_element]
#[dmn_element]
#[business_context_element]
#[derive(Debug, Clone, PartialEq)]
pub struct OrganizationUnit {
  /// Collection of [Decision] that are made by this [OrganizationUnit].
  pub(crate) decisions_made: Vec<HRef>,
  /// Collection of [Decision] that are owned by this [OrganizationUnit].
  pub(crate) decisions_owned: Vec<HRef>,
}

impl OrganizationUnit {
  pub fn decisions_made(&self) -> &Vec<HRef> {
    &self.decisions_made
  }
  pub fn decisions_owned(&self) -> &Vec<HRef> {
    &self.decisions_owned
  }
}

/// In DMN model, the [DrgElement] is the abstract superclass for all DMN elements.
///
/// All DMN elements are contained within [Definitions] and that have a graphical
/// representation in a DRD. This enumeration specifies the list
/// of [DRGElements](DrgElement) contained in [Definitions].
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum DrgElement {
  Decision(Decision),
  InputData(InputData),
  BusinessKnowledgeModel(BusinessKnowledgeModel),
  DecisionService(DecisionService),
  KnowledgeSource(KnowledgeSource),
}

/// Enumeration of specific requirements.
pub enum Requirement {
  Information(InformationRequirement),
  Knowledge(KnowledgeRequirement),
  Authority(AuthorityRequirement),
}

/// [Definitions] element is the outermost containing object
/// for all elements of a DMN decision model.
/// It defines the scope of visibility and the namespace
/// for all contained elements.
#[named_element]
#[dmn_element]
#[derive(Debug, Clone, PartialEq)]
pub struct Definitions {
  /// This attribute identifies the expression language used in
  /// [LiteralExpressions](LiteralExpression) within the scope
  /// of this [Definitions]. The _Default_ is FEEL.
  /// This value **MAY** be overridden on each individual [LiteralExpression].
  /// The language **SHALL** be specified in a URI format.
  pub(crate) expression_language: Option<Uri>,
  /// This attribute identifies the type language used in
  /// [LiteralExpressions](LiteralExpression) within the scope
  /// of this [Definitions]. The _Default_ is FEEL.
  /// This value **MAY** be overridden on each individual [ItemDefinition].
  /// The language **SHALL** be specified in a URI format.
  pub(crate) type_language: Option<Uri>,
  /// Name of the tool used to export the XML serialization.
  pub(crate) exporter: Option<String>,
  /// Version of the tool used to export the XML serialization.
  pub(crate) exporter_version: Option<String>,
  /// Container for the instances of [ItemDefinition] that are contained in this [Definitions].
  pub(crate) item_definitions: Vec<ItemDefinition>,
  /// Container for the instances of [DrgElement] that are contained in this [Definitions].
  pub(crate) drg_elements: Vec<DrgElement>,
  /// Container for the instances of [BusinessContextElement] that are contained in this [Definitions].
  pub(crate) business_context_elements: Vec<BusinessContextElementInstance>,
  /// Container used to import externally defined elements and make them available
  /// for use by elements in this [Definitions].
  pub(crate) imports: Vec<Import>,
  /// Optional diagram interchange information contained within this [Definitions].
  pub(crate) dmndi: Option<Dmndi>,
}

impl Definitions {
  /// Returns the reference to optional expression language used within the scope of this [Definitions].
  pub fn expression_language(&self) -> &Option<String> {
    &self.expression_language
  }

  /// Returns reference to the type language used within the scope of this [Definitions].
  pub fn type_language(&self) -> &Option<String> {
    &self.type_language
  }

  /// Returns reference to the name of the tool used to export the XML serialization.
  pub fn exporter(&self) -> &Option<String> {
    &self.exporter
  }

  /// Returns reference to the version of the tool used to export the XML serialization.
  pub fn exporter_version(&self) -> &Option<String> {
    &self.exporter_version
  }

  /// Returns reference to the container of instances of [ItemDefinition] contained in this [Definitions].
  pub fn item_definitions(&self) -> &Vec<ItemDefinition> {
    &self.item_definitions
  }

  /// Returns reference to the container of instances of [Import] contained in this [Definitions].
  pub fn imports(&self) -> &Vec<Import> {
    &self.imports
  }

  /// Returns reference to optional [Dmndi] container.
  pub fn dmndi(&self) -> &Option<Dmndi> {
    &self.dmndi
  }

  /// Returns reference to [DrgElements](DrgElement) container.
  pub fn drg_elements(&self) -> Iter<DrgElement> {
    self.drg_elements.iter()
  }

  /// Returns all decision definitions.
  pub fn decisions(&self) -> Vec<Decision> {
    self
      .drg_elements
      .iter()
      .filter_map(|drg_element| {
        if let DrgElement::Decision(decision) = drg_element {
          Some(decision.clone())
        } else {
          None
        }
      })
      .collect()
  }

  /// Returns all business knowledge model definitions.
  pub fn business_knowledge_models(&self) -> Vec<BusinessKnowledgeModel> {
    self
      .drg_elements
      .iter()
      .filter_map(|drg_element| {
        if let DrgElement::BusinessKnowledgeModel(bkm) = drg_element {
          Some(bkm.clone())
        } else {
          None
        }
      })
      .collect()
  }

  /// Returns all decision services definitions.
  pub fn decision_services(&self) -> Vec<DecisionService> {
    self
      .drg_elements
      .iter()
      .filter_map(|drg_element| {
        if let DrgElement::DecisionService(decision_service) = drg_element {
          Some(decision_service.clone())
        } else {
          None
        }
      })
      .collect()
  }

  /// Returns all knowledge source definitions.
  pub fn knowledge_sources(&self) -> Vec<&KnowledgeSource> {
    self
      .drg_elements
      .iter()
      .filter_map(|drg_element| {
        if let DrgElement::KnowledgeSource(knowledge_source) = drg_element {
          Some(knowledge_source)
        } else {
          None
        }
      })
      .collect()
  }

  /// Returns all input data definitions.
  pub fn input_data(&self) -> Vec<InputData> {
    self
      .drg_elements
      .iter()
      .filter_map(|drg_element| {
        if let DrgElement::InputData(input_data) = drg_element {
          Some(input_data.clone())
        } else {
          None
        }
      })
      .collect()
  }

  /// Returns performance indicators.
  pub fn performance_indicators(&self) -> Vec<&PerformanceIndicator> {
    self
      .business_context_elements
      .iter()
      .filter_map(|item| match item {
        BusinessContextElementInstance::PerformanceIndicator(performance_indicator) => Some(performance_indicator),
        _ => None,
      })
      .collect()
  }

  /// Returns organisation units.
  pub fn organisation_units(&self) -> Vec<&OrganizationUnit> {
    self
      .business_context_elements
      .iter()
      .filter_map(|item| match item {
        BusinessContextElementInstance::OrganizationUnit(organisation_unit) => Some(organisation_unit),
        _ => None,
      })
      .collect()
  }

  /// Returns decision with specified identifier.
  pub fn get_decision(&self, id: &str) -> Option<&Decision> {
    for drg_element in &self.drg_elements {
      if let DrgElement::Decision(decision) = drg_element {
        if decision.id() == id {
          return Some(decision);
        }
      }
    }
    None
  }

  /// Returns input data with specified identifier.
  pub fn get_input_data(&self, id: &str) -> Option<&InputData> {
    for drg_element in &self.drg_elements {
      if let DrgElement::InputData(input_data) = drg_element {
        if input_data.id() == id {
          return Some(input_data);
        }
      }
    }
    None
  }

  /// Returns business knowledge model with specified identifier.
  pub fn get_business_knowledge_model(&self, id: &str) -> Option<&BusinessKnowledgeModel> {
    for drg_element in &self.drg_elements {
      if let DrgElement::BusinessKnowledgeModel(business_knowledge_model) = drg_element {
        if business_knowledge_model.id() == id {
          return Some(business_knowledge_model);
        }
      }
    }
    None
  }

  /// Returns knowledge source with specified identifier.
  pub fn get_knowledge_source(&self, id: &str) -> Option<&KnowledgeSource> {
    for drg_element in &self.drg_elements {
      if let DrgElement::KnowledgeSource(knowledge_source) = drg_element {
        if knowledge_source.id() == id {
          return Some(knowledge_source);
        }
      }
    }
    None
  }

  /// Returns a requirement with specified identifier.
  pub fn get_requirement(&self, id: &str) -> Option<Requirement> {
    for drg_element in &self.drg_elements {
      match drg_element {
        DrgElement::Decision(decision) => {
          for knowledge_requirement in &decision.knowledge_requirements {
            if knowledge_requirement.id() == id {
              return Some(Requirement::Knowledge(knowledge_requirement.clone()));
            }
          }
          for information_requirement in &decision.information_requirements {
            if information_requirement.id() == id {
              return Some(Requirement::Information(information_requirement.clone()));
            }
          }
          for authority_requirement in &decision.authority_requirements {
            if authority_requirement.id() == id {
              return Some(Requirement::Authority(authority_requirement.clone()));
            }
          }
        }
        DrgElement::BusinessKnowledgeModel(business_knowledge_model) => {
          for knowledge_requirement in &business_knowledge_model.knowledge_requirements {
            if knowledge_requirement.id() == id {
              return Some(Requirement::Knowledge(knowledge_requirement.clone()));
            }
          }
          for authority_requirement in &business_knowledge_model.authority_requirements {
            if authority_requirement.id() == id {
              return Some(Requirement::Authority(authority_requirement.clone()));
            }
          }
        }
        DrgElement::KnowledgeSource(knowledge_source) => {
          for authority_requirement in &knowledge_source.authority_requirements {
            if authority_requirement.id() == id {
              return Some(Requirement::Authority(authority_requirement.clone()));
            }
          }
        }
        _ => {}
      }
    }
    None
  }
}

#[named_element]
#[dmn_element]
#[derive(Debug, Clone, PartialEq)]
pub struct InformationItem {
  /// Qualified name of the type of this [InformationItem].
  pub(crate) type_ref: String,
  /// Optional `FEEL` type of this [InformationItem].
  pub(crate) feel_type: Option<FeelType>,
}

impl InformationItem {
  /// Returns qualified name of the type of this [InformationItem].
  pub fn type_ref(&self) -> &String {
    &self.type_ref
  }
}

impl FeelTypedElement for InformationItem {
  /// Returns a reference to optional `FEEL` type of this element.
  fn feel_type(&self) -> &Option<FeelType> {
    &self.feel_type
  }
  /// Sets the `FEEL` type for this element.
  fn set_feel_type(&mut self, feel_type: FeelType) {
    self.feel_type = Some(feel_type);
  }
}

/// [InputData] is used to model the inputs of a decision whose values
/// are defined outside the decision model.
#[named_element]
#[dmn_element]
#[derive(Debug, Clone, PartialEq)]
pub struct InputData {
  /// The instance of [InformationItem] that stores the result of this [InputData].
  pub(crate) variable: InformationItem,
}

impl RequiredVariable for InputData {
  /// Returns reference to a variable for this [BusinessKnowledgeModel].  
  fn variable(&self) -> &InformationItem {
    &self.variable
  }
}

/// `Import` class is used when referencing external elements.
///
/// These elements can either be a DMN [DRGElement](DrgElement) or [ItemDefinition]
/// instances contained in other [Definitions] elements, or non-DMN elements,
/// such as an XML Schema or a PMML file.
#[named_element]
#[dmn_element]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
  /// Specifies the style of import associated with this [Import].
  pub(crate) import_type: String,
  /// Identifies the location of the imported element.
  pub(crate) location_uri: Option<String>,
}

impl Import {
  /// Returns reference to the import type for this [Import].
  pub fn import_type(&self) -> &str {
    &self.import_type
  }

  /// Returns reference to the optional location URI for this [Import].
  pub fn location_uri(&self) -> &Option<String> {
    &self.location_uri
  }

  /// Returns reference to the namespace of this [Import].
  pub fn namespace(&self) -> &str {
    &self.namespace
  }
}

/// An enumeration of concrete instances of abstract [Expression], which are:
/// - [Conditional],
/// - [Context],
/// - [DecisionTable],
/// - [Every],
/// - [Filter],
/// - [For],
/// - [FunctionDefinition],
/// - [Invocation],
/// - [List],
/// - [LiteralExpression],
/// - [Relation],
/// - [Some].
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionInstance {
  Conditional(Box<Conditional>),
  Context(Box<Context>),
  DecisionTable(Box<DecisionTable>),
  Every(Box<Every>),
  Filter(Box<Filter>),
  For(Box<For>),
  FunctionDefinition(Box<FunctionDefinition>),
  Invocation(Box<Invocation>),
  List(Box<List>),
  LiteralExpression(Box<LiteralExpression>),
  Relation(Box<Relation>),
  Some(Box<Some>),
}

/// A [Context] is composed of any number of model context entries, which are instances of [ContextEntry].
#[dmn_element]
#[expression]
#[derive(Debug, Clone, PartialEq)]
pub struct Context {
  /// This attribute lists the instances of [ContextEntry] that compose this [Context].
  pub(crate) context_entries: Vec<ContextEntry>,
}

impl Context {
  /// Return a reference to context entries that compose this [Context].
  pub fn context_entries(&self) -> &Vec<ContextEntry> {
    &self.context_entries
  }
}

/// The class [ContextEntry] is used to model `FEEL` context entries when a context is modeled as a [Context] element.
#[derive(Debug, Clone, PartialEq)]
pub struct ContextEntry {
  /// The instance of [InformationItem] that is contained in this [ContextEntry],
  /// and whose name is the key in the modeled context entry.
  pub variable: Option<InformationItem>,
  /// The instance of [Expression] that is the expression in this [ContextEntry].
  pub value: ExpressionInstance,
}

/// [LiteralExpression] is used to model a value expression whose value
/// is specified by text in some specified expression language.
#[dmn_element]
#[expression]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralExpression {
  /// The text of this [LiteralExpression].
  /// It SHALL be a valid expression in the `expression_language`.
  pub(crate) text: Option<String>,
  /// Identifies the expression language used in this [LiteralExpression].
  pub(crate) expression_language: Option<String>,
  /// The instance of [ImportedValue](Import) that specifies
  /// where the text of this [LiteralExpression] is located.
  pub(crate) imported_values: Option<Import>,
}

impl LiteralExpression {
  pub fn text(&self) -> &Option<String> {
    &self.text
  }

  pub fn expression_language(&self) -> Option<String> {
    self.expression_language.clone()
  }

  pub fn imported_values(&self) -> Option<Import> {
    self.imported_values.clone()
  }
}

/// [Invocation] is a mechanism for the evaluation of value expressions.
///
/// [Invocation] is a mechanism that permits the evaluation of one value expression
/// – the invoked expression – inside another value expression
/// – the invoking expression – by binding locally the input variables of the invoked
/// expression to values inside the invoking expression.
#[dmn_element]
#[expression]
#[derive(Debug, Clone, PartialEq)]
pub struct Invocation {
  /// An expression whose value is a function.
  pub(crate) called_function: ExpressionInstance,
  /// Instances of [Binding] used to bind the formal parameters of the called function in this [Invocation].
  pub(crate) bindings: Vec<Binding>,
}

impl Invocation {
  /// Returns a reference to called function which is an instance of [Expression].
  pub fn called_function(&self) -> &ExpressionInstance {
    &self.called_function
  }
  /// Returns a reference to the collection of binding instances.
  pub fn bindings(&self) -> &Vec<Binding> {
    &self.bindings
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
  /// The [InformationItem] on which the `calledFunction` of the owning
  /// instance of [Invocation] depends that is bound by this [Binding].
  pub(crate) parameter: InformationItem,
  /// The instance of [Expression] to which the parameter in this [Binding] is
  /// bound when the owning instance of [Invocation] is evaluated.
  pub(crate) binding_formula: Option<ExpressionInstance>,
}

impl Binding {
  /// Returns a reference to parameter.
  pub fn parameter(&self) -> &InformationItem {
    &self.parameter
  }
  /// Returns a reference to binding formula.
  pub fn binding_formula(&self) -> &Option<ExpressionInstance> {
    &self.binding_formula
  }
}

/// [Decision]
#[named_element]
#[dmn_element]
#[derive(Debug, Clone, PartialEq)]
pub struct Decision {
  /// A natural language question that characterizes the [Decision],
  /// such that the output of the [Decision] is an answer to the question.
  pub(crate) question: Option<String>,
  /// A natural language description of the answers allowed for the question
  /// such as `Yes/No`, a list of allowed values, a range of numeric values etc.
  pub(crate) allowed_answers: Option<String>,
  /// The instance of [InformationItem] that stores the result of this [Decision].
  pub(crate) variable: InformationItem,
  /// The instance of the [Expression] for the [Decision].
  pub(crate) decision_logic: Option<ExpressionInstance>,
  /// Collection of the instances of [InformationRequirement] that compose this [Decision].
  pub(crate) information_requirements: Vec<InformationRequirement>,
  /// Collection of the instances of [KnowledgeRequirement] that compose this [Decision].
  pub(crate) knowledge_requirements: Vec<KnowledgeRequirement>,
  /// Collection of the instances of [AuthorityRequirement] that compose this [Decision].
  pub(crate) authority_requirements: Vec<AuthorityRequirement>,
  //TODO add the following:
  //  supported_objectives
  //  impacted_performance_indicator
  //  decision_maker
  //  decision_owner
  //  using_processes
  //  using_tasks
}

impl Decision {
  /// Returns a reference to a natural language question that characterizes the [Decision].
  pub fn question(&self) -> &Option<String> {
    &self.question
  }
  /// Returns a reference to a natural language description of the answers allowed for the question defined in this [Decision].
  pub fn allowed_answers(&self) -> &Option<String> {
    &self.allowed_answers
  }
  /// Return a reference to a variable that stores the result of this [Decision].
  pub fn variable(&self) -> &InformationItem {
    &self.variable
  }
  /// Returns a reference to optional [Expression].
  pub fn decision_logic(&self) -> &Option<ExpressionInstance> {
    &self.decision_logic
  }
  /// Returns a reference to collection of [InformationRequirement].
  pub fn information_requirements(&self) -> &Vec<InformationRequirement> {
    &self.information_requirements
  }
  /// Returns a reference to collection of [KnowledgeRequirement].
  pub fn knowledge_requirements(&self) -> &Vec<KnowledgeRequirement> {
    &self.knowledge_requirements
  }
  /// Returns a reference to collection of [AuthorityRequirement].
  pub fn authority_requirements(&self) -> &Vec<AuthorityRequirement> {
    &self.authority_requirements
  }
}

/// The class [InformationRequirement] is used to model an information requirement,
/// as represented by a plain arrow in a DRD.
#[dmn_element]
#[derive(Debug, Clone, PartialEq)]
pub struct InformationRequirement {
  /// Reference to [Decision] that this [InformationRequirement] associates
  /// with its containing  [Decision] element.
  pub(crate) required_decision: Option<HRef>,
  /// Reference to [InputData] that this [InformationRequirement] associates
  /// with its containing [Decision] element.
  pub(crate) required_input: Option<HRef>,
}

impl InformationRequirement {
  /// Returns reference to optional URI pointing a [Decision].
  pub fn required_decision(&self) -> &Option<HRef> {
    &self.required_decision
  }

  /// Returns reference to optional URI pointing an [InputData].
  pub fn required_input(&self) -> &Option<HRef> {
    &self.required_input
  }
}

/// The class [KnowledgeRequirement] is used to model a knowledge requirement,
/// as represented by a dashed arrow in a DRD.
#[dmn_element]
#[derive(Debug, Clone, PartialEq)]
pub struct KnowledgeRequirement {
  /// Reference to [Invocable] that this [KnowledgeRequirement] associates with
  /// its containing [Decision] or [BusinessKnowledgeModel] element.
  pub(crate) required_knowledge: HRef,
}

impl KnowledgeRequirement {
  /// Returns a reference to the [Invocable].
  pub fn required_knowledge(&self) -> &HRef {
    &self.required_knowledge
  }
}

/// The class [AuthorityRequirement] is used to model an authority requirement,
/// as represented by an arrow drawn with a dashed line and a filled circular head in a DRD
#[dmn_element]
#[derive(Debug, Clone, PartialEq)]
pub struct AuthorityRequirement {
  /// The instance of [KnowledgeSource] that this [AuthorityRequirement] associates
  /// with its containing [KnowledgeSource], [Decision] or [BusinessKnowledgeModel] element.
  pub(crate) required_authority: Option<HRef>,
  /// The instance of [Decision] that this [AuthorityRequirement] associates
  /// with its containing [KnowledgeSource] element.
  pub(crate) required_decision: Option<HRef>,
  /// The instance of [InputData] that this [AuthorityRequirement] associates
  /// with its containing [KnowledgeSource] element.
  pub(crate) required_input: Option<HRef>,
}

impl AuthorityRequirement {
  /// Returns reference to optional [KnowledgeSource].
  pub fn required_authority(&self) -> &Option<HRef> {
    &self.required_authority
  }
  /// Returns reference to optional [Decision].
  pub fn required_decision(&self) -> &Option<HRef> {
    &self.required_decision
  }
  /// Returns reference to optional [InputData].
  pub fn required_input(&self) -> &Option<HRef> {
    &self.required_input
  }
}

/// The class [KnowledgeSource] is used to model authoritative knowledge sources in a decision model.
/// In a DRD, an instance of [KnowledgeSource] is represented by a `knowledge source` diagram element.
#[named_element]
#[dmn_element]
#[derive(Debug, Clone, PartialEq)]
pub struct KnowledgeSource {
  /// Collection of the instances of [AuthorityRequirement] that compose this [Decision].
  pub(crate) authority_requirements: Vec<AuthorityRequirement>,
}

impl KnowledgeSource {
  /// Returns a reference to collection of [AuthorityRequirement].
  pub fn authority_requirements(&self) -> &Vec<AuthorityRequirement> {
    &self.authority_requirements
  }
}

/// A business knowledge model.
///
/// A business knowledge model has an abstract part, representing reusable,
/// invocable decision logic, and a concrete part, which mandates that the decision logic
/// must be a single FEEL boxed function definition.
#[named_element]
#[dmn_element]
#[derive(Debug, Clone, PartialEq)]
pub struct BusinessKnowledgeModel {
  /// Variable that is bound to the function defined by the [FunctionDefinition] for this [BusinessKnowledgeModel].
  pub(crate) variable: InformationItem,
  /// The function that encapsulates the logic encapsulated by this [BusinessKnowledgeModel].
  pub(crate) encapsulated_logic: Option<FunctionDefinition>,
  /// This attribute lists the instances of [KnowledgeRequirement] that compose this [BusinessKnowledgeModel].
  pub(crate) knowledge_requirements: Vec<KnowledgeRequirement>,
  /// This attribute lists the instances of [AuthorityRequirement] that compose this [BusinessKnowledgeModel].
  pub(crate) authority_requirements: Vec<AuthorityRequirement>,
}

impl BusinessKnowledgeModel {
  /// Returns reference to a variable for this [BusinessKnowledgeModel].
  pub fn encapsulated_logic(&self) -> &Option<FunctionDefinition> {
    &self.encapsulated_logic
  }
  /// Returns reference to the collection of instances of [KnowledgeRequirement] that compose this [BusinessKnowledgeModel].
  pub fn knowledge_requirements(&self) -> &Vec<KnowledgeRequirement> {
    &self.knowledge_requirements
  }
  /// Returns reference to the collection of instances of [AuthorityRequirement] that compose this [BusinessKnowledgeModel].
  pub fn authority_requirements(&self) -> &Vec<AuthorityRequirement> {
    &self.authority_requirements
  }
}

impl RequiredVariable for BusinessKnowledgeModel {
  /// Returns reference to a variable for this [BusinessKnowledgeModel].
  fn variable(&self) -> &InformationItem {
    &self.variable
  }
}

/// The [DecisionService] class is used to define named decision services
/// against the decision model contained in an instance of [Definitions].
#[named_element]
#[dmn_element]
#[derive(Debug, Clone, PartialEq)]
pub struct DecisionService {
  /// Variable for this [DecisionService].
  pub(crate) variable: InformationItem,
  /// Collection of references to the instances of [Decision] required to be output by this [DecisionService].
  pub(crate) output_decisions: Vec<HRef>,
  /// Collection of references to the instances of [Decision] to be encapsulated in this [DecisionService].
  pub(crate) encapsulated_decisions: Vec<HRef>,
  /// Collection of references to the instances of [Decision] required as input by this [DecisionService].
  pub(crate) input_decisions: Vec<HRef>,
  /// Collection of references to the instances of [InputData] required as input by this [DecisionService].
  pub(crate) input_data: Vec<HRef>,
}

impl DecisionService {
  /// Returns a reference to collection of references to input [Decision]s for this [DecisionService].
  pub fn input_decisions(&self) -> &Vec<HRef> {
    &self.input_decisions
  }
  /// Returns a reference to collection of references to encapsulated [Decision]s for this [DecisionService].
  pub fn encapsulated_decisions(&self) -> &Vec<HRef> {
    &self.encapsulated_decisions
  }
  /// Returns a reference to collection of references to output [Decision]s for this [DecisionService].
  pub fn output_decisions(&self) -> &Vec<HRef> {
    &self.output_decisions
  }
  /// Returns a reference to collection of references to [InputData] for this [DecisionService].
  pub fn input_data(&self) -> &Vec<HRef> {
    &self.input_data
  }
}

impl RequiredVariable for DecisionService {
  /// Returns reference to a variable for this [DecisionService].
  fn variable(&self) -> &InformationItem {
    &self.variable
  }
}

/// Item definition types.
#[derive(Debug, Clone, PartialEq)]
pub enum ItemDefinitionType {
  SimpleType(FeelType),
  ReferencedType(String, String),
  ComponentType,
  CollectionOfSimpleType(FeelType),
  CollectionOfReferencedType(String, String),
  CollectionOfComponentType,
  FunctionType,
}

/// [ItemDefinition] is used to model the inputs of a decision,
/// whose values are defined outside the decision model.
#[named_element]
#[dmn_element]
#[expression]
#[derive(Debug, Clone, PartialEq)]
pub struct ItemDefinition {
  /// This attribute identifies the type language used to specify the base
  /// type of this [ItemDefinition]. This value overrides the type
  /// language specified in the [Definitions] element. The default is `FEEL`.
  /// The language `SHALL` be specified in URI format.
  pub(crate) type_language: Option<String>,
  /// This attribute contains `FEEL` built-in type only when the `type_language` attribute
  /// is `FEEL` and the `type_ref` attribute defines one of the built-in `FEEL` types.
  pub(crate) feel_type: Option<FeelType>,
  /// Possible values or ranges of values in the base type that are allowed in this [ItemDefinition].
  pub(crate) allowed_values: Option<UnaryTests>,
  /// Defines zero or more nested [ItemDefinitions](ItemDefinition) that compose this [ItemDefinition].
  pub(crate) item_components: Vec<ItemDefinition>,
  /// Setting this flag to true indicates that the actual values defined by
  /// this [ItemDefinition] are collections of allowed values.
  /// The default value is [false].
  pub(crate) is_collection: bool,
  /// Describes an optional [FunctionItem] that compose this [ItemDefinition].
  pub(crate) function_item: Option<FunctionItem>,
}

impl ItemDefinition {
  /// Returns reference to the type language used within the scope of this [ItemDefinition].
  pub fn type_language(&self) -> &Option<String> {
    &self.type_language
  }
  /// Returns reference to possible values or ranges of values
  /// in the base type that are allowed in this [ItemDefinition].
  pub fn allowed_values(&self) -> &Option<UnaryTests> {
    &self.allowed_values
  }
  /// Returns reference to nested [ItemDefinitions](ItemDefinition) that compose this [ItemDefinition].
  pub fn item_components(&self) -> &Vec<ItemDefinition> {
    &self.item_components
  }
  /// Returns mutable reference to nested [ItemDefinitions](ItemDefinition) that compose this [ItemDefinition].
  pub fn item_components_mut(&mut self) -> &mut Vec<ItemDefinition> {
    &mut self.item_components
  }
  /// Returns flag indicating if the actual values are collections of allowed values.
  pub fn is_collection(&self) -> bool {
    self.is_collection
  }
  /// Returns a reference to optional `FEEL` type.
  pub fn feel_type(&self) -> &Option<FeelType> {
    &self.feel_type
  }
  /// Sets the `FEEL` type for this element.
  pub fn set_feel_type(&mut self, feel_type: FeelType) {
    self.feel_type = Some(feel_type);
  }
  /// Returns a reference to an optional [FunctionItem] that compose this [ItemDefinition].
  pub fn function_item(&self) -> &Option<FunctionItem> {
    &self.function_item
  }
}

/// [UnaryTests] is used to model a boolean test, where the argument
/// to be tested is implicit or denoted with a **?**.
/// Test is specified by text in some specified expression language.
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryTests {
  /// The text of this [UnaryTests].
  /// It SHALL be a valid expression in the expressionLanguage.
  pub(crate) text: Option<String>,
  /// This attribute identifies the expression language used in this [UnaryTests].
  /// This value overrides the expression language specified for the containing
  /// instance of DecisionRequirementDiagram.
  /// The language SHALL be specified in a URI format.
  pub(crate) expression_language: Option<String>,
}

impl UnaryTests {
  /// Returns reference to optional text of this [UnaryTests].
  pub fn text(&self) -> &Option<String> {
    &self.text
  }
  /// Returns reference to optional expression language used in this [UnaryTests].
  pub fn expression_language(&self) -> &Option<String> {
    &self.expression_language
  }
}

/// [FunctionItem] defines the signature of a function:
/// the parameters and the output type of the function.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionItem {
  /// Reference to output type of the function.
  pub(crate) output_type_ref: Option<String>,
  /// Function parameters as [InformationItems](InformationItem).
  pub(crate) parameters: Vec<InformationItem>,
}

impl FunctionItem {
  /// Returns reference to output type of function.
  pub fn output_type_ref(&self) -> &Option<String> {
    &self.output_type_ref
  }
  /// Returns reference to function parameters defined
  /// as collection of [InformationItems](InformationItem).
  pub fn parameters(&self) -> &Vec<InformationItem> {
    &self.parameters
  }
}

/// Defines the type of the [FunctionDefinition].
/// The default value is `FEEL`. Supported values also include `Java` and `PMML`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionKind {
  Feel,
  Java,
  Pmml,
}

/// [FunctionItem] defines the signature of a function:
/// the parameters and the output type of the function.
#[dmn_element]
#[expression]
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
  /// Container for instances of [InformationItem] that are the parameters of this [FunctionDefinition].
  pub(crate) formal_parameters: Vec<InformationItem>,
  /// The instance of [Expression] that is the body in this [FunctionDefinition].
  pub(crate) body: Option<ExpressionInstance>,
  /// Type of this [FunctionDefinition], the default value is FEEL.
  pub(crate) kind: FunctionKind,
}

impl FunctionDefinition {
  /// Returns reference to container of [InformationItem] of this [FunctionDefinition].
  pub fn formal_parameters(&self) -> &Vec<InformationItem> {
    &self.formal_parameters
  }
  /// Returns reference to [Expression] that is the body in this [FunctionDefinition].
  pub fn body(&self) -> &Option<ExpressionInstance> {
    &self.body
  }
  /// Returns the type of this [FunctionDefinition].
  pub fn kind(&self) -> &FunctionKind {
    &self.kind
  }
}

/// A [Relation] is convenient a shorthand for a list of similar contexts.
///
/// A [Relation] has a column instead of repeated `ContextEntry`s,
/// and a `List` is used for every row, with one of the `List`’s
/// expression for each column value.
#[dmn_element]
#[expression]
#[derive(Debug, Clone, PartialEq)]
pub struct Relation {
  /// This attribute lists the instances of [List] that are the rows in this [Relation].
  pub(crate) rows: Vec<List>,
  /// This attributes lists the instances of [InformationItem] that define the columns in this [Relation].
  pub(crate) columns: Vec<InformationItem>,
}

impl Relation {
  /// Returns a reference to collection of relation's rows.
  pub fn rows(&self) -> &Vec<List> {
    &self.rows
  }
  /// Returns a reference to collection of relation's columns.
  pub fn columns(&self) -> &Vec<InformationItem> {
    &self.columns
  }
}

/// A [List] is simply a list of elements, which are instances of [Expression]s.
#[dmn_element]
#[expression]
#[derive(Debug, Clone, PartialEq)]
pub struct List {
  /// This attribute lists the instances of [Expression] that are the elements of this [List].
  pub(crate) elements: Vec<ExpressionInstance>,
}

impl List {
  /// Returns a reference to collection of list's elements.
  pub fn elements(&self) -> &Vec<ExpressionInstance> {
    &self.elements
  }
}

/// A [Conditional] is a representation of a visual way to express an if statement.
#[dmn_element]
#[expression]
#[derive(Debug, Clone, PartialEq)]
pub struct Conditional {
  /// This attribute holds the expression that is evaluated by the conditional expression.
  pub(crate) if_expression: ChildExpression,
  /// This attribute holds the expression that will be evaluated when the condition in the if statement evaluates to `true`.
  pub(crate) then_expression: ChildExpression,
  /// This attribute holds the expression that will be evaluated when the condition in the if statement evaluates to `false`.
  pub(crate) else_expression: ChildExpression,
}

impl Conditional {
  pub fn if_expression(&self) -> &ChildExpression {
    &self.if_expression
  }

  pub fn then_expression(&self) -> &ChildExpression {
    &self.then_expression
  }

  pub fn else_expression(&self) -> &ChildExpression {
    &self.else_expression
  }
}

/// A [Filter] is a visual way to express list filtering.
#[dmn_element]
#[expression]
#[derive(Debug, Clone, PartialEq)]
pub struct Filter {
  /// This attribute holds the expression that is evaluated as the collection to be filtered.
  pub(crate) in_expression: ChildExpression,
  /// This attribute holds the expression that is used to filter the collection.
  pub(crate) match_expression: ChildExpression,
}

impl Filter {
  pub fn in_expression(&self) -> &ChildExpression {
    &self.in_expression
  }

  pub fn match_expression(&self) -> &ChildExpression {
    &self.match_expression
  }
}

/// A [For] is a visual representation of a loop.
#[dmn_element]
#[expression]
#[derive(Debug, Clone, PartialEq)]
pub struct For {
  /// This attribute holds name of the iterator variable that will be populated at each iteration.
  pub(crate) iterator_variable: String,
  /// This attribute holds the expression that is evaluated as the collection to be processed.
  pub(crate) in_expression: TypedChildExpression,
  /// This attribute holds the expression that is evaluated to create the new collection that will be returned.
  pub(crate) return_expression: ChildExpression,
}

impl For {
  pub fn iterator_variable(&self) -> &String {
    &self.iterator_variable
  }

  pub fn in_expression(&self) -> &TypedChildExpression {
    &self.in_expression
  }

  pub fn return_expression(&self) -> &ChildExpression {
    &self.return_expression
  }
}

/// A [Every] is a visual representation of an expression where all
/// `satisfies` needs to be true for it to return true.
#[dmn_element]
#[expression]
#[derive(Debug, Clone, PartialEq)]
pub struct Every {
  /// This attribute holds name of the iterator variable that will be populated at each iteration.
  pub(crate) iterator_variable: String,
  /// This attribute holds the expression that is evaluated as the collection to be processed.
  pub(crate) in_expression: TypedChildExpression,
  /// This attribute holds the expression that is evaluated to determine if the current item satisfies a condition.
  pub(crate) satisfies_expression: ChildExpression,
}

impl Every {
  pub fn iterator_variable(&self) -> &String {
    &self.iterator_variable
  }

  pub fn in_expression(&self) -> &TypedChildExpression {
    &self.in_expression
  }

  pub fn satisfies_expression(&self) -> &ChildExpression {
    &self.satisfies_expression
  }
}

/// A [Some] is a visual representation of an expression where at least one of the
/// `satisfies` needs to be true for it to return true.
#[dmn_element]
#[expression]
#[derive(Debug, Clone, PartialEq)]
pub struct Some {
  /// This attribute holds name of the iterator variable that will be populated at each iteration.
  pub(crate) iterator_variable: String,
  /// This attribute holds the expression that is evaluated as the collection to be processed.
  pub(crate) in_expression: TypedChildExpression,
  /// This attribute holds the expression that is evaluated to determine if the current item satisfies a condition.
  pub(crate) satisfies_expression: ChildExpression,
}

impl Some {
  pub fn iterator_variable(&self) -> &String {
    &self.iterator_variable
  }

  pub fn in_expression(&self) -> &TypedChildExpression {
    &self.in_expression
  }

  pub fn satisfies_expression(&self) -> &ChildExpression {
    &self.satisfies_expression
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChildExpression {
  /// Optional identifier of this [ChildExpression].
  pub(crate) id: DmnId,
  /// The instance of [Expression] trait that is the expression in this [ChildExpression].
  pub(crate) value: ExpressionInstance,
}

impl ChildExpression {
  pub fn id(&self) -> &DmnId {
    &self.id
  }

  pub fn value(&self) -> &ExpressionInstance {
    &self.value
  }
}

#[expression]
#[derive(Debug, Clone, PartialEq)]
pub struct TypedChildExpression {
  /// Optional identifier of this [TypedChildExpression].
  pub(crate) id: DmnId,
  /// The instance of [Expression] trait that is the expression in this [TypedChildExpression].
  pub(crate) value: ExpressionInstance,
}

impl TypedChildExpression {
  pub fn id(&self) -> &DmnId {
    &self.id
  }

  pub fn value(&self) -> &ExpressionInstance {
    &self.value
  }
}

/// Decision table.
#[dmn_element]
#[expression]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionTable {
  /// Information item name, for which the decision table is its value expression.
  /// This is usually the name of the decision or the name of business knowledge model for
  /// which the decision table provides the decision logic.
  pub(crate) information_item_name: Option<String>,
  /// List of instances of input clause that compose this decision table.
  pub(crate) input_clauses: Vec<InputClause>,
  /// List of instances of output clause that compose this decision table.
  pub(crate) output_clauses: Vec<OutputClause>,
  /// List of instances of rule annotation clause that compose this decision table.
  pub(crate) annotations: Vec<RuleAnnotationClause>,
  /// List of instances of decision rule that compose this decision table.
  pub(crate) rules: Vec<DecisionRule>,
  /// Hit policy associated with the instance of the decision table.
  pub(crate) hit_policy: HitPolicy,
  /// Optional aggregation type when the hit policy is `COLLECT`.
  pub(crate) aggregation: Option<BuiltinAggregator>,
  /// Preferred representation of the instance of the decision table.
  pub(crate) preferred_orientation: DecisionTableOrientation,
  /// Optional output label for the description of the decision table output,
  /// may be the same as the name of the information item for which the
  /// decision table is the value expression.
  pub(crate) output_label: Option<String>,
}

impl Display for DecisionTable {
  /// Implements [Display] trait for [DecisionTable].
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut buffer = String::new();
    buffer.push_str("Decision table:\n");
    buffer.push_str(format!(">> preferred orientation: {}\n", self.preferred_orientation).as_str());
    buffer.push_str(">> information item name: ");
    if let Some(text) = &self.information_item_name {
      buffer.push_str(format!("\n'{text}'\n").as_str());
    } else {
      buffer.push_str("none\n");
    }
    buffer.push_str(format!(">> hit policy: {}\n", self.hit_policy).as_str());
    buffer.push_str(">> aggregation: ");
    if let Some(aggregation) = &self.aggregation {
      buffer.push_str(format!("{aggregation}\n").as_str());
    } else {
      buffer.push_str("none\n");
    }
    buffer.push_str(">> output label: ");
    if let Some(text) = &self.output_label {
      buffer.push_str(format!("\n'{text}'\n").as_str());
    } else {
      buffer.push_str("none\n");
    }
    write!(f, "{buffer}")
  }
}

impl DecisionTable {
  /// Creates a new decision table.
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    information_item_name: Option<String>,
    input_clauses: Vec<InputClause>,
    output_clauses: Vec<OutputClause>,
    annotations: Vec<RuleAnnotationClause>,
    rules: Vec<DecisionRule>,
    hit_policy: HitPolicy,
    aggregation: Option<BuiltinAggregator>,
    preferred_orientation: DecisionTableOrientation,
    output_label: Option<String>,
  ) -> Self {
    Self {
      namespace: "".to_string(),
      model_name: "".to_string(),
      id: DmnId::Generated(gen_id()),
      description: None,
      label: None,
      extension_elements: vec![],
      extension_attributes: vec![],
      type_ref: None,
      information_item_name,
      input_clauses,
      output_clauses,
      annotations,
      rules,
      hit_policy,
      aggregation,
      preferred_orientation,
      output_label,
    }
  }

  /// Returns the information item name.
  pub fn information_item_name(&self) -> &Option<String> {
    &self.information_item_name
  }

  /// Returns an iterator over input clauses.
  pub fn input_clauses(&self) -> Iter<InputClause> {
    self.input_clauses.iter()
  }

  /// Returns an iterator over output clauses.
  pub fn output_clauses(&self) -> Iter<OutputClause> {
    self.output_clauses.iter()
  }

  /// Returns an iterator over annotations.
  pub fn annotations(&self) -> Iter<RuleAnnotationClause> {
    self.annotations.iter()
  }

  /// Returns an iterator over the rules.
  pub fn rules(&self) -> Iter<DecisionRule> {
    self.rules.iter()
  }

  /// Returns the [HitPolicy] of this decision table.
  pub fn hit_policy(&self) -> HitPolicy {
    self.hit_policy
  }

  /// Returns the aggregation when the [HitPolicy] is `COLLECT`.
  pub fn aggregation(&self) -> &Option<BuiltinAggregator> {
    &self.aggregation
  }

  /// Returns preferred orientation for this decision table.
  pub fn preferred_orientation(&self) -> &DecisionTableOrientation {
    &self.preferred_orientation
  }

  /// Return an output label.
  pub fn output_label(&self) -> &Option<String> {
    &self.output_label
  }

  /// Returns `true` when allowed input and/or allowed output values are present in decision table.
  pub fn allowed_values_present(&self) -> bool {
    for input_clause in &self.input_clauses {
      if input_clause.allowed_input_values.is_some() {
        return true;
      }
    }
    for output_clause in &self.output_clauses {
      if output_clause.allowed_output_values.is_some() {
        return true;
      }
    }
    false
  }
}

/// Orientation of the decision table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionTableOrientation {
  /// Decision table is presented horizontally, rules are presented as rows.
  RuleAsRow,
  /// Decision table is presented vertically, rules are presented as columns.
  RuleAsColumn,
  /// Decision table is presented as crosstab, rules are composed of two input dimensions.
  CrossTable,
}

impl Display for DecisionTableOrientation {
  /// Implements [Display] trait for [DecisionTableOrientation].
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      DecisionTableOrientation::RuleAsRow => write!(f, "Rule-as-Row"),
      DecisionTableOrientation::RuleAsColumn => write!(f, "Rule-as-Column"),
      DecisionTableOrientation::CrossTable => write!(f, "CrossTable"),
    }
  }
}

impl TryFrom<&str> for DecisionTableOrientation {
  type Error = DsntkError;
  /// Tries to construct a decision table orientation from its text representation.
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value.trim() {
      "Rule-as-Row" => Ok(DecisionTableOrientation::RuleAsRow),
      "Rule-as-Column" => Ok(DecisionTableOrientation::RuleAsColumn),
      "CrossTable" => Ok(DecisionTableOrientation::CrossTable),
      other => Err(err_invalid_decision_table_orientation(other)),
    }
  }
}

/// Hit policy.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HitPolicy {
  /// `UNIQUE` hit policy. No overlapping rules are allowed, only single rule can be matched.
  /// This is the default value for hit policy. Crosstab decision tables may have only unique hit policy.
  Unique,
  /// `ANY` hit policy. Rules may overlap, but all matching rules show equal output entries.
  /// If matching rules have non-equal output entries, this policy is incorrect and the result is undefined.
  Any,
  /// `PRIORITY` hit policy. Multiple rules can match, with different output entries for each output.
  /// This policy returns matching rule with the highest priority. Output priorities are specified
  /// in the ordered list of output values, in decreasing order of priority.
  Priority,
  /// `FIRST` hit policy...
  First,
  /// `COLLECT` hit policy...
  Collect(BuiltinAggregator),
  /// `OUTPUT ORDER` hit policy...
  OutputOrder,
  /// `RULE ORDER` hit policy...
  RuleOrder,
}

impl Display for HitPolicy {
  /// Implements [Display] trait for [HitPolicy].
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      HitPolicy::Unique => write!(f, "U"),
      HitPolicy::Any => write!(f, "A"),
      HitPolicy::Priority => write!(f, "P"),
      HitPolicy::First => write!(f, "F"),
      HitPolicy::Collect(aggregator) => write!(f, "{aggregator}"),
      HitPolicy::OutputOrder => write!(f, "O"),
      HitPolicy::RuleOrder => write!(f, "R"),
    }
  }
}

impl TryFrom<&str> for HitPolicy {
  type Error = DsntkError;
  /// Creates a hit policy from text.
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value.trim() {
      "U" => Ok(HitPolicy::Unique),
      "A" => Ok(HitPolicy::Any),
      "P" => Ok(HitPolicy::Priority),
      "F" => Ok(HitPolicy::First),
      "R" => Ok(HitPolicy::RuleOrder),
      "O" => Ok(HitPolicy::OutputOrder),
      "C" => Ok(HitPolicy::Collect(BuiltinAggregator::List)),
      "C+" => Ok(HitPolicy::Collect(BuiltinAggregator::Sum)),
      "C#" => Ok(HitPolicy::Collect(BuiltinAggregator::Count)),
      "C<" => Ok(HitPolicy::Collect(BuiltinAggregator::Min)),
      "C>" => Ok(HitPolicy::Collect(BuiltinAggregator::Max)),
      other => Err(err_invalid_decision_table_hit_policy(other)),
    }
  }
}

/// Aggregator function for `COLLECT` [hit policy](HitPolicy).
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BuiltinAggregator {
  /// The result of the decision table is a list of output entries.
  List,
  /// The result of the decision table is the number of outputs.
  Count,
  /// The result of the decision table is the sum of all the outputs.
  Sum,
  /// The result of the decision table is the smallest value of all the outputs.
  Min,
  /// The result of the decision table is the largest value of all the outputs.
  Max,
}

impl Display for BuiltinAggregator {
  /// Implements [Display] trait for [BuiltinAggregator].
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        BuiltinAggregator::List => "C",
        BuiltinAggregator::Count => "C#",
        BuiltinAggregator::Sum => "C+",
        BuiltinAggregator::Min => "C<",
        BuiltinAggregator::Max => "C>",
      }
    )
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputClause {
  /// The subject of this input clause, text representation of unary tests.
  pub input_expression: String,
  /// Optional unary tests that constrain the result of input expression of this input clause.
  pub allowed_input_values: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputClause {
  /// Type reference may specify the type to be used as decision table's output when more than one output clause is present.
  pub type_ref: Option<String>,
  /// The name of the output component when the decision table contains more than one output clause.
  pub name: Option<String>,
  /// Unary tests that constrain the result of output entries corresponding to this output clause.
  pub allowed_output_values: Option<String>,
  /// Default output expression, selected in incomplete table when no rules match for the decision table.
  pub default_output_entry: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleAnnotationClause {
  /// Name that is used as the name of the rule annotation column of the containing decision table.
  pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionRule {
  /// Ordered list of input entries that compose this decision rule.
  pub input_entries: Vec<InputEntry>,
  /// Ordered list of output entries that compose this decision rule.
  pub output_entries: Vec<OutputEntry>,
  /// Ordered list of rule annotations that compose this decision rule.
  pub annotation_entries: Vec<AnnotationEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputEntry {
  /// Text representation of unary test that composes this input entry.
  pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputEntry {
  /// Text representation of literal expression that composes this output entry.
  pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationEntry {
  /// Text representing this rule annotation.
  pub text: String,
}

/// [Dmndi] is a container for the shared [DmnStyle](DmnStyle)s
/// and all [DmnDiagram]s defined in [Definitions].
#[derive(Debug, Clone, PartialEq)]
pub struct Dmndi {
  /// A list of shared [DmnStyle] that can be referenced
  /// by all [DmnDiagram] and [DmnDiagramElement].
  pub styles: Vec<DmnStyle>,
  /// A list of [DmnDiagram].
  pub diagrams: Vec<DmnDiagram>,
}

/// Defines possible elements of [DmnDiagramElement].
#[derive(Debug, Clone, PartialEq)]
pub enum DmnDiagramElement {
  DmnShape(DmnShape),
  DmnEdge(DmnEdge),
}

/// [DmnDiagram] is the container of [DmnDiagramElement] ([DmnShape] (s) and [DmnEdge] (s)).
/// [DmnDiagram] cannot include other [DmnDiagrams](DmnDiagram).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DmnDiagram {
  /// [DmnDiagram] id.
  pub id: Option<String>,
  /// The name of the diagram. Default is empty [String].
  pub name: Option<String>,
  /// The documentation of the diagram. Default is empty [String].
  pub documentation: String,
  /// The resolution of the diagram expressed in user units per inch. Default is 300.
  pub resolution: f64,
  /// A list of [DmnDiagramElement] ([DmnShape] and [DmnEdge]) that are depicted in this diagram.
  pub diagram_elements: Vec<DmnDiagramElement>,
  /// A reference to a [DmnStyle] defined in the [Dmndi] that serves as the default styling
  /// of the [DmnDiagramElement] in this [DmnDiagram].
  pub shared_style: Option<String>,
  /// A [DmnStyle] that defines the default styling for this diagram.
  /// Properties defined in that style override the ones in the 'sharedStyle'.
  pub local_style: Option<DmnStyle>,
  /// The size of this diagram. If not specified, the [DmnDiagram] is unbounded.
  pub size: Option<DcDimension>,
}

/// [DmnShape] represents a [Decision], a [BusinessKnowledgeModel], an [InputData] element,
/// a [KnowledgeSource], a [DecisionService] or a [TextAnnotation] that is depicted on the diagram.
#[derive(Debug, Clone, PartialEq)]
pub struct DmnShape {
  /// Unique identifier of this [DmnShape].
  pub id: Option<String>,
  /// The [DcBounds] of the shape relative to the origin of its parent [DmnDiagram]. The [DcBounds] MUST be specified.
  pub bounds: DcBounds,
  /// A  reference  to  a  [Decision], a [BusinessKnowledgeModel], an [InputData] element,
  /// a [KnowledgeSource], a [DecisionService] or a [TextAnnotation] MUST be specified.
  pub dmn_element_ref: Option<String>,
  /// If the [DmnShape] depicts an [InputData] element then this attribute is used to determine
  /// if the [InputData] is listed on the [Decision] element (true) or drawn as separate notational elements in the DRD (false).
  pub is_listed_input_data: bool,
  /// If the [DmnShape] depicts a [DecisionService], this attribute references a [DmnDecisionServiceDividerLine] that defines,
  /// where the [DmnShape] is divided into two parts by a straight solid line.
  /// This can be the case when a [DmnShape] depicts a [DecisionService],
  /// where the set of output decisions is smaller than the set of encapsulated decisions.
  /// The start and end waypoints of the `decisionServiceDividerLine` **MUST** be on the border of the [DmnShape].
  pub decision_service_divider_line: Option<DmnDecisionServiceDividerLine>,
  /// If the [DmnShape] depicts a [DecisionService], this attribute indicates
  /// if it should be depicted expanded (`false`) or collapsed (`true`).
  /// Default is `false`.
  pub is_collapsed: bool,
  /// A reference to a [DmnStyle] defined in the [Dmndi].
  pub shared_style: Option<String>,
  /// A [DmnStyle] that defines the styling for this element.
  pub local_style: Option<DmnStyle>,
  /// An optional label when this [DmnElement] has a visible text label.
  pub label: Option<DmnLabel>,
}

/// Struct defines line inside [DecisionService].
#[derive(Debug, Clone, PartialEq)]
pub struct DmnDecisionServiceDividerLine {
  pub id: Option<String>,
  /// A list of points relative to the origin of its parent [DmnDiagram] that specifies
  /// the connected line segments of the edge. At least two (2) waypoint`s MUST be specified.
  /// Waypoint must be on the border of the [DmnShape].
  pub way_points: Vec<DcPoint>,
  /// A reference to a [DmnStyle] defined in the [Dmndi].
  pub shared_style: Option<String>,
  /// A [DmnStyle] that defines the styling for this element.
  pub local_style: Option<DmnStyle>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DmnEdge {
  pub id: Option<String>,
  /// A list of points relative to the origin of its parent [DmnDiagram] that specifies
  /// the connected line segments of the edge. At least two (2) waypoints MUST be specified.
  pub way_points: Vec<DcPoint>,
  /// A reference to a [InformationRequirement], [KnowledgeRequirement],
  /// [AuthorityRequirement] or an [Association], MUST be specified.
  pub dmn_element_ref: Option<String>,
  /// The actual [DmnDiagramElement] this [DmnEdge] is connecting from.
  /// MUST be specified when the [DmnEdge] has a source.
  pub source_element: Option<String>,
  /// The actual [DmnDiagramElement] this [DmnEdge] is connecting to.
  /// MUST be specified when the [DmnEdge] has a target.
  pub target_element: Option<String>,
  /// A reference to a [DmnStyle] defined in the [Dmndi].
  pub shared_style: Option<String>,
  /// A [DmnStyle] that defines the styling for this element.
  pub local_style: Option<DmnStyle>,
  /// An optional label when this [DmnElement] has a visible text label.
  pub label: Option<DmnLabel>,
}

//FIXME verify this struct
/// tdb
#[derive(Debug, Clone)]
pub struct Association {}

//FIXME verify this struct
/// tdb
#[derive(Debug, Clone)]
pub struct TextAnnotation {}

/// [DmnStyle] is used to keep some non-normative visual attributes such as color and font.
#[derive(Debug, Clone, PartialEq)]
pub struct DmnStyle {
  /// A unique identifier for this style so it can be referenced.
  /// Only styles defined in the [Dmndi] can be referenced by [DmnDiagramElement] and [DmnDiagram].
  pub id: String,
  /// The color use to fill the shape.Does not apply to [DmnEdge]. Default is `white`.
  pub fill_color: Option<DcColor>,
  /// The color use to draw the shape borders. Default is `black`.
  pub stroke_color: Option<DcColor>,
  /// The color use to draw the label. Default is `black`.
  pub font_color: Option<DcColor>,
  /// A comma-separated list of Font Name that can be used to display the text. Default is `Arial`.
  pub font_family: Option<String>,
  /// The size in points of the font to use to display the text. Default is `8` points.
  pub font_size: Option<f64>,
  /// If the text should be displayed in Italic. Default is `false`.
  pub font_italic: Option<bool>,
  /// If the text should be displayed in Bold. Default is `false`.
  pub font_bold: Option<bool>,
  /// If the text should be underlined. Default is `false`.
  pub font_underline: Option<bool>,
  /// If the text should be stroked through. Default is `false`.
  pub font_strike_through: Option<bool>,
  /// How text should be positioned horizontally within the Label bounds.
  /// Default depends on the [DmnDiagramElement] the label is attached to (see 13.5).
  pub label_horizontal_alignment: Option<DcAlignmentKind>,
  /// How  the  text  should  be  positioned  vertically  inside  the  Label  bounds.
  /// Default depends on the [DmnDiagramElement] the label is attached to (see 13.5).
  /// Start means `top` and end means `bottom`.
  pub label_vertical_alignment: Option<DcAlignmentKind>,
}

/// Struct represents the depiction of some textual information about a DMN element.
#[derive(Debug, Clone, PartialEq)]
pub struct DmnLabel {
  /// The bounds of the [DmnLabel]. When not specified, the label is positioned
  /// at its default position as determined in clause 13.5.
  pub bounds: Option<DcBounds>,
  /// An optional pretty printed text that MUST be displayed
  /// instead of the [DmnElement's](DmnElement) name if it is present.
  pub text: Option<String>,
  /// A reference to a [DmnStyle] defined in the [Dmndi].
  pub shared_style: Option<String>,
}

/// Defines RGB color.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DcColor {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

impl DcColor {
  pub fn white() -> Self {
    Self {
      red: 0xFF,
      green: 0xFF,
      blue: 0xFF,
    }
  }

  pub fn black() -> Self {
    Self { red: 0x0, green: 0x0, blue: 0x0 }
  }
}

/// Defines point.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DcPoint {
  pub x: f64,
  pub y: f64,
}

/// Defines bounds.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DcBounds {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
}

/// Defines dimensions.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DcDimension {
  pub width: f64,
  pub height: f64,
}

/// Defines the kind of element alignment.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DcAlignmentKind {
  /// Left or top.
  Start,
  /// Right or bottom.
  End,
  /// Center or middle.
  Center,
}

/// Defines known colors.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DcKnownColor {
  Aqua = 0x00FFFF,
  Black = 0x000000,
  Blue = 0x0000FF,
  Fuchsia = 0xFF00FF,
  Gray = 0x808080,
  Green = 0x008000,
  Lime = 0x00FF00,
  Maroon = 0x800000,
  Navy = 0x000080,
  Olive = 0x808000,
  Orange = 0xFFA500,
  Purple = 0x800080,
  Red = 0xFF0000,
  Silver = 0xC0C0C0,
  Teal = 0x008080,
  White = 0xFFFFFF,
  Yellow = 0xFFFF00,
}
