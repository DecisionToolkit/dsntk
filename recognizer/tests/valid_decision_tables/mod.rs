mod horizontal;
mod vertical;

use dsntk_recognizer::{
  AnnotationClause, AnnotationEntry, BuiltinAggregator, DecisionRule, DecisionTable, DecisionTableOrientation, HitPolicy, InputClause, InputEntry, OutputClause, OutputEntry,
  from_markdown, from_unicode,
};

use dsntk_examples::decision_tables::*;

#[allow(clippy::type_complexity)]
fn t_eq(
  expected: &(
    Option<&'static str>,
    Option<&'static str>,
    HitPolicy,
    Option<BuiltinAggregator>,
    DecisionTableOrientation,
    Vec<InputClause>,
    Vec<OutputClause>,
    Vec<AnnotationClause>,
    Vec<DecisionRule>,
  ),
  actual: DecisionTable,
) {
  assert_eq!(actual.information_item_name, expected.0.map(|s| s.to_string()), "information item names differ");
  assert_eq!(actual.output_label, expected.1.map(|s| s.to_string()), "output labels differ");
  assert_eq!(actual.hit_policy, expected.2, "hit policies differ");
  assert_eq!(actual.aggregation, expected.3, "aggregations differ");
  assert_eq!(actual.preferred_orientation, expected.4, "preferred orientations differ");
  assert_eq!(actual.input_clauses.len(), expected.5.len(), "number of input clauses differ");
  for (i, actual_input_clause) in actual.input_clauses.iter().enumerate() {
    assert_eq!(*actual_input_clause, expected.5[i], "input clauses differ");
  }
  assert_eq!(actual.output_clauses.len(), expected.6.len(), "number of output clauses differ");
  for (i, actual_output_clause) in actual.output_clauses.iter().enumerate() {
    assert_eq!(*actual_output_clause, expected.6[i], "output clauses differ");
  }
  assert_eq!(actual.annotation_clauses.len(), expected.7.len(), "number of annotation clauses differ");
  for (i, actual_annotation_clause) in actual.annotation_clauses.iter().enumerate() {
    assert_eq!(*actual_annotation_clause, expected.7[i], "annotation clauses differ");
  }
  assert_eq!(actual.rules.len(), expected.8.len(), "number of rules differ");
  for (i, actual_rule) in actual.rules.iter().enumerate() {
    assert_eq!(*actual_rule, expected.8[i], "rules differ");
  }
}

fn t_input_clauses(list: &[(&str, Option<&'static str>)]) -> Vec<InputClause> {
  let mut input_clauses = vec![];
  for input_clause in list {
    input_clauses.push(InputClause {
      input_expression: input_clause.0.to_string(),
      allowed_input_values: input_clause.1.map(|s| s.to_string()),
    });
  }

  input_clauses
}

fn t_output_clauses(list: &[(Option<&'static str>, Option<&'static str>, Option<&'static str>)]) -> Vec<OutputClause> {
  let mut output_clauses = vec![];
  for output_clause in list {
    output_clauses.push(OutputClause {
      output_component_name: output_clause.0.map(|s| s.to_string()),
      allowed_output_values: output_clause.1.map(|s| s.to_string()),
      default_output_value: output_clause.2.map(|s| s.to_string()),
    });
  }
  output_clauses
}

fn t_annotation_clauses(list: &[&'static str]) -> Vec<AnnotationClause> {
  let mut annotation_clauses = vec![];
  for annotation_clause in list {
    annotation_clauses.push(AnnotationClause {
      name: annotation_clause.to_string(),
    });
  }
  annotation_clauses
}

fn t_rules<'a>(list: &[(&[&'a str], &[&'a str], &[&'a str])]) -> Vec<DecisionRule> {
  let mut decision_rules = vec![];
  for rule in list {
    decision_rules.push(DecisionRule {
      input_entries: rule.0.iter().map(|s| InputEntry { text: s.to_string() }).collect::<Vec<InputEntry>>(),
      output_entries: rule.1.iter().map(|s| OutputEntry { text: s.to_string() }).collect::<Vec<OutputEntry>>(),
      annotation_entries: rule.2.iter().map(|s| AnnotationEntry { text: s.to_string() }).collect::<Vec<AnnotationEntry>>(),
    });
  }
  decision_rules
}
