//! # Builder for item definition type evaluators

use crate::errors::*;
use crate::model_definitions::{DefDefinitions, DefItemDefinition, DefKey};
use crate::type_ref::type_ref_to_feel_type;
use dsntk_common::Result;
use dsntk_feel::{FeelType, Name, FEEL_TYPE_NAME_ANY};
use dsntk_model::ItemDefinitionType;
use std::collections::{BTreeMap, HashMap};

/// Type of function that evaluates the item definition type.
type ItemDefinitionTypeEvaluatorFn = Box<dyn Fn(&ItemDefinitionTypeEvaluator) -> Option<FeelType> + Send + Sync>;

/// Type alias for a collection of information item types.
pub type InformationItemTypes = HashMap<DefKey, FeelType>;

/// Item definition type evaluators.
#[derive(Default)]
pub struct ItemDefinitionTypeEvaluator {
  evaluators: HashMap<DefKey, ItemDefinitionTypeEvaluatorFn>,
}

impl ItemDefinitionTypeEvaluator {
  /// Creates item definition type evaluators.
  pub fn new(definitions: &DefDefinitions) -> Result<Self> {
    let mut evaluators = HashMap::new();
    for item_definition in definitions.item_definitions() {
      let evaluator = build_item_definition_type_evaluator(item_definition)?;
      let namespace = item_definition.namespace();
      let type_ref = item_definition.name();
      let def_key = DefKey::new(namespace, type_ref);
      evaluators.insert(def_key, evaluator);
    }
    Ok(Self { evaluators })
  }

  /// Returns FEEL type for specified type reference.
  pub fn information_item_type(&self, namespace: &str, type_ref: &str) -> Option<FeelType> {
    if type_ref == FEEL_TYPE_NAME_ANY {
      return Some(FeelType::Any);
    }
    if let Some(simple_type_ref) = type_ref_to_feel_type(type_ref) {
      Some(simple_type_ref.clone())
    } else {
      self.eval(&DefKey::new(namespace, type_ref))
    }
  }

  /// Returns a map of all types for model defined information items.
  pub fn information_item_types(&self) -> InformationItemTypes {
    let mut item_types = HashMap::new();
    for def_key in self.evaluators.keys() {
      if let Some(feel_type) = self.information_item_type(def_key.namespace(), def_key.id()) {
        item_types.insert(def_key.clone(), feel_type);
      }
    }
    item_types
  }

  /// Evaluates a type of the item definition with specified key.
  fn eval(&self, def_key: &DefKey) -> Option<FeelType> {
    if let Some(evaluator) = self.evaluators.get(def_key) {
      evaluator(self)
    } else {
      None
    }
  }
}

pub fn build_item_definition_type_evaluator(item_definition: &DefItemDefinition) -> Result<ItemDefinitionTypeEvaluatorFn> {
  match item_definition.item_definition_type()? {
    ItemDefinitionType::SimpleType(feel_type) => simple_type(feel_type),
    ItemDefinitionType::ReferencedType(namespace, ref_type) => referenced_type(DefKey::new(&namespace, &ref_type)),
    ItemDefinitionType::ComponentType => component_type(item_definition),
    ItemDefinitionType::CollectionOfSimpleType(feel_type) => collection_of_simple_type(feel_type),
    ItemDefinitionType::CollectionOfReferencedType(namespace, ref_type) => collection_of_referenced_type(DefKey::new(&namespace, &ref_type)),
    ItemDefinitionType::CollectionOfComponentType => collection_of_component_type(item_definition),
    ItemDefinitionType::FunctionType => function_type(item_definition),
  }
}

fn simple_type(feel_type: FeelType) -> Result<ItemDefinitionTypeEvaluatorFn> {
  match feel_type {
    FeelType::Any => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::Any))),
    FeelType::String => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::String))),
    FeelType::Number => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::Number))),
    FeelType::Boolean => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::Boolean))),
    FeelType::Date => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::Date))),
    FeelType::Time => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::Time))),
    FeelType::DateTime => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::DateTime))),
    FeelType::DaysAndTimeDuration => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::DaysAndTimeDuration))),
    FeelType::YearsAndMonthsDuration => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::YearsAndMonthsDuration))),
    _ => Err(err_unsupported_feel_type(feel_type, "e")),
  }
}

fn referenced_type(def_key: DefKey) -> Result<ItemDefinitionTypeEvaluatorFn> {
  Ok(Box::new(move |evaluators: &ItemDefinitionTypeEvaluator| evaluators.eval(&def_key)))
}

fn component_type(item_definition: &DefItemDefinition) -> Result<ItemDefinitionTypeEvaluatorFn> {
  let mut type_evaluators: Vec<(Name, ItemDefinitionTypeEvaluatorFn)> = vec![];
  for component_item_definition in item_definition.item_components() {
    type_evaluators.push((
      component_item_definition.feel_name().clone(),
      build_item_definition_type_evaluator(component_item_definition)?,
    ));
  }
  Ok(Box::new(move |evaluators: &ItemDefinitionTypeEvaluator| {
    let mut entries = BTreeMap::new();
    for (component_name, component_evaluator) in &type_evaluators {
      if let Some(feel_type) = component_evaluator(evaluators) {
        entries.insert(component_name.clone(), feel_type);
      }
    }
    Some(FeelType::Context(entries))
  }))
}

fn collection_of_simple_type(feel_type: FeelType) -> Result<ItemDefinitionTypeEvaluatorFn> {
  match feel_type {
    FeelType::Any => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::Any)))),
    FeelType::String => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::String)))),
    FeelType::Number => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::Number)))),
    FeelType::Boolean => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::Boolean)))),
    FeelType::Date => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::Date)))),
    FeelType::Time => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::Time)))),
    FeelType::DateTime => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::DateTime)))),
    FeelType::DaysAndTimeDuration => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::DaysAndTimeDuration)))),
    FeelType::YearsAndMonthsDuration => Ok(Box::new(move |_: &ItemDefinitionTypeEvaluator| Some(FeelType::list(&FeelType::YearsAndMonthsDuration)))),
    _ => Err(err_unsupported_feel_type(feel_type, "f")),
  }
}

fn collection_of_referenced_type(def_key: DefKey) -> Result<ItemDefinitionTypeEvaluatorFn> {
  Ok(Box::new(move |evaluators: &ItemDefinitionTypeEvaluator| {
    evaluators.eval(&def_key).map(|feel_type| FeelType::List(Box::new(feel_type)))
  }))
}

fn collection_of_component_type(item_definition: &DefItemDefinition) -> Result<ItemDefinitionTypeEvaluatorFn> {
  let mut type_evaluators: Vec<(Name, ItemDefinitionTypeEvaluatorFn)> = vec![];
  for component_item_definition in item_definition.item_components() {
    type_evaluators.push((
      component_item_definition.feel_name().clone(),
      build_item_definition_type_evaluator(component_item_definition)?,
    ));
  }
  Ok(Box::new(move |evaluators: &ItemDefinitionTypeEvaluator| {
    let mut entries = BTreeMap::new();
    for (component_name, component_evaluator) in &type_evaluators {
      if let Some(feel_type) = component_evaluator(evaluators) {
        entries.insert(component_name.clone(), feel_type);
      }
    }
    Some(FeelType::List(Box::new(FeelType::Context(entries))))
  }))
}

fn function_type(item_definition: &DefItemDefinition) -> Result<ItemDefinitionTypeEvaluatorFn> {
  let namespace = item_definition.namespace().to_string();
  let mut output_type_ref = FEEL_TYPE_NAME_ANY.to_string();
  let mut parameters_type_ref = vec![];
  if let Some(function_item) = item_definition.function_item() {
    if let Some(type_ref) = function_item.output_type_ref() {
      type_ref.clone_into(&mut output_type_ref);
    }
    for parameter_information_item in function_item.parameters() {
      parameters_type_ref.push(parameter_information_item.type_ref().to_string());
    }
  }
  Ok(Box::new(move |evaluator: &ItemDefinitionTypeEvaluator| {
    let output_type = evaluator.information_item_type(&namespace, &output_type_ref).unwrap_or(FeelType::Any);
    let parameter_types = parameters_type_ref
      .iter()
      .map(|parameter_type_ref| evaluator.information_item_type(&namespace, parameter_type_ref).unwrap_or(FeelType::Any))
      .collect::<Vec<FeelType>>();
    Some(FeelType::function(&parameter_types, &output_type))
  }))
}

#[cfg(test)]
mod tests {
  use crate::item_definition_type::ItemDefinitionTypeEvaluator;
  use crate::model_definitions::{DefDefinitions, DefKey};
  use dsntk_examples::item_definition::*;
  use dsntk_feel::{FeelType, Name};

  const NAMESPACE: &str = "https://dsntk.io";

  /// Utility function for building item definition type evaluator from definitions.
  fn build_evaluator(xml: &str) -> ItemDefinitionTypeEvaluator {
    let definitions = dsntk_model::from_xml(xml).unwrap();
    let mut def_definitions = DefDefinitions::default();
    def_definitions.add_model(&definitions);
    ItemDefinitionTypeEvaluator::new(&def_definitions).unwrap()
  }

  #[test]
  fn simple_type_string() {
    let evaluator = build_evaluator(DMN_0101);
    assert_eq!(Some(FeelType::String), evaluator.eval(&DefKey::new(NAMESPACE, "tCustomerName")));
  }

  #[test]
  fn simple_type_number() {
    let evaluator = build_evaluator(DMN_0102);
    assert_eq!(Some(FeelType::Number), evaluator.eval(&DefKey::new(NAMESPACE, "tMonthlySalary")));
  }

  #[test]
  fn simple_type_boolean() {
    let evaluator = build_evaluator(DMN_0103);
    assert_eq!(Some(FeelType::Boolean), evaluator.eval(&DefKey::new(NAMESPACE, "tIsAffordable")));
  }

  #[test]
  fn simple_type_date() {
    let evaluator = build_evaluator(DMN_0104);
    assert_eq!(Some(FeelType::Date), evaluator.eval(&DefKey::new(NAMESPACE, "tBirthday")));
  }

  #[test]
  fn simple_type_time() {
    let evaluator = build_evaluator(DMN_0105);
    assert_eq!(Some(FeelType::Time), evaluator.eval(&DefKey::new(NAMESPACE, "tDeliveryTime")));
  }

  #[test]
  fn simple_type_date_time() {
    let evaluator = build_evaluator(DMN_0106);
    assert_eq!(Some(FeelType::DateTime), evaluator.eval(&DefKey::new(NAMESPACE, "tAppointment")));
  }

  #[test]
  fn simple_type_days_and_time_duration() {
    let evaluator = build_evaluator(DMN_0107);
    assert_eq!(Some(FeelType::DaysAndTimeDuration), evaluator.eval(&DefKey::new(NAMESPACE, "tCourseDuration")));
  }

  #[test]
  fn simple_type_years_and_month_duration() {
    let evaluator = build_evaluator(DMN_0108);
    assert_eq!(Some(FeelType::YearsAndMonthsDuration), evaluator.eval(&DefKey::new(NAMESPACE, "tGrowthDuration")));
  }

  #[test]
  fn referenced_type_string() {
    let evaluator = build_evaluator(DMN_0201);
    assert_eq!(Some(FeelType::String), evaluator.eval(&DefKey::new(NAMESPACE, "tCustomerName")));
  }

  #[test]
  fn referenced_type_number() {
    let evaluator = build_evaluator(DMN_0202);
    assert_eq!(Some(FeelType::Number), evaluator.eval(&DefKey::new(NAMESPACE, "tMonthlySalary")));
  }

  #[test]
  fn component_type() {
    let evaluator = build_evaluator(DMN_0301);
    let name_principal: Name = "principal".into();
    let name_rate: Name = "rate".into();
    let name_term_months: Name = "termMonths".into();
    let type_number = FeelType::Number;
    let component_type = FeelType::context(&[(&name_principal, &type_number), (&name_rate, &type_number), (&name_term_months, &type_number)]);
    assert_eq!(Some(component_type), evaluator.eval(&DefKey::new(NAMESPACE, "tLoan")));
  }

  #[test]
  fn collection_of_simple_type_string() {
    let evaluator = build_evaluator(DMN_0401);
    assert_eq!(Some(FeelType::list(&FeelType::String)), evaluator.eval(&DefKey::new(NAMESPACE, "tItems")));
  }

  #[test]
  fn collection_of_simple_type_number() {
    let evaluator = build_evaluator(DMN_0402);
    assert_eq!(Some(FeelType::list(&FeelType::Number)), evaluator.eval(&DefKey::new(NAMESPACE, "tItems")));
  }

  #[test]
  fn collection_of_simple_type_boolean() {
    let evaluator = build_evaluator(DMN_0403);
    assert_eq!(Some(FeelType::list(&FeelType::Boolean)), evaluator.eval(&DefKey::new(NAMESPACE, "tItems")));
  }

  #[test]
  fn collection_of_simple_type_date() {
    let evaluator = build_evaluator(DMN_0404);
    assert_eq!(Some(FeelType::list(&FeelType::Date)), evaluator.eval(&DefKey::new(NAMESPACE, "tItems")));
  }

  #[test]
  fn collection_of_simple_type_time() {
    let evaluator = build_evaluator(DMN_0405);
    assert_eq!(Some(FeelType::list(&FeelType::Time)), evaluator.eval(&DefKey::new(NAMESPACE, "tItems")));
  }

  #[test]
  fn collection_of_simple_type_date_time() {
    let evaluator = build_evaluator(DMN_0406);
    assert_eq!(Some(FeelType::list(&FeelType::DateTime)), evaluator.eval(&DefKey::new(NAMESPACE, "tItems")));
  }

  #[test]
  fn collection_of_simple_type_days_and_time_duration() {
    let evaluator = build_evaluator(DMN_0407);
    assert_eq!(Some(FeelType::list(&FeelType::DaysAndTimeDuration)), evaluator.eval(&DefKey::new(NAMESPACE, "tItems")));
  }

  #[test]
  fn collection_of_simple_type_years_and_months_duration() {
    let evaluator = build_evaluator(DMN_0408);
    assert_eq!(Some(FeelType::list(&FeelType::YearsAndMonthsDuration)), evaluator.eval(&DefKey::new(NAMESPACE, "tItems")));
  }

  #[test]
  fn test_evaluate_input_data_0501_1() {
    let evaluator = build_evaluator(DMN_0501);
    assert_eq!(Some(FeelType::list(&FeelType::String)), evaluator.eval(&DefKey::new(NAMESPACE, "tItems")));
  }

  #[test]
  fn test_evaluate_input_data_0601_1() {
    let evaluator = build_evaluator(DMN_0601);
    let name_number: Name = "number".into();
    let name_name: Name = "name".into();
    let name_manager: Name = "manager".into();
    let type_number = FeelType::Number;
    let type_string = FeelType::String;
    let component_type = FeelType::context(&[(&name_number, &type_number), (&name_name, &type_string), (&name_manager, &type_string)]);
    let list_type = FeelType::list(&component_type);
    assert_eq!(Some(list_type), evaluator.eval(&DefKey::new(NAMESPACE, "tItems")));
  }
}
