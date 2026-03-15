use crate::model_evaluator::ModelEvaluator;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::FeelScope;
use dsntk_model::DmnElement;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::sync::{Arc, LazyLock};
use walkdir::WalkDir;

mod compatibility;
mod exhaustive;
mod input_data;
mod simulation;
mod various;

macro_rules! from_examples {
  ($model_name:tt) => {
    model_evaluator_from_examples!($model_name);
    model_namespace_from_examples!($model_name);
    model_name_from_examples!($model_name);
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

macro_rules! model_evaluator {
  ($model_name:tt) => {
    static MODEL_EVALUATOR: LazyLock<Arc<ModelEvaluator>> = LazyLock::new(|| build_model_evaluator($model_name));
  };
}

macro_rules! static_context {
  ($name:tt, $content:tt) => {
    static $name: LazyLock<FeelContext> = LazyLock::new(|| context($content));
  };
}

use dsntk_model::NamedElement;
use {from_examples, model_evaluator, model_evaluator_from_examples, model_name_from_examples, model_namespace_from_examples, static_context};

/// Utility function that creates a `FEEL` context from specified input expression.
pub fn context(input: &str) -> FeelContext {
  let scope = FeelScope::default();
  match dsntk_feel_parser::parse_context(&scope, input, false) {
    Ok(node) => {
      let evaluator = dsntk_feel_evaluator::prepare(&node);
      match evaluator(&scope) {
        Value::Context(ctx) => ctx,
        other => panic!("ERROR: expected context value, actual value is: {}", other),
      }
    }
    Err(reason) => panic!("ERROR: parsing context failed with reason: {}", reason),
  }
}

/// Utility function that builds a model evaluator from single XML model definitions.
fn build_model_evaluator(model_content: &str) -> Arc<ModelEvaluator> {
  let definitions = dsntk_model::from_xml(model_content).unwrap();
  ModelEvaluator::new(&[definitions]).unwrap()
}

/// Utility function that builds a model evaluator from multiple XML model definitions.
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

/// Utility function that returns a model names from a single DMN model.
fn build_model_name(model_content: &str) -> String {
  let definitions = dsntk_model::from_xml(model_content).unwrap();
  definitions.name().to_string()
}

/// Utility function that evaluates a [Decision] specified by name and compares the result.
fn assert_decision(model_evaluator: &ModelEvaluator, model_namespace: &str, model_name: &str, invocable: &str, input_data: &FeelContext, expected: &str) {
  let actual = model_evaluator.evaluate_invocable(model_namespace, model_name, invocable, input_data).to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the decision does not match the expected value:\n  expected: {expected}\n    actual: {actual}\n"
  );
}

/// Utility function that evaluates a [BusinessKnowledgeModel] specified by name and compares the result.
fn assert_business_knowledge_model(model_evaluator: &ModelEvaluator, model_namespace: &str, model_name: &str, invocable_name: &str, input_data: &FeelContext, expected: &str) {
  let actual = model_evaluator.evaluate_invocable(model_namespace, model_name, invocable_name, input_data).to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the business knowledge model does not match the expected value:\n  expected: {expected}\n    actual: {actual}\n"
  );
}

/// Utility function that evaluates a [DecisionService] specified by name and compares the result with expected value.
fn assert_decision_service(model_evaluator: &ModelEvaluator, model_namespace: &str, model_name: &str, invocable_name: &str, input_data: &FeelContext, expected: &str) {
  let actual = model_evaluator.evaluate_invocable(model_namespace, model_name, invocable_name, input_data).to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the decision service does not match the expected value:\n  expected: {expected}\n    actual: {actual}\n"
  );
}

/// This utility function compares the number of test cases defined in `./compatibility` directory
/// with the actual number of benchmarks defined in `../../benches/compatibility` directory.
#[test]
fn compare_the_number_of_benchmarks_with_tests() {
  let tests = count_lines("src/tests/compatibility", "#[test]");
  let benches = count_lines("benches/compatibility", "#[bench]");
  let keys_tests = tests.keys().cloned().collect::<BTreeSet<String>>();
  let keys_benches = benches.keys().cloned().collect::<BTreeSet<String>>();
  let keys: BTreeSet<String> = keys_tests.union(&keys_benches).cloned().collect::<BTreeSet<String>>();
  let mut total_ct = 0_usize;
  let mut total_cb = 0_usize;
  for key in keys {
    let ct = *tests.get(&key).unwrap_or(&0);
    total_ct += ct;
    let cb = *benches.get(&key).unwrap_or(&0);
    total_cb += cb;
    let marker = if ct != cb { "*" } else { "" };
    println!("{:30} {:>12} {:>12} {:3} {}", key, ct, cb, marker, ct - cb);
  }
  println!("{:30} {:>12} {:>12}", "TOTAL", total_ct, total_cb);
}

/// Counts test cases defined in test files in the specified directory.
fn count_lines(dir: &str, pattern: &str) -> BTreeMap<String, usize> {
  let mut results = BTreeMap::new();
  for entry_result in WalkDir::new(dir).into_iter() {
    let entry = entry_result.unwrap();
    let path = entry.path();
    if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") && path.file_name().unwrap().to_string_lossy().starts_with("dmn_") {
      let content = fs::read_to_string(path).unwrap();
      let count = content.lines().filter(|line| line.contains(pattern)).count();
      results.insert(path.strip_prefix(dir).unwrap().display().to_string(), count);
    }
  }
  results
}
