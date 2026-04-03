//! # Builder for decision service evaluators

use crate::model_builder::ModelBuilder;
use crate::model_definitions::{DefDecisionService, DefDefinitions, DefKey};
use crate::model_evaluator::ModelEvaluator;
use crate::variable::Variable;
use dsntk_common::Result;
use dsntk_feel::closure::Closure;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::{Evaluator, FeelScope, FeelType, Name, value_null};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Type of closure that evaluates a decision service.
///
/// (global_context, input_data, model evaluator, output data) -> output variable name
///
type DecisionServiceEvaluatorFn = Box<dyn Fn(&FeelContext, &FeelContext, &ModelEvaluator, &mut FeelContext) -> Name + Send + Sync>;

type DecisionServiceEvaluatorEntry = (Variable, Vec<(Name, FeelType)>, DecisionServiceEvaluatorFn, Option<Evaluator>);

/// Decision service evaluator.
#[derive(Default)]
pub struct DecisionServiceEvaluator {
  evaluators: RwLock<HashMap<DefKey, DecisionServiceEvaluatorEntry>>,
}

impl DecisionServiceEvaluator {
  /// Creates a new decision service evaluator.
  pub fn new(definitions: &DefDefinitions, model_builder: &ModelBuilder) -> Result<Self> {
    let mut evaluators = HashMap::new();
    for decision_service in definitions.decision_services() {
      let model_namespace = decision_service.namespace();
      let model_name = decision_service.model_name();
      let id = decision_service.id();
      let invocable_name = decision_service.name().to_string();
      let evaluator = build_decision_service_evaluator(decision_service, model_builder)?;
      let def_key = DefKey::new(model_namespace, id);
      evaluators.insert(def_key.clone(), evaluator);
      model_builder.add_decision_service_invocable(model_namespace.to_string(), model_name.to_string(), invocable_name, def_key);
    }
    Ok(Self {
      evaluators: RwLock::new(evaluators),
    })
  }

  /// Creates function definition evaluators for all decision service evaluators.
  pub fn build_function_definitions(&self, model_evaluator: &Arc<ModelEvaluator>) {
    if let Ok(mut evaluators) = self.evaluators.write() {
      let identifiers = evaluators.keys().cloned().collect::<Vec<DefKey>>();
      for decision_service_id in identifiers {
        let evaluator = Arc::clone(model_evaluator);
        evaluators.entry(decision_service_id.clone()).and_modify(|entry| {
          let output_variable_type = entry.0.feel_type().clone();
          let body_evaluator = Box::new(move |scope: &FeelScope| {
            let global_context = FeelContext::default();
            let input_data = scope.peek().unwrap_or_default();
            let mut output_data = FeelContext::default();
            let decision_service_evaluator = evaluator.decision_service_evaluator();
            let opt_out_variable_name = decision_service_evaluator.evaluate(&decision_service_id, &global_context, &input_data, &evaluator, &mut output_data);
            if let Some(out_variable_name) = opt_out_variable_name
              && let Some(result_value) = output_data.get_entry(&out_variable_name)
            {
              return result_value.clone();
            }
            value_null!()
          });
          let function_body = dsntk_feel::FunctionBody::DecisionService(Arc::new(body_evaluator));
          let function_definition = Value::FunctionDefinition(entry.1.clone(), function_body, false, Closure::default(), FeelContext::default(), output_variable_type);
          let decision_service_as_function_definition_evaluator = Box::new(move |_: &FeelScope| function_definition.clone());
          entry.3 = Some(decision_service_as_function_definition_evaluator);
        });
      }
    }
  }

  /// Evaluates a decision service with specified identifier.
  pub fn evaluate(
    &self,
    def_key: &DefKey,
    global_context: &FeelContext,
    input_data: &FeelContext,
    model_evaluator: &ModelEvaluator,
    output_data: &mut FeelContext,
  ) -> Option<Name> {
    self
      .evaluators
      .read()
      .ok()?
      .get(def_key)
      .map(|entry| entry.2(global_context, input_data, model_evaluator, output_data))
  }

  /// Returns a decision service as function definition with specified identifier.
  pub fn evaluate_fd(&self, def_key: &DefKey, input_data: &FeelContext, output_data: &mut FeelContext) -> Option<Name> {
    if let Ok(evaluators) = self.evaluators.read()
      && let Some((variable, _, _, Some(evaluator))) = evaluators.get(def_key)
    {
      let scope: FeelScope = input_data.clone().into();
      let function_definition = evaluator(&scope) as Value;
      let output_variable_name = variable.name().clone();
      output_data.set_entry(&output_variable_name, function_definition);
      return Some(output_variable_name);
    }
    None
  }
}

fn build_decision_service_evaluator(decision_service: &DefDecisionService, model_builder: &ModelBuilder) -> Result<DecisionServiceEvaluatorEntry> {
  let item_definition_type_evaluator = model_builder.item_definition_type_evaluator();
  let input_data_evaluator = model_builder.input_data_evaluator();
  let decision_evaluator = model_builder.decision_evaluator();

  let mut output_variable: Variable = decision_service.variable().into();
  output_variable.update_feel_type(item_definition_type_evaluator);

  // prepare output variable name for this decision
  let output_variable_name = output_variable.name().clone();

  // prepare output variable type for this decision
  let output_variable_type = output_variable.feel_type().clone();

  // prepare references to required input data
  let input_data_references: Vec<DefKey> = decision_service.input_data().iter().map(|href| href.into()).collect();

  // prepare references to input decisions
  let input_decisions: Vec<DefKey> = decision_service.input_decisions().iter().map(|href| href.into()).collect();

  // prepare references to encapsulated decisions
  let encapsulated_decisions: Vec<DefKey> = decision_service.encapsulated_decisions().iter().map(|href| href.into()).collect();

  // prepare references to output decisions
  let output_decisions: Vec<DefKey> = decision_service.output_decisions().iter().map(|href| href.into()).collect();

  // prepare a container for formal parameters accepted by this decision service
  let mut formal_parameters: Vec<(Name, FeelType)> = vec![];

  // fills the list of formal parameters based on required input data
  // these parameters are placed before input parameters defined by input decisions
  for input_data_id in &input_data_references {
    if let Some(input_data_variable) = input_data_evaluator.get_variable(input_data_id) {
      let parameter_name = input_data_variable.name().clone();
      let parameter_type = input_data_variable.resolve_feel_type(item_definition_type_evaluator);
      formal_parameters.push((parameter_name, parameter_type));
    }
  }

  // prepare evaluators from output variables of input decisions
  // these evaluators will evaluate results from input decisions and values from input data
  // simultaneously, fills the list of formal parameters based on output variables of input decisions
  let mut input_decision_results_evaluators = vec![];
  for def_key in &input_decisions {
    if let Some(decision_output_variable) = decision_evaluator.get_variable(def_key) {
      let parameter_name = decision_output_variable.name().clone();
      let parameter_type = decision_output_variable.resolve_feel_type(item_definition_type_evaluator);
      formal_parameters.push((parameter_name, parameter_type));
      let evaluator = decision_output_variable.build_evaluator();
      input_decision_results_evaluators.push(evaluator);
    }
  }

  // build decision service evaluator closure
  let decision_service_evaluator = Box::new(
    move |global_context: &FeelContext, input_data: &FeelContext, model_evaluator: &ModelEvaluator, output_data: &mut FeelContext| {
      // acquire all evaluators needed
      let item_definition_evaluator = model_evaluator.item_definition_evaluator();
      let input_data_evaluator = model_evaluator.input_data_evaluator();
      let decision_evaluator = model_evaluator.decision_evaluator();
      // evaluate input decisions and store the results in separate context
      let mut input_decisions_results = FeelContext::default();
      input_decisions.iter().for_each(|def_key| {
        decision_evaluator.evaluate(def_key, global_context, input_data, model_evaluator, &mut input_decisions_results);
      });
      // now evaluate input data for encapsulated and output decisions and store them in separate context
      let mut evaluated_input_data = FeelContext::default();
      // first take values from evaluated input decisions...
      let input_decision_results_value = Value::Context(input_decisions_results);
      for evaluator in &input_decision_results_evaluators {
        let (name, value) = evaluator(&input_decision_results_value, item_definition_evaluator);
        evaluated_input_data.set_entry(&name, value);
      }
      // ...and then take values from provided input data
      let input_data_values = Value::Context(input_data.clone());
      for evaluator in &input_decision_results_evaluators {
        let (name, value) = evaluator(&input_data_values, item_definition_evaluator);
        evaluated_input_data.set_entry(&name, value);
      }
      // evaluate required inputs (from required input data references)
      input_data_references.iter().for_each(|input_data_id| {
        if let Some((name, value)) = input_data_evaluator.evaluate(input_data_id, &input_data_values, item_definition_evaluator) {
          evaluated_input_data.set_entry(&name, value);
        }
      });
      // prepare context for evaluated result data for this decision service
      let mut evaluated_ctx = FeelContext::default();
      // evaluate encapsulated decisions
      encapsulated_decisions.iter().for_each(|def_key| {
        decision_evaluator.evaluate(def_key, global_context, &evaluated_input_data, model_evaluator, &mut evaluated_ctx);
      });
      // evaluate output decisions
      let mut output_names = vec![];
      output_decisions.iter().for_each(|def_key| {
        if let Some(output_name) = decision_evaluator.evaluate(def_key, global_context, &evaluated_input_data, model_evaluator, &mut evaluated_ctx) {
          output_names.push(output_name);
        }
      });
      // prepare the result from this decision service
      if output_names.len() == 1 {
        if let Some(value) = evaluated_ctx.get_entry(&output_names[0]) {
          let single_result = value.to_owned();
          let coerced_single_result = single_result.coerced(&output_variable_type);
          output_data.set_entry(&output_variable_name, coerced_single_result);
        }
      } else {
        let mut output_ctx = FeelContext::default();
        output_names.iter().for_each(|output_name| {
          if let Some(value) = evaluated_ctx.get_entry(output_name) {
            output_ctx.set_entry(output_name, value.to_owned());
          }
        });
        let complex_result = Value::Context(output_ctx);
        let coerced_complex_result = complex_result.coerced(&output_variable_type);
        output_data.set_entry(&output_variable_name, coerced_complex_result);
      }
      output_variable_name.clone()
    },
  );
  Ok((output_variable, formal_parameters, decision_service_evaluator, None))
}
