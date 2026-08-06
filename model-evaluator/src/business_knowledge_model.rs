//! # Builder for business knowledge model evaluators

use crate::boxed_expressions::*;
use crate::errors::*;
use crate::model_builder::ModelBuilder;
use crate::model_definitions::{DefBusinessKnowledgeModel, DefDefinitions, DefKey};
use crate::model_evaluator::ModelEvaluator;
use dsntk_common::Result;
use dsntk_feel::closure::Closure;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::{FeelScope, FeelType, FunctionBody, Name};
use dsntk_model::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Type of closure that evaluates business knowledge model.
///
/// (global_context, input data, model evaluator, output data)
///
type BusinessKnowledgeModelEvaluatorFn = Box<dyn Fn(&FeelContext, &FeelContext, &ModelEvaluator, &mut FeelContext) -> Name + Send + Sync>;

/// Business knowledge model evaluator.
#[derive(Default)]
pub struct BusinessKnowledgeModelEvaluator {
  evaluators: Arc<HashMap<DefKey, BusinessKnowledgeModelEvaluatorFn>>,
}

impl BusinessKnowledgeModelEvaluator {
  /// Creates a new business knowledge model evaluator.
  pub fn new(definitions: &DefDefinitions, model_builder: &ModelBuilder) -> Result<Self> {
    let mut evaluators = HashMap::new();
    for business_knowledge_model in definitions.business_knowledge_models() {
      let function_definition = business_knowledge_model.encapsulated_logic().as_ref().ok_or_else(err_empty_encapsulated_logic)?;
      let evaluator = build_bkm_evaluator(definitions, business_knowledge_model, function_definition, model_builder)?;
      let model_namespace = business_knowledge_model.namespace();
      let model_name = business_knowledge_model.model_name();
      let id = business_knowledge_model.id();
      let invocable_name = business_knowledge_model.name().to_string();
      let output_variable_name = business_knowledge_model.variable().name().to_owned();
      let def_key = DefKey::new(model_namespace, id);
      evaluators.insert(def_key.clone(), evaluator);
      model_builder.add_bkm_invocable(model_namespace.to_string(), model_name.to_string(), invocable_name, def_key, output_variable_name);
    }
    Ok(Self { evaluators: Arc::new(evaluators) })
  }

  /// Evaluates a business knowledge model with specified identifier.
  /// When a required business knowledge model is found, then its evaluator
  /// is executed, and the result is stored in `evaluated_ctx`.
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
      .get(def_key)
      .map(|evaluator_entry| evaluator_entry(global_context, input_data, model_evaluator, output_data))
  }
}

fn build_bkm_evaluator(
  definitions: &DefDefinitions,
  business_knowledge_model: &DefBusinessKnowledgeModel,
  function_definition: &FunctionDefinition,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let item_definition_type_evaluator = model_builder.item_definition_type_evaluator();
  let mut local_context = FeelContext::default();
  let mut formal_parameters = vec![];
  for information_item in function_definition.formal_parameters() {
    let feel_type = item_definition_type_evaluator
      .information_item_type(information_item.namespace(), information_item.type_ref())
      .ok_or_else(err_empty_feel_type)?;
    let feel_name = information_item.feel_name();
    formal_parameters.push((feel_name.clone(), feel_type.clone()));
    local_context.set_entry(feel_name, Value::FeelType(feel_type));
  }
  let output_variable_name = business_knowledge_model.variable().name().clone();
  let output_variable_type = item_definition_type_evaluator
    .information_item_type(business_knowledge_model.variable().namespace(), business_knowledge_model.variable().type_ref())
    .unwrap_or(FeelType::Any);
  let mut knowledge_requirements: Vec<DefKey> = vec![];
  for knowledge_requirement in business_knowledge_model.knowledge_requirements() {
    knowledge_requirements.push(knowledge_requirement.required_knowledge().into());
  }
  // bring into context the variables from knowledge requirements
  bring_knowledge_requirements_into_context(definitions, business_knowledge_model.knowledge_requirements(), &mut local_context)?;
  //TODO verify the above line - there was no such example in models
  if let Some(expression_instance) = function_definition.body() {
    let scope: FeelScope = local_context.into();
    build_bkm_expression_instance_evaluator(
      &scope,
      formal_parameters,
      expression_instance,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
      model_builder,
    )
  } else {
    let output_variable_name = business_knowledge_model.variable().name().clone();
    Ok(Box::new(move |_: &FeelContext, _: &FeelContext, _: &ModelEvaluator, _: &mut FeelContext| {
      output_variable_name.clone()
    }))
  }
}

fn build_bkm_expression_instance_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  expression_instance: &ExpressionInstance,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<DefKey>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  match expression_instance {
    ExpressionInstance::Conditional(conditional) => build_bkm_conditional_evaluator(
      scope,
      formal_parameters,
      conditional,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
      model_builder,
    ),
    ExpressionInstance::Context(context) => build_bkm_context_evaluator(
      scope,
      formal_parameters,
      context,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
      model_builder,
    ),
    ExpressionInstance::DecisionTable(decision_table) => build_bkm_decision_table_evaluator(
      scope,
      formal_parameters,
      decision_table,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
      model_builder,
    ),
    ExpressionInstance::Every(every) => build_bkm_every_evaluator(
      scope,
      formal_parameters,
      every,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
      model_builder,
    ),
    ExpressionInstance::Filter(filter) => build_bkm_filter_evaluator(
      scope,
      formal_parameters,
      filter,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
      model_builder,
    ),
    ExpressionInstance::For(r#for) => build_bkm_for_evaluator(
      scope,
      formal_parameters,
      r#for,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
      model_builder,
    ),
    ExpressionInstance::FunctionDefinition(function_definition) => build_bkm_function_definition_evaluator(
      scope,
      formal_parameters,
      function_definition,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
      model_builder,
    ),
    ExpressionInstance::Invocation(invocation) => build_bkm_invocation_evaluator(
      scope,
      formal_parameters,
      invocation,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
      model_builder,
    ),
    ExpressionInstance::LiteralExpression(literal_expression) => build_bkm_literal_expression_evaluator(
      scope,
      formal_parameters,
      literal_expression,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
      model_builder,
    ),
    ExpressionInstance::List(list) => build_bkm_list_evaluator(
      scope,
      formal_parameters,
      list,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
      model_builder,
    ),
    ExpressionInstance::Relation(relation) => build_bkm_relation_evaluator(
      scope,
      formal_parameters,
      relation,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
      model_builder,
    ),
    ExpressionInstance::Some(some) => build_bkm_some_evaluator(
      scope,
      formal_parameters,
      some,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
      model_builder,
    ),
  }
}

fn build_bkm_conditional_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  conditional: &Conditional,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<DefKey>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_conditional_evaluator(scope, conditional, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function_definition = Value::function_definition(
    formal_parameters,
    FunctionBody::Conditional(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function_definition, knowledge_requirements)
}

fn build_bkm_context_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  context: &Context,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<DefKey>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_context_evaluator(scope, context, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function = Value::function_definition(
    formal_parameters,
    FunctionBody::Context(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function, knowledge_requirements)
}

fn build_bkm_decision_table_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  decision_table: &DecisionTable,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<DefKey>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_decision_table_evaluator(scope, decision_table, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function = Value::function_definition(
    formal_parameters,
    FunctionBody::DecisionTable(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function, knowledge_requirements)
}

fn build_bkm_every_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  every: &Every,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<DefKey>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_every_evaluator(scope, every, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function_definition = Value::function_definition(
    formal_parameters,
    FunctionBody::Every(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function_definition, knowledge_requirements)
}

fn build_bkm_filter_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  filter: &Filter,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<DefKey>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_filter_evaluator(scope, filter, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function_definition = Value::function_definition(
    formal_parameters,
    FunctionBody::Filter(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function_definition, knowledge_requirements)
}

fn build_bkm_for_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  r#for: &For,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<DefKey>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_for_evaluator(scope, r#for, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function_definition = Value::function_definition(formal_parameters, FunctionBody::For(Arc::new(evaluator)), false, closure, closure_ctx, output_variable_type);
  build_bkm_evaluator_from_function_definition(output_variable_name, function_definition, knowledge_requirements)
}

fn build_bkm_function_definition_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  function_definition: &FunctionDefinition,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<DefKey>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_function_definition_evaluator(scope, function_definition, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function = Value::function_definition(
    formal_parameters,
    FunctionBody::FunctionDefinition(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function, knowledge_requirements)
}

fn build_bkm_invocation_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  invocation: &Invocation,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<DefKey>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_invocation_evaluator(scope, invocation, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function = Value::function_definition(
    formal_parameters,
    FunctionBody::Invocation(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function, knowledge_requirements)
}

fn build_bkm_list_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  list: &List,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<DefKey>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_list_evaluator(scope, list, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function = Value::function_definition(
    formal_parameters,
    FunctionBody::List(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function, knowledge_requirements)
}

fn build_bkm_literal_expression_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  literal_expression: &LiteralExpression,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<DefKey>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_literal_expression_evaluator(scope, literal_expression, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function = Value::function_definition(
    formal_parameters,
    FunctionBody::LiteralExpression(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function, knowledge_requirements)
}

fn build_bkm_relation_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  relation: &Relation,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<DefKey>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_relation_evaluator(scope, relation, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function_definition = Value::function_definition(
    formal_parameters,
    FunctionBody::Relation(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function_definition, knowledge_requirements)
}

fn build_bkm_some_evaluator(
  scope: &FeelScope,
  formal_parameters: Vec<(Name, FeelType)>,
  some: &Some,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: Vec<DefKey>,
  model_builder: &ModelBuilder,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let (evaluator, _) = build_some_evaluator(scope, some, model_builder)?;
  let closure = Closure::default();
  let closure_ctx = FeelContext::default();
  let function_definition = Value::function_definition(
    formal_parameters,
    FunctionBody::Some(Arc::new(evaluator)),
    false,
    closure,
    closure_ctx,
    output_variable_type,
  );
  build_bkm_evaluator_from_function_definition(output_variable_name, function_definition, knowledge_requirements)
}

fn build_bkm_evaluator_from_function_definition(
  output_variable_name: Name,
  function_definition: Value,
  knowledge_requirements: Vec<DefKey>,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  Ok(Box::new(
    move |global_context: &FeelContext, input_data: &FeelContext, model_evaluator: &ModelEvaluator, output_data: &mut FeelContext| {
      let business_knowledge_model_evaluator = model_evaluator.business_knowledge_model_evaluator();
      let decision_service_evaluator = model_evaluator.decision_service_evaluator();
      knowledge_requirements.iter().for_each(|def_key| {
        //TODO refactor: call either business knowledge model or decision service,  but not both!
        business_knowledge_model_evaluator.evaluate(def_key, global_context, input_data, model_evaluator, output_data);
        decision_service_evaluator.evaluate(def_key, global_context, input_data, model_evaluator, output_data);
      });
      output_data.set_entry(&output_variable_name, function_definition.clone());
      output_variable_name.clone()
    },
  ))
}
