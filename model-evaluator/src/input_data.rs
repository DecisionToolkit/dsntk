//! # Builder for input data evaluators

use crate::item_definition::ItemDefinitionEvaluator;
use crate::model_definitions::{DefDefinitions, DefKey};
use crate::variable::{Variable, VariableEvaluatorFn};
use dsntk_feel::values::Value;
use dsntk_feel::Name;
use std::collections::HashMap;
use std::sync::Arc;

pub type InputDataEvaluatorEntry = (Variable, VariableEvaluatorFn);

/// Input data evaluator.
#[derive(Default)]
pub struct InputDataEvaluator {
  evaluators: Arc<HashMap<DefKey, InputDataEvaluatorEntry>>,
}

impl InputDataEvaluator {
  /// Builds a new input data evaluator.
  pub fn new(definitions: &DefDefinitions) -> Self {
    let mut evaluators = HashMap::new();
    for input_data in definitions.input_data() {
      let input_data_namespace = input_data.namespace();
      let input_data_id = input_data.id();
      let variable: Variable = input_data.variable().into();
      let evaluator = variable.build_evaluator();
      let def_key = DefKey::new(input_data_namespace, input_data_id);
      evaluators.insert(def_key, (variable, evaluator));
    }
    Self { evaluators: Arc::new(evaluators) }
  }

  /// Evaluates input data.
  pub fn evaluate(&self, def_key: &DefKey, value: &Value, item_definition_evaluator: &ItemDefinitionEvaluator) -> Option<(Name, Value)> {
    self.evaluators.get(def_key).map(|evaluator| evaluator.1(value, item_definition_evaluator))
  }

  /// Returns the variable for input data definition.
  pub fn get_variable(&self, def_key: &DefKey) -> Option<&Variable> {
    self.evaluators.get(def_key).map(|entry| &entry.0)
  }
}

#[cfg(test)]
mod tests {
  use crate::input_data::InputDataEvaluator;
  use crate::item_definition::ItemDefinitionEvaluator;
  use crate::model_definitions::{DefDefinitions, DefKey};
  use dsntk_examples::input_data::*;
  use dsntk_feel::values::Value;
  use dsntk_feel::{value_null, value_number, FeelNumber, Name};

  const NAMESPACE: &str = "https://dsntk.io";

  /// Utility function for building input data evaluator from definitions,
  /// and item definition evaluator from definitions.
  fn build_evaluators(content: &str) -> (InputDataEvaluator, ItemDefinitionEvaluator) {
    let definitions = dsntk_model::from_xml(content).unwrap();
    let mut def_definitions = DefDefinitions::default();
    def_definitions.add_model(&definitions);
    (InputDataEvaluator::new(&def_definitions), ItemDefinitionEvaluator::new(&def_definitions).unwrap())
  }

  #[test]
  fn _0001_1() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0001);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let variable = input_data_evaluator.get_variable(&def_key).unwrap();
    assert_eq!("Full Name", variable.name().to_string());
    assert_eq!(NAMESPACE, variable.namespace());
    let context_str = r#"{Full Name: "John"}"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Full", "Name"]), Value::String("John".to_string()))),
      input_data_evaluator.evaluate(&def_key, &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0001_2() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0001);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let variable = input_data_evaluator.get_variable(&def_key).unwrap();
    assert_eq!("Full Name", variable.name().to_string());
    assert_eq!(NAMESPACE, variable.namespace());
    let context_str = r#"{Full Name: "Phillip"}"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Full", "Name"]), Value::String("Phillip".to_string()))),
      input_data_evaluator.evaluate(&def_key, &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0001_3() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0001);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let variable = input_data_evaluator.get_variable(&def_key).unwrap();
    assert_eq!("Full Name", variable.name().to_string());
    assert_eq!(NAMESPACE, variable.namespace());
    let context_str = r#"{Full Name: 50.0}"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Full", "Name"]), value_null!("after coercion"))),
      input_data_evaluator.evaluate(&def_key, &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0002_1() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0002);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let variable = input_data_evaluator.get_variable(&def_key).unwrap();
    assert_eq!("Monthly Salary", variable.name().to_string());
    assert_eq!(NAMESPACE, variable.namespace());
    let context_str = r#"{Monthly Salary: 12000.00}"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Monthly", "Salary"]), value_number!(12000))),
      input_data_evaluator.evaluate(&def_key, &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0002_2() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0002);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let variable = input_data_evaluator.get_variable(&def_key).unwrap();
    assert_eq!("Monthly Salary", variable.name().to_string());
    assert_eq!(NAMESPACE, variable.namespace());
    let context_str = r#"{Monthly Salary: 8135.35}"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Monthly", "Salary"]), value_number!(813535, 2))),
      input_data_evaluator.evaluate(&def_key, &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0002_3() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0002);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let variable = input_data_evaluator.get_variable(&def_key).unwrap();
    assert_eq!("Monthly Salary", variable.name().to_string());
    assert_eq!(NAMESPACE, variable.namespace());
    let context_str = r#"{Monthly Salary: "12000.00"}"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Monthly", "Salary"]), value_null!("after coercion"))),
      input_data_evaluator.evaluate(&def_key, &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0003_1() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0003);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let variable = input_data_evaluator.get_variable(&def_key).unwrap();
    assert_eq!("Is Affordable", variable.name().to_string());
    assert_eq!(NAMESPACE, variable.namespace());
    let context_str = r#"{Is Affordable: true}"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Is", "Affordable"]), Value::Boolean(true))),
      input_data_evaluator.evaluate(&def_key, &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0003_2() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0003);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let variable = input_data_evaluator.get_variable(&def_key).unwrap();
    assert_eq!("Is Affordable", variable.name().to_string());
    assert_eq!(NAMESPACE, variable.namespace());
    let context_str = r#"{Is Affordable: false}"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Is", "Affordable"]), Value::Boolean(false))),
      input_data_evaluator.evaluate(&def_key, &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0003_3() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0003);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let variable = input_data_evaluator.get_variable(&def_key).unwrap();
    assert_eq!("Is Affordable", variable.name().to_string());
    assert_eq!(NAMESPACE, variable.namespace());
    let context_str = r#"{Is Affordable: "no"}"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Is", "Affordable"]), value_null!("after coercion"))),
      input_data_evaluator.evaluate(&def_key, &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0004_1() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0004);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let variable = input_data_evaluator.get_variable(&def_key).unwrap();
    assert_eq!("Employment Status", variable.name().to_string());
    assert_eq!(NAMESPACE, variable.namespace());
    let context_str = r#"{Employment Status: "EMPLOYED"}"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let name = Name::new(&["Employment", "Status"]);
    assert_eq!(
      Some((name, Value::String("EMPLOYED".to_string()))),
      input_data_evaluator.evaluate(&def_key, &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0005_1() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0005);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let variable = input_data_evaluator.get_variable(&def_key).unwrap();
    assert_eq!("Always Null", variable.name().to_string());
    assert_eq!(NAMESPACE, variable.namespace());
    let context_str = r#"{Always Null: null}"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Always", "Null"]), value_null!())),
      input_data_evaluator.evaluate(&def_key, &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0005_2() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0005);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let variable = input_data_evaluator.get_variable(&def_key).unwrap();
    assert_eq!("Always Null", variable.name().to_string());
    assert_eq!(NAMESPACE, variable.namespace());
    let context_str = r#"{Always Null: 10}"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Always", "Null"]), value_null!("after coercion"))),
      input_data_evaluator.evaluate(&def_key, &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0006_1() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0006);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let variable = input_data_evaluator.get_variable(&def_key).unwrap();
    assert_eq!("Employment Status", variable.name().to_string());
    assert_eq!(NAMESPACE, variable.namespace());
    let context_str = r#"{Employment Status: "EMPLOYED"}"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let name = Name::new(&["Employment", "Status"]);
    assert_eq!(
      Some((name, value_null!("input data evaluator: item definition evaluator 'tEmploymentStatus' not found"))),
      input_data_evaluator.evaluate(&def_key, &Value::Context(context), &item_definitions_evaluator)
    );
  }
}
