//! Handlers for loading markdown-based DMN projects.
//!
//! Scans a project directory of `.md` files with YAML front matter,
//! builds the DRG, resolves input types via TypeRegistry, and parses
//! decision tables from markdown bodies.

use actix_web::{get, post, web, HttpResponse};
use dsntk_common::Jsonify;
use dsntk_feel::values::Value;
use dsntk_feel::{FeelScope, FeelType, Name};
use dsntk_model_evaluator::evaluate_decision_table_with_trace;
use dsntk_type_registry::front_matter::extract_body;
use dsntk_type_registry::resolver::resolve_data_type;
use dsntk_type_registry::scanner::scan_folder;
use dsntk_type_registry::{build_drg, DrgEdgeKind, TypeRegistry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

const CONTENT_TYPE: &str = "application/json";

/// A field descriptor for generating input controls.
#[derive(Debug, Clone, Serialize)]
struct FieldDescriptor {
  /// Dotted path for nested fields (e.g. "Monthly.Income").
  path: String,
  /// Display name.
  name: String,
  /// FEEL type: "number", "string", "boolean", "date", "context".
  feel_type: String,
  /// Allowed values for enums/select controls.
  #[serde(skip_serializing_if = "Option::is_none")]
  allowed_values: Option<Vec<String>>,
  /// Whether the field is optional.
  optional: bool,
}

/// A graph node for the viewer.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
enum ProjectNode {
  #[serde(rename = "input_data")]
  InputData { id: String, name: String, fields: Vec<FieldDescriptor> },
  #[serde(rename = "decision")]
  Decision {
    id: String,
    name: String,
    hit_policy: String,
    input_columns: Vec<String>,
    output_columns: Vec<String>,
    rules: Vec<ProjectRule>,
    annotation_columns: Vec<String>,
  },
  #[serde(rename = "bkm")]
  Bkm {
    id: String,
    name: String,
    hit_policy: String,
    input_columns: Vec<String>,
    output_columns: Vec<String>,
    rules: Vec<ProjectRule>,
    parameters: Vec<ProjectParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    feel_expression: Option<String>,
  },
  #[serde(rename = "knowledge_source")]
  KnowledgeSource {
    id: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<String>,
  },
}

/// A rule row in a decision table.
#[derive(Debug, Clone, Serialize)]
struct ProjectRule {
  index: usize,
  input_entries: Vec<String>,
  output_entries: Vec<String>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  annotations: Vec<String>,
}

/// A BKM parameter.
#[derive(Debug, Clone, Serialize)]
struct ProjectParam {
  name: String,
  param_type: String,
}

/// A graph edge.
#[derive(Debug, Clone, Serialize)]
struct ProjectEdge {
  source: String,
  target: String,
  kind: String,
  label: String,
}

/// The full project response.
#[derive(Debug, Clone, Serialize)]
struct ProjectResponse {
  nodes: Vec<ProjectNode>,
  edges: Vec<ProjectEdge>,
  evaluation_order: Vec<String>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  scenarios: Vec<serde_json::Value>,
}

/// GET /api/v1/project?dir=<path>
///
/// Loads a markdown DMN project directory and returns the full graph
/// with resolved types and parsed decision tables.
#[get("/api/v1/project")]
async fn load_project(query: web::Query<std::collections::HashMap<String, String>>) -> HttpResponse {
  let Some(dir_path) = query.get("dir") else {
    return HttpResponse::BadRequest().content_type(CONTENT_TYPE).body(r#"{"error":"missing 'dir' query parameter"}"#);
  };

  let project_dir = Path::new(dir_path);
  if !project_dir.is_dir() {
    return HttpResponse::BadRequest()
      .content_type(CONTENT_TYPE)
      .body(format!(r#"{{"error":"directory not found: {}"}}"#, dir_path));
  }

  // Build the DRG from markdown files.
  let drg = match build_drg(project_dir) {
    Ok(drg) => drg,
    Err(e) => {
      return HttpResponse::InternalServerError()
        .content_type(CONTENT_TYPE)
        .body(format!(r#"{{"error":"failed to build DRG: {}"}}"#, e));
    }
  };

  // Scan for type definitions (TypeScript/JSON Schema files).
  let registry = scan_type_files(project_dir);

  // Get evaluation order.
  let evaluation_order = drg.topological_order().unwrap_or_default();

  // Build nodes and edges.
  let mut nodes = Vec::new();
  let mut edges = Vec::new();

  for node_id in drg.node_ids() {
    let Some(drg_node) = drg.get_node(&node_id) else {
      continue;
    };
    let dmn = &drg_node.dmn;
    let base_dir = Path::new(&drg_node.file_path).parent().unwrap_or(project_dir);

    match dmn.node_type.as_str() {
      "input-data" => {
        let fields = resolve_input_fields(dmn, base_dir, &registry);
        nodes.push(ProjectNode::InputData {
          id: dmn.id.clone(),
          name: dmn.name.clone(),
          fields,
        });
      }
      "decision" => {
        let (hp, ic, oc, rules, ac) = parse_decision_table_from_file(&drg_node.file_path);
        nodes.push(ProjectNode::Decision {
          id: dmn.id.clone(),
          name: dmn.name.clone(),
          hit_policy: hp,
          input_columns: ic,
          output_columns: oc,
          rules,
          annotation_columns: ac,
        });
      }
      "bkm" => {
        let (hp, ic, oc, rules, _ac) = parse_decision_table_from_file(&drg_node.file_path);
        let parameters = dmn
          .signature
          .as_ref()
          .map(|sig| {
            sig
              .parameters
              .iter()
              .map(|p| ProjectParam {
                name: p.name.clone(),
                param_type: p.param_type.clone(),
              })
              .collect()
          })
          .unwrap_or_default();
        nodes.push(ProjectNode::Bkm {
          id: dmn.id.clone(),
          name: dmn.name.clone(),
          hit_policy: hp,
          input_columns: ic,
          output_columns: oc,
          rules,
          parameters,
          feel_expression: dmn.feel_expression.clone(),
        });
      }
      "knowledge-source" => {
        nodes.push(ProjectNode::KnowledgeSource {
          id: dmn.id.clone(),
          name: dmn.name.clone(),
          owner: dmn.owner.clone(),
        });
      }
      _ => {}
    }

    // Build edges from requires/governed-by/supported-by.
    if let Some(ref requires) = dmn.requires {
      for target_id in requires {
        let label = drg.get_node(target_id).map(|n| n.dmn.name.clone()).unwrap_or_default();
        edges.push(ProjectEdge {
          source: target_id.clone(),
          target: dmn.id.clone(),
          kind: "requires".to_string(),
          label,
        });
      }
    }
    if let Some(ref governed_by) = dmn.governed_by {
      for target_id in governed_by {
        let label = drg.get_node(target_id).map(|n| n.dmn.name.clone()).unwrap_or_default();
        edges.push(ProjectEdge {
          source: target_id.clone(),
          target: dmn.id.clone(),
          kind: "governed-by".to_string(),
          label,
        });
      }
    }
    if let Some(ref supported_by) = dmn.supported_by {
      for target_id in supported_by {
        let label = drg.get_node(target_id).map(|n| n.dmn.name.clone()).unwrap_or_default();
        edges.push(ProjectEdge {
          source: target_id.clone(),
          target: dmn.id.clone(),
          kind: "supported-by".to_string(),
          label,
        });
      }
    }
  }

  // Load scenarios from scenarios.json if present.
  let scenarios = load_scenarios(project_dir);

  let response = ProjectResponse {
    nodes,
    edges,
    evaluation_order,
    scenarios,
  };
  HttpResponse::Ok()
    .content_type(CONTENT_TYPE)
    .body(serde_json::to_string(&response).unwrap_or_else(|_| r#"{"error":"serialization failed"}"#.to_string()))
}

/// Scans the project directory tree for type definition files.
fn scan_type_files(project_dir: &Path) -> TypeRegistry {
  // Walk subdirectories looking for types/ folders or .ts/.json files.
  let mut registry = TypeRegistry::new();
  for entry in walkdir::WalkDir::new(project_dir).max_depth(3).into_iter().filter_map(|e| e.ok()) {
    let path = entry.path();
    if path.is_dir() && path.file_name().map(|n| n == "types").unwrap_or(false) {
      if let Ok(r) = scan_folder(path) {
        let _ = registry.merge(r);
      }
    }
  }
  registry
}

/// Resolves input-data node fields from the TypeRegistry.
fn resolve_input_fields(dmn: &dsntk_type_registry::DmnNode, base_dir: &Path, registry: &TypeRegistry) -> Vec<FieldDescriptor> {
  let Some(ref data_type) = dmn.data_type else {
    return vec![];
  };

  let Ok(entry) = resolve_data_type(data_type, base_dir, registry) else {
    return vec![];
  };

  let mut fields = Vec::new();
  flatten_feel_type(&entry.feel_type, "", &entry, &mut fields);
  fields
}

/// Recursively flattens a FeelType into field descriptors.
fn flatten_feel_type(feel_type: &FeelType, prefix: &str, entry: &dsntk_type_registry::TypeEntry, fields: &mut Vec<FieldDescriptor>) {
  match feel_type {
    FeelType::Context(entries) => {
      for (name, field_type) in entries {
        let field_name = name.to_string();
        let path = if prefix.is_empty() { field_name.clone() } else { format!("{}.{}", prefix, field_name) };
        let is_optional = entry.optional_fields.get(&field_name).copied().unwrap_or(false);

        match field_type {
          FeelType::Context(_) => {
            // Recurse into nested contexts.
            flatten_feel_type(field_type, &path, entry, fields);
          }
          _ => {
            fields.push(FieldDescriptor {
              path: path.clone(),
              name: field_name,
              feel_type: feel_type_name(field_type),
              allowed_values: entry.allowed_values.clone(),
              optional: is_optional,
            });
          }
        }
      }
    }
    _ => {
      let field_name = if prefix.is_empty() {
        "value".to_string()
      } else {
        prefix.rsplit('.').next().unwrap_or(prefix).to_string()
      };
      fields.push(FieldDescriptor {
        path: prefix.to_string(),
        name: field_name,
        feel_type: feel_type_name(feel_type),
        allowed_values: entry.allowed_values.clone(),
        optional: false,
      });
    }
  }
}

/// Converts FeelType to a simple string name for the frontend.
fn feel_type_name(feel_type: &FeelType) -> String {
  match feel_type {
    FeelType::Number => "number".to_string(),
    FeelType::String => "string".to_string(),
    FeelType::Boolean => "boolean".to_string(),
    FeelType::Date => "date".to_string(),
    FeelType::DateTime => "dateTime".to_string(),
    FeelType::Time => "time".to_string(),
    FeelType::DaysAndTimeDuration => "duration".to_string(),
    FeelType::YearsAndMonthsDuration => "duration".to_string(),
    FeelType::List(inner) => format!("list<{}>", feel_type_name(inner)),
    FeelType::Context(_) => "context".to_string(),
    FeelType::Any => "any".to_string(),
    FeelType::Null => "null".to_string(),
    _ => "any".to_string(),
  }
}

/// Reads a markdown file, extracts the body, and parses the decision table.
fn parse_decision_table_from_file(file_path: &str) -> (String, Vec<String>, Vec<String>, Vec<ProjectRule>, Vec<String>) {
  let Ok(content) = std::fs::read_to_string(file_path) else {
    return (String::new(), vec![], vec![], vec![], vec![]);
  };

  let Some(body) = extract_body(&content) else {
    return (String::new(), vec![], vec![], vec![], vec![]);
  };

  let Ok(dt) = dsntk_recognizer::from_markdown(body, false) else {
    return (String::new(), vec![], vec![], vec![], vec![]);
  };

  let hit_policy = format!("{}", dt.hit_policy);
  let input_columns: Vec<String> = dt.input_clauses.iter().map(|ic| ic.input_expression.clone()).collect();
  let output_columns: Vec<String> = dt.output_clauses.iter().map(|oc| oc.output_component_name.clone().unwrap_or_default()).collect();
  let annotation_columns: Vec<String> = dt.annotation_clauses.iter().map(|ac| ac.name.clone()).collect();

  let rules: Vec<ProjectRule> = dt
    .rules
    .iter()
    .enumerate()
    .map(|(i, rule)| ProjectRule {
      index: i,
      input_entries: rule.input_entries.iter().map(|ie| ie.text.clone()).collect(),
      output_entries: rule.output_entries.iter().map(|oe| oe.text.clone()).collect(),
      annotations: rule.annotation_entries.iter().map(|ae| ae.text.clone()).collect(),
    })
    .collect();

  (hit_policy, input_columns, output_columns, rules, annotation_columns)
}

/// Loads scenario data from scenarios.json in the project directory.
fn load_scenarios(project_dir: &Path) -> Vec<serde_json::Value> {
  let path = project_dir.join("scenarios.json");
  let Ok(content) = std::fs::read_to_string(&path) else {
    return vec![];
  };
  let Ok(parsed) = serde_json::from_str::<Vec<serde_json::Value>>(&content) else {
    return vec![];
  };
  parsed
}

// --- Project evaluation endpoint ---

/// Request body for project evaluation.
#[derive(Debug, Deserialize)]
struct EvaluateProjectRequest {
  dir: String,
  inputs: HashMap<String, serde_json::Value>,
}

/// Per-node evaluation result.
#[derive(Debug, Clone, Serialize)]
struct NodeEvalResult {
  node_id: String,
  node_name: String,
  matched_rules: Vec<usize>,
  output_value: serde_json::Value,
  cell_evaluations: Vec<serde_json::Value>,
}

/// Full evaluation response.
#[derive(Debug, Clone, Serialize)]
struct EvaluateProjectResponse {
  results: Vec<NodeEvalResult>,
  evaluation_order: Vec<String>,
}

/// POST /api/v1/project/evaluate
///
/// Evaluates a markdown DMN project with the given inputs.
/// Walks the DRG in topological order, evaluating each decision table
/// and propagating results to downstream nodes.
#[post("/api/v1/project/evaluate")]
async fn evaluate_project(body: web::Json<EvaluateProjectRequest>) -> HttpResponse {
  let project_dir = Path::new(&body.dir);
  if !project_dir.is_dir() {
    return HttpResponse::BadRequest()
      .content_type(CONTENT_TYPE)
      .body(format!(r#"{{"error":"directory not found: {}"}}"#, body.dir));
  }

  // Build the DRG.
  let drg = match build_drg(project_dir) {
    Ok(drg) => drg,
    Err(e) => {
      return HttpResponse::InternalServerError()
        .content_type(CONTENT_TYPE)
        .body(format!(r#"{{"error":"failed to build DRG: {}"}}"#, e));
    }
  };

  let evaluation_order = drg.topological_order().unwrap_or_default();

  // Build the FEEL scope from input data.
  let scope = FeelScope::default();
  let mut ctx = dsntk_feel::context::FeelContext::default();

  // Populate context with input values.
  // Each input is stored both as a named context (for path expressions)
  // and with its fields flattened to top-level scope entries (for direct column name lookups).
  for (node_id, value) in &body.inputs {
    if let Some(drg_node) = drg.get_node(node_id) {
      let name = Name::new(&[&drg_node.dmn.name]);
      let feel_value = json_to_feel_value(value);
      ctx.set_entry(&name, feel_value.clone());
      // Flatten: if the value is a context, set each field as a top-level scope entry.
      // Keys should use the FEEL names matching the decision table columns.
      if let Value::Context(ref inner_ctx) = feel_value {
        for (entry_name, entry_value) in inner_ctx.iter() {
          ctx.set_entry(entry_name, entry_value.clone());
        }
      }
    }
  }
  scope.push(ctx);

  // Evaluate decisions in topological order.
  let mut results = Vec::new();

  for node_id in &evaluation_order {
    let Some(drg_node) = drg.get_node(node_id) else {
      continue;
    };

    match drg_node.dmn.node_type.as_str() {
      "decision" | "bkm" => {
        let Ok(content) = std::fs::read_to_string(&drg_node.file_path) else {
          continue;
        };

        // Check if this node has a FEEL expression (expression-only BKM).
        let (eval_value, matched_rules, cell_evals) = if let Some(ref feel_expr) = drg_node.dmn.feel_expression {
          match dsntk_feel_parser::parse_expression(&scope, feel_expr, false) {
            Ok(ast_node) => {
              let evaluator = dsntk_feel_evaluator::prepare(&ast_node);
              let value = evaluator(&scope);
              (value, vec![], vec![])
            }
            Err(e) => (Value::Null(Some(format!("{}", e))), vec![], vec![]),
          }
        } else {
          // Decision table evaluation (existing logic).
          let Some(md_body) = extract_body(&content) else {
            continue;
          };
          let Ok(recognized_dt) = dsntk_recognizer::from_markdown(md_body, false) else {
            continue;
          };
          let model_dt: dsntk_model::DecisionTable = recognized_dt.into();
          match evaluate_decision_table_with_trace(&scope, &model_dt) {
            Ok(r) => {
              let ce: Vec<serde_json::Value> = r.cell_evaluations.iter().map(|ce| serde_json::to_value(ce).unwrap_or_default()).collect();
              (r.value, r.matched_rules, ce)
            }
            Err(e) => (Value::Null(Some(format!("{}", e))), vec![], vec![]),
          }
        };

        // Store the result in scope so downstream decisions can use it.
        let name = Name::new(&[&drg_node.dmn.name]);
        if let Some(mut top_ctx) = scope.pop() {
          top_ctx.set_entry(&name, eval_value.clone());

          // Store under output-name if specified (for expression BKMs like "LTV", "DSCR").
          if let Some(ref out_name) = drg_node.dmn.output_name {
            let output_key = Name::new(&[out_name]);
            top_ctx.set_entry(&output_key, eval_value.clone());
          }

          // For decision tables, also store under output column names.
          if drg_node.dmn.feel_expression.is_none() {
            if let Some(md_body) = extract_body(&content) {
              if let Ok(recognized_dt) = dsntk_recognizer::from_markdown(md_body, false) {
                let model_dt: dsntk_model::DecisionTable = recognized_dt.into();
                let output_clauses: Vec<_> = model_dt.output_clauses().collect();
                if output_clauses.len() == 1 {
                  if let Some(ref output_name) = output_clauses[0].name {
                    let out_name = Name::new(&[output_name]);
                    top_ctx.set_entry(&out_name, eval_value.clone());
                  }
                } else {
                  if let Value::Context(ref out_ctx) = eval_value {
                    for (entry_name, entry_value) in out_ctx.iter() {
                      top_ctx.set_entry(entry_name, entry_value.clone());
                    }
                  }
                }
              }
            }
          }

          scope.push(top_ctx);
        }

        let output_json = serde_json::to_value(eval_value.jsonify()).unwrap_or(serde_json::Value::Null);
        results.push(NodeEvalResult {
          node_id: node_id.clone(),
          node_name: drg_node.dmn.name.clone(),
          matched_rules,
          output_value: output_json,
          cell_evaluations: cell_evals,
        });
      }
      _ => {}
    }
  }

  let response = EvaluateProjectResponse { results, evaluation_order };
  HttpResponse::Ok()
    .content_type(CONTENT_TYPE)
    .body(serde_json::to_string(&response).unwrap_or_else(|_| r#"{"error":"serialization failed"}"#.to_string()))
}

/// Converts camelCase to Title Case (e.g., "bureauScore" -> "Bureau Score").
fn camel_to_title_case(s: &str) -> String {
  let mut result = String::with_capacity(s.len() + 4);
  for (i, ch) in s.chars().enumerate() {
    if ch.is_uppercase() && i > 0 {
      result.push(' ');
    }
    if i == 0 {
      result.extend(ch.to_uppercase());
    } else {
      result.push(ch);
    }
  }
  result
}

/// Converts a JSON value to a FEEL value.
fn json_to_feel_value(json: &serde_json::Value) -> Value {
  match json {
    serde_json::Value::Null => Value::Null(None),
    serde_json::Value::Bool(b) => Value::Boolean(*b),
    serde_json::Value::Number(n) => {
      let s = n.to_string();
      match s.parse::<dsntk_feel::FeelNumber>() {
        Ok(num) => Value::Number(num),
        Err(_) => Value::Null(None),
      }
    }
    serde_json::Value::String(s) => Value::String(s.clone()),
    serde_json::Value::Array(arr) => {
      let values: Vec<Value> = arr.iter().map(json_to_feel_value).collect();
      Value::List(values)
    }
    serde_json::Value::Object(obj) => {
      let mut ctx = dsntk_feel::context::FeelContext::default();
      for (key, val) in obj {
        ctx.set_entry(&Name::new(&[key]), json_to_feel_value(val));
      }
      Value::Context(ctx)
    }
  }
}
