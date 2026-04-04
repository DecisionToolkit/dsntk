//! # Error definitions for the type registry

use dsntk_common::{DsntkError, ToErrorMessage};

/// Type registry error.
#[derive(ToErrorMessage)]
struct TypeRegistryError(String);

pub fn err_type_not_found(type_name: &str) -> DsntkError {
  TypeRegistryError(format!("type '{type_name}' not found in registry")).into()
}

pub fn err_type_not_found_in_schema(type_name: &str, schema_path: &str) -> DsntkError {
  TypeRegistryError(format!("type '{type_name}' not found in schema '{schema_path}'")).into()
}

pub fn err_ambiguous_type(type_name: &str, file1: &str, file2: &str) -> DsntkError {
  TypeRegistryError(format!("ambiguous type '{type_name}' found in both '{file1}' and '{file2}'")).into()
}

pub fn err_schema_file_not_found(path: &str) -> DsntkError {
  TypeRegistryError(format!("schema file or directory not found: '{path}'")).into()
}

pub fn err_typescript_parse(file: &str, message: &str) -> DsntkError {
  TypeRegistryError(format!("failed to parse TypeScript file '{file}': {message}")).into()
}

pub fn err_json_schema_parse(file: &str, message: &str) -> DsntkError {
  TypeRegistryError(format!("failed to parse JSON Schema file '{file}': {message}")).into()
}

pub fn err_invalid_front_matter(message: &str) -> DsntkError {
  TypeRegistryError(format!("invalid front matter: {message}")).into()
}

pub fn err_invalid_node_type(node_type: &str) -> DsntkError {
  TypeRegistryError(format!("invalid node type: '{node_type}'")).into()
}

pub fn err_missing_return_type(bkm_name: &str) -> DsntkError {
  TypeRegistryError(format!("BKM '{bkm_name}' is missing a return-type declaration")).into()
}

pub fn err_invalid_link_target(link_key: &str, target: &str, expected_type: &str) -> DsntkError {
  TypeRegistryError(format!("'{link_key}' link target '{target}' must be of type '{expected_type}'")).into()
}

pub fn err_bkm_invocation_arity(bkm_name: &str, expected: usize, actual: usize) -> DsntkError {
  TypeRegistryError(format!("BKM '{bkm_name}' expects {expected} arguments but {actual} were provided")).into()
}

pub fn err_enum_violation(value: &str, allowed: &[String]) -> DsntkError {
  TypeRegistryError(format!("value '{value}' is not in allowed values: [{}]", allowed.join(", "))).into()
}

pub fn err_drg_cycle(cycle_path: &str) -> DsntkError {
  TypeRegistryError(format!("cycle detected in decision requirements graph: {cycle_path}")).into()
}

pub fn err_drg_unresolved_link(source_file: &str, target_id: &str) -> DsntkError {
  TypeRegistryError(format!("unresolved link in '{source_file}': target node '{target_id}' not found")).into()
}

pub fn err_drg_duplicate_node_id(id: &str, file1: &str, file2: &str) -> DsntkError {
  TypeRegistryError(format!("duplicate node id '{id}' in '{file1}' and '{file2}'")).into()
}

pub fn err_drg_no_md_files(dir: &str) -> DsntkError {
  TypeRegistryError(format!("no markdown DMN files found in '{dir}'")).into()
}

pub fn err_drg_file_read(path: &str, message: &str) -> DsntkError {
  TypeRegistryError(format!("failed to read '{path}': {message}")).into()
}

pub fn err_drg_invalid_edge_type(source_id: &str, link_key: &str, target_id: &str, expected_type: &str, actual_type: &str) -> DsntkError {
  TypeRegistryError(format!(
    "node '{source_id}' has '{link_key}' link to '{target_id}' which is type '{actual_type}', expected '{expected_type}'"
  ))
  .into()
}
