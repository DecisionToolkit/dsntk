//! # Decision Requirements Graph (DRG) construction and validation
//!
//! Scans a markdown DMN project directory, parses front matter from all `.md` files,
//! builds a petgraph DiGraph, validates structural constraints, and produces
//! a topological evaluation order.

use crate::errors::*;
use crate::front_matter::{parse_front_matter, DmnNode};
use dsntk_common::Result;
use petgraph::algo::toposort;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

/// The kind of edge in the DRG.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrgEdgeKind {
  /// `requires` — information requirement (data flow).
  InformationRequirement,
  /// `governed-by` — authority requirement (→ knowledge-source).
  AuthorityRequirement,
  /// `supported-by` — knowledge requirement (→ BKM).
  KnowledgeRequirement,
}

/// A parsed DMN node with its source file path.
#[derive(Debug, Clone)]
pub struct DrgNode {
  pub dmn: DmnNode,
  pub file_path: String,
}

/// The Decision Requirements Graph for a markdown DMN project.
#[derive(Debug)]
pub struct Drg {
  graph: DiGraph<String, DrgEdgeKind>,
  node_index: HashMap<String, NodeIndex>,
  nodes: HashMap<String, DrgNode>,
}

impl Drg {
  pub fn node_count(&self) -> usize {
    self.graph.node_count()
  }

  pub fn edge_count(&self) -> usize {
    self.graph.edge_count()
  }

  pub fn edge_kinds(&self) -> Vec<DrgEdgeKind> {
    self.graph.edge_weights().copied().collect()
  }

  /// Returns node IDs in topological order (inputs first, terminal decisions last).
  pub fn topological_order(&self) -> Result<Vec<String>> {
    let sorted = toposort(&self.graph, None).map_err(|cycle| {
      let node_id = &self.graph[cycle.node_id()];
      err_drg_cycle(node_id)
    })?;
    let mut result: Vec<String> = sorted.into_iter().map(|idx| self.graph[idx].clone()).collect();
    result.reverse();
    Ok(result)
  }

  pub fn get_node(&self, id: &str) -> Option<&DrgNode> {
    self.nodes.get(id)
  }

  pub fn node_ids(&self) -> Vec<String> {
    self.node_index.keys().cloned().collect()
  }
}

/// Scans a project directory for `.md` files, parses their front matter,
/// and builds a validated DRG.
pub fn build_drg(project_dir: &Path) -> Result<Drg> {
  // Phase 1: scan and parse all .md files
  let drg_nodes = scan_md_files(project_dir)?;
  if drg_nodes.is_empty() {
    return Err(err_drg_no_md_files(&project_dir.to_string_lossy()));
  }

  // Phase 2: build the graph — add all nodes
  let mut graph: DiGraph<String, DrgEdgeKind> = DiGraph::new();
  let mut node_index: HashMap<String, NodeIndex> = HashMap::new();
  let mut nodes: HashMap<String, DrgNode> = HashMap::new();

  for drg_node in &drg_nodes {
    let id = &drg_node.dmn.id;
    if let Some(existing) = nodes.get(id) {
      return Err(err_drg_duplicate_node_id(id, &existing.file_path, &drg_node.file_path));
    }
    let idx = graph.add_node(id.clone());
    node_index.insert(id.clone(), idx);
    nodes.insert(id.clone(), drg_node.clone());
  }

  // Phase 3: add edges — validate all targets exist
  for drg_node in &drg_nodes {
    let source_idx = node_index[&drg_node.dmn.id];

    if let Some(ref requires) = drg_node.dmn.requires {
      for target_id in requires {
        let target_idx = node_index.get(target_id).ok_or_else(|| err_drg_unresolved_link(&drg_node.file_path, target_id))?;
        graph.add_edge(source_idx, *target_idx, DrgEdgeKind::InformationRequirement);
      }
    }

    if let Some(ref governed_by) = drg_node.dmn.governed_by {
      for target_id in governed_by {
        let target_idx = node_index.get(target_id).ok_or_else(|| err_drg_unresolved_link(&drg_node.file_path, target_id))?;
        graph.add_edge(source_idx, *target_idx, DrgEdgeKind::AuthorityRequirement);
      }
    }

    if let Some(ref supported_by) = drg_node.dmn.supported_by {
      for target_id in supported_by {
        let target_idx = node_index.get(target_id).ok_or_else(|| err_drg_unresolved_link(&drg_node.file_path, target_id))?;
        graph.add_edge(source_idx, *target_idx, DrgEdgeKind::KnowledgeRequirement);
      }
    }
  }

  // Phase 4: validate edge-type constraints
  for drg_node in &drg_nodes {
    let source_id = &drg_node.dmn.id;

    if let Some(ref governed_by) = drg_node.dmn.governed_by {
      for target_id in governed_by {
        if let Some(target_node) = nodes.get(target_id) {
          if target_node.dmn.node_type != "knowledge-source" {
            return Err(err_drg_invalid_edge_type(
              source_id,
              "governed-by",
              target_id,
              "knowledge-source",
              &target_node.dmn.node_type,
            ));
          }
        }
      }
    }

    if let Some(ref supported_by) = drg_node.dmn.supported_by {
      for target_id in supported_by {
        if let Some(target_node) = nodes.get(target_id) {
          if target_node.dmn.node_type != "bkm" {
            return Err(err_drg_invalid_edge_type(source_id, "supported-by", target_id, "bkm", &target_node.dmn.node_type));
          }
        }
      }
    }
  }

  // Phase 5: validate acyclicity (toposort returns Err if a cycle exists)
  if let Err(cycle) = toposort(&graph, None) {
    let node_id = &graph[cycle.node_id()];
    return Err(err_drg_cycle(node_id));
  }

  Ok(Drg { graph, node_index, nodes })
}

/// Recursively scans a directory for `.md` files and parses their front matter.
fn scan_md_files(dir: &Path) -> Result<Vec<DrgNode>> {
  let mut nodes = Vec::new();
  for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
    let path = entry.path();
    if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("md") {
      let content = std::fs::read_to_string(path).map_err(|e| err_drg_file_read(&path.to_string_lossy(), &e.to_string()))?;
      // Skip .md files without front matter (e.g., README.md)
      if !content.trim_start().starts_with("---") {
        continue;
      }
      match parse_front_matter(&content) {
        Ok(fm) => {
          nodes.push(DrgNode {
            dmn: fm.dmn,
            file_path: path.to_string_lossy().to_string(),
          });
        }
        Err(_) => {
          // Skip markdown files that have --- but don't have valid DMN front matter
          continue;
        }
      }
    }
  }
  Ok(nodes)
}
