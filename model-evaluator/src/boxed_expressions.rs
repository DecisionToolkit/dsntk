//! # Boxed expressions

use crate::decision_table;
use crate::errors::*;
use crate::model_builder::ModelBuilder;
use crate::model_definitions::*;
use dsntk_common::Result;
use dsntk_feel::closure::Closure;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::{Evaluator, FeelScope, FeelType, FunctionBody, IntervalType, value_null};
use dsntk_feel_evaluator::{EveryExpressionEvaluator, FilterExpressionEvaluator, ForExpressionEvaluator, SomeExpressionEvaluator};
use dsntk_feel_parser::{ClosureBuilder, parse_name};
use dsntk_model::*;
use std::sync::Arc;

pub fn bring_knowledge_requirements_into_context(def_definitions: &DefDefinitions, knowledge_requirements: &[DefKnowledgeRequirement], ctx: &mut FeelContext) -> Result<()> {
  for knowledge_requirement in knowledge_requirements {
    let def_href = knowledge_requirement.required_knowledge();
    if let Some(business_knowledge_model) = def_definitions.business_knowledge_model_by_key(def_href.namespace(), def_href.id()) {
      let output_variable_name = business_knowledge_model.variable().name();
      if let Some(import_name) = def_href.import_name() {
        ctx.create_entries(&[import_name.clone(), output_variable_name.clone()], value_null!());
      } else {
        ctx.set_entry(output_variable_name, value_null!());
      }
      bring_knowledge_requirements_into_context(def_definitions, business_knowledge_model.knowledge_requirements(), ctx)?;
    } else if let Some(decision_service) = def_definitions.decision_service_by_id(def_href.namespace(), def_href.id()) {
      let output_variable_name = decision_service.variable().name();
      if let Some(import_name) = def_href.import_name() {
        ctx.create_entries(&[import_name.clone(), output_variable_name.clone()], value_null!());
      } else {
        ctx.set_entry(output_variable_name, value_null!());
      }
    } else {
      return Err(err_business_knowledge_model_with_reference_not_found(def_href.namespace(), def_href.id()));
    }
  }
  Ok(())
}

pub fn build_expression_instance_evaluator(scope: &FeelScope, expression_instance: &ExpressionInstance, model_builder: &ModelBuilder) -> Result<(Evaluator, Closure)> {
  match expression_instance {
    ExpressionInstance::Conditional(conditional) => build_conditional_evaluator(scope, conditional, model_builder),
    ExpressionInstance::Context(context) => build_context_evaluator(scope, context, model_builder),
    ExpressionInstance::DecisionTable(decision_table) => build_decision_table_evaluator(scope, decision_table, model_builder),
    ExpressionInstance::Every(every) => build_every_evaluator(scope, every, model_builder),
    ExpressionInstance::Filter(filter) => build_filter_evaluator(scope, filter, model_builder),
    ExpressionInstance::For(r#for) => build_for_evaluator(scope, r#for, model_builder),
    ExpressionInstance::FunctionDefinition(function_definition) => build_function_definition_evaluator(scope, function_definition, model_builder),
    ExpressionInstance::Invocation(invocation) => build_invocation_evaluator(scope, invocation, model_builder),
    ExpressionInstance::LiteralExpression(literal_expression) => build_literal_expression_evaluator(scope, literal_expression, model_builder),
    ExpressionInstance::List(list) => build_list_evaluator(scope, list, model_builder),
    ExpressionInstance::Relation(relation) => build_relation_evaluator(scope, relation, model_builder),
    ExpressionInstance::Some(some) => build_some_evaluator(scope, some, model_builder),
  }
}

pub fn build_context_evaluator(scope: &FeelScope, context: &Context, model_builder: &ModelBuilder) -> Result<(Evaluator, Closure)> {
  let item_definition_type_evaluator = model_builder.item_definition_type_evaluator();
  let mut entry_evaluators = vec![];
  scope.push(FeelContext::default());
  for context_entry in context.context_entries() {
    if let Some(variable) = &context_entry.variable {
      let variable_name = variable.feel_name();
      let variable_type = item_definition_type_evaluator
        .information_item_type(variable.namespace(), variable.type_ref())
        .ok_or_else(err_empty_feel_type)?;
      let (evaluator, _) = build_expression_instance_evaluator(scope, &context_entry.value, model_builder)?;
      scope.set_name(variable_name.clone());
      entry_evaluators.push((Some(variable_name.clone()), variable_type, evaluator));
    } else {
      let (evaluator, _) = build_expression_instance_evaluator(scope, &context_entry.value, model_builder)?;
      entry_evaluators.push((None, FeelType::Any, evaluator));
    }
  }
  scope.pop();
  let context_evaluator = Box::new(move |scope: &FeelScope| {
    let mut evaluated_context = FeelContext::default();
    for (opt_variable_name, variable_type, evaluator) in &entry_evaluators {
      let value = evaluator(scope) as Value;
      let coerced_value = value.coerced(variable_type);
      match opt_variable_name {
        Some(variable_name) => {
          scope.set_value(variable_name, coerced_value.clone());
          evaluated_context.set_entry(variable_name, coerced_value);
        }
        None => return coerced_value,
      }
    }
    Value::Context(evaluated_context)
  });
  Ok((
    build_coerced_result_evaluator(context_evaluator, context, context.namespace(), model_builder),
    Closure::default(),
  ))
}

pub fn build_decision_table_evaluator(scope: &FeelScope, decision_table: &DecisionTable, model_builder: &ModelBuilder) -> Result<(Evaluator, Closure)> {
  let evaluator = decision_table::build_decision_table_evaluator(scope, decision_table)?;
  let decision_table_evaluator = Box::new(move |scope: &FeelScope| evaluator(scope));
  Ok((
    build_coerced_result_evaluator(decision_table_evaluator, decision_table, decision_table.namespace(), model_builder),
    Closure::default(),
  ))
}

pub fn build_function_definition_evaluator(scope: &FeelScope, function_definition: &FunctionDefinition, model_builder: &ModelBuilder) -> Result<(Evaluator, Closure)> {
  let item_definition_type_evaluator = model_builder.item_definition_type_evaluator();
  // resolve function definition's formal parameters
  let mut parameters = vec![];
  let mut parameters_ctx = FeelContext::default();
  for parameter in function_definition.formal_parameters() {
    let parameter_name = parameter.feel_name().clone();
    let parameter_type = item_definition_type_evaluator
      .information_item_type(parameter.namespace(), parameter.type_ref())
      .ok_or_else(err_empty_feel_type)?;
    parameters_ctx.set_entry(&parameter_name, Value::FeelType(parameter_type.clone()));
    parameters.push((parameter_name, parameter_type));
  }
  // resolve function definition's result type
  let result_type = if let Some(type_ref) = function_definition.type_ref() {
    item_definition_type_evaluator
      .information_item_type(function_definition.namespace(), type_ref)
      .ok_or_else(err_empty_feel_type)?
  } else {
    FeelType::Any
  };
  // check if the function is external
  match function_definition.kind() {
    FunctionKind::Feel => {
      // prepare function definition's body evaluator
      let body_expression_instance = function_definition.body().as_ref().ok_or_else(err_empty_function_body)?;
      scope.push(parameters_ctx);
      let (body_evaluator, mut closure) = build_expression_instance_evaluator(scope, body_expression_instance, model_builder)?;
      for (name, _) in &parameters {
        closure.remove(name.clone());
      }
      scope.pop();
      let function_body_evaluator = Arc::new(body_evaluator);
      let function_body = FunctionBody::LiteralExpression(function_body_evaluator);
      let function_definition_closure = closure.clone();
      // prepare the evaluator
      let function_definition_evaluator = Box::new(move |scope: &FeelScope| {
        let mut closure_ctx = FeelContext::default();
        for closure_name in closure.iter() {
          if let Some(closure_value) = scope.search_entry(closure_name) {
            closure_ctx.create_entry(closure_name, closure_value);
          }
        }
        Value::FunctionDefinition(parameters.clone(), function_body.clone(), false, closure.clone(), closure_ctx, result_type.clone())
      });
      Ok((
        build_coerced_result_evaluator(function_definition_evaluator, function_definition, function_definition.namespace(), model_builder),
        function_definition_closure,
      ))
    }
    FunctionKind::Java => {
      let body_expression_instance = function_definition.body().as_ref().ok_or_else(err_empty_function_body)?;
      scope.push(parameters_ctx);
      let (body_evaluator, _) = build_expression_instance_evaluator(scope, body_expression_instance, model_builder)?;
      scope.pop();
      let function_definition_evaluator = Box::new(move |scope: &FeelScope| {
        if let Value::Context(java_mapping) = body_evaluator(scope) {
          if let Some(Value::String(class_name)) = java_mapping.get_entry(&"class".into()) {
            if let Some(Value::String(method_signature)) = java_mapping.get_entry(&"method signature".into()) {
              let java_class_name = class_name.to_owned();
              let java_method_signature = method_signature.to_owned();
              let java_evaluator = Box::new(move |_: &FeelScope| Value::ExternalJavaFunction(java_class_name.clone(), java_method_signature.clone())) as Evaluator;
              let function_body_evaluator = Arc::new(java_evaluator);
              let function_body = FunctionBody::External(function_body_evaluator);
              Value::FunctionDefinition(parameters.clone(), function_body, true, Closure::default(), FeelContext::default(), result_type.clone())
            } else {
              value_null!("invalid Java function mapping, no method signature entry in context {}", java_mapping)
            }
          } else {
            value_null!("invalid Java function mapping, no class name entry in context {}", java_mapping)
          }
        } else {
          value_null!("expected context as external function mapping")
        }
      });
      Ok((
        build_coerced_result_evaluator(function_definition_evaluator, function_definition, function_definition.namespace(), model_builder),
        Closure::default(),
      ))
    }
    FunctionKind::Pmml => {
      let body_expression_instance = function_definition.body().as_ref().ok_or_else(err_empty_function_body)?;
      scope.push(parameters_ctx);
      let (body_evaluator, _) = build_expression_instance_evaluator(scope, body_expression_instance, model_builder)?;
      scope.pop();
      let function_definition_evaluator = Box::new(move |scope: &FeelScope| {
        if let Value::Context(pmml_mapping) = body_evaluator(scope) {
          if let Some(Value::String(document)) = pmml_mapping.get_entry(&"document".into()) {
            if let Some(Value::String(model_name)) = pmml_mapping.get_entry(&"model".into()) {
              let pmml_document = document.to_owned();
              let pmml_model_name = model_name.to_owned();
              let pmml_evaluator = Box::new(move |_: &FeelScope| Value::ExternalPmmlFunction(pmml_document.clone(), pmml_model_name.clone())) as Evaluator;
              let function_body_evaluator = Arc::new(pmml_evaluator);
              let function_body = FunctionBody::External(function_body_evaluator);
              Value::FunctionDefinition(parameters.clone(), function_body, true, Closure::default(), FeelContext::default(), result_type.clone())
            } else {
              value_null!("invalid PMML function mapping, no model name entry in context {}", pmml_mapping)
            }
          } else {
            value_null!("invalid PMML function mapping, no document entry in context {}", pmml_mapping)
          }
        } else {
          value_null!("expected context as external function mapping")
        }
      });
      Ok((
        build_coerced_result_evaluator(function_definition_evaluator, function_definition, function_definition.namespace(), model_builder),
        Closure::default(),
      ))
    }
  }
}

pub fn build_invocation_evaluator(scope: &FeelScope, invocation: &Invocation, model_builder: &ModelBuilder) -> Result<(Evaluator, Closure)> {
  let item_definition_type_evaluator = model_builder.item_definition_type_evaluator();
  let mut bindings = vec![];
  let (function_evaluator, _) = build_expression_instance_evaluator(scope, invocation.called_function(), model_builder)?;
  for binding in invocation.bindings() {
    if let Some(binding_formula) = binding.binding_formula() {
      let param_name = binding.parameter().feel_name().clone();
      let param_type = item_definition_type_evaluator
        .information_item_type(binding.parameter().namespace(), binding.parameter().type_ref())
        .ok_or_else(err_empty_feel_type)?;
      let (evaluator, _) = build_expression_instance_evaluator(scope, binding_formula, model_builder)?;
      bindings.push((param_name, param_type, evaluator));
    }
  }
  let invocation_evaluator = Box::new(move |scope: &FeelScope| {
    let mut params_ctx = FeelContext::default();
    for (param_name, param_type, evaluator) in &bindings {
      let param_value = evaluator(scope) as Value;
      let coerced_param_value = param_value.coerced(param_type);
      // if the parameter value is null after coercion, then do not call the invocable, just return this null value
      if coerced_param_value.is_invalid_coercion() {
        return coerced_param_value;
      }
      params_ctx.set_entry(param_name, param_value.coerced(param_type))
    }
    if let Value::FunctionDefinition(formal_parameters, body, false, _, closure_ctx, result_type) = function_evaluator(scope) {
      for (formal_parameter_name, formal_parameter_type) in formal_parameters {
        if let Some(param_value) = params_ctx.get(&formal_parameter_name) {
          let coerced_param_value = param_value.coerced(&formal_parameter_type);
          if coerced_param_value.is_invalid_coercion() {
            return coerced_param_value;
          }
        } else {
          return value_null!("argument not provided: {}", formal_parameter_name);
        }
      }
      scope.push(closure_ctx); // push closure_ctx <----
      scope.push(params_ctx); //  push params_ctx <---  |
      let value = body.evaluate(scope); //     |  |
      scope.pop(); // pop params_ctx <----------------  |
      scope.pop(); // pop closure_ctx <-----------------
      value.coerced(&result_type)
    } else {
      value_null!("expected function definition in invocation evaluator")
    }
  });
  Ok((
    build_coerced_result_evaluator(invocation_evaluator, invocation, invocation.namespace(), model_builder),
    Closure::default(),
  ))
}

pub fn build_literal_expression_evaluator(scope: &FeelScope, literal_expression: &LiteralExpression, model_builder: &ModelBuilder) -> Result<(Evaluator, Closure)> {
  let text = literal_expression.text().as_ref().ok_or_else(err_empty_literal_expression)?;
  let node = dsntk_feel_parser::parse_expression(scope, text, false)?;
  let closure = ClosureBuilder::from_node(&node);
  let literal_expression_evaluator = dsntk_feel_evaluator::prepare(&node);
  Ok((
    build_coerced_result_evaluator(literal_expression_evaluator, literal_expression, literal_expression.namespace(), model_builder),
    closure,
  ))
}

pub fn build_list_evaluator(scope: &FeelScope, list: &List, model_builder: &ModelBuilder) -> Result<(Evaluator, Closure)> {
  let mut item_evaluators = vec![];
  for list_item in list.elements() {
    let (item_evaluator, _) = build_expression_instance_evaluator(scope, list_item, model_builder)?;
    item_evaluators.push(item_evaluator)
  }
  let list_evaluator = Box::new(move |scope: &FeelScope| {
    let mut results = vec![];
    for item_evaluator in &item_evaluators {
      results.push(item_evaluator(scope));
    }
    Value::List(results)
  });
  Ok((build_coerced_result_evaluator(list_evaluator, list, list.namespace(), model_builder), Closure::default()))
}

pub fn build_relation_evaluator(scope: &FeelScope, relation: &Relation, model_builder: &ModelBuilder) -> Result<(Evaluator, Closure)> {
  let mut rows = vec![];
  for row in relation.rows() {
    let mut row_evaluators = vec![];
    for (i, element) in row.elements().iter().enumerate() {
      if let Some(column) = relation.columns().get(i) {
        let name = column.feel_name().clone();
        let (row_evaluator, _) = build_expression_instance_evaluator(scope, element, model_builder)?;
        row_evaluators.push((name, row_evaluator));
      }
    }
    rows.push(row_evaluators);
  }
  let relation_evaluator = Box::new(move |scope: &FeelScope| {
    let mut results = vec![];
    for row in &rows {
      let mut evaluated_context = FeelContext::default();
      for (name, evaluator) in row {
        evaluated_context.set_entry(name, evaluator(scope));
      }
      results.push(Value::Context(evaluated_context));
    }
    Value::List(results)
  });
  Ok((
    build_coerced_result_evaluator(relation_evaluator, relation, relation.namespace(), model_builder),
    Closure::default(),
  ))
}

pub fn build_conditional_evaluator(scope: &FeelScope, conditional: &Conditional, model_builder: &ModelBuilder) -> Result<(Evaluator, Closure)> {
  let (if_evaluator, _) = build_expression_instance_evaluator(scope, conditional.if_expression().value(), model_builder)?;
  let (then_evaluator, _) = build_expression_instance_evaluator(scope, conditional.then_expression().value(), model_builder)?;
  let (else_evaluator, _) = build_expression_instance_evaluator(scope, conditional.else_expression().value(), model_builder)?;
  let conditional_evaluator = Box::new(move |scope: &FeelScope| {
    let Value::Boolean(condition) = if_evaluator(scope) else {
      return value_null!("condition is not a boolean expression");
    };
    if condition { then_evaluator(scope) } else { else_evaluator(scope) }
  });
  Ok((
    build_coerced_result_evaluator(conditional_evaluator, conditional, conditional.namespace(), model_builder),
    Closure::default(),
  ))
}

pub fn build_filter_evaluator(scope: &FeelScope, filter: &Filter, model_builder: &ModelBuilder) -> Result<(Evaluator, Closure)> {
  let (in_evaluator, _) = build_expression_instance_evaluator(scope, filter.in_expression().value(), model_builder)?;
  let (match_evaluator, _) = build_expression_instance_evaluator(scope, filter.match_expression().value(), model_builder)?;
  // prepare `filter` evaluator
  let evaluator = Box::new(move |scope: &FeelScope| {
    let filter_expression_evaluator = FilterExpressionEvaluator::default();
    filter_expression_evaluator.evaluate(scope, in_evaluator(scope), &match_evaluator)
  });
  Ok((build_coerced_result_evaluator(evaluator, filter, filter.namespace(), model_builder), Closure::default()))
}

pub fn build_for_evaluator(scope: &FeelScope, r#for: &For, model_builder: &ModelBuilder) -> Result<(Evaluator, Closure)> {
  // get the name of the iterator variable
  let iterator_variable = parse_name(scope, r#for.iterator_variable(), false)?;
  // prepare evaluator of values the loop should iterate over
  let (in_evaluator, _) = build_expression_instance_evaluator(scope, r#for.in_expression().value(), model_builder)?;
  // prepare context with the names of iteration variables
  let mut variables_ctx = FeelContext::default();
  variables_ctx.set_null(iterator_variable.clone());
  scope.push(variables_ctx);
  // prepare the `for` loop returned value evaluator
  let (return_evaluator, _) = build_expression_instance_evaluator(scope, r#for.return_expression().value(), model_builder)?;
  scope.pop();
  // prepare `for` evaluator
  let evaluator = Box::new(move |scope: &FeelScope| {
    let mut for_expression_evaluator = ForExpressionEvaluator::default();
    let iterator_variable = iterator_variable.clone();
    match in_evaluator(scope) {
      value @ Value::List(_) => for_expression_evaluator.add_list(iterator_variable, value),
      Value::Range(start, IntervalType::Closed, end, IntervalType::Closed) => {
        if let Err(reason) = for_expression_evaluator.add_interval(iterator_variable, *start, *end) {
          return value_null!(reason);
        }
      }
      _ => {}
    }
    Value::List(for_expression_evaluator.evaluate(scope, &return_evaluator))
  });
  Ok((build_coerced_result_evaluator(evaluator, r#for, r#for.namespace(), model_builder), Closure::default()))
}

pub fn build_every_evaluator(scope: &FeelScope, every: &Every, model_builder: &ModelBuilder) -> Result<(Evaluator, Closure)> {
  // get the name of the iterator variable
  let iterator_variable = parse_name(scope, every.iterator_variable(), false)?;
  // prepare evaluator of values the loop should iterate over
  let (in_evaluator, _) = build_expression_instance_evaluator(scope, every.in_expression().value(), model_builder)?;
  // prepare context with the names of iteration variables
  let mut variables_ctx = FeelContext::default();
  variables_ctx.set_null(iterator_variable.clone());
  scope.push(variables_ctx);
  // prepare the `every` loop satisfies condition evaluator
  let (satisfies_evaluator, _) = build_expression_instance_evaluator(scope, every.satisfies_expression().value(), model_builder)?;
  scope.pop();
  // prepare `every` evaluator
  let evaluator = Box::new(move |scope: &FeelScope| {
    let mut every_expression_evaluator = EveryExpressionEvaluator::default();
    let iterator_variable = iterator_variable.clone();
    let in_clause = in_evaluator(scope);
    every_expression_evaluator.add_list(iterator_variable, in_clause);
    every_expression_evaluator.evaluate(scope, &satisfies_evaluator)
  });
  Ok((build_coerced_result_evaluator(evaluator, every, every.namespace(), model_builder), Closure::default()))
}

pub fn build_some_evaluator(scope: &FeelScope, some: &Some, model_builder: &ModelBuilder) -> Result<(Evaluator, Closure)> {
  // get the name of the iterator variable
  let iterator_variable = parse_name(scope, some.iterator_variable(), false)?;
  // prepare evaluator of values the loop should iterate over
  let (in_evaluator, _) = build_expression_instance_evaluator(scope, some.in_expression().value(), model_builder)?;
  // prepare context with the names of iteration variables
  let mut variables_ctx = FeelContext::default();
  variables_ctx.set_null(iterator_variable.clone());
  scope.push(variables_ctx);
  // prepare the `some` loop satisfies condition evaluator
  let (satisfies_evaluator, _) = build_expression_instance_evaluator(scope, some.satisfies_expression().value(), model_builder)?;
  scope.pop();
  // prepare `some` evaluator
  let evaluator = Box::new(move |scope: &FeelScope| {
    let mut some_expression_evaluator = SomeExpressionEvaluator::default();
    let iterator_variable = iterator_variable.clone();
    let in_clause = in_evaluator(scope);
    some_expression_evaluator.add_list(iterator_variable, in_clause);
    some_expression_evaluator.evaluate(scope, &satisfies_evaluator)
  });
  Ok((build_coerced_result_evaluator(evaluator, some, some.namespace(), model_builder), Closure::default()))
}

/// Builds an evaluator that provides coercion for output type of the expression.
fn build_coerced_result_evaluator(evaluator: Evaluator, expression: &dyn Expression, namespace: &str, model_builder: &ModelBuilder) -> Evaluator {
  if let Some(type_ref) = expression.type_ref()
    && let Some(feel_type) = model_builder.item_definition_type_evaluator().information_item_type(namespace, type_ref)
  {
    let coerced_result_evaluator = Box::new(move |scope: &FeelScope| evaluator(scope).coerced(&feel_type));
    return coerced_result_evaluator;
  }
  evaluator
}
