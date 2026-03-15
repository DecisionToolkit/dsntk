//! # Compatibility benchmarks

use dsntk_examples::*;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::FeelScope;
use dsntk_model::DmnElement;
use dsntk_model_evaluator::ModelEvaluator;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, LazyLock};
use test::Bencher;

mod level_2;
mod level_3;
mod non_compliant;

const ENV_DRY_RUN: &str = "DSNTK_DRY_BENCHMARK_RUN";

static BENCHMARK_COUNT: AtomicUsize = AtomicUsize::new(0);

macro_rules! from_examples {
  ($model_name:tt) => {
    model_evaluator_from_examples!($model_name);
    model_namespace_from_examples!($model_name);
    model_name_from_examples!($model_name);
  };
}

macro_rules! iter {
  ($b:tt, $benchmark:expr) => {
    match std::env::var(ENV_DRY_RUN) {
      Ok(_) => {
        BENCHMARK_COUNT.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
        print!("total benchmark count: {} | ", BENCHMARK_COUNT.load(core::sync::atomic::Ordering::SeqCst));
      }
      Err(_) => {
        $b.iter(|| $benchmark);
      }
    }
  };
}

macro_rules! model_evaluator_from_examples {
  ($model_name:tt) => {
    static MODEL_EVALUATOR: LazyLock<Arc<ModelEvaluator>> = LazyLock::new(|| build_model_evaluator(dsntk_examples::$model_name));
  };
}

macro_rules! model_namespace_from_examples {
  ($model_name:tt) => {
    static MODEL_NAMESPACE: LazyLock<String> = LazyLock::new(|| build_model_namespace(dsntk_examples::$model_name));
  };
}

macro_rules! model_name_from_examples {
  ($model_name:tt) => {
    static MODEL_NAME: LazyLock<String> = LazyLock::new(|| build_model_name(dsntk_examples::$model_name));
  };
}

macro_rules! static_context {
  ($name:tt, $content:tt) => {
    static $name: LazyLock<FeelContext> = LazyLock::new(|| context($content));
  };
}

use dsntk_model::NamedElement;
use {from_examples, iter, model_evaluator_from_examples, model_name_from_examples, model_namespace_from_examples, static_context};

/// Utility function that builds a model evaluator from a single DMN model.
fn build_model_evaluator(model_content: &str) -> Arc<ModelEvaluator> {
  let definitions = dsntk_model::from_xml(model_content).unwrap();
  ModelEvaluator::new(&[definitions]).unwrap()
}

/// Utility function that builds a model evaluator from multiple DMN models.
fn build_model_evaluators(model_content: &[&str]) -> Arc<ModelEvaluator> {
  let mut definitions = vec![];
  for content in model_content {
    definitions.push(dsntk_model::from_xml(content).unwrap());
  }
  ModelEvaluator::new(&definitions).unwrap()
}

/// Utility function that returns a model namespace from a single DMN model.
fn build_model_namespace(model_content: &str) -> String {
  let definitions = dsntk_model::from_xml(model_content).unwrap();
  definitions.namespace().to_string()
}

/// Utility function that returns a model namespace from a single DMN model.
fn build_model_name(model_content: &str) -> String {
  let definitions = dsntk_model::from_xml(model_content).unwrap();
  definitions.name().to_string()
}

/// Utility function that creates a FEEL context from specified input expression.
pub fn context(input: &str) -> FeelContext {
  let scope = FeelScope::default();
  match dsntk_feel_parser::parse_context(&scope, input, false) {
    Ok(node) => {
      let evaluator = dsntk_feel_evaluator::prepare(&node);
      match evaluator(&scope) {
        Value::Context(ctx) => ctx,
        other => panic!("ERROR: expected context value, actual value is: {}", other as Value),
      }
    }
    Err(reason) => panic!("ERROR: parsing context failed with reason: {reason}"),
  }
}

/// Utility function that evaluates a `Decision` specified by name and compares the result.
fn assert_decision(model_evaluator: &ModelEvaluator, model_namespace: &str, model_name: &str, invocable_name: &str, input_data: &FeelContext, expected: &str) {
  let actual = model_evaluator.evaluate_invocable(model_namespace, model_name, invocable_name, input_data).to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the decision does not match the expected value:\n  expected: {expected}\n    actual: {actual}\n"
  );
}

/// Utility function that evaluates a `BusinessKnowledgeModel` specified by name and compares the result.
fn assert_business_knowledge_model(model_evaluator: &ModelEvaluator, model_namespace: &str, model_name: &str, invocable_name: &str, input_data: &FeelContext, expected: &str) {
  let actual = model_evaluator.evaluate_invocable(model_namespace, model_name, invocable_name, input_data).to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the business knowledge model does not match the expected value:\n  expected: {expected}\n    actual: {actual}\n"
  );
}

/// Utility function that evaluates a `DecisionService` specified by name and compares the result with expected value.
fn assert_decision_service(model_evaluator: &ModelEvaluator, model_namespace: &str, model_name: &str, invocable_name: &str, input_data: &FeelContext, expected: &str) {
  let actual = model_evaluator.evaluate_invocable(model_namespace, model_name, invocable_name, input_data).to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the decision service does not match the expected value:\n  expected: {expected}\n    actual: {actual}\n"
  );
}
