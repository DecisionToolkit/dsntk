use crate::graph::{build_drg, DrgEdgeKind};
use std::path::PathBuf;

fn fixture_path(name: &str) -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/tests/fixtures").join(name)
}

/// A valid 3-node project produces a graph with 3 nodes and 2 edges.
#[test]
fn _0001() {
  let drg = build_drg(&fixture_path("graph_simple")).unwrap();
  assert_eq!(drg.node_count(), 3);
  assert_eq!(drg.edge_count(), 2);
}

/// All edge kinds in the simple fixture are InformationRequirement.
#[test]
fn _0002() {
  let drg = build_drg(&fixture_path("graph_simple")).unwrap();
  for edge in drg.edge_kinds() {
    assert_eq!(edge, DrgEdgeKind::InformationRequirement);
  }
}

/// Topological order puts inputs before decisions.
#[test]
fn _0003() {
  let drg = build_drg(&fixture_path("graph_simple")).unwrap();
  let order = drg.topological_order().unwrap();
  let eligibility_pos = order.iter().position(|id| id == "eligibility").unwrap();
  let applicant_pos = order.iter().position(|id| id == "applicant").unwrap();
  let loan_pos = order.iter().position(|id| id == "loan_amount").unwrap();
  assert!(applicant_pos < eligibility_pos);
  assert!(loan_pos < eligibility_pos);
}

/// A project with a cycle returns an error containing "cycle".
#[test]
fn _0004() {
  let result = build_drg(&fixture_path("graph_cycle"));
  assert!(result.is_err());
  let msg = result.unwrap_err().to_string();
  assert!(msg.contains("cycle"), "expected 'cycle' in error: {msg}");
}

/// A project with an unresolved link returns an error containing the target id.
#[test]
fn _0005() {
  let result = build_drg(&fixture_path("graph_missing_link"));
  assert!(result.is_err());
  let msg = result.unwrap_err().to_string();
  assert!(msg.contains("does_not_exist"), "expected 'does_not_exist' in error: {msg}");
}

/// An empty directory returns an error.
#[test]
fn _0006() {
  let dir = std::env::temp_dir().join("dsntk_test_empty_drg");
  let _ = std::fs::create_dir_all(&dir);
  let result = build_drg(&dir);
  assert!(result.is_err());
  let _ = std::fs::remove_dir_all(&dir);
}

/// governed-by pointing to a knowledge-source is valid.
#[test]
fn _0007() {
  let drg = build_drg(&fixture_path("graph_governed_by")).unwrap();
  assert_eq!(drg.node_count(), 3);
  let ks_edges: Vec<_> = drg.edge_kinds().into_iter().filter(|k| *k == DrgEdgeKind::AuthorityRequirement).collect();
  assert_eq!(ks_edges.len(), 1);
}

/// governed-by pointing to a decision (not knowledge-source) fails.
#[test]
fn _0008() {
  let result = build_drg(&fixture_path("graph_bad_governed_by"));
  assert!(result.is_err());
  let msg = result.unwrap_err().to_string();
  assert!(msg.contains("knowledge-source"), "expected 'knowledge-source' in error: {msg}");
}

/// supported-by pointing to a knowledge-source (not bkm) fails.
#[test]
fn _0009() {
  let result = build_drg(&fixture_path("graph_bad_supported_by"));
  assert!(result.is_err());
  let msg = result.unwrap_err().to_string();
  assert!(msg.contains("bkm"), "expected 'bkm' in error: {msg}");
}

/// The loan_pricing fixture builds a valid DRG with multiple node types and edge kinds.
#[test]
fn _0010() {
  let drg = build_drg(&fixture_path("loan_pricing")).unwrap();
  assert!(drg.node_count() >= 3, "expected at least 3 nodes, got {}", drg.node_count());
  let order = drg.topological_order().unwrap();
  assert_eq!(order.len(), drg.node_count());
}
