//! Builder for item definition evaluators.

use crate::errors::*;
use crate::model_definitions::{DefDefinitions, DefItemDefinition, DefKey};
use dsntk_common::Result;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::{Value, Values};
use dsntk_feel::{value_null, Evaluator, FeelScope, FeelType, Name};
use dsntk_feel_parser::AstNode;
use dsntk_model::ItemDefinitionType;
use std::collections::HashMap;
use std::sync::Arc;

/// Type of closure that evaluates input data conformant with item definition.
pub type ItemDefinitionEvaluatorFn = Box<dyn Fn(&Value, &ItemDefinitionEvaluator) -> Value + Send + Sync>;

/// Item definition evaluator.
#[derive(Default)]
pub struct ItemDefinitionEvaluator {
  evaluators: Arc<HashMap<DefKey, ItemDefinitionEvaluatorFn>>,
}

impl ItemDefinitionEvaluator {
  /// Creates new item definition evaluator based on provided definitions.
  pub fn new(definitions: &DefDefinitions) -> Result<Self> {
    let mut evaluators = HashMap::new();
    for item_definition in definitions.item_definitions() {
      let evaluator = build_item_definition_evaluator(item_definition)?;
      let namespace = item_definition.namespace();
      let type_ref = item_definition.name();
      let def_key = DefKey::new(namespace, type_ref);
      evaluators.insert(def_key, evaluator);
    }
    Ok(Self { evaluators: Arc::new(evaluators) })
  }

  /// Evaluates item definition with specified type reference name.
  pub fn eval(&self, def_key: &DefKey, value: &Value) -> Option<Value> {
    self.evaluators.get(def_key).map(|evaluator| evaluator(value, self))
  }
}

pub fn build_item_definition_evaluator(item_definition: &DefItemDefinition) -> Result<ItemDefinitionEvaluatorFn> {
  // prepare optional allowed values evaluator
  let av_evaluator = build_allowed_values_evaluator(item_definition)?;
  // build item definition evaluator
  match item_definition.item_definition_type()? {
    ItemDefinitionType::SimpleType(feel_type) => build_simple_type_evaluator(feel_type, av_evaluator),
    ItemDefinitionType::ReferencedType(namespace, type_ref) => build_referenced_type_evaluator(DefKey::new(&namespace, &type_ref)),
    ItemDefinitionType::ComponentType => build_component_type_evaluator(item_definition),
    ItemDefinitionType::CollectionOfSimpleType(feel_type) => build_collection_of_simple_type_evaluator(feel_type, av_evaluator),
    ItemDefinitionType::CollectionOfReferencedType(namespace, type_ref) => build_collection_of_referenced_type_evaluator(DefKey::new(&namespace, &type_ref), av_evaluator),
    ItemDefinitionType::CollectionOfComponentType => build_collection_of_component_type_evaluator(item_definition),
    ItemDefinitionType::FunctionType => build_function_type_evaluator(),
  }
}

fn build_allowed_values_evaluator(item_definition: &DefItemDefinition) -> Result<Option<Evaluator>> {
  let mut av_evaluator = None;
  if let Some(unary_tests) = item_definition.allowed_values() {
    if let Some(text) = unary_tests.text() {
      let scope = FeelScope::default();
      let unary_tests_node = dsntk_feel_parser::parse_unary_tests(&scope, text, false)?;
      let node = AstNode::In(Box::new(AstNode::Name("?".into())), Box::new(unary_tests_node));
      av_evaluator = Some(dsntk_feel_evaluator::prepare(&node));
    }
  }
  Ok(av_evaluator)
}

fn check_allowed_values(value: Value, av_evaluator: Option<&Evaluator>) -> Value {
  if let Some(evaluator) = av_evaluator {
    let scope = FeelScope::default();
    scope.set_value(&"?".into(), value.clone());
    if evaluator(&scope).is_true() {
      value
    } else {
      value_null!("value '{}' is not allowed", value)
    }
  } else {
    value
  }
}

fn build_simple_type_evaluator(feel_type: FeelType, av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
  fn build_any_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| check_allowed_values(value.to_owned(), av_evaluator.as_ref()))
  }

  fn build_null_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| match value {
      Value::Null(_) => check_allowed_values(value.to_owned(), av_evaluator.as_ref()),
      _ => value_null!("expected type 'Null', actual type is '{}' in value '{}'", value.type_of(), value),
    })
  }

  fn build_string_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| match value {
      Value::String(_) => check_allowed_values(value.to_owned(), av_evaluator.as_ref()),
      null @ Value::Null(_) => null.clone(),
      _ => value_null!("expected type 'string', actual type is '{}' in value '{}'", value.type_of(), value),
    })
  }

  fn build_number_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| match value {
      Value::Number(_) => check_allowed_values(value.to_owned(), av_evaluator.as_ref()),
      null @ Value::Null(_) => null.clone(),
      _ => value_null!("expected type 'number', actual type is '{}' in value '{}'", value.type_of(), value),
    })
  }

  fn build_boolean_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| match value {
      Value::Boolean(_) => check_allowed_values(value.to_owned(), av_evaluator.as_ref()),
      null @ Value::Null(_) => null.clone(),
      _ => value_null!("expected type 'boolean', actual type is '{}' in value '{}'", value.type_of(), value),
    })
  }

  fn build_date_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| match value {
      Value::Date(_) => check_allowed_values(value.to_owned(), av_evaluator.as_ref()),
      null @ Value::Null(_) => null.clone(),
      _ => value_null!("expected type 'date', actual type is '{}' in value '{}'", value.type_of(), value),
    })
  }

  fn build_time_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| match value {
      Value::Time(_) => check_allowed_values(value.to_owned(), av_evaluator.as_ref()),
      null @ Value::Null(_) => null.clone(),
      _ => value_null!("expected type 'time', actual type is '{}' in value '{}'", value.type_of(), value),
    })
  }

  fn build_date_time_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| match value {
      Value::DateTime(_) => check_allowed_values(value.to_owned(), av_evaluator.as_ref()),
      null @ Value::Null(_) => null.clone(),
      _ => value_null!("expected type 'date and time', actual type is '{}' in value '{}'", value.type_of(), value),
    })
  }

  fn build_dt_duration_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| match value {
      Value::DaysAndTimeDuration(_) => check_allowed_values(value.to_owned(), av_evaluator.as_ref()),
      null @ Value::Null(_) => null.clone(),
      _ => value_null!("expected type 'days and time duration', actual type is '{}' in value '{}'", value.type_of(), value),
    })
  }

  fn build_ym_duration_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| match value {
      Value::YearsAndMonthsDuration(_) => check_allowed_values(value.to_owned(), av_evaluator.as_ref()),
      null @ Value::Null(_) => null.clone(),
      _ => value_null!("expected type 'years and months duration', actual type is '{}' in value '{}'", value.type_of(), value),
    })
  }
  match feel_type {
    FeelType::Any => Ok(build_any_evaluator(av_evaluator)),
    FeelType::Null => Ok(build_null_evaluator(av_evaluator)),
    FeelType::String => Ok(build_string_evaluator(av_evaluator)),
    FeelType::Number => Ok(build_number_evaluator(av_evaluator)),
    FeelType::Boolean => Ok(build_boolean_evaluator(av_evaluator)),
    FeelType::Date => Ok(build_date_evaluator(av_evaluator)),
    FeelType::Time => Ok(build_time_evaluator(av_evaluator)),
    FeelType::DateTime => Ok(build_date_time_evaluator(av_evaluator)),
    FeelType::DaysAndTimeDuration => Ok(build_dt_duration_evaluator(av_evaluator)),
    FeelType::YearsAndMonthsDuration => Ok(build_ym_duration_evaluator(av_evaluator)),
    _ => Err(err_unsupported_feel_type(feel_type, "a")),
  }
}

fn build_referenced_type_evaluator(def_key: DefKey) -> Result<ItemDefinitionEvaluatorFn> {
  Ok(Box::new(move |value: &Value, evaluators: &ItemDefinitionEvaluator| {
    evaluators.eval(&def_key, value).unwrap_or_else(|| value_null!("no evaluator"))
  }))
}

fn build_component_type_evaluator(item_definition: &DefItemDefinition) -> Result<ItemDefinitionEvaluatorFn> {
  let mut component_evaluators: Vec<(Name, ItemDefinitionEvaluatorFn)> = vec![];
  for component_item_definition in item_definition.item_components() {
    component_evaluators.push((component_item_definition.feel_name().clone(), build_item_definition_evaluator(component_item_definition)?));
  }
  let av_evaluator = build_allowed_values_evaluator(item_definition)?;
  Ok(Box::new(move |value: &Value, evaluators: &ItemDefinitionEvaluator| {
    if let Value::Context(ctx) = value {
      let mut evaluated_ctx = FeelContext::default();
      for (component_name, component_evaluator) in &component_evaluators {
        if let Some(component_value) = ctx.get_entry(component_name) {
          evaluated_ctx.set_entry(component_name, component_evaluator(component_value, evaluators));
        } else {
          return value_null!("item definition evaluator (Component): name not found: {} in context: {}", component_name, ctx);
        }
      }
      check_allowed_values(Value::Context(evaluated_ctx), av_evaluator.as_ref())
    } else {
      value_null!("item definition evaluator (Component): expected context value, actual value is: {}", value)
    }
  }))
}

fn build_collection_of_simple_type_evaluator(feel_type: FeelType, av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
  fn build_any_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        check_allowed_values(Value::List(values.clone()), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }

  fn build_null_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values {
          if let Value::Null(_) = item_value {
            evaluated_values.push(item_value.clone());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected null");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected null")
      }
    }))
  }

  fn build_string_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values {
          if let Value::String(_) = item_value {
            evaluated_values.push(item_value.clone());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected string");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }

  fn build_number_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values {
          if let Value::Number(_) = item_value {
            evaluated_values.push(item_value.clone());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected number");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }

  fn build_boolean_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values {
          if let Value::Boolean(_) = item_value {
            evaluated_values.push(item_value.to_owned());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected boolean");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }

  fn build_date_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values {
          if let Value::Date(_) = item_value {
            evaluated_values.push(item_value.to_owned());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected date");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }

  fn build_time_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values {
          if let Value::Time(_) = item_value {
            evaluated_values.push(item_value.to_owned());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected time");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }

  fn build_date_and_time_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values {
          if let Value::DateTime(_) = item_value {
            evaluated_values.push(item_value.to_owned());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected date and time");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }

  fn build_days_and_time_duration_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values {
          if let Value::DaysAndTimeDuration(_) = item_value {
            evaluated_values.push(item_value.to_owned());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected days and time duration");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }

  fn build_months_and_years_duration_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values {
          if let Value::YearsAndMonthsDuration(_) = item_value {
            evaluated_values.push(item_value.to_owned());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected months and years duration");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }
  // build evaluator based on FEEL type
  match feel_type {
    FeelType::Any => build_any_evaluator(av_evaluator),
    FeelType::Null => build_null_evaluator(av_evaluator),
    FeelType::String => build_string_evaluator(av_evaluator),
    FeelType::Number => build_number_evaluator(av_evaluator),
    FeelType::Boolean => build_boolean_evaluator(av_evaluator),
    FeelType::Date => build_date_evaluator(av_evaluator),
    FeelType::Time => build_time_evaluator(av_evaluator),
    FeelType::DateTime => build_date_and_time_evaluator(av_evaluator),
    FeelType::DaysAndTimeDuration => build_days_and_time_duration_evaluator(av_evaluator),
    FeelType::YearsAndMonthsDuration => build_months_and_years_duration_evaluator(av_evaluator),
    _ => Err(err_unsupported_feel_type(feel_type, "b")),
  }
}

fn build_collection_of_referenced_type_evaluator(def_key: DefKey, av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
  Ok(Box::new(move |value: &Value, evaluators: &ItemDefinitionEvaluator| {
    if let Value::List(values) = value {
      let mut evaluated_values = Values::default();
      for item_value in values {
        if let Some(evaluated_value) = evaluators.eval(&def_key, item_value) {
          evaluated_values.push(evaluated_value);
        } else {
          return value_null!("no evaluator defined for type reference '{}'", def_key);
        }
      }
      check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
    } else {
      value_null!("expected list, actual type is '{}' in value '{}'", value.type_of(), value)
    }
  }))
}

fn build_collection_of_component_type_evaluator(item_definition: &DefItemDefinition) -> Result<ItemDefinitionEvaluatorFn> {
  let mut component_evaluators: Vec<(Name, ItemDefinitionEvaluatorFn)> = vec![];
  for component_item_definition in item_definition.item_components() {
    component_evaluators.push((component_item_definition.feel_name().clone(), build_item_definition_evaluator(component_item_definition)?));
  }
  let av_evaluator = build_allowed_values_evaluator(item_definition)?;
  Ok(Box::new(move |value: &Value, evaluators: &ItemDefinitionEvaluator| {
    if let Value::List(values) = value {
      let mut evaluated_values = Values::default();
      for item_value in values {
        if let Value::Context(ctx) = item_value {
          let mut evaluated_ctx = FeelContext::default();
          for (component_name, component_evaluator) in &component_evaluators {
            if let Some(component_value) = ctx.get_entry(component_name) {
              evaluated_ctx.set_entry(component_name, component_evaluator(component_value, evaluators));
            } else {
              return value_null!("name '{}' not found in context '{}'", component_name, ctx);
            }
          }
          evaluated_values.push(Value::Context(evaluated_ctx))
        } else {
          return value_null!("expected context, actual type is '{}' in value '{}'", item_value.type_of(), item_value);
        }
      }
      check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
    } else {
      value_null!("expected list, actual type is '{}' in value '{}'", value.type_of(), value)
    }
  }))
}

fn build_function_type_evaluator() -> Result<ItemDefinitionEvaluatorFn> {
  Ok(Box::new(move |_: &Value, _: &ItemDefinitionEvaluator| {
    value_null!("function type evaluator not implemented yet")
  }))
}

#[cfg(test)]
mod tests {
  use crate::item_definition::ItemDefinitionEvaluator;
  use crate::model_definitions::{DefDefinitions, DefKey};
  use dsntk_examples::item_definition::*;
  use dsntk_feel::context::FeelContext;
  use dsntk_feel::values::Value;
  use dsntk_feel::{value_null, value_number, Name};
  use dsntk_feel_temporal::{FeelDate, FeelDateTime, FeelDaysAndTimeDuration, FeelTime, FeelYearsAndMonthsDuration};

  const NAMESPACE: &str = "https://dsntk.io";

  /// Utility function for building item definition evaluator from definitions.
  fn build_evaluator(xml: &str) -> ItemDefinitionEvaluator {
    let definitions = dsntk_model::from_xml(xml).unwrap();
    let mut def_definitions = DefDefinitions::default();
    def_definitions.add_model(&definitions);
    ItemDefinitionEvaluator::new(&def_definitions).unwrap()
  }

  #[test]
  fn test_evaluate_input_data_0101_1() {
    let evaluator = build_evaluator(DMN_0101);
    let context_str = r#"{ Customer Name : "Whistler" }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Customer", "Name"])).unwrap();
    assert_eq!(
      Value::String("Whistler".to_string()),
      evaluator.eval(&DefKey::new(NAMESPACE, "tCustomerName"), value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0101_2() {
    let evaluator = build_evaluator(DMN_0101);
    let context_str = r#"{ Customer Name : 12000 }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Customer", "Name"])).unwrap();
    assert_eq!(
      value_null!("expected type 'string', actual type is 'number' in value '12000'"),
      evaluator.eval(&DefKey::new(NAMESPACE, "tCustomerName"), value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0102_1() {
    let evaluator = build_evaluator(DMN_0102);
    let context_str = r#"{ Monthly Salary : 12000.00 }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Monthly", "Salary"])).unwrap();
    assert_eq!(value_number!(12_000), evaluator.eval(&DefKey::new(NAMESPACE, "tMonthlySalary"), value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0102_2() {
    let evaluator = build_evaluator(DMN_0102);
    let context_str = r#"{ Monthly Salary : true }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Monthly", "Salary"])).unwrap();
    assert_eq!(
      value_null!("expected type 'number', actual type is 'boolean' in value 'true'"),
      evaluator.eval(&DefKey::new(NAMESPACE, "tMonthlySalary"), value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0103_1() {
    let evaluator = build_evaluator(DMN_0103);
    let context_str = r#"{ Is Affordable : true }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Is", "Affordable"])).unwrap();
    assert_eq!(Value::Boolean(true), evaluator.eval(&DefKey::new(NAMESPACE, "tIsAffordable"), value).unwrap());
    let context_str = r#"{ Is Affordable : false }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Is", "Affordable"])).unwrap();
    assert_eq!(Value::Boolean(false), evaluator.eval(&DefKey::new(NAMESPACE, "tIsAffordable"), value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0103_2() {
    let evaluator = build_evaluator(DMN_0103);
    let context_str = r#"{ Is Affordable : "Yes" }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Is", "Affordable"])).unwrap();
    assert_eq!(
      value_null!(r#"expected type 'boolean', actual type is 'string' in value '"Yes"'"#),
      evaluator.eval(&DefKey::new(NAMESPACE, "tIsAffordable"), value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0104_1() {
    let evaluator = build_evaluator(DMN_0104);
    let context_str = r#"{ Birthday : date("1982-04-12") }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Birthday"])).unwrap();
    assert_eq!(
      Value::Date(FeelDate::new(1982, 4, 12).unwrap()),
      evaluator.eval(&DefKey::new(NAMESPACE, "tBirthday"), value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0105_1() {
    let evaluator = build_evaluator(DMN_0105);
    let context_str = r#"{ Delivery Time : time("18:35:23") }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Delivery", "Time"])).unwrap();
    assert_eq!(
      Value::Time(FeelTime::local_opt(18, 35, 23, 0).unwrap()),
      evaluator.eval(&DefKey::new(NAMESPACE, "tDeliveryTime"), value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0106_1() {
    let evaluator = build_evaluator(DMN_0106);
    let context_str = r#"{ Appointment : date and time("2021-10-12T18:35:23") }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Appointment"])).unwrap();
    assert_eq!(
      Value::DateTime(FeelDateTime::new(FeelDate::new(2021, 10, 12).unwrap(), FeelTime::local_opt(18, 35, 23, 0).unwrap())),
      evaluator.eval(&DefKey::new(NAMESPACE, "tAppointment"), value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0107_1() {
    let evaluator = build_evaluator(DMN_0107);
    let context_str = r#"{ Course Duration : duration("P2DT3H") }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Course", "Duration"])).unwrap();
    assert_eq!(
      Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::from_s(183600)),
      evaluator.eval(&DefKey::new(NAMESPACE, "tCourseDuration"), value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0108_1() {
    let evaluator = build_evaluator(DMN_0108);
    let context_str = r#"{ Growth Duration : duration("P2Y5M") }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Growth", "Duration"])).unwrap();
    assert_eq!(
      Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::from_ym(2, 5)),
      evaluator.eval(&DefKey::new(NAMESPACE, "tGrowthDuration"), value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0201_1() {
    let evaluator = build_evaluator(DMN_0201);
    let context_str = r#"{ Customer Name : "Bloomberg" }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Customer", "Name"])).unwrap();
    assert_eq!(
      Value::String("Bloomberg".to_string()),
      evaluator.eval(&DefKey::new(NAMESPACE, "tCustomerName"), value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0202_1() {
    let evaluator = build_evaluator(DMN_0202);
    let context_str = r#"{ Monthly Salary : 12000.00 }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Monthly", "Salary"])).unwrap();
    assert_eq!(value_number!(12_000), evaluator.eval(&DefKey::new(NAMESPACE, "tMonthlySalary"), value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0301_1() {
    let evaluator = build_evaluator(DMN_0301);
    let context_str = r#"{ Loan : { principal: 10, rate: 60, termMonths: 28 } }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Loan"])).unwrap();
    let mut ctx = FeelContext::default();
    ctx.set_entry(&"principal".into(), value_number!(10));
    ctx.set_entry(&"rate".into(), value_number!(60));
    ctx.set_entry(&"termMonths".into(), value_number!(28));
    let expected = Value::Context(ctx);
    assert_eq!(expected, evaluator.eval(&DefKey::new(NAMESPACE, "tLoan"), value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0401_1() {
    let evaluator = build_evaluator(DMN_0401);
    let context_str = r#"{ Items : ["Mercury", "Venus", "Earth", "Mars"] }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(vec![
      Value::String("Mercury".to_string()),
      Value::String("Venus".to_string()),
      Value::String("Earth".to_string()),
      Value::String("Mars".to_string()),
    ]);
    assert_eq!(expected, evaluator.eval(&DefKey::new(NAMESPACE, "tItems"), value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0402_1() {
    let evaluator = build_evaluator(DMN_0402);
    let context_str = r#"{ Items : [9000.00, 10000.00, 11000.00, 12000.00, 13000.00] }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(vec![
      value_number!(9_000),
      value_number!(10_000),
      value_number!(11_000),
      value_number!(12_000),
      value_number!(13_000),
    ]);
    assert_eq!(expected, evaluator.eval(&DefKey::new(NAMESPACE, "tItems"), value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0403_1() {
    let evaluator = build_evaluator(DMN_0403);
    let context_str = r#"{ Items : [true, false, false, true, true] }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(vec![
      Value::Boolean(true),
      Value::Boolean(false),
      Value::Boolean(false),
      Value::Boolean(true),
      Value::Boolean(true),
    ]);
    assert_eq!(expected, evaluator.eval(&DefKey::new(NAMESPACE, "tItems"), value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0404_1() {
    let evaluator = build_evaluator(DMN_0404);
    let context_str = r#"{ Items : [date("2021-10-10"), date("2021-10-11"), date("2021-10-12")] }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(vec![
      Value::Date(FeelDate::new(2021, 10, 10).unwrap()),
      Value::Date(FeelDate::new(2021, 10, 11).unwrap()),
      Value::Date(FeelDate::new(2021, 10, 12).unwrap()),
    ]);
    assert_eq!(expected, evaluator.eval(&DefKey::new(NAMESPACE, "tItems"), value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0405_1() {
    let evaluator = build_evaluator(DMN_0405);
    let context_str = r#"{ Items : [time("12:21:35"), time("12:21:36"), time("12:21:37")] }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(vec![
      Value::Time(FeelTime::local_opt(12, 21, 35, 0).unwrap()),
      Value::Time(FeelTime::local_opt(12, 21, 36, 0).unwrap()),
      Value::Time(FeelTime::local_opt(12, 21, 37, 0).unwrap()),
    ]);
    assert_eq!(expected, evaluator.eval(&DefKey::new(NAMESPACE, "tItems"), value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0406_1() {
    let evaluator = build_evaluator(DMN_0406);
    let context_str = r#"{ Items : [date and time("2021-10-10T21:23:18"), date and time("2021-10-11T12:18:59")] }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(vec![
      Value::DateTime(FeelDateTime::new(FeelDate::new(2021, 10, 10).unwrap(), FeelTime::local_opt(21, 23, 18, 0).unwrap())),
      Value::DateTime(FeelDateTime::new(FeelDate::new(2021, 10, 11).unwrap(), FeelTime::local_opt(12, 18, 59, 0).unwrap())),
    ]);
    assert_eq!(expected, evaluator.eval(&DefKey::new(NAMESPACE, "tItems"), value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0407_1() {
    let evaluator = build_evaluator(DMN_0407);
    let context_str = r#"{ Items : [duration("P2DT3H")] }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(vec![Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::from_s(183600))]);
    assert_eq!(expected, evaluator.eval(&DefKey::new(NAMESPACE, "tItems"), value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0408_1() {
    let evaluator = build_evaluator(DMN_0408);
    let context_str = r#"{ Items : [duration("P2Y3M"), duration("P2Y4M")] }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(vec![
      Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::from_ym(2, 3)),
      Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::from_ym(2, 4)),
    ]);
    assert_eq!(expected, evaluator.eval(&DefKey::new(NAMESPACE, "tItems"), value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0501_1() {
    let evaluator = build_evaluator(DMN_0501);
    let context_str = r#"{ Items : ["Mercury", "Venus", "Earth"] }"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(vec![
      Value::String("Mercury".to_string()),
      Value::String("Venus".to_string()),
      Value::String("Earth".to_string()),
    ]);
    assert_eq!(expected, evaluator.eval(&DefKey::new(NAMESPACE, "tItems"), value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0601_1() {
    let evaluator = build_evaluator(DMN_0601);
    let context_str = r#"{Items:[{number:1,name:"One",manager:"John"},{number:2,name:"Two",manager:"Mike"},{number:3,name:"Three",manager:"Bob"}]}"#;
    let context = dsntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let mut ctx_1 = FeelContext::default();
    ctx_1.set_entry(&"number".into(), value_number!(1));
    ctx_1.set_entry(&"name".into(), Value::String("One".to_string()));
    ctx_1.set_entry(&"manager".into(), Value::String("John".to_string()));
    let mut ctx_2 = FeelContext::default();
    ctx_2.set_entry(&"number".into(), value_number!(2));
    ctx_2.set_entry(&"name".into(), Value::String("Two".to_string()));
    ctx_2.set_entry(&"manager".into(), Value::String("Mike".to_string()));
    let mut ctx_3 = FeelContext::default();
    ctx_3.set_entry(&"number".into(), value_number!(3));
    ctx_3.set_entry(&"name".into(), Value::String("Three".to_string()));
    ctx_3.set_entry(&"manager".into(), Value::String("Bob".to_string()));
    let expected = Value::List(vec![Value::Context(ctx_1), Value::Context(ctx_2), Value::Context(ctx_3)]);
    assert_eq!(Some(expected), evaluator.eval(&DefKey::new(NAMESPACE, "tItems"), value));
  }
}
