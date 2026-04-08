//! Builder for decision table evaluators.

use crate::trace::{trace_is_active, CellEvaluation};
use dsntk_common::Result;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::{value_null, Evaluator, FeelScope, Name};
use dsntk_feel_parser::AstNode;
use dsntk_model::{BuiltinAggregator, DecisionTable, HitPolicy};
use std::cmp::Ordering;

/// Checks if an AST node contains comparison expressions that should be evaluated directly
/// rather than wrapped in `AstNode::In` expressions.
fn contains_comparison_expressions(node: &AstNode) -> bool {
  match node {
    AstNode::ExpressionList(items) => {
      // If any item in the expression list is a comparison operation,
      // we should evaluate the expressions directly
      items.iter().any(contains_comparison_operators)
    }
    _ => contains_comparison_operators(node),
  }
}

/// Recursively checks if an AST node contains comparison operators.
fn contains_comparison_operators(node: &AstNode) -> bool {
  match node {
    AstNode::Lt(_, _) | AstNode::Le(_, _) | AstNode::Gt(_, _) | AstNode::Ge(_, _) | AstNode::Eq(_, _) | AstNode::Nq(_, _) => true,
    AstNode::FunctionInvocation(func, args) => contains_comparison_operators(func) || contains_comparison_operators(args),
    AstNode::Path(lhs, rhs) => contains_comparison_operators(lhs) || contains_comparison_operators(rhs),
    AstNode::Neg(expr) => contains_comparison_operators(expr),
    AstNode::Add(left, right) => contains_comparison_operators(left) || contains_comparison_operators(right),
    AstNode::Sub(left, right) => contains_comparison_operators(left) || contains_comparison_operators(right),
    AstNode::Mul(left, right) => contains_comparison_operators(left) || contains_comparison_operators(right),
    AstNode::Div(left, right) => contains_comparison_operators(left) || contains_comparison_operators(right),
    AstNode::Exp(left, right) => contains_comparison_operators(left) || contains_comparison_operators(right),
    AstNode::And(left, right) => contains_comparison_operators(left) || contains_comparison_operators(right),
    AstNode::Or(left, right) => contains_comparison_operators(left) || contains_comparison_operators(right),
    AstNode::Between(expr, start, end) => contains_comparison_operators(expr) || contains_comparison_operators(start) || contains_comparison_operators(end),
    AstNode::In(left, right) => contains_comparison_operators(left) || contains_comparison_operators(right),
    AstNode::InstanceOf(expr, _) => contains_comparison_operators(expr),
    AstNode::List(items) => items.iter().any(contains_comparison_operators),
    AstNode::Context(items) => items.iter().any(contains_comparison_operators),
    AstNode::ContextEntry(_, value) => contains_comparison_operators(value),
    AstNode::Filter(expr, filter) => contains_comparison_operators(expr) || contains_comparison_operators(filter),
    AstNode::Every(_, expr) => contains_comparison_operators(expr),
    AstNode::Some(_, expr) => contains_comparison_operators(expr),
    AstNode::For(_, expr) => contains_comparison_operators(expr),
    AstNode::If(condition, then_expr, else_expr) => {
      contains_comparison_operators(condition) || contains_comparison_operators(then_expr) || contains_comparison_operators(else_expr)
    }
    AstNode::PositionalParameters(items) => items.iter().any(contains_comparison_operators),
    AstNode::NamedParameters(items) => items.iter().any(contains_comparison_operators),
    AstNode::NamedParameter(_, expr) => contains_comparison_operators(expr),
    AstNode::FormalParameters(items) => items.iter().any(contains_comparison_operators),
    AstNode::FormalParameter(_, expr) => contains_comparison_operators(expr),
    AstNode::ExpressionList(items) => items.iter().any(contains_comparison_operators),
    _ => false,
  }
}

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
  cell_evaluations: Vec<CellEvaluation>,
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

      // Check if the input entry node contains comparison expressions (like "Date < today()")
      // If so, we should evaluate the comparison directly instead of wrapping in AstNode::In
      let node = if contains_comparison_expressions(&input_entry_node) {
        // For comparison expressions, evaluate them directly
        input_entry_node
      } else if let Some(input_values_node) = input_values {
        // Traditional unary test with allowed input values
        let left = AstNode::In(Box::new(input_expression.clone()), Box::new(input_values_node.clone()));
        let right = AstNode::In(Box::new(input_expression.clone()), Box::new(input_entry_node));
        AstNode::And(Box::new(left), Box::new(right))
      } else {
        // Traditional unary test without allowed input values
        AstNode::In(Box::new(input_expression.clone()), Box::new(input_entry_node))
      };

      input_entries_evaluators.push(dsntk_feel_evaluator::prepare(&node));
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
  let tracing = trace_is_active();
  let mut cell_evaluations = vec![];
  let mut evaluated_rules = vec![];
  for (rule_index, parsed_rule) in parsed_decision_table.rules.iter().enumerate() {
    let mut input_entry_values = vec![];
    let mut matches = true;
    for (col_index, evaluator) in parsed_rule.input_entries_evaluators.iter().enumerate() {
      let input_value: Value = evaluator(scope);

      // Handle expression lists containing comparison results
      let is_true = match &input_value {
        Value::ExpressionList(values) => {
          // For expression lists, all expressions must evaluate to true
          values.iter().all(|value| value.is_true())
        }
        _ => input_value.is_true(),
      };

      if tracing {
        cell_evaluations.push(CellEvaluation {
          rule_index,
          column_index: col_index,
          expression: String::new(),
          input_value: serde_json::json!(format!("{}", input_value)),
          result: is_true,
        });
      }

      if !is_true {
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
    cell_evaluations,
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
  use dsntk_feel::{value_number, FeelNumber};
  use dsntk_model::DecisionTable;

  #[test]
  fn test() {
    let decision_table: DecisionTable = dsntk_recognizer::from_unicode(H_000210, false).unwrap().into();
    let scope = context(r#"{Customer:"Business", Order:-3.23 }"#).into();
    let evaluator = build_decision_table_evaluator(&scope, &decision_table).unwrap();
    assert_eq!(value_number!(10, 2), evaluator(&scope));
  }
}
