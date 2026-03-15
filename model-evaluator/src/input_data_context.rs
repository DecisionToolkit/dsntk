//! # Builder for input data context evaluators

use crate::item_definition_context::ItemDefinitionContextEvaluator;
use crate::model_definitions::{DefDefinitions, DefInputData, DefKey};
use crate::type_ref::type_ref_to_feel_type;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::FeelType;
use std::collections::HashMap;

/// Type of closure that evaluates input data context.
type InputDataContextEvaluatorFn = Box<dyn Fn(&mut FeelContext, &ItemDefinitionContextEvaluator) -> FeelType + Send + Sync>;

/// Input data context evaluator.
#[derive(Default)]
pub struct InputDataContextEvaluator {
  evaluators: HashMap<DefKey, InputDataContextEvaluatorFn>,
}

impl InputDataContextEvaluator {
  /// Creates a new input data context evaluator based on provided definitions.
  pub fn new(definitions: &DefDefinitions) -> Self {
    let mut evaluators = HashMap::new();
    for input_data in definitions.input_data() {
      let input_data_namespace = input_data.namespace();
      let input_data_id = input_data.id();
      let evaluator = input_data_context_evaluator(input_data);
      let def_key = DefKey::new(input_data_namespace, input_data_id);
      evaluators.insert(def_key, evaluator);
    }
    Self { evaluators }
  }

  /// Evaluates input data context with specified identifier.
  pub fn eval(&self, def_key: &DefKey, ctx: &mut FeelContext, item_definition_context_evaluator: &ItemDefinitionContextEvaluator) -> FeelType {
    if let Some(evaluator) = self.evaluators.get(def_key) {
      evaluator(ctx, item_definition_context_evaluator)
    } else {
      FeelType::Any
    }
  }
}

pub fn input_data_context_evaluator(input_data: &DefInputData) -> InputDataContextEvaluatorFn {
  let namespace = input_data.variable().namespace().to_string();
  let type_ref = input_data.variable().type_ref().to_string();
  let name = input_data.variable().name().clone();
  if let Some(simple_type_ref) = type_ref_to_feel_type(&type_ref) {
    Box::new(move |ctx: &mut FeelContext, _: &ItemDefinitionContextEvaluator| {
      ctx.set_entry(&name, Value::FeelType(simple_type_ref.clone()));
      simple_type_ref.clone()
    })
  } else {
    Box::new(move |ctx: &mut FeelContext, evaluator: &ItemDefinitionContextEvaluator| evaluator.eval(&DefKey::new(&namespace, &type_ref), &name, ctx))
  }
}

#[cfg(test)]
mod tests {
  use crate::input_data_context::InputDataContextEvaluator;
  use crate::item_definition_context::ItemDefinitionContextEvaluator;
  use crate::model_definitions::{DefDefinitions, DefKey};
  use dsntk_examples::input_data::*;
  use dsntk_feel::context::FeelContext;
  use dsntk_feel::FeelType;

  const NAMESPACE: &str = "https://dsntk.io";

  /// Utility function for building input data context evaluator from definitions,
  /// and item definition context evaluator from definitions.
  fn build_evaluators(xml: &str) -> (InputDataContextEvaluator, ItemDefinitionContextEvaluator) {
    let definitions = dsntk_model::from_xml(xml).unwrap();
    let mut def_definitions = DefDefinitions::default();
    def_definitions.add_model(&definitions);
    let input_data_context_evaluator = InputDataContextEvaluator::new(&def_definitions);
    let item_definition_context_evaluator = ItemDefinitionContextEvaluator::new(&def_definitions).unwrap();
    (input_data_context_evaluator, item_definition_context_evaluator)
  }

  #[test]
  fn _0001() {
    let (evaluator, item_definition_context_evaluator) = build_evaluators(DMN_0001);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let expected_type = FeelType::String;
    let mut ctx = FeelContext::default();
    let actual_type = evaluator.eval(&def_key, &mut ctx, &item_definition_context_evaluator);
    assert_eq!(expected_type, actual_type);
    assert_eq!("{Full Name: type(string)}", ctx.to_string());
  }

  #[test]
  fn _0002() {
    let (evaluator, item_definition_context_evaluator) = build_evaluators(DMN_0002);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let expected_type = FeelType::Number;
    let mut ctx = FeelContext::default();
    let actual_type = evaluator.eval(&def_key, &mut ctx, &item_definition_context_evaluator);
    assert_eq!(expected_type, actual_type);
    assert_eq!("{Monthly Salary: type(number)}", ctx.to_string());
  }

  #[test]
  fn _0003() {
    let (evaluator, item_definition_context_evaluator) = build_evaluators(DMN_0003);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let expected_type = FeelType::Boolean;
    let mut ctx = FeelContext::default();
    let actual_type = evaluator.eval(&def_key, &mut ctx, &item_definition_context_evaluator);
    assert_eq!(expected_type, actual_type);
    assert_eq!("{Is Affordable: type(boolean)}", ctx.to_string());
  }

  #[test]
  fn _0004() {
    let (evaluator, item_definition_context_evaluator) = build_evaluators(DMN_0004);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let expected_type = FeelType::String;
    let mut ctx = FeelContext::default();
    let actual_type = evaluator.eval(&def_key, &mut ctx, &item_definition_context_evaluator);
    assert_eq!(expected_type, actual_type);
    assert_eq!("{Employment Status: type(string)}", ctx.to_string());
  }

  #[test]
  fn _0005() {
    let (evaluator, item_definition_context_evaluator) = build_evaluators(DMN_0005);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let expected_type = FeelType::Any;
    let mut ctx = FeelContext::default();
    let actual_type = evaluator.eval(&def_key, &mut ctx, &item_definition_context_evaluator);
    assert_eq!(expected_type, actual_type);
    assert_eq!("{}", ctx.to_string()); //TODO Investigate this case.
  }

  #[test]
  fn _0006() {
    let (evaluator, item_definition_context_evaluator) = build_evaluators(DMN_0006);
    let def_key = DefKey::new(NAMESPACE, "_input_1");
    let expected_type = FeelType::Any;
    let mut ctx = FeelContext::default();
    let actual_type = evaluator.eval(&def_key, &mut ctx, &item_definition_context_evaluator);
    assert_eq!(expected_type, actual_type);
    assert_eq!("{}", ctx.to_string()); //TODO Investigate this case too.
  }
}
