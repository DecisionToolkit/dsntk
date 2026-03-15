//! Builder for item definition context evaluators.

use crate::errors::*;
use crate::model_definitions::{DefDefinitions, DefItemDefinition, DefKey};
use dsntk_common::Result;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::{FeelType, Name};
use dsntk_model::ItemDefinitionType;
use std::collections::{BTreeMap, HashMap};

/// Type of closure that evaluates the item definition context.
type ItemDefinitionContextEvaluatorFn = Box<dyn Fn(&Name, &mut FeelContext, &ItemDefinitionContextEvaluator) -> FeelType + Send + Sync>;

/// Item definition type evaluators.
#[derive(Default)]
pub struct ItemDefinitionContextEvaluator {
  evaluators: HashMap<DefKey, ItemDefinitionContextEvaluatorFn>,
}

impl ItemDefinitionContextEvaluator {
  /// Creates item definition type evaluators.
  pub fn new(definitions: &DefDefinitions) -> Result<Self> {
    let mut evaluators = HashMap::new();
    for item_definition in definitions.item_definitions() {
      let evaluator = item_definition_context_evaluator(item_definition)?;
      let namespace = item_definition.namespace();
      let type_ref = item_definition.name();
      let def_key = DefKey::new(namespace, type_ref);
      evaluators.insert(def_key, evaluator);
    }
    Ok(Self { evaluators })
  }

  /// Evaluates a type from item definition with specified type reference name.
  pub fn eval(&self, def_key: &DefKey, type_reference_name: &Name, ctx: &mut FeelContext) -> FeelType {
    if let Some(evaluator) = self.evaluators.get(def_key) {
      evaluator(type_reference_name, ctx, self)
    } else {
      FeelType::Any
    }
  }
}

fn item_definition_context_evaluator(item_definition: &DefItemDefinition) -> Result<ItemDefinitionContextEvaluatorFn> {
  match item_definition.item_definition_type()? {
    ItemDefinitionType::SimpleType(feel_type) => simple_type_context_evaluator(feel_type),
    ItemDefinitionType::ReferencedType(namespace, ref_type) => referenced_type_context_evaluator(DefKey::new(&namespace, &ref_type)),
    ItemDefinitionType::ComponentType => component_type_context_evaluator(item_definition),
    ItemDefinitionType::CollectionOfSimpleType(feel_type) => collection_of_simple_type_context_evaluator(feel_type),
    ItemDefinitionType::CollectionOfReferencedType(namespace, ref_type) => collection_of_referenced_type_context_evaluator(DefKey::new(&namespace, &ref_type)),
    ItemDefinitionType::CollectionOfComponentType => collection_of_component_type_context_evaluator(item_definition),
    ItemDefinitionType::FunctionType => function_type_context_evaluator(item_definition),
  }
}

fn simple_type_context_evaluator(feel_type: FeelType) -> Result<ItemDefinitionContextEvaluatorFn> {
  if matches!(
    feel_type,
    FeelType::Any
      | FeelType::Null
      | FeelType::String
      | FeelType::Number
      | FeelType::Boolean
      | FeelType::Date
      | FeelType::Time
      | FeelType::DateTime
      | FeelType::DaysAndTimeDuration
      | FeelType::YearsAndMonthsDuration
  ) {
    Ok(Box::new(move |name: &Name, ctx: &mut FeelContext, _: &ItemDefinitionContextEvaluator| {
      ctx.set_entry(name, Value::FeelType(feel_type.clone()));
      feel_type.clone()
    }))
  } else {
    Err(err_unsupported_feel_type(feel_type, "c"))
  }
}

fn referenced_type_context_evaluator(def_key: DefKey) -> Result<ItemDefinitionContextEvaluatorFn> {
  Ok(Box::new(move |name: &Name, ctx: &mut FeelContext, evaluator: &ItemDefinitionContextEvaluator| {
    evaluator.eval(&def_key, name, ctx)
  }))
}

fn component_type_context_evaluator(item_definition: &DefItemDefinition) -> Result<ItemDefinitionContextEvaluatorFn> {
  let mut context_evaluators = vec![];
  for component_item_definition in item_definition.item_components() {
    context_evaluators.push((component_item_definition.feel_name().clone(), item_definition_context_evaluator(component_item_definition)?));
  }
  Ok(Box::new(move |name: &Name, ctx: &mut FeelContext, evaluator: &ItemDefinitionContextEvaluator| {
    let mut entries = BTreeMap::new();
    let mut evaluated_ctx = FeelContext::default();
    for (component_name, component_evaluator) in &context_evaluators {
      let feel_type = component_evaluator(component_name, &mut evaluated_ctx, evaluator);
      entries.insert(component_name.clone(), feel_type);
    }
    ctx.set_entry(name, Value::Context(evaluated_ctx));
    FeelType::Context(entries)
  }))
}

fn collection_of_simple_type_context_evaluator(feel_type: FeelType) -> Result<ItemDefinitionContextEvaluatorFn> {
  if matches!(
    feel_type,
    FeelType::Any
      | FeelType::Null
      | FeelType::String
      | FeelType::Number
      | FeelType::Boolean
      | FeelType::Date
      | FeelType::Time
      | FeelType::DateTime
      | FeelType::DaysAndTimeDuration
      | FeelType::YearsAndMonthsDuration
  ) {
    Ok(Box::new(move |name: &Name, ctx: &mut FeelContext, _: &ItemDefinitionContextEvaluator| {
      let list_type = FeelType::List(Box::new(feel_type.clone()));
      let list = Value::List(vec![Value::FeelType(feel_type.clone())]);
      ctx.set_entry(name, list);
      list_type
    }))
  } else {
    Err(err_unsupported_feel_type(feel_type, "d"))
  }
}

fn collection_of_referenced_type_context_evaluator(def_key: DefKey) -> Result<ItemDefinitionContextEvaluatorFn> {
  Ok(Box::new(move |name: &Name, ctx: &mut FeelContext, evaluator: &ItemDefinitionContextEvaluator| {
    let mut evaluated_ctx = FeelContext::default();
    let feel_type = evaluator.eval(&def_key, name, &mut evaluated_ctx);
    let list_type = FeelType::List(Box::new(feel_type.clone()));
    let list = Value::List(vec![Value::FeelType(feel_type)]);
    ctx.set_entry(name, list);
    list_type
  }))
}

fn collection_of_component_type_context_evaluator(item_definition: &DefItemDefinition) -> Result<ItemDefinitionContextEvaluatorFn> {
  let mut context_evaluators = vec![];
  for component_item_definition in item_definition.item_components() {
    context_evaluators.push((component_item_definition.feel_name().clone(), item_definition_context_evaluator(component_item_definition)?));
  }
  Ok(Box::new(move |name: &Name, ctx: &mut FeelContext, evaluator: &ItemDefinitionContextEvaluator| {
    let mut entries = BTreeMap::new();
    let mut evaluated_ctx = FeelContext::default();
    for (component_name, component_evaluator) in &context_evaluators {
      let feel_type = component_evaluator(component_name, &mut evaluated_ctx, evaluator);
      entries.insert(component_name.clone(), feel_type);
    }
    let list_type = FeelType::List(Box::new(FeelType::Context(entries)));
    let list = Value::List(vec![Value::Context(evaluated_ctx)]);
    ctx.set_entry(name, list);
    list_type
  }))
}

fn function_type_context_evaluator(_item_definition: &DefItemDefinition) -> Result<ItemDefinitionContextEvaluatorFn> {
  Ok(Box::new(move |_name: &Name, _ctx: &mut FeelContext, _: &ItemDefinitionContextEvaluator| FeelType::Any))
  //TODO Implement function type.
}

#[cfg(test)]
mod tests {
  use crate::item_definition_context::ItemDefinitionContextEvaluator;
  use crate::model_definitions::{DefDefinitions, DefKey};
  use dsntk_examples::item_definition::*;
  use dsntk_feel::context::FeelContext;
  use dsntk_feel::values::Value;
  use dsntk_feel::{FeelType, Name};

  const NAMESPACE: &str = "https://dsntk.io";

  /// Utility function for building item definition evaluator from definitions.
  fn build_evaluator(xml: &str) -> ItemDefinitionContextEvaluator {
    let definitions = dsntk_model::from_xml(xml).unwrap();
    let mut def_definitions = DefDefinitions::default();
    def_definitions.add_model(&definitions);
    ItemDefinitionContextEvaluator::new(&def_definitions).unwrap()
  }

  #[test]
  fn simple_type_string() {
    let evaluator = build_evaluator(DMN_0101);
    let mut ctx = FeelContext::default();
    let expected_type = FeelType::String;
    let mut expected_context = FeelContext::default();
    let variable_name: Name = "Customer Name".into();
    expected_context.set_entry(&variable_name, Value::FeelType(FeelType::String));
    let actual_type = evaluator.eval(&DefKey::new(NAMESPACE, "tCustomerName"), &variable_name, &mut ctx);
    assert_eq!(expected_type, actual_type);
    assert_eq!(expected_context, ctx);
    assert_eq!("{Customer Name: type(string)}", ctx.to_string());
  }

  #[test]
  fn simple_type_number() {
    // let evaluator = build_evaluator(DMN_0102);
    // let mut ctx = FeelContext::default();
    // let expected_type = FeelType::Number;
    // let mut expected_context = FeelContext::default();
    // expected_context.set_entry(&"Monthly Salary".into(), Value::FeelType(FeelType::Number));
    // let actual_type = evaluator.eval("tMonthlySalary", &"Monthly Salary".into(), &mut ctx);
    // assert_eq!(expected_type, actual_type);
    // assert_eq!(expected_context, ctx);
    // assert_eq!("{Monthly Salary: type(number)}", ctx.to_string());
  }
  /*
      #[test]
      fn simple_type_boolean() {
        let definitions = &dsntk_model::parse(DMN_0103).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::Boolean), evaluators.eval("tIsAffordable"));
      }

      #[test]
      fn simple_type_date() {
        let definitions = &dsntk_model::parse(DMN_0104).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::Date), evaluators.eval("tBirthday"));
      }

      #[test]
      fn simple_type_time() {
        let definitions = &dsntk_model::parse(DMN_0105).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::Time), evaluators.eval("tDeliveryTime"));
      }

      #[test]
      fn simple_type_date_time() {
        let definitions = &dsntk_model::parse(DMN_0106).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::DateTime), evaluators.eval("tAppointment"));
      }

      #[test]
      fn simple_type_days_and_time_duration() {
        let definitions = &dsntk_model::parse(DMN_0107).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::DaysAndTimeDuration), evaluators.eval("tCourseDuration"));
      }

      #[test]
      fn simple_type_years_and_month_duration() {
        let definitions = &dsntk_model::parse(DMN_0108).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::YearsAndMonthsDuration), evaluators.eval("tGrowthDuration"));
      }
  */

  #[test]
  fn referenced_type_string() {
    // let evaluator = build_evaluator(DMN_0201);
    // let mut ctx = FeelContext::default();
    // let expected_type = FeelType::String;
    // let mut expected_context = FeelContext::default();
    // expected_context.set_entry(&"Customer Name".into(), Value::FeelType(FeelType::String));
    // let actual_type = evaluator.eval("tCustomerName", &"Customer Name".into(), &mut ctx);
    // assert_eq!(expected_type, actual_type);
    // assert_eq!(expected_context, ctx);
    // assert_eq!("{Customer Name: type(string)}", ctx.to_string());
  }

  /*
  #[test]
  fn referenced_type_number() {
    let definitions = &dsntk_model::parse(DMN_0202).unwrap();
    let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
    assert_eq!(Some(FeelType::Number), evaluators.eval("tMonthlySalary"));
  }
  */

  #[test]
  fn component_type() {
    // let evaluator = build_evaluator(DMN_0301);
    // let mut ctx = FeelContext::default();
    // let name_principal: Name = "principal".into();
    // let name_rate: Name = "rate".into();
    // let name_term_months: Name = "termMonths".into();
    // let type_number = FeelType::Number;
    // let expected_type = FeelType::context(&[(&name_principal, &type_number), (&name_rate, &type_number), (&name_term_months, &type_number)]);
    // let mut inner_context = FeelContext::default();
    // inner_context.set_entry(&name_principal, Value::FeelType(type_number.clone()));
    // inner_context.set_entry(&name_rate, Value::FeelType(type_number.clone()));
    // inner_context.set_entry(&name_term_months, Value::FeelType(type_number));
    // let mut expected_context = FeelContext::default();
    // expected_context.set_entry(&"Loan".into(), Value::Context(inner_context));
    // let actual_type = evaluator.eval("tLoan", &"Loan".into(), &mut ctx);
    // assert_eq!(expected_type, actual_type);
    // assert_eq!(expected_context, ctx);
    // assert_eq!("{Loan: {principal: type(number), rate: type(number), termMonths: type(number)}}", ctx.to_string());
  }

  #[test]
  fn collection_of_simple_type_string() {
    let evaluator = build_evaluator(DMN_0401);
    let mut ctx = FeelContext::default();
    let expected_type = FeelType::List(Box::new(FeelType::String));
    let mut expected_context = FeelContext::default();
    let variable_name: Name = "Items".into();
    expected_context.set_entry(&variable_name, Value::List(vec![Value::FeelType(FeelType::String)]));
    let actual_type = evaluator.eval(&DefKey::new(NAMESPACE, "tItems"), &variable_name, &mut ctx);
    assert_eq!(expected_type, actual_type);
    assert_eq!(expected_context, ctx);
    assert_eq!("{Items: [type(string)]}", ctx.to_string());
  }

  /*
      #[test]
      fn collection_of_simple_type_number() {
        let definitions = &dsntk_model::parse(DMN_0402).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::list(&FeelType::Number)), evaluators.eval("tItems"));
      }

      #[test]
      fn collection_of_simple_type_boolean() {
        let definitions = &dsntk_model::parse(DMN_0403).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::list(&FeelType::Boolean)), evaluators.eval("tItems"));
      }

      #[test]
      fn collection_of_simple_type_date() {
        let definitions = &dsntk_model::parse(DMN_0404).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::list(&FeelType::Date)), evaluators.eval("tItems"));
      }

      #[test]
      fn collection_of_simple_type_time() {
        let definitions = &dsntk_model::parse(DMN_0405).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::list(&FeelType::Time)), evaluators.eval("tItems"));
      }

      #[test]
      fn collection_of_simple_type_date_time() {
        let definitions = &dsntk_model::parse(DMN_0406).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::list(&FeelType::DateTime)), evaluators.eval("tItems"));
      }

      #[test]
      fn collection_of_simple_type_days_and_time_duration() {
        let definitions = &dsntk_model::parse(DMN_0407).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::list(&FeelType::DaysAndTimeDuration)), evaluators.eval("tItems"));
      }

      #[test]
      fn collection_of_simple_type_years_and_months_duration() {
        let definitions = &dsntk_model::parse(DMN_0408).unwrap();
        let evaluators = ItemDefinitionTypeEvaluator::new(definitions).unwrap();
        assert_eq!(Some(FeelType::list(&FeelType::YearsAndMonthsDuration)), evaluators.eval("tItems"));
      }

  */
  #[test]
  fn collection_of_referenced_type_string() {
    // let evaluator = build_evaluator(DMN_0501);
    // let mut ctx = FeelContext::default();
    // let expected_type = FeelType::List(Box::new(FeelType::String));
    // let mut expected_context = FeelContext::default();
    // expected_context.set_entry(&"Items".into(), Value::List(Values::new(vec![Value::FeelType(FeelType::String)])));
    // let actual_type = evaluator.eval("tItems", &"Items".into(), &mut ctx);
    // assert_eq!(expected_type, actual_type);
    // assert_eq!(expected_context, ctx);
    // assert_eq!("{Items: [type(string)]}", ctx.to_string());
  }

  #[test]
  fn collection_of_component_type() {
    // let evaluator = build_evaluator(DMN_0601);
    // let mut ctx = FeelContext::default();
    // let name_manager: Name = "manager".into();
    // let name_name: Name = "name".into();
    // let name_number: Name = "number".into();
    // let type_number = FeelType::Number;
    // let type_string = FeelType::String;
    // let expected_type = FeelType::list(&FeelType::context(&[
    //   (&name_manager, &type_string),
    //   (&name_name, &type_string),
    //   (&name_number, &type_number),
    // ]));
    // let mut inner_context = FeelContext::default();
    // inner_context.set_entry(&name_manager, Value::FeelType(type_string.clone()));
    // inner_context.set_entry(&name_name, Value::FeelType(type_string));
    // inner_context.set_entry(&name_number, Value::FeelType(type_number));
    // let mut expected_context = FeelContext::default();
    // expected_context.set_entry(&"Items".into(), Value::List(Values::new(vec![Value::Context(inner_context)])));
    // let actual_type = evaluator.eval("tItems", &"Items".into(), &mut ctx);
    // assert_eq!("{Items: [{manager: type(string), name: type(string), number: type(number)}]}", ctx.to_string());
    // assert_eq!(expected_type, actual_type);
    // assert_eq!(expected_context, ctx);
  }
}
