//! # ASCII report of the DMN model

use antex::{Color, ColorMode, NodeBuilder, StyledText, Text, TreeNode, leaf, node};
use dsntk_common::HRef;
use dsntk_feel::Name;
use dsntk_model::*;
use std::fmt::Write;

const LABEL_ALLOWED_ANSWERS: &str = "allowed answers";
const LABEL_BUSINESS_KNOWLEDGE_MODELS: &str = "Business knowledge models";
const LABEL_DECISION_SERVICES: &str = "Decision services";
const LABEL_DECISIONS: &str = "Decisions";
const LABEL_DECISIONS_MADE: &str = "decisions made";
const LABEL_DECISIONS_OWNED: &str = "decisions owned";
const LABEL_DESCRIPTION: &str = "description";
const LABEL_EXPRESSION_LANGUAGE: &str = "expression language";
const LABEL_ID: &str = "id";
const LABEL_INPUT_DATA: &str = "Input data";
const LABEL_KNOWLEDGE_SOURCES: &str = "Knowledge sources";
const LABEL_LABEL: &str = "label";
const LABEL_IMPACTING_DECISIONS: &str = "impacting decisions";
const LABEL_MODEL: &str = "Model";
const LABEL_NAME: &str = "name";
const LABEL_FEEL_NAME: &str = "FEEL name";
const LABEL_NAMESPACE: &str = "namespace";
const LABEL_ORGANISATION_UNITS: &str = "Organisation units";
const LABEL_PERFORMANCE_INDICATORS: &str = "Performance indicators";
const LABEL_QUESTION: &str = "question";
const LABEL_TYPE: &str = "type";
const LABEL_TYPE_LANGUAGE: &str = "type language";
const LABEL_URI: &str = "URI";
const LABEL_VARIABLE: &str = "variable (output)";

/// Default color of the tree arms.
const DEFAULT_COLOR: Color = Color::White;

/// Color palette.
struct Colors {
  color_default: u8,
  color_name: u8,
  color_label: u8,
  color_id: u8,
  color_uri: u8,
  color_description: u8,
  color_type: u8,
  color_href: u8,
}

impl Colors {
  /// Creates a new color palette based on color mode.
  fn new() -> Self {
    Self {
      color_default: 255,
      color_name: 184,
      color_label: 209,
      color_id: 82,
      color_uri: 56,
      color_description: 255,
      color_type: 74,
      color_href: 61,
    }
  }
  fn default(&self) -> u8 {
    self.color_default
  }
  fn name(&self) -> u8 {
    self.color_name
  }
  fn label(&self) -> u8 {
    self.color_label
  }
  fn id(&self) -> u8 {
    self.color_id
  }
  fn uri(&self) -> u8 {
    self.color_uri
  }
  fn description(&self) -> u8 {
    self.color_description
  }
  fn typ(&self) -> u8 {
    self.color_type
  }
  fn href(&self) -> u8 {
    self.color_href
  }
}

/// Prints the model report to standard output.
pub fn print_model(definitions: Definitions, cm: ColorMode) {
  let colors = &Colors::new();
  let indent = 2;
  let mut output = String::new();
  write_model(&mut output, &definitions, cm, colors, indent);
  write_decisions(&mut output, &definitions, cm, colors, indent);
  write_business_knowledge_models(&mut output, &definitions, cm, colors, indent);
  write_decision_services(&mut output, &definitions, cm, colors, indent);
  write_knowledge_sources(&mut output, &definitions, cm, colors, indent);
  write_input_data(&mut output, &definitions, cm, colors, indent);
  write_performance_indicators(&mut output, &definitions, cm, colors, indent);
  write_organisation_units(&mut output, &definitions, cm, colors, indent);
  println!("{}", output);
}

/// Writes a title text.
fn write_title(w: &mut dyn Write, title: &str) {
  let bar = "─".repeat(title.len());
  let _ = writeln!(w);
  let _ = writeln!(w, "┌─{}─┐", bar);
  let _ = writeln!(w, "│ {} │", title);
  let _ = writeln!(w, "└─{}─┘", bar);
}

/// Writes an empty line when counter > 0.
fn write_empty_line(w: &mut dyn Write, counter: usize) {
  if counter > 0 {
    let _ = writeln!(w);
  }
}

/// Write model properties.
fn write_model(w: &mut dyn Write, definitions: &Definitions, cm: ColorMode, colors: &Colors, indent: usize) {
  write_title(w, LABEL_MODEL);
  let node_model = builder_name(definitions.name(), cm, colors)
    .child(build_feel_name(definitions.feel_name(), cm, colors))
    .child(build_namespace_leaf(definitions.namespace(), cm, colors))
    .opt_child(build_label(definitions.label(), cm, colors))
    .opt_child(build_id(definitions.opt_id(), cm, colors))
    .opt_child(build_labeled_uri(LABEL_EXPRESSION_LANGUAGE, definitions.expression_language(), cm, colors))
    .opt_child(build_labeled_uri(LABEL_TYPE_LANGUAGE, definitions.type_language(), cm, colors))
    .opt_child(build_opt_labeled_text("exporter", definitions.exporter(), cm, colors.default()))
    .opt_child(build_opt_labeled_text("exporter version", definitions.exporter_version(), cm, colors.default()))
    .opt_child(build_description(definitions.description(), cm, colors))
    .opt_child(build_extension_elements(definitions.extension_elements(), colors))
    .opt_child(build_extension_attributes(definitions.extension_attributes(), colors))
    .end();
  _ = write!(w, "{:indent$}", node_model);
}

/// Write decisions.
fn write_decisions(w: &mut dyn Write, definitions: &Definitions, cm: ColorMode, colors: &Colors, indent: usize) {
  let decisions = definitions.decisions();
  if !decisions.is_empty() {
    write_title(w, LABEL_DECISIONS);
  }
  for (i, decision) in decisions.iter().enumerate() {
    let node_decision = builder_name(decision.name(), cm, colors)
      .child(build_feel_name(decision.feel_name(), cm, colors))
      .opt_child(build_label(decision.label(), cm, colors))
      .opt_child(build_id(decision.opt_id(), cm, colors))
      .opt_child(build_opt_labeled_multiline_text(LABEL_QUESTION, decision.question(), cm, colors.default()))
      .opt_child(build_opt_labeled_multiline_text(LABEL_ALLOWED_ANSWERS, decision.allowed_answers(), cm, colors.default()))
      .opt_child(build_description(decision.description(), cm, colors))
      .child(build_variable(decision.variable(), cm, colors))
      .opt_child(build_authority_requirements(decision.authority_requirements(), cm, colors))
      .opt_child(build_extension_elements(decision.extension_elements(), colors))
      .opt_child(build_extension_attributes(decision.extension_attributes(), colors))
      .end();
    write_empty_line(w, i);
    _ = write!(w, "{:indent$}", node_decision);
  }
}

/// Write business knowledge models.
fn write_business_knowledge_models(w: &mut dyn Write, definitions: &Definitions, cm: ColorMode, colors: &Colors, indent: usize) {
  let business_knowledge_models = definitions.business_knowledge_models();
  if !business_knowledge_models.is_empty() {
    write_title(w, LABEL_BUSINESS_KNOWLEDGE_MODELS);
  }
  for (i, bkm) in business_knowledge_models.iter().enumerate() {
    let node_decision = builder_name(bkm.name(), cm, colors)
      .child(build_feel_name(bkm.feel_name(), cm, colors))
      .opt_child(build_label(bkm.label(), cm, colors))
      .opt_child(build_id(bkm.opt_id(), cm, colors))
      .opt_child(build_description(bkm.description(), cm, colors))
      .child(build_variable(bkm.variable(), cm, colors))
      .opt_child(build_authority_requirements(bkm.authority_requirements(), cm, colors))
      .opt_child(build_extension_elements(bkm.extension_elements(), colors))
      .opt_child(build_extension_attributes(bkm.extension_attributes(), colors))
      .end();
    write_empty_line(w, i);
    _ = write!(w, "{:indent$}", node_decision);
  }
}

/// Write decision services.
fn write_decision_services(w: &mut dyn Write, definitions: &Definitions, cm: ColorMode, colors: &Colors, indent: usize) {
  let decision_services = definitions.decision_services();
  if !decision_services.is_empty() {
    write_title(w, LABEL_DECISION_SERVICES);
  }
  for (i, decision_service) in decision_services.iter().enumerate() {
    let node_decision = builder_name(decision_service.name(), cm, colors)
      .child(build_feel_name(decision_service.feel_name(), cm, colors))
      .opt_child(build_label(decision_service.label(), cm, colors))
      .opt_child(build_id(decision_service.opt_id(), cm, colors))
      .opt_child(build_description(decision_service.description(), cm, colors))
      .child(build_variable(decision_service.variable(), cm, colors))
      .opt_child(build_extension_elements(decision_service.extension_elements(), colors))
      .opt_child(build_extension_attributes(decision_service.extension_attributes(), colors))
      .end();
    write_empty_line(w, i);
    _ = write!(w, "{:indent$}", node_decision);
  }
}

/// Write knowledge sources.
fn write_knowledge_sources(w: &mut dyn Write, definitions: &Definitions, cm: ColorMode, colors: &Colors, indent: usize) {
  let knowledge_sources = definitions.knowledge_sources();
  if !knowledge_sources.is_empty() {
    write_title(w, LABEL_KNOWLEDGE_SOURCES);
  }
  for (i, knowledge_source) in knowledge_sources.iter().enumerate() {
    let node_decision = builder_name(knowledge_source.name(), cm, colors)
      .child(build_feel_name(knowledge_source.feel_name(), cm, colors))
      .opt_child(build_label(knowledge_source.label(), cm, colors))
      .opt_child(build_id(knowledge_source.opt_id(), cm, colors))
      .opt_child(build_description(knowledge_source.description(), cm, colors))
      .opt_child(build_extension_elements(knowledge_source.extension_elements(), colors))
      .opt_child(build_extension_attributes(knowledge_source.extension_attributes(), colors))
      .opt_child(build_authority_requirements(knowledge_source.authority_requirements(), cm, colors))
      .end();
    write_empty_line(w, i);
    _ = write!(w, "{:indent$}", node_decision);
  }
}

/// Write input data.
fn write_input_data(w: &mut dyn Write, definitions: &Definitions, cm: ColorMode, colors: &Colors, indent: usize) {
  let input_data = definitions.input_data();
  if !input_data.is_empty() {
    write_title(w, LABEL_INPUT_DATA);
  }
  for (i, input) in input_data.iter().enumerate() {
    let node_input = builder_name(input.name(), cm, colors)
      .child(build_feel_name(input.feel_name(), cm, colors))
      .opt_child(build_label(input.label(), cm, colors))
      .opt_child(build_id(input.opt_id(), cm, colors))
      .opt_child(build_description(input.description(), cm, colors))
      .child(build_variable(input.variable(), cm, colors))
      .opt_child(build_extension_elements(input.extension_elements(), colors))
      .opt_child(build_extension_attributes(input.extension_attributes(), colors))
      .end();
    write_empty_line(w, i);
    _ = write!(w, "{:indent$}", node_input);
  }
}

/// Write performance indicators.
fn write_performance_indicators(w: &mut dyn Write, definitions: &Definitions, cm: ColorMode, colors: &Colors, indent: usize) {
  let performance_indicators = definitions.performance_indicators();
  if !performance_indicators.is_empty() {
    write_title(w, LABEL_PERFORMANCE_INDICATORS);
  }
  for (i, performance_indicator) in performance_indicators.iter().enumerate() {
    // prepare a node with impacting decisions
    let mut impacting_decisions_builder = node(DEFAULT_COLOR, cm).line().s(LABEL_IMPACTING_DECISIONS).s(':').end();
    for impacting_decision in performance_indicator.impacting_decisions() {
      impacting_decisions_builder.add_child(build_href(impacting_decision, cm, colors));
    }
    let node_impacting_decisions = impacting_decisions_builder.end();
    let node_performance_indicator = builder_name(performance_indicator.name(), cm, colors)
      .child(build_feel_name(performance_indicator.feel_name(), cm, colors))
      .opt_child(build_label(performance_indicator.label(), cm, colors))
      .opt_child(build_id(performance_indicator.opt_id(), cm, colors))
      .opt_child(build_description(performance_indicator.description(), cm, colors))
      .opt_child(build_uri(performance_indicator.uri(), cm, colors))
      .child(node_impacting_decisions)
      .opt_child(build_extension_elements(performance_indicator.extension_elements(), colors))
      .opt_child(build_extension_attributes(performance_indicator.extension_attributes(), colors))
      .end();
    write_empty_line(w, i);
    _ = write!(w, "{:indent$}", node_performance_indicator);
  }
}

/// Write organization units.
fn write_organisation_units(w: &mut dyn Write, definitions: &Definitions, cm: ColorMode, colors: &Colors, indent: usize) {
  let organisation_units = definitions.organisation_units();
  if !organisation_units.is_empty() {
    write_title(w, LABEL_ORGANISATION_UNITS);
  }
  for (i, organisation_units) in definitions.organisation_units().iter().enumerate() {
    // prepare a node with decisions made
    let mut decisions_made_builder = node(DEFAULT_COLOR, cm).line().s(LABEL_DECISIONS_MADE).s(':').end();
    for decision_made in organisation_units.decisions_made() {
      decisions_made_builder.add_child(build_href(decision_made, cm, colors));
    }
    let node_decisions_made = decisions_made_builder.end();
    // prepare a node with decisions owned
    let mut decisions_made_builder = node(DEFAULT_COLOR, cm).line().s(LABEL_DECISIONS_OWNED).s(':').end();
    for decision_owned in organisation_units.decisions_owned() {
      decisions_made_builder.add_child(build_href(decision_owned, cm, colors));
    }
    let node_decisions_owned = decisions_made_builder.end();
    let node_organisation_unit = builder_name(organisation_units.name(), cm, colors)
      .child(build_feel_name(organisation_units.feel_name(), cm, colors))
      .opt_child(build_label(organisation_units.label(), cm, colors))
      .opt_child(build_id(organisation_units.opt_id(), cm, colors))
      .opt_child(build_description(organisation_units.description(), cm, colors))
      .opt_child(build_uri(organisation_units.uri(), cm, colors))
      .child(node_decisions_made)
      .child(node_decisions_owned)
      .opt_child(build_extension_elements(organisation_units.extension_elements(), colors))
      .opt_child(build_extension_attributes(organisation_units.extension_attributes(), colors))
      .end();
    write_empty_line(w, i);
    _ = write!(w, "{:indent$}", node_organisation_unit);
  }
}

/// Prepares a node builder containing a name as a root element.
fn builder_name(text: &str, cm: ColorMode, colors: &Colors) -> NodeBuilder {
  node(DEFAULT_COLOR, cm).line().color_256(colors.name()).s(text).reset().end()
}

/// Builds a leaf node containing a name.
fn build_name(text: &str, cm: ColorMode, colors: &Colors) -> TreeNode {
  build_labeled_text(LABEL_NAME, text, cm, colors.name())
}

/// Builds a leaf node containing a FEEL name.
fn build_feel_name(feel_name: &Name, cm: ColorMode, colors: &Colors) -> TreeNode {
  build_labeled_text(LABEL_FEEL_NAME, &feel_name.to_string(), cm, colors.name())
}

/// Builds a leaf node containing the value of the identifier.
fn build_id(id: Option<&String>, cm: ColorMode, colors: &Colors) -> Option<TreeNode> {
  build_opt_labeled_text(LABEL_ID, &id.cloned(), cm, colors.id())
}

/// Builds a leaf node containing description.
fn build_description(opt_text: &Option<String>, cm: ColorMode, colors: &Colors) -> Option<TreeNode> {
  build_opt_labeled_multiline_text(LABEL_DESCRIPTION, opt_text, cm, colors.description())
}

/// Builds a leaf node containing the value of the label.
fn build_label(opt_text: &Option<String>, cm: ColorMode, colors: &Colors) -> Option<TreeNode> {
  build_opt_labeled_text(LABEL_LABEL, opt_text, cm, colors.label())
}

/// Builds a leaf node containing a label.
fn build_href(href: &HRef, cm: ColorMode, colors: &Colors) -> TreeNode {
  let text = href.id();
  leaf(cm).line().color_256(colors.href()).s('#').s(text).reset().end().end()
}

/// Builds a leaf node containing URI.
fn build_uri(opt_text: &Option<String>, cm: ColorMode, colors: &Colors) -> Option<TreeNode> {
  build_opt_labeled_text(LABEL_URI, opt_text, cm, colors.uri())
}

/// Builds a leaf node containing a labeled URI.
fn build_labeled_uri(label: &str, opt_text: &Option<String>, cm: ColorMode, colors: &Colors) -> Option<TreeNode> {
  build_opt_labeled_text(label, opt_text, cm, colors.uri())
}

/// Builds a leaf node containing a namespace.
fn build_namespace_leaf(text: &str, cm: ColorMode, colors: &Colors) -> TreeNode {
  build_labeled_text(LABEL_NAMESPACE, text, cm, colors.uri())
}

/// Builds a leaf node containing a type.
fn build_type(text: &str, cm: ColorMode, colors: &Colors) -> TreeNode {
  build_labeled_text(LABEL_TYPE, text, cm, colors.typ())
}

/// Builds a node containing output variable properties.
fn build_variable(variable: &InformationItem, cm: ColorMode, colors: &Colors) -> TreeNode {
  node(DEFAULT_COLOR, cm)
    .line()
    .s(LABEL_VARIABLE)
    .end()
    .child(build_name(variable.name(), cm, colors))
    .child(build_feel_name(variable.feel_name(), cm, colors))
    .opt_child(build_id(variable.opt_id(), cm, colors))
    .opt_child(build_label(variable.label(), cm, colors))
    .child(build_type(variable.type_ref(), cm, colors))
    .opt_child(build_description(variable.description(), cm, colors))
    .opt_child(build_extension_elements(variable.extension_elements(), colors))
    .opt_child(build_extension_attributes(variable.extension_attributes(), colors))
    .end()
}

fn build_extension_elements(_extension_elements: &[ExtensionElement], _colors: &Colors) -> Option<TreeNode> {
  None
}

fn build_extension_attributes(_extension_attributes: &[ExtensionAttribute], _colors: &Colors) -> Option<TreeNode> {
  None
}

fn build_authority_requirements(authority_requirements: &Vec<AuthorityRequirement>, cm: ColorMode, colors: &Colors) -> Option<TreeNode> {
  if !authority_requirements.is_empty() {
    let mut authority_requirements_builder = node(DEFAULT_COLOR, cm).line().s("authority requirements:").end();
    for authority_requirement in authority_requirements {
      let node_authority = node(DEFAULT_COLOR, cm)
        .line()
        .s("authority requirement")
        .end()
        .opt_child(build_id(authority_requirement.opt_id(), cm, colors))
        .opt_child(build_label(authority_requirement.label(), cm, colors))
        .opt_child(build_description(authority_requirement.description(), cm, colors))
        .opt_child(build_extension_elements(authority_requirement.extension_elements(), colors))
        .opt_child(build_extension_attributes(authority_requirement.extension_attributes(), colors))
        .end();
      authority_requirements_builder.add_child(node_authority);
    }
    return Some(authority_requirements_builder.end());
  }
  None
}

/// Builds a leaf with a single line containing label and coloured value.
fn build_labeled_text(label: &str, text: &str, cm: ColorMode, color: u8) -> TreeNode {
  leaf(cm).line().s(label).s(": ").color_256(color).s(text).reset().end().end()
}

/// Builds optional a leaf with a single line containing label and coloured value.
fn build_opt_labeled_text(label: &str, opt_text: &Option<String>, cm: ColorMode, color: u8) -> Option<TreeNode> {
  opt_text.as_ref().map(|text| leaf(cm).line().s(label).s(": ").color_256(color).s(text).reset().end().end())
}

/// Builds a node containing optional labeled multiline text.
fn build_opt_labeled_multiline_text(label: &str, text: &Option<String>, cm: ColorMode, color: u8) -> Option<TreeNode> {
  text.as_ref().map(|text| {
    // prepare the label of the node
    let label = Text::new(cm).s(label).s(':');
    // prepare the leaf node builder
    let mut leaf_builder = leaf(cm).line().s(label).end();
    // calculate the left margin of the multiline text
    let mut left_margin = if text.is_empty() { 0 } else { usize::MAX };
    for line in text.lines().filter(|s| !s.trim().is_empty()) {
      let leading_spaces = line.len() - line.trim_start().len();
      if leading_spaces < left_margin {
        left_margin = leading_spaces;
      }
    }
    // add multiple lines to the leaf node without the left margin
    for line in text.lines().filter(|s| !s.trim().is_empty()) {
      let indented_line = Text::new(cm).s("  ").color_256(color).s(line[left_margin..].trim_end()).reset();
      leaf_builder.add_line(indented_line);
    }
    // return the leaf node
    leaf_builder.end()
  })
}
