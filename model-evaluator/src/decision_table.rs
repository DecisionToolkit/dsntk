//! Builder for decision table evaluators.

use dsntk_common::Result;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::{Evaluator, FeelScope, Name, value_null};
use dsntk_feel_parser::AstNode;
use dsntk_model::{BuiltinAggregator, DecisionTable, HitPolicy};
use std::cmp::Ordering;
use std::sync::LazyLock;

/// The name `?`, the tested value inside a unary test, interned once.
///
/// See the note on `NAME_QUESTION_MARK` in `item_definition.rs`, which binds the same
/// name for the allowed-values test of an item definition.
static NAME_QUESTION_MARK: LazyLock<Name> = LazyLock::new(|| "?".into());

/// Parsed rule of the decision table.
/// Input entries and output entries are parsed into evaluation clauses
/// and stored in this structure.
struct ParsedRule {
  input_entries_evaluators: Vec<Evaluator>,
  output_entries_evaluators: Vec<Evaluator>,
}

/// Parsed decision table.
/// All expressions contained in different parts of the decision
/// table are parsed into evaluation clauses and stored in this structure.
struct ParsedDecisionTable {
  component_names: Vec<Name>,
  output_values_evaluators: Vec<Option<Evaluator>>,
  default_output_values_evaluators: Vec<Option<Evaluator>>,
  rules: Vec<ParsedRule>,
}

/// Evaluated rule of a decision table.
struct EvaluatedRule {
  matches: bool,
  output_entry_values: Vec<Value>,
}

/// Evaluated decision table.
/// All evaluation clauses from [ParsedDecisionTable] are executed
/// in specified context and results are stored as [Values](Value) in this structure.
struct EvaluatedDecisionTable {
  component_names: Vec<Name>,
  output_values: Vec<Value>,
  default_output_values: Vec<Value>,
  evaluated_rules: Vec<EvaluatedRule>,
}

impl EvaluatedDecisionTable {
  /// Returns all matching rules in rule order.
  fn get_matching_rules(&self) -> Vec<&EvaluatedRule> {
    self.evaluated_rules.iter().filter(|evaluated_rule| evaluated_rule.matches).collect()
  }
  /// Returns all matching rules in decreasing order of priority.
  fn get_matching_rules_prioritized(&self) -> Vec<&EvaluatedRule> {
    let mut rules: Vec<&EvaluatedRule> = self.evaluated_rules.iter().filter(|v| v.matches).collect();
    let compare = |x: &&EvaluatedRule, y: &&EvaluatedRule| {
      for (v1, v2) in x.output_entry_values.iter().zip(y.output_entry_values.iter()) {
        let index1 = self.output_values.iter().position(|o| o == v1);
        let index2 = self.output_values.iter().position(|o| o == v2);
        match (index1, index2) {
          (Some(ix1), Some(ix2)) => {
            if ix1 < ix2 {
              return Ordering::Less;
            }
            if ix1 > ix2 {
              return Ordering::Greater;
            }
          }
          (Some(_), None) => return Ordering::Less,
          (None, Some(_)) => return Ordering::Greater,
          _ => {}
        }
      }
      Ordering::Equal
    };
    rules.sort_by(compare);
    rules
  }
  /// Returns a result composed from values taken from evaluated output entries.
  fn get_result(&self, evaluated_rule: &EvaluatedRule) -> Value {
    if evaluated_rule.output_entry_values.len() > 1 {
      if evaluated_rule.output_entry_values.len() != self.component_names.len() {
        return value_null!("err_number_of_output_values_differ_from_component_names");
      }
      let mut result: FeelContext = Default::default();
      for (i, value) in evaluated_rule.output_entry_values.iter().enumerate() {
        result.set_entry(&self.component_names[i], value.clone());
      }
      Value::Context(result)
    } else {
      evaluated_rule.output_entry_values[0].clone()
    }
  }
  /// Returns a result composed from values taken from evaluated output entries.
  fn get_results(&self, evaluated_rules: &[&EvaluatedRule]) -> Value {
    let mut values = vec![];
    for evaluated_rule in evaluated_rules {
      values.push(self.get_result(evaluated_rule));
    }
    Value::List(values)
  }

  fn evaluate_default_output_value(&self) -> Value {
    match self.default_output_values.len() {
      0 => value_null!("no rules matched, no output value defined"),
      1 => self.default_output_values[0].clone(),
      _ => {
        let mut result: FeelContext = Default::default();
        for (i, value) in self.default_output_values.iter().enumerate() {
          result.set_entry(&self.component_names[i], value.clone());
        }
        Value::Context(result)
      }
    }
  }

  fn evaluate_hit_policy_unique(&self) -> Value {
    let matching_rules = self.get_matching_rules();
    if matching_rules.is_empty() {
      return self.evaluate_default_output_value();
    }
    if matching_rules.len() > 1 {
      return value_null!("err_multiple_rules_match_in_unique_hit_policy");
    }
    self.get_result(matching_rules[0])
  }

  fn evaluate_hit_policy_any(&self) -> Value {
    let matching_rules = self.get_matching_rules();
    if matching_rules.is_empty() {
      return self.evaluate_default_output_value();
    }
    let first_result = self.get_result(matching_rules[0]);
    for evaluated_rule in matching_rules {
      let result = self.get_result(evaluated_rule);
      if result != first_result {
        return value_null!("err_all_matching_rules_must_have_the_same_value");
      }
    }
    first_result
  }

  fn evaluate_hit_policy_priority(&self) -> Value {
    let matching_rules = self.get_matching_rules_prioritized();
    if matching_rules.is_empty() {
      return self.evaluate_default_output_value();
    }
    self.get_result(matching_rules[0])
  }

  fn evaluate_hit_policy_first(&self) -> Value {
    let matching_rules = self.get_matching_rules();
    if matching_rules.is_empty() {
      return self.evaluate_default_output_value();
    }
    self.get_result(matching_rules[0])
  }

  fn evaluate_hit_policy_rule_order(&self) -> Value {
    let matching_rules = self.get_matching_rules();
    if matching_rules.is_empty() {
      return self.evaluate_default_output_value();
    }
    self.get_results(&matching_rules)
  }

  fn evaluate_hit_policy_output_order(&self) -> Value {
    let matching_rules = self.get_matching_rules_prioritized();
    if matching_rules.is_empty() {
      return self.evaluate_default_output_value();
    }
    self.get_results(&matching_rules)
  }

  fn evaluate_hit_policy_collect_list(&self) -> Value {
    let matching_rules = self.get_matching_rules();
    if matching_rules.is_empty() {
      return self.evaluate_default_output_value();
    }
    self.get_results(&matching_rules)
  }

  fn evaluate_hit_policy_collect_count(&self) -> Value {
    let matching_rules = self.get_matching_rules();
    if matching_rules.is_empty() {
      return self.evaluate_default_output_value();
    }
    Value::Number(matching_rules.len().into())
  }

  fn evaluate_hit_policy_collect_sum(&self) -> Value {
    if self.component_names.len() > 1 {
      return value_null!("err_aggregators_not_allowed_for_compound_outputs");
    }
    let matching_rules = self.get_matching_rules();
    if matching_rules.is_empty() {
      return self.evaluate_default_output_value();
    }
    let output_values = matching_rules
      .iter()
      .map(|evaluated_rule| evaluated_rule.output_entry_values[0].clone())
      .collect::<Vec<Value>>();
    dsntk_feel_evaluator::evaluate_sum(output_values)
  }

  fn evaluate_hit_policy_collect_min(&self) -> Value {
    if self.component_names.len() > 1 {
      return value_null!("err_aggregators_not_allowed_for_compound_outputs");
    }
    let matching_rules = self.get_matching_rules();
    if matching_rules.is_empty() {
      return self.evaluate_default_output_value();
    }
    let output_values = matching_rules
      .iter()
      .map(|evaluated_rule| evaluated_rule.output_entry_values[0].clone())
      .collect::<Vec<Value>>();
    dsntk_feel_evaluator::evaluate_min(output_values)
  }

  fn evaluate_hit_policy_collect_max(&self) -> Value {
    if self.component_names.len() > 1 {
      return value_null!("err_aggregators_not_allowed_for_compound_outputs");
    }
    let matching_rules = self.get_matching_rules();
    if matching_rules.is_empty() {
      return self.evaluate_default_output_value();
    }
    let output_values = matching_rules
      .iter()
      .map(|evaluated_rule| evaluated_rule.output_entry_values[0].clone())
      .collect::<Vec<Value>>();
    dsntk_feel_evaluator::evaluate_max(output_values)
  }
}

/// Builds the evaluator of one unary-tests node `t` against a tested expression `e`.
///
/// When no test in `t` uses the name `?`, this builds the plain `e in (t)` node the
/// engine always built. So the ordinary entry costs exactly what it did. When a test
/// uses `?`, DMN 1.3 Table 55 applies: `e in t` means `{?: e, r: t}.r`. So the test is
/// the boolean value of its own expression, evaluated with `?` bound to the value of
/// `e`. The tests of the same entry that do not use `?` keep the plain `in` semantics,
/// and the entry is satisfied when any of its tests is.
///
/// Used for decision-table input entries and for the allowed-values test of an item
/// definition. These are the two places the model hands unary tests to the engine.
pub(crate) fn build_unary_tests_evaluator(tested_expression: &AstNode, unary_tests: AstNode) -> Evaluator {
  if !dsntk_feel_evaluator::references_name(&unary_tests, &NAME_QUESTION_MARK) {
    let node = AstNode::In(Box::new(tested_expression.clone()), Box::new(unary_tests));
    return dsntk_feel_evaluator::prepare(&node);
  }
  // Split the tests that use `?` from the tests that do not.
  let (negated, items) = match unary_tests {
    AstNode::ExpressionList(items) => (false, items),
    AstNode::NegatedList(items) => (true, items),
    other => (false, vec![other]),
  };
  let (question_items, plain_items): (Vec<AstNode>, Vec<AstNode>) = items
    .into_iter()
    .partition(|item| dsntk_feel_evaluator::references_name(item, &NAME_QUESTION_MARK));
  let question_evaluators: Vec<Evaluator> = question_items.iter().map(dsntk_feel_evaluator::prepare).collect();
  let plain_evaluator: Option<Evaluator> = if plain_items.is_empty() {
    None
  } else {
    // The plain tests keep their old shape, including the negation of a `not(...)` list.
    let list = if negated { AstNode::NegatedList(plain_items) } else { AstNode::ExpressionList(plain_items) };
    Some(dsntk_feel_evaluator::prepare(&AstNode::In(Box::new(tested_expression.clone()), Box::new(list))))
  };
  let tested_evaluator = dsntk_feel_evaluator::prepare(tested_expression);
  Box::new(move |scope: &FeelScope| {
    let plain = plain_evaluator.as_ref().map(|evaluator| evaluator(scope).is_true());
    let value = tested_evaluator(scope);
    let mut ctx = FeelContext::default();
    ctx.set_entry(&NAME_QUESTION_MARK, value);
    scope.push(ctx);
    let question = question_evaluators.iter().any(|evaluator| evaluator(scope).is_true());
    scope.pop();
    let satisfied = if negated {
      // `not(t1, ..., tn)` is satisfied when no test is. The plain part carries its own
      // negation already, because it was rebuilt as a negated list above.
      plain.unwrap_or(true) && !question
    } else {
      plain.unwrap_or(false) || question
    };
    Value::Boolean(satisfied)
  })
}

fn parse_decision_table(scope: &FeelScope, decision_table: &DecisionTable) -> Result<ParsedDecisionTable> {
  // parse input expressions and input values
  let mut input_expressions_and_values = vec![];
  for input_clause in decision_table.input_clauses() {
    let input_expression = dsntk_feel_parser::parse_expression(scope, &input_clause.input_expression, false)?;
    let input_values = if let Some(input) = &input_clause.allowed_input_values {
      let node = dsntk_feel_parser::parse_unary_tests(scope, input, false)?;
      Some(node)
    } else {
      None
    };
    input_expressions_and_values.push((input_expression, input_values))
  }
  // parse output values and output component names
  let mut component_names = vec![];
  let mut output_values_nodes = vec![];
  let mut default_output_values_nodes = vec![];
  for output_clause in decision_table.output_clauses() {
    if let Some(text) = &output_clause.allowed_output_values {
      let node = dsntk_feel_parser::parse_unary_tests(scope, text, false)?;
      output_values_nodes.push(Some(node));
    } else {
      output_values_nodes.push(None);
    }
    if let Some(text) = &output_clause.default_output_entry {
      let node = dsntk_feel_parser::parse_unary_tests(scope, text, false)?;
      default_output_values_nodes.push(Some(node));
    } else {
      default_output_values_nodes.push(None);
    }
    if let Some(name) = &output_clause.name {
      component_names.push(dsntk_feel_parser::parse_name(scope, name, false)?);
    }
  }
  // parse all rules
  let mut parsed_rules = vec![];
  for rule in decision_table.rules() {
    // parse input clause
    let mut input_entries_evaluators = vec![];
    for (i, (input_expression, input_values)) in input_expressions_and_values.iter().enumerate() {
      let input_entry_node = dsntk_feel_parser::parse_unary_tests(scope, &rule.input_entries[i].text, false)?;
      let uses_question_mark = dsntk_feel_evaluator::references_name(&input_entry_node, &NAME_QUESTION_MARK)
        || input_values.as_ref().is_some_and(|values| dsntk_feel_evaluator::references_name(values, &NAME_QUESTION_MARK));
      if uses_question_mark {
        // At least one test uses `?`, so the entry needs `?` bound to the value of the
        // input expression. See [build_unary_tests_evaluator].
        let entry_evaluator = build_unary_tests_evaluator(input_expression, input_entry_node);
        if let Some(input_values_node) = input_values {
          let values_evaluator = build_unary_tests_evaluator(input_expression, input_values_node.clone());
          input_entries_evaluators.push(Box::new(move |scope: &FeelScope| {
            Value::Boolean(values_evaluator(scope).is_true() && entry_evaluator(scope).is_true())
          }) as Evaluator);
        } else {
          input_entries_evaluators.push(entry_evaluator);
        }
      } else if let Some(input_values_node) = input_values {
        let left = AstNode::In(Box::new(input_expression.clone()), Box::new(input_values_node.clone()));
        let right = AstNode::In(Box::new(input_expression.clone()), Box::new(input_entry_node));
        let node = AstNode::And(Box::new(left), Box::new(right));
        input_entries_evaluators.push(dsntk_feel_evaluator::prepare(&node));
      } else {
        let node = AstNode::In(Box::new(input_expression.clone()), Box::new(input_entry_node));
        input_entries_evaluators.push(dsntk_feel_evaluator::prepare(&node));
      }
    }
    // parse output clause
    let mut output_entries_evaluators = vec![];
    for (i, output_values) in output_values_nodes.iter().enumerate() {
      let output_entry_node = dsntk_feel_parser::parse_expression(scope, &rule.output_entries[i].text, false)?;
      if let Some(output_value_node) = output_values {
        let node = AstNode::Out(Box::new(output_entry_node), Box::new(output_value_node.clone()));
        output_entries_evaluators.push(dsntk_feel_evaluator::prepare(&node));
      } else {
        output_entries_evaluators.push(dsntk_feel_evaluator::prepare(&output_entry_node));
      }
    }
    parsed_rules.push(ParsedRule {
      input_entries_evaluators,
      output_entries_evaluators,
    })
  }
  let mut output_values_evaluators = vec![];
  for opt_node in output_values_nodes {
    if let Some(node) = opt_node {
      output_values_evaluators.push(Some(dsntk_feel_evaluator::prepare(&node)));
    } else {
      output_values_evaluators.push(None);
    }
  }
  let mut default_output_values_evaluators = vec![];
  for opt_node in default_output_values_nodes {
    if let Some(node) = opt_node {
      default_output_values_evaluators.push(Some(dsntk_feel_evaluator::prepare(&node)));
    } else {
      default_output_values_evaluators.push(None);
    }
  }
  Ok(ParsedDecisionTable {
    component_names,
    output_values_evaluators,
    default_output_values_evaluators,
    rules: parsed_rules,
  })
}

fn evaluate_parsed_decision_table(scope: &FeelScope, parsed_decision_table: &ParsedDecisionTable) -> EvaluatedDecisionTable {
  // evaluate only non-empty output values
  let mut output_values = vec![];
  for evaluator in parsed_decision_table.output_values_evaluators.iter().flatten() {
    let value = evaluator(scope);
    if let Value::ExpressionList(values) = value {
      output_values.append(&mut values.to_owned());
    }
  }
  // evaluate only non-empty default output values
  let mut default_output_values = vec![];
  for evaluator in parsed_decision_table.default_output_values_evaluators.iter().flatten() {
    if let Value::ExpressionList(values) = evaluator(scope) {
      default_output_values.append(&mut values.to_owned());
    }
  }
  // evaluate all rules
  let mut evaluated_rules = vec![];
  for parsed_rule in &parsed_decision_table.rules {
    let mut input_entry_values = vec![];
    let mut matches = true;
    for evaluator in &parsed_rule.input_entries_evaluators {
      let input_value: Value = evaluator(scope);
      if !input_value.is_true() {
        matches = false;
      }
      input_entry_values.push(input_value);
    }
    let mut output_entry_values = vec![];
    for evaluator in &parsed_rule.output_entries_evaluators {
      output_entry_values.push(evaluator(scope));
    }
    evaluated_rules.push(EvaluatedRule { matches, output_entry_values })
  }
  EvaluatedDecisionTable {
    component_names: parsed_decision_table.component_names.clone(),
    output_values,
    default_output_values,
    evaluated_rules,
  }
}

pub fn build_decision_table_evaluator(scope: &FeelScope, decision_table: &DecisionTable) -> Result<Evaluator> {
  let hit_policy = decision_table.hit_policy();
  let parsed_decision_table = parse_decision_table(scope, decision_table)?;
  Ok(Box::new(move |scope: &FeelScope| {
    let evaluated_decision_table = evaluate_parsed_decision_table(scope, &parsed_decision_table);
    match hit_policy {
      HitPolicy::Unique => evaluated_decision_table.evaluate_hit_policy_unique(),
      HitPolicy::Any => evaluated_decision_table.evaluate_hit_policy_any(),
      HitPolicy::Priority => evaluated_decision_table.evaluate_hit_policy_priority(),
      HitPolicy::First => evaluated_decision_table.evaluate_hit_policy_first(),
      HitPolicy::RuleOrder => evaluated_decision_table.evaluate_hit_policy_rule_order(),
      HitPolicy::OutputOrder => evaluated_decision_table.evaluate_hit_policy_output_order(),
      HitPolicy::Collect(aggregator) => match aggregator {
        BuiltinAggregator::List => evaluated_decision_table.evaluate_hit_policy_collect_list(),
        BuiltinAggregator::Count => evaluated_decision_table.evaluate_hit_policy_collect_count(),
        BuiltinAggregator::Sum => evaluated_decision_table.evaluate_hit_policy_collect_sum(),
        BuiltinAggregator::Min => evaluated_decision_table.evaluate_hit_policy_collect_min(),
        BuiltinAggregator::Max => evaluated_decision_table.evaluate_hit_policy_collect_max(),
      },
    }
  }))
}

#[cfg(test)]
mod tests {
  use super::build_decision_table_evaluator;
  use crate::tests::context;
  use dsntk_examples::decision_tables::H_000210;
  use dsntk_feel::values::Value;
  use dsntk_feel::{FeelNumber, value_number};
  use dsntk_model::DecisionTable;

  #[test]
  fn test() {
    let decision_table: DecisionTable = dsntk_recognizer::from_unicode(H_000210, false).unwrap().into();
    let scope = context(r#"{Customer:"Business", Order:-3.23 }"#).into();
    let evaluator = build_decision_table_evaluator(&scope, &decision_table).unwrap();
    assert_eq!(value_number!(10, 2), evaluator(&scope));
  }

  /// A decision table whose first input entry is a boolean expression over `?`.
  ///
  /// DMN 1.3 Table 55: when the test is a boolean expression that uses `?`, the entry
  /// `e in t` means `{?: e, r: t}.r`. So the boolean itself is the answer. Without that
  /// binding, the expression would evaluate to null, and the rule would silently never
  /// match.
  const DT_QUESTION_MARK: &str = r#"
┌───┬──────────────────────╥───────┐
│ F │       Payment        ║       │
╞═══╪══════════════════════╬═══════╡
│ 1 │ contains(?, "VENMO") ║ "P2P" │
├───┼──────────────────────╫───────┤
│ 2 │          -           ║ "OK"  │
└───┴──────────────────────╨───────┘
"#;

  #[test]
  fn question_mark_in_input_entry_matches() {
    let decision_table: DecisionTable = dsntk_recognizer::from_unicode(DT_QUESTION_MARK, false).unwrap().into();
    let scope = context(r#"{Payment: "VENMO-123"}"#).into();
    let evaluator = build_decision_table_evaluator(&scope, &decision_table).unwrap();
    assert_eq!(Value::String("P2P".to_string()), evaluator(&scope));
  }

  #[test]
  fn question_mark_in_input_entry_does_not_match() {
    let decision_table: DecisionTable = dsntk_recognizer::from_unicode(DT_QUESTION_MARK, false).unwrap().into();
    let scope = context(r#"{Payment: "ZELLE-99"}"#).into();
    let evaluator = build_decision_table_evaluator(&scope, &decision_table).unwrap();
    assert_eq!(Value::String("OK".to_string()), evaluator(&scope));
  }

  /// The equality form `? = "X"`. It parses as one expression, not as a unary
  /// comparison, so it takes the same bound-`?` path as a function call over `?`.
  const DT_QUESTION_MARK_EQUALITY: &str = r#"
┌───┬──────────╥───────┐
│ F │  Letter  ║       │
╞═══╪══════════╬═══════╡
│ 1 │ ? = "X"  ║ "hit" │
├───┼──────────╫───────┤
│ 2 │    -     ║ "def" │
└───┴──────────╨───────┘
"#;

  #[test]
  fn question_mark_equality_in_input_entry() {
    let decision_table: DecisionTable = dsntk_recognizer::from_unicode(DT_QUESTION_MARK_EQUALITY, false).unwrap().into();
    let evaluator = build_decision_table_evaluator(&context("{Letter: \"X\"}").into(), &decision_table).unwrap();
    assert_eq!(Value::String("hit".to_string()), evaluator(&context("{Letter: \"X\"}").into()));
    assert_eq!(Value::String("def".to_string()), evaluator(&context("{Letter: \"Y\"}").into()));
  }

  /// A `?` test mixed with plain tests in one entry. The entry is satisfied when any
  /// of its tests is. So the plain tests must keep their old semantics next to the
  /// bound-`?` test.
  const DT_QUESTION_MARK_MIXED: &str = r#"
┌───┬─────────────────────────────╥───────┐
│ F │           Payment           ║       │
╞═══╪═════════════════════════════╬═══════╡
│ 1 │ "WIRE", contains(?, "P2P")  ║ "yes" │
├───┼─────────────────────────────╫───────┤
│ 2 │              -              ║ "no"  │
└───┴─────────────────────────────╨───────┘
"#;

  #[test]
  fn question_mark_mixed_with_plain_tests() {
    let decision_table: DecisionTable = dsntk_recognizer::from_unicode(DT_QUESTION_MARK_MIXED, false).unwrap().into();
    let evaluator = build_decision_table_evaluator(&context("{Payment: \"WIRE\"}").into(), &decision_table).unwrap();
    assert_eq!(Value::String("yes".to_string()), evaluator(&context("{Payment: \"WIRE\"}").into()));
    assert_eq!(Value::String("yes".to_string()), evaluator(&context("{Payment: \"XP2P\"}").into()));
    assert_eq!(Value::String("no".to_string()), evaluator(&context("{Payment: \"ACH\"}").into()));
  }
}
