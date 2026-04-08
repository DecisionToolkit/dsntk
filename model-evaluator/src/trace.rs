//! Trace data structures for capturing DMN evaluation steps.

use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;

/// Top-level evaluation trace.
#[derive(Debug, Clone, Serialize)]
pub struct EvaluationTrace {
  pub graph: TraceGraph,
  pub evaluation_order: Vec<String>,
  pub steps: Vec<TraceStep>,
}

/// Graph structure of the DRG.
#[derive(Debug, Clone, Serialize)]
pub struct TraceGraph {
  pub nodes: Vec<TraceNode>,
  pub edges: Vec<TraceEdge>,
}

/// A node in the DRG trace.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum TraceNode {
  #[serde(rename = "input_data")]
  InputData { id: String, name: String, value: Option<serde_json::Value> },
  #[serde(rename = "decision_table")]
  DecisionTable {
    id: String,
    name: String,
    hit_policy: String,
    input_columns: Vec<String>,
    output_columns: Vec<String>,
    rules: Vec<TraceRule>,
  },
}

/// A rule in a decision table trace.
#[derive(Debug, Clone, Serialize)]
pub struct TraceRule {
  pub index: usize,
  pub input_entries: Vec<String>,
  pub output_entries: Vec<String>,
}

/// An edge in the DRG trace.
#[derive(Debug, Clone, Serialize)]
pub struct TraceEdge {
  pub source: String,
  pub target: String,
  pub label: String,
}

/// A single evaluation step.
#[derive(Debug, Clone, Serialize)]
pub struct TraceStep {
  pub node_id: String,
  pub input_values: HashMap<String, serde_json::Value>,
  pub matched_rules: Vec<usize>,
  pub output_value: serde_json::Value,
  pub cell_evaluations: Vec<CellEvaluation>,
}

/// Evaluation result of a single cell in a decision table.
#[derive(Debug, Clone, Serialize)]
pub struct CellEvaluation {
  pub rule_index: usize,
  pub column_index: usize,
  pub expression: String,
  pub input_value: serde_json::Value,
  pub result: bool,
}

/// Collects trace steps during evaluation.
#[derive(Debug, Clone)]
pub struct TraceCollector {
  pub steps: Vec<TraceStep>,
}

impl TraceCollector {
  /// Creates a new empty trace collector.
  pub fn new() -> Self {
    Self { steps: Vec::new() }
  }

  /// Pushes a step into the collector.
  pub fn push_step(&mut self, step: TraceStep) {
    self.steps.push(step);
  }
}

impl Default for TraceCollector {
  fn default() -> Self {
    Self::new()
  }
}

thread_local! {
  static TRACE_COLLECTOR: RefCell<Option<TraceCollector>> = const { RefCell::new(None) };
}

/// Starts trace collection on the current thread.
pub fn trace_start() {
  TRACE_COLLECTOR.with(|tc| {
    *tc.borrow_mut() = Some(TraceCollector::new());
  });
}

/// Finishes trace collection and returns the collector if active.
pub fn trace_finish() -> Option<TraceCollector> {
  TRACE_COLLECTOR.with(|tc| tc.borrow_mut().take())
}

/// Pushes a step into the active trace collector.
pub fn trace_push_step(step: TraceStep) {
  TRACE_COLLECTOR.with(|tc| {
    if let Some(ref mut collector) = *tc.borrow_mut() {
      collector.push_step(step);
    }
  });
}

/// Returns whether trace collection is active on the current thread.
pub fn trace_is_active() -> bool {
  TRACE_COLLECTOR.with(|tc| tc.borrow().is_some())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_trace_graph_structure() {
    let graph = TraceGraph {
      nodes: vec![
        TraceNode::InputData {
          id: "input1".to_string(),
          name: "Age".to_string(),
          value: Some(serde_json::json!(25)),
        },
        TraceNode::DecisionTable {
          id: "dt1".to_string(),
          name: "Eligibility".to_string(),
          hit_policy: "UNIQUE".to_string(),
          input_columns: vec!["Age".to_string()],
          output_columns: vec!["Eligible".to_string()],
          rules: vec![TraceRule {
            index: 0,
            input_entries: vec![">= 18".to_string()],
            output_entries: vec!["true".to_string()],
          }],
        },
      ],
      edges: vec![TraceEdge {
        source: "input1".to_string(),
        target: "dt1".to_string(),
        label: "requires".to_string(),
      }],
    };
    let json = serde_json::to_string(&graph).unwrap();
    assert!(json.contains("\"type\":\"input_data\""));
    assert!(json.contains("\"type\":\"decision_table\""));
    assert!(json.contains("\"hit_policy\":\"UNIQUE\""));
    assert!(json.contains("\"source\":\"input1\""));
    assert!(json.contains("\"target\":\"dt1\""));
  }

  #[test]
  fn test_trace_collector_thread_local() {
    assert!(!trace_is_active());
    trace_start();
    assert!(trace_is_active());
    trace_push_step(TraceStep {
      node_id: "node1".to_string(),
      input_values: HashMap::new(),
      matched_rules: vec![0],
      output_value: serde_json::json!("ok"),
      cell_evaluations: vec![],
    });
    trace_push_step(TraceStep {
      node_id: "node2".to_string(),
      input_values: HashMap::new(),
      matched_rules: vec![1, 2],
      output_value: serde_json::json!(42),
      cell_evaluations: vec![CellEvaluation {
        rule_index: 1,
        column_index: 0,
        expression: "> 10".to_string(),
        input_value: serde_json::json!(42),
        result: true,
      }],
    });
    let collector = trace_finish().unwrap();
    assert_eq!(collector.steps.len(), 2);
    assert_eq!(collector.steps[0].node_id, "node1");
    assert_eq!(collector.steps[1].matched_rules, vec![1, 2]);
    assert!(!trace_is_active());
  }
}
