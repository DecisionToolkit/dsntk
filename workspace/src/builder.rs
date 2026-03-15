//! # Workspace builder

use antex::{Color, ColorMode, StyledText, Text};
use dsntk_common::{encode_segments, to_rdnn};
use dsntk_model::{Definitions, DmnElement};
use dsntk_model_evaluator::ModelEvaluator;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use walkdir::WalkDir;

/// Workspace builder.
pub struct WorkspaceBuilder {
  /// Color mode.
  cm: ColorMode,
  /// Flag indicating the level of verbosity.
  verbose: bool,
  /// Total number of model files found.
  file_count: usize,
  /// The number of successfully loaded models.
  loaded_count: usize,
  /// The number of models that failed to load.
  failed_loads_count: usize,
  /// The number of workspaces that failed to load.
  failed_deployments_count: usize,
  /// Map: workspace name -> model definitions.
  workspace_definitions: HashMap<String, Vec<Definitions>>,
  /// Map: workspace name -> namespaces in workspace
  workspace_namespaces: HashMap<String, HashSet<String>>,
  /// Map: workspace name -> (file_name, rdnn)
  workspace_models: HashMap<String, HashMap<String, String>>,
  /// Map: invocable path -> (workspace name, model namespace, model name, invocable name)
  pub(crate) invocables: HashMap<String, (String, String, String, String)>,
  /// Map: workspace name -> model evaluator
  pub(crate) evaluators: HashMap<String, Arc<ModelEvaluator>>,
}

impl WorkspaceBuilder {
  /// Creates a new workspace builder.
  pub fn new(dirs: &[PathBuf], cm: ColorMode, verbose: bool) -> Self {
    let mut builder = Self {
      cm,
      verbose,
      file_count: 0,
      loaded_count: 0,
      failed_loads_count: 0,
      failed_deployments_count: 0,
      workspace_definitions: Default::default(),
      workspace_namespaces: Default::default(),
      workspace_models: Default::default(),
      invocables: Default::default(),
      evaluators: Default::default(),
    };
    builder.load(dirs);
    builder
  }

  /// Loads models from multiple directories.
  fn load(&mut self, dirs: &[PathBuf]) {
    for dir in dirs {
      self.load_from_dir(dir);
    }
    self.display_summary();
  }

  /// Scans the specified directory and loads decision models from files, builds the workspaces.
  fn load_from_dir(&mut self, dir: &Path) {
    // load model files
    for entry_result in WalkDir::new(dir).into_iter() {
      match entry_result {
        Ok(entry) => {
          let path = entry.path();
          if path.is_file() && path.extension().is_some_and(|ext| ext == "dmn") {
            self.file_count += 1;
            let workspace_name = self.workspace_name(dir, path);
            self.load_model(&workspace_name, path);
          }
        }
        Err(reason) => self.err_file_operation(reason.to_string()),
      }
    }
    // build evaluators
    for (workspace_name, loaded_definitions) in &self.workspace_definitions {
      match ModelEvaluator::new(loaded_definitions) {
        Ok(evaluator) => {
          self.evaluators.insert(workspace_name.to_string(), evaluator);
        }
        Err(reason) => {
          self.err_deployment_failure(workspace_name, reason.to_string());
          self.failed_deployments_count += 1;
        }
      }
    }
    // build invocable paths
    for (workspace_name, evaluator) in &self.evaluators {
      for (model_namespace, model_name, invocable_name) in evaluator.invocables().list() {
        let invocable_path = format!(
          "{}{}/{}/{}",
          if !workspace_name.is_empty() { format!("{}/", workspace_name) } else { "".to_string() },
          to_rdnn(&model_namespace).unwrap(),
          model_name,
          invocable_name
        );
        self
          .invocables
          .insert(invocable_path, (workspace_name.clone(), model_namespace, model_name, invocable_name));
      }
    }
  }

  /// Checks if namespaces are duplicated in the workspace.
  fn check_namespace_duplicates(&self, file: &Path, workspace_name: &str, namespace: &str) -> bool {
    if let Some(namespaces) = self.workspace_namespaces.get(workspace_name) {
      if namespaces.contains(namespace) {
        let file_name = self.workspace_models.get(workspace_name).unwrap().get(namespace).unwrap();
        self.err_duplicated_namespace(workspace_name, file, namespace, file_name);
        return false;
      }
    }
    true
  }

  /// Loads decision model from specified file.
  fn load_model(&mut self, workspace_name: &str, file: &Path) {
    match fs::read_to_string(file) {
      Ok(xml) => match dsntk_model::from_xml(&xml) {
        Ok(definitions) => {
          let namespace = definitions.namespace().to_string();
          if to_rdnn(&namespace).is_some() {
            if self.check_namespace_duplicates(file, workspace_name, &namespace) {
              self
                .workspace_definitions
                .entry(workspace_name.to_string())
                .and_modify(|loaded_definitions| {
                  loaded_definitions.push(definitions.clone());
                })
                .or_insert(vec![definitions]);
              self
                .workspace_namespaces
                .entry(workspace_name.to_string())
                .and_modify(|loaded_namespaces| {
                  loaded_namespaces.insert(namespace.clone());
                })
                .or_insert({
                  let mut set = HashSet::new();
                  set.insert(namespace.clone());
                  set
                });
              self
                .workspace_models
                .entry(workspace_name.to_string())
                .and_modify(|loaded_models| {
                  loaded_models.insert(namespace.clone(), file.to_string_lossy().to_string());
                })
                .or_insert({
                  let mut map = HashMap::new();
                  map.insert(namespace.clone(), file.to_string_lossy().to_string());
                  map
                });
              self.loaded_count += 1;
            }
          } else {
            self.err_invalid_namespace(workspace_name, file, &namespace);
            self.failed_loads_count += 1;
          }
        }
        Err(reason) => {
          self.err_file_load(workspace_name, file, reason.to_string());
          self.failed_loads_count += 1;
        }
      },
      Err(reason) => {
        self.err_file_load(workspace_name, file, reason.to_string());
        self.failed_loads_count += 1;
      }
    }
  }

  /// Displays the summary of the loading process.
  fn display_summary(&self) {
    // display the number of found files
    Text::new(self.cm)
      .color(if self.file_count > 0 { Color::Green } else { Color::Red })
      .s("Found ")
      .s(self.file_count)
      .plural(" model", self.file_count)
      .dot()
      .cprintln();
    // display the number of successfully loaded files
    if self.loaded_count > 0 {
      Text::new(self.cm)
        .green()
        .s("Loaded ")
        .s(self.loaded_count)
        .plural(" model", self.loaded_count)
        .dot()
        .cprintln();
    }
    // display the number of failed loads
    if self.failed_loads_count > 0 {
      Text::new(self.cm)
        .red()
        .s("Failed to load ")
        .s(self.failed_loads_count)
        .plural(" model", self.failed_loads_count)
        .dot()
        .cprintln();
    }
    // display the number of successfully deployed invocables
    let deployed_count = self.evaluators.values().map(|evaluator| evaluator.invocables().len()).sum();
    Text::new(self.cm)
      .color(if deployed_count > 0 { Color::Green } else { Color::Red })
      .s("Deployed ")
      .s(deployed_count)
      .plural(" invocable", deployed_count)
      .dot()
      .cprintln();
    // display the number of failed deployments
    if self.failed_deployments_count > 0 {
      Text::new(self.cm)
        .red()
        .s("Failed to deploy ")
        .s(self.failed_deployments_count)
        .plural(" workspace", self.failed_deployments_count)
        .dot()
        .cprintln();
    }
    if self.verbose {
      self.display_deployed_invocables();
    }
  }

  /// Displays the names of the deployed invocables (encoded URLs).
  fn display_deployed_invocables(&self) {
    let mut invocable_paths = self.invocables.keys().cloned().collect::<Vec<String>>();
    invocable_paths.sort();
    let invocable_count = invocable_paths.len();
    if invocable_count > 0 {
      self.text().nl().yellow().s("Deployed ").plural("invocable", invocable_count).colon().cprintln();
    }
    for key in invocable_paths {
      if let Some((workspace_name, model_namespace, model_name, invocable_name)) = self.invocables.get(&key) {
        print!("  ");
        // print workspace name containing the directory structure (URL encoded)
        let segment_1 = encode_segments(workspace_name);
        if !segment_1.is_empty() {
          self.text().magenta().s(segment_1).clear().slash().print();
        }
        // print model namespace converted to DNN (URL encoded)
        let rdnn = to_rdnn(model_namespace).unwrap_or_default();
        let segment_2 = encode_segments(&rdnn);
        if !segment_2.is_empty() {
          self.text().cyan().s(segment_2).clear().slash().print();
        }
        // print model name (URL encoded)
        let segment_3 = encode_segments(model_name);
        if !segment_3.is_empty() {
          self.text().magenta().s(segment_3).clear().slash().print();
        }
        // print invocable name (URL encoded)
        let segment_4 = encode_segments(invocable_name);
        if !segment_4.is_empty() {
          self.text().cyan().s(segment_4).clear().print();
        }
        println!();
      }
    }
    if invocable_count > 0 {
      println!();
    }
  }

  /// Returns workspace name created from provided parent and child paths.
  fn workspace_name(&self, parent_path: &Path, child_path: &Path) -> String {
    let canonical_parent_path = parent_path.canonicalize().unwrap();
    let canonical_child_path = child_path.canonicalize().unwrap();
    let mut workspace_name = canonical_child_path.parent().unwrap();
    workspace_name = workspace_name.strip_prefix(canonical_parent_path).unwrap();
    workspace_name
      .to_string_lossy()
      .replace('\\', "/")
      .trim_start_matches('/')
      .trim_end_matches('/')
      .to_string()
  }

  /// Prints file loading error details.
  fn err_file_load(&self, workspace_name: &str, file: &Path, reason: String) {
    self.error_2(self.join(workspace_name, file), reason);
  }

  /// Prints duplicated namespace error details.
  fn err_duplicated_namespace(&self, workspace_name: &str, file: &Path, namespace: &str, file_name: &str) {
    self.error_2(
      self.join(workspace_name, file),
      format!("duplicated namespace '{}' in file {}", namespace, self.join(workspace_name, Path::new(file_name))),
    );
  }

  /// Prints invalid namespace error details.
  fn err_invalid_namespace(&self, workspace_name: &str, file: &Path, namespace: &str) {
    self.error_2(self.join(workspace_name, file), format!("invalid namespace: '{namespace}'"));
  }

  /// Prints deployment error details.
  fn err_deployment_failure(&self, workspace_name: &str, reason: String) {
    self.error_2(
      if workspace_name.is_empty() { "/" } else { workspace_name },
      format!("deployment failed with reason: {}", reason),
    );
  }

  /// Prints file operation error details.
  fn err_file_operation(&self, reason: String) {
    self.error_1(reason);
  }

  /// Joins the workspace name with the name of the file.
  fn join(&self, workspace_name: &str, file: &Path) -> String {
    if workspace_name.is_empty() {
      format!("{}", file.file_name().unwrap().to_string_lossy())
    } else {
      format!("{}/{}", workspace_name, file.file_name().unwrap().to_string_lossy())
    }
  }

  /// Utility function for printing styled error message with one argument.
  fn error_1<T: Display>(&self, s1: T) {
    self.text().s('[').red().s("error").clear().s(']').space().red().s(s1).cprintln();
  }

  /// Utility function for printing styled error message with two arguments.
  fn error_2<A: Display, B: Display>(&self, s1: A, s2: B) {
    self
      .text()
      .s('[')
      .red()
      .s("error")
      .clear()
      .s("][")
      .blue()
      .s(s1)
      .clear()
      .s(']')
      .space()
      .red()
      .s(s2)
      .cprintln();
  }

  /// Utility function for instantiating a styled text with preset color mode.
  fn text(&self) -> Text {
    Text::new(self.cm)
  }
}
