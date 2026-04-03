//! # Builder for decision evaluators

use crate::boxed_expressions::*;
use crate::model_builder::ModelBuilder;
use crate::model_definitions::*;
use crate::model_evaluator::ModelEvaluator;
use crate::variable::Variable;
use dsntk_common::Result;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::{FeelScope, Name, value_null};
use std::collections::HashMap;
use std::sync::Arc;

/// Type alias for closures that evaluate decisions.
///
/// (global_context, input data, model evaluator, output data) -> output variable name
///
type DecisionEvaluatorFn = Box<dyn Fn(&FeelContext, &FeelContext, &ModelEvaluator, &mut FeelContext) -> Name + Send + Sync>;

/// Type alias for decision's output variable combined with decision's evaluator function.
///
/// (variable, decision evaluator function)
///
type DecisionEvaluatorEntry = (Variable, DecisionEvaluatorFn);

/// Decision evaluator.
#[derive(Default)]
pub struct DecisionEvaluator {
  evaluators: Arc<HashMap<DefKey, DecisionEvaluatorEntry>>,
}

impl DecisionEvaluator {
  /// Creates a new decision evaluator.
  pub fn new(definitions: &DefDefinitions, model_builder: &ModelBuilder) -> Result<Self> {
    let mut evaluators = HashMap::new();
    for decision in definitions.decisions() {
      let evaluator_entry = build_decision_evaluator(definitions, decision, model_builder)?;
      let model_namespace = decision.namespace();
      let model_name = decision.model_name();
      let id = decision.id();
      let invocable_name = decision.name().to_string();
      let def_key = DefKey::new(model_namespace, id);
      evaluators.insert(def_key.clone(), evaluator_entry);
      model_builder.add_decision_invocable(model_namespace.to_string(), model_name.to_string(), invocable_name, def_key);
    }
    Ok(Self { evaluators: Arc::new(evaluators) })
  }

  /// Evaluates a decision identified by specified `decision_id`.
  pub fn evaluate(
    &self,
    def_key: &DefKey,
    global_context: &FeelContext,
    input_data: &FeelContext,
    model_evaluator: &ModelEvaluator,
    evaluated_ctx: &mut FeelContext,
  ) -> Option<Name> {
    self
      .evaluators
      .get(def_key)
      .map(|evaluator_entry| evaluator_entry.1(global_context, input_data, model_evaluator, evaluated_ctx))
  }

  /// Returns the variable for specified decision.
  pub fn get_variable(&self, def_key: &DefKey) -> Option<&Variable> {
    self.evaluators.get(def_key).map(|entry| &entry.0)
  }
}

/// Builds and returns decision evaluator.
fn build_decision_evaluator(def_definitions: &DefDefinitions, def_decision: &DefDecision, model_builder: &ModelBuilder) -> Result<DecisionEvaluatorEntry> {
  // acquire all needed intermediary evaluators
  let item_definition_type_evaluator = model_builder.item_definition_type_evaluator();
  let item_definition_context_evaluator = model_builder.item_definition_context_evaluator();
  let input_data_context_evaluator = model_builder.input_data_context_evaluator();

  // get the output variable properties
  let mut output_variable: Variable = def_decision.variable().into();
  output_variable.update_feel_type(item_definition_type_evaluator);

  // prepare output variable name for processed decision
  let output_variable_name = output_variable.name().clone();

  // prepare output variable type for processed decision
  let output_variable_type = output_variable.feel_type().clone();

  // holds variables for required decisions and required knowledge
  let mut build_requirements_ctx = FeelContext::default();

  // hods variables for required inputs
  let mut input_requirements_ctx = FeelContext::default();

  // bring into context the variables from this decision's knowledge requirements
  bring_knowledge_requirements_into_context(def_definitions, def_decision.knowledge_requirements(), &mut build_requirements_ctx)?;

  // bring into context the variables from information requirements
  for information_requirement in def_decision.information_requirements() {
    // bring into context the variable from required decision
    if let Some(def_href) = information_requirement.required_decision()
      && let Some(required_def_decision) = def_definitions.decision_by_key(def_href.namespace(), def_href.id())
    {
      let variable_name = required_def_decision.variable().name().clone();
      let variable_namespace = required_def_decision.variable().namespace();
      let variable_type_ref = required_def_decision.variable().type_ref();
      let variable_type = item_definition_context_evaluator.eval(&DefKey::new(variable_namespace, variable_type_ref), &variable_name, &mut build_requirements_ctx);
      if let Some(import_name) = def_href.import_name() {
        build_requirements_ctx.create_entries(&[import_name.clone(), variable_name], Value::FeelType(variable_type));
      } else {
        build_requirements_ctx.set_entry(&variable_name, Value::FeelType(variable_type));
      }
    }
    // bring into context the variable from required input
    if let Some(href) = information_requirement.required_input()
      && let Some(required_input) = def_definitions.input_data_by_key(href.namespace(), href.id())
    {
      let variable_name = required_input.variable().name();
      let variable_type = input_data_context_evaluator.eval(&href.into(), &mut input_requirements_ctx, item_definition_context_evaluator);
      input_requirements_ctx.set_entry(variable_name, Value::FeelType(variable_type));
    }
  }

  // prepare a scope and build expression instance evaluator
  let scope: FeelScope = build_requirements_ctx.into();
  scope.push(input_requirements_ctx.clone());

  // prepare expression instance for this decision
  let evaluator = if let Some(expression_instance) = def_decision.decision_logic().as_ref() {
    let (evaluator, _) = build_expression_instance_evaluator(&scope, expression_instance, model_builder)?;
    evaluator
  } else {
    Box::new(move |_: &FeelScope| value_null!("no decision logic defined in decision"))
  };

  // prepare required knowledge, required decisions and required input data references
  let mut required_knowledge_references: Vec<(Option<Name>, DefKey)> = vec![];
  let mut required_decision_references: Vec<(Option<Name>, DefKey)> = vec![];
  let mut required_input_data_references: Vec<DefKey> = vec![];

  // required business knowledge models and decision services
  for knowledge_requirement in def_decision.knowledge_requirements() {
    let required_knowledge = knowledge_requirement.required_knowledge();
    required_knowledge_references.push((required_knowledge.import_name().cloned(), required_knowledge.into()));
  }

  // required decisions and required input data
  for information_requirement in def_decision.information_requirements() {
    if let Some(href) = information_requirement.required_decision() {
      required_decision_references.push((href.import_name().cloned(), href.into()))
    }
    if let Some(href) = information_requirement.required_input() {
      required_input_data_references.push(href.into())
    }
  }

  // build decision evaluator closure
  let decision_evaluator = Box::new(
    move |global_context: &FeelContext, input_data_ctx: &FeelContext, model_evaluator: &ModelEvaluator, output_data_ctx: &mut FeelContext| {
      let business_knowledge_model_evaluator = model_evaluator.business_knowledge_model_evaluator();
      let decision_service_evaluator = model_evaluator.decision_service_evaluator();
      let decision_evaluator = model_evaluator.decision_evaluator();
      let input_data_evaluator = model_evaluator.input_data_evaluator();
      let item_definition_evaluator = model_evaluator.item_definition_evaluator();

      // prepare context containing values from required knowledge, required decision services and required decisions
      let mut requirements_ctx: FeelContext = Default::default();

      // make a locally mutable copy of the input data
      let mut input_data = input_data_ctx.clone();

      // evaluate required knowledge given as value from business knowledge models
      required_knowledge_references.iter().for_each(|(import_name, def_key)| {
        if let Some(import_name_parent) = import_name.clone() {
          if let Some(name) = business_knowledge_model_evaluator.evaluate(def_key, global_context, &input_data, model_evaluator, &mut requirements_ctx) {
            requirements_ctx.move_entry(name, import_name_parent);
          }
        } else {
          business_knowledge_model_evaluator.evaluate(def_key, global_context, &input_data, model_evaluator, &mut requirements_ctx);
        }
      });

      // evaluate required knowledge given as value from decision service function definitions
      required_knowledge_references.iter().for_each(|(import_name, def_key)| {
        if let Some(import_name_parent) = import_name.clone() {
          if let Some(name) = decision_service_evaluator.evaluate_fd(def_key, &input_data, &mut requirements_ctx) {
            requirements_ctx.move_entry(name, import_name_parent);
          }
        } else {
          decision_service_evaluator.evaluate_fd(def_key, &input_data, &mut requirements_ctx);
        }
      });

      // evaluate required decisions given as values from decisions
      required_decision_references.iter().for_each(|(import_name, def_key)| {
        if let Some(import_name_parent) = import_name.clone() {
          let mut import_input_data = input_data.clone();
          if import_input_data.is_context(&import_name_parent) {
            if let Some(Value::Context(ctx)) = import_input_data.remove_entry(&import_name_parent) {
              import_input_data.zip(&ctx);
            }
            input_data.remove_entry(&import_name_parent);
          }
          if let Some(name) = decision_evaluator.evaluate(def_key, global_context, &import_input_data, model_evaluator, &mut requirements_ctx) {
            requirements_ctx.move_entry(name, import_name_parent);
          }
        } else {
          decision_evaluator.evaluate(def_key, global_context, &input_data, model_evaluator, &mut requirements_ctx);
        }
      });

      // values from required knowledge may be overridden by input data
      requirements_ctx.overwrite(&input_data);

      // prepare context containing values from required input data
      let mut required_input_ctx: FeelContext = Default::default();
      let input_data = Value::Context(input_data);
      required_input_data_references.iter().for_each(|input_data_id| {
        if let Some((name, value)) = input_data_evaluator.evaluate(input_data_id, &input_data, item_definition_evaluator) {
          required_input_ctx.set_entry(&name, value);
        }
      });
      required_input_ctx.zip(&requirements_ctx);

      // prepare the evaluation scope
      let scope: FeelScope = global_context.clone().into();
      scope.append(required_input_ctx.into());

      // evaluate the result
      let decision_result = evaluator(&scope);

      // coerce the output value
      let coerced_decision_result = decision_result.coerced(&output_variable_type);

      // place the result under the name of the output variable
      output_data_ctx.set_entry(&output_variable_name, coerced_decision_result);

      // return the name of the output variable
      output_variable_name.clone()
    },
  );

  // return the output variable and decision evaluator function
  Ok((output_variable, decision_evaluator))
}
