//! # Documentation generator for decision models

use crate::auto_size::AutoSize;
use crate::defs::*;
use crate::horizontal_decision_table::create_horizontal_decision_table_elements;
use crate::styles::{create_decision_table_style, create_document_style, create_model_style};
use domrs::{HtmlBodyElement, HtmlDocument, HtmlElement, HtmlHeadElement, HtmlStyleElement, h1};
use dsntk_model::*;
use std::collections::HashMap;
use std::fmt::Write;

/// Utility constant equal to 2×PI.
const PI_2: f64 = std::f64::consts::PI * 2.0;

/// Amplitude of the wave in knowledge source symbol.
const AMPLITUDE: f64 = 20.0;

/// Effective style of the diagram element.
#[derive(Clone)]
struct Style {
  fill_color: DcColor,
  stroke_color: DcColor,
  font_color: DcColor,
  font_family: String,
  font_size: u64,
  font_italic: bool,
  font_bold: bool,
  font_underline: bool,
  font_strike_through: bool,
  label_horizontal_alignment: DcAlignmentKind,
  label_vertical_alignment: DcAlignmentKind,
}

impl Style {
  /// Creates a new style with default values.
  fn new() -> Self {
    Self {
      fill_color: DcColor::white(),
      stroke_color: DcColor::black(),
      font_color: DcColor::black(),
      font_family: "Arial".to_string(),
      font_size: 8, // points
      font_italic: false,
      font_bold: false,
      font_underline: false,
      font_strike_through: false,
      label_horizontal_alignment: DcAlignmentKind::Center,
      label_vertical_alignment: DcAlignmentKind::Center,
    }
  }

  /// Applies style attributes from [DmnStyle].
  fn apply(&mut self, style: &DmnStyle) {
    if let Some(fill_color) = style.fill_color {
      self.fill_color = fill_color;
    }
    if let Some(stroke_color) = style.stroke_color {
      self.stroke_color = stroke_color;
    }
    if let Some(font_color) = style.font_color {
      self.font_color = font_color;
    }
    if let Some(font_family) = &style.font_family {
      self.font_family = font_family.to_string();
    }
    if let Some(font_size) = style.font_size {
      self.font_size = font_size.round() as u64;
    }
    if let Some(font_italic) = style.font_italic {
      self.font_italic = font_italic;
    }
    if let Some(font_bold) = style.font_bold {
      self.font_bold = font_bold;
    }
    if let Some(font_underline) = style.font_underline {
      self.font_underline = font_underline;
    }
    if let Some(font_strike_through) = style.font_strike_through {
      self.font_strike_through = font_strike_through;
    }
    if let Some(horizontal_alignment) = style.label_horizontal_alignment {
      self.label_horizontal_alignment = horizontal_alignment;
    }
    if let Some(vertical_alignment) = style.label_vertical_alignment {
      self.label_vertical_alignment = vertical_alignment;
    }
  }

  /// Returns style for all figures.
  fn svg_figure_style(&self) -> String {
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "fill:{};", Self::rgb(&self.fill_color));
    let _ = write!(&mut buffer, "stroke:{};", Self::rgb(&self.stroke_color));
    buffer
  }

  /// Returns style for span.
  fn svg_text_span_style(&self) -> String {
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "color:{};", Self::rgb(&self.font_color));
    let _ = write!(&mut buffer, "font-family:{};", self.font_family);
    let _ = write!(&mut buffer, "font-size:{}pt;", self.font_size);
    let _ = write!(
      &mut buffer,
      "vertical-align:{};",
      match self.label_vertical_alignment {
        DcAlignmentKind::Start => "top",
        DcAlignmentKind::Center => "middle",
        DcAlignmentKind::End => "bottom",
      }
    );
    if self.font_italic {
      let _ = write!(&mut buffer, "font-style:italic;");
    }
    if self.font_bold {
      let _ = write!(&mut buffer, "font-weight:bold;");
    }
    match (self.font_underline, self.font_strike_through) {
      (true, true) => {
        let _ = write!(&mut buffer, "text-decoration:underline line-through;");
      }
      (true, false) => {
        let _ = write!(&mut buffer, "text-decoration:underline;");
      }
      (false, true) => {
        let _ = write!(&mut buffer, "text-decoration:line-through;");
      }
      _ => {}
    }
    buffer
  }

  /// Returns a style for text.
  fn svg_text_div_style(&self) -> String {
    let mut buffer = String::new();
    let _ = write!(
      &mut buffer,
      "text-align:{};",
      match self.label_horizontal_alignment {
        DcAlignmentKind::Start => "left",
        DcAlignmentKind::Center => "center",
        DcAlignmentKind::End => "right",
      }
    );
    buffer
  }

  /// Returns RGB color definition.
  fn rgb(color: &DcColor) -> String {
    format!("rgb({},{},{})", color.red, color.green, color.blue)
  }
}

/// HTML documentation generator.
pub struct HTMLGenerator {
  diagram_shared_styles: HashMap<String, DmnStyle>,
}

impl HTMLGenerator {
  /// Creates a new HTML documentation generator.
  pub fn new() -> Self {
    Self {
      diagram_shared_styles: HashMap::new(),
    }
  }

  /// Generates HTML documentation for specified definitions.
  pub fn dmn_model_to_html(&mut self, definitions: &Definitions) -> String {
    // prepare shared styles
    if let Some(dmndi) = definitions.dmndi() {
      for dmn_style in &dmndi.styles {
        self.diagram_shared_styles.insert(dmn_style.id.clone(), dmn_style.clone());
      }
    }
    // prepare the document body
    let mut body = HtmlBodyElement::default();
    // add section with model title
    let document_title = definitions.name();
    body.add_child(h1!(document_title));
    body.add_child(create_description_in_container(definitions.description()));
    // add model diagrams
    body.add_child(HtmlElement::h2(HEADING_MODEL_DIAGRAMS));
    for html_element in self.create_model_diagrams(definitions) {
      body.add_child(html_element);
    }
    // add model elements
    body.add_child(HtmlElement::h2(HEADING_MODEL_ELEMENTS));
    for html_element in create_model_elements(definitions) {
      body.add_child(html_element);
    }
    //HtmlDocument::new(document_title, "en", &[DMN_MODEL_CSS, DECISION_TABLE_CSS], body).to_string()
    HtmlDocument::default()
      .default_doctype()
      .default_language()
      .default_namespace()
      .head(
        HtmlHeadElement::default()
          .default_charset()
          .title(document_title)
          .stylesheet(FONT_NORMAL)
          .stylesheet(FONT_CONDENSED)
          .stylesheet(FONT_MONO)
          .style(HtmlStyleElement::new(create_document_style() + create_model_style() + create_decision_table_style())),
      )
      .body(body)
      .to_string()
  }

  /// Generates HTML document for specified decision table.
  pub fn decision_table_to_html(&self, decision_table: &DecisionTable) -> String {
    let mut body = HtmlBodyElement::default();
    // add title
    let document_title = if let Some(information_item_name) = &decision_table.information_item_name() {
      information_item_name
    } else if let Some(output_label) = &decision_table.output_label() {
      output_label
    } else {
      "Decision Table"
    };
    body.add_child(HtmlElement::h1(document_title));
    // add decision table
    match &decision_table.preferred_orientation() {
      DecisionTableOrientation::RuleAsRow => {
        body.add_child(create_horizontal_decision_table_elements(decision_table));
      }
      DecisionTableOrientation::RuleAsColumn => {
        //TODO implement
      }
      DecisionTableOrientation::CrossTable => {
        //TODO implement
      }
    }
    HtmlDocument::default()
      .default_doctype()
      .default_language()
      .default_namespace()
      .head(
        HtmlHeadElement::default()
          .default_charset()
          .title(document_title)
          .stylesheet(FONT_NORMAL)
          .stylesheet(FONT_CONDENSED)
          .stylesheet(FONT_MONO)
          .style(HtmlStyleElement::new(create_document_style() + create_decision_table_style())),
      )
      .body(body)
      .to_string()
  }

  /// Creates a collection of model diagrams.
  fn create_model_diagrams(&self, definitions: &Definitions) -> Vec<HtmlElement> {
    let mut html_elements = vec![]; // collection of sections, each section contains singe diagram
    if let Some(dmndi) = definitions.dmndi() {
      for diagram in &dmndi.diagrams {
        let mut size = AutoSize::new();
        let mut diagram_style = Style::new();
        if let Some(diagram_shared_style_id) = &diagram.shared_style
          && let Some(diagram_shared_style) = self.diagram_shared_styles.get(diagram_shared_style_id)
        {
          diagram_style.apply(diagram_shared_style);
        }
        if let Some(diagram_local_style) = &diagram.local_style {
          diagram_style.apply(diagram_local_style);
        }
        let mut html_svg_content = vec![];
        for diagram_element in &diagram.diagram_elements {
          match diagram_element {
            DmnDiagramElement::DmnShape(shape) => {
              size.discover_from_shape(shape);
              if let Some(dmn_element_ref) = &shape.dmn_element_ref {
                if let Some(decision) = definitions.get_decision(dmn_element_ref.as_str()) {
                  html_svg_content.push(self.create_svg_decision(diagram_style.clone(), shape, decision));
                } else if let Some(input_data) = definitions.get_input_data(dmn_element_ref.as_str()) {
                  html_svg_content.push(self.create_svg_input_data(diagram_style.clone(), shape, input_data));
                } else if let Some(business_knowledge_model) = definitions.get_business_knowledge_model(dmn_element_ref.as_str()) {
                  html_svg_content.push(self.create_svg_business_knowledge_model(diagram_style.clone(), shape, business_knowledge_model));
                } else if let Some(knowledge_source) = definitions.get_knowledge_source(dmn_element_ref.as_str()) {
                  html_svg_content.push(self.create_svg_knowledge_source(diagram_style.clone(), shape, knowledge_source));
                }
              }
            }
            DmnDiagramElement::DmnEdge(edge) => {
              size.discover_from_edge(edge);
              if let Some(id) = &edge.dmn_element_ref
                && let Some(requirement) = definitions.get_requirement(id)
              {
                match requirement {
                  Requirement::Information(_) => {
                    html_svg_content.push(create_svg_edge_solid_with_black_arrow(&edge.way_points));
                  }
                  Requirement::Knowledge(_) => {
                    html_svg_content.push(create_svg_edge_dashed_with_thin_arrow(&edge.way_points));
                  }
                  Requirement::Authority(_) => {
                    html_svg_content.push(create_svg_edge_dashed_with_end_point(&edge.way_points));
                  }
                }
              }
            }
          }
        }
        let diagram_name = diagram.name.as_ref().unwrap_or(&"".to_string()).clone();
        html_elements.push(create_svg_tag(&diagram_name, size.dimension(diagram), html_svg_content));
      }
    }
    html_elements
  }

  /// Creates SVG shape representing a decision.
  fn create_svg_decision(&self, mut style: Style, shape: &DmnShape, decision: &Decision) -> HtmlElement {
    self.apply_shape_style(&mut style, shape);
    let mut rect = HtmlElement::new("rect");
    rect.set_attribute("x", shape.bounds.x.to_string());
    rect.set_attribute("y", shape.bounds.y);
    rect.set_attribute("width", shape.bounds.width);
    rect.set_attribute("height", shape.bounds.height);
    rect.set_attribute("style", style.svg_figure_style());
    let text = create_svg_text(&shape.bounds, &style, &get_label_text(shape, decision.name()));
    create_svg_group(vec![rect, text])
  }

  /// Creates SVG shape representing an input data.
  fn create_svg_input_data(&self, mut style: Style, shape: &DmnShape, input_data: &InputData) -> HtmlElement {
    self.apply_shape_style(&mut style, shape);
    let radius = shape.bounds.height / 2.0;
    let mut rect = HtmlElement::new("rect");
    rect.set_attribute("x", shape.bounds.x);
    rect.set_attribute("y", shape.bounds.y);
    rect.set_attribute("width", shape.bounds.width);
    rect.set_attribute("height", shape.bounds.height);
    rect.set_attribute("rx", radius);
    rect.set_attribute("ry", radius);
    rect.set_attribute("style", style.svg_figure_style());
    let text = create_svg_text(&shape.bounds, &style, &get_label_text(shape, input_data.name()));
    create_svg_group(vec![rect, text])
  }

  /// Creates SVG shape representing a business knowledge model.
  fn create_svg_business_knowledge_model(&self, mut style: Style, shape: &DmnShape, bkm: &BusinessKnowledgeModel) -> HtmlElement {
    self.apply_shape_style(&mut style, shape);
    let (x, y, w, h) = (shape.bounds.x, shape.bounds.y, shape.bounds.width, shape.bounds.height);
    let points = format!(
      "{},{} {},{} {},{} {},{} {},{} {},{}",
      x,
      y + 15.0,
      x + 15.0,
      y,
      x + w,
      y,
      x + w,
      y + h - 15.0,
      x + w - 15.0,
      y + h,
      x,
      y + h
    );
    let mut polygon = HtmlElement::new("polygon");
    polygon.set_attribute("points", points);
    polygon.set_attribute("style", style.svg_figure_style());
    let text = create_svg_text(&shape.bounds, &style, &get_label_text(shape, bkm.name()));
    create_svg_group(vec![polygon, text])
  }

  /// Creates SVG shape representing a knowledge source.
  fn create_svg_knowledge_source(&self, mut style: Style, shape: &DmnShape, knowledge_source: &KnowledgeSource) -> HtmlElement {
    self.apply_shape_style(&mut style, shape);
    let mut path = HtmlElement::new("path");
    path.set_attribute("d", build_knowledge_source_path(&shape.bounds));
    path.set_attribute("style", style.svg_figure_style());
    let bounds = DcBounds {
      height: shape.bounds.height - (AMPLITUDE / 2.0) - 5.0,
      ..shape.bounds
    };
    let svg_text = create_svg_text(&bounds, &style, &get_label_text(shape, knowledge_source.name()));
    create_svg_group(vec![path, svg_text])
  }

  /// Applies style attributes from [DmnShape].
  fn apply_shape_style(&self, style: &mut Style, shape: &DmnShape) {
    if let Some(shape_shared_style_id) = &shape.shared_style
      && let Some(shape_shared_style) = self.diagram_shared_styles.get(shape_shared_style_id)
    {
      style.apply(shape_shared_style);
    }
    if let Some(shape_local_style) = &shape.local_style {
      style.apply(shape_local_style);
    }
  }
}

/// Creates a collection of model elements.
fn create_model_elements(definitions: &Definitions) -> Vec<HtmlElement> {
  let mut html_elements = vec![];
  for decision_service in &definitions.decision_services() {
    html_elements.push(create_model_element_decision_service(decision_service));
  }
  for decision in &definitions.decisions() {
    html_elements.push(create_model_element_decision(decision));
  }
  for bkm in &definitions.business_knowledge_models() {
    html_elements.push(create_model_element_business_knowledge_model(bkm));
  }
  for knowledge_source in &definitions.knowledge_sources() {
    html_elements.push(create_model_element_knowledge_source(knowledge_source));
  }
  for input_data in &definitions.input_data() {
    html_elements.push(create_model_element_input_data(input_data));
  }
  html_elements
}

/// Creates model element containing decision service details.
fn create_model_element_decision_service(decision_service: &DecisionService) -> HtmlElement {
  let mut element_name = HtmlElement::div().class(CLASS_MODEL_ELEMENT_NAME);
  element_name.set_content(decision_service.name());
  let mut element_type = HtmlElement::div().class(CLASS_MODEL_ELEMENT_TYPE);
  element_type.set_content("(Decision Service)");
  let variable_details = create_variable_details(decision_service.variable(), HEADING_OUTPUT_DATA);
  //
  let mut model_element_container = HtmlElement::div().class(CLASS_MODEL_ELEMENT_CONTAINER);
  model_element_container.add_children(&[element_name, element_type, variable_details]);
  model_element_container
}

/// Creates model element containing decision details.
fn create_model_element_decision(decision: &Decision) -> HtmlElement {
  // prepare the container for the details
  let mut model_element_container = HtmlElement::div().class(CLASS_MODEL_ELEMENT_CONTAINER);
  // prepare the name of the element
  let mut element_name = HtmlElement::div().class(CLASS_MODEL_ELEMENT_NAME);
  element_name.set_content(decision.name());
  model_element_container.add_child(element_name);
  // prepare the type of the element
  let mut element_type = HtmlElement::div().class(CLASS_MODEL_ELEMENT_TYPE);
  element_type.set_content("(Decision)");
  model_element_container.add_child(element_type);
  // prepare description for the decision when provided
  model_element_container.add_child(create_description_in_container(decision.description()));
  // prepare output variable details
  let variable_details = create_variable_details(decision.variable(), HEADING_OUTPUT_DATA);
  model_element_container.add_child(variable_details);
  // prepare the decision logic details
  model_element_container.add_opt_child(create_model_expression_instance(decision.decision_logic()));
  // return the container with filled up content
  model_element_container
}

/// Creates model element containing business knowledge model details.
fn create_model_element_business_knowledge_model(bkm: &BusinessKnowledgeModel) -> HtmlElement {
  let mut element_name = HtmlElement::div().class(CLASS_MODEL_ELEMENT_NAME);
  element_name.set_content(bkm.name());
  let mut element_type = HtmlElement::div().class(CLASS_MODEL_ELEMENT_TYPE);
  element_type.set_content("(Business Knowledge Model)");
  let variable_details = create_variable_details(bkm.variable(), HEADING_OUTPUT_DATA);
  //
  let mut model_element_container = HtmlElement::div().class(CLASS_MODEL_ELEMENT_CONTAINER);
  model_element_container.add_children(&[element_name, element_type, variable_details]);
  model_element_container
}

/// Creates model element containing knowledge source details.
fn create_model_element_knowledge_source(knowledge_source: &KnowledgeSource) -> HtmlElement {
  let mut element_name = HtmlElement::div().class(CLASS_MODEL_ELEMENT_NAME);
  element_name.set_content(knowledge_source.name());
  let mut element_type = HtmlElement::div().class(CLASS_MODEL_ELEMENT_TYPE);
  element_type.set_content("(Knowledge Source)");
  let mut model_element_container = HtmlElement::div().class(CLASS_MODEL_ELEMENT_CONTAINER);
  model_element_container.add_children(&[element_name, element_type]);
  model_element_container
}

/// Creates model element containing input data details.
fn create_model_element_input_data(input_data: &InputData) -> HtmlElement {
  let mut element_name = HtmlElement::div().class(CLASS_MODEL_ELEMENT_NAME);
  element_name.set_content(input_data.name());
  let mut element_type = HtmlElement::div().class(CLASS_MODEL_ELEMENT_TYPE);
  element_type.set_content("(Input Data)");
  let variable_details = create_variable_details(input_data.variable(), HEADING_INPUT_DATA);
  //
  let mut model_element_container = HtmlElement::div().class(CLASS_MODEL_ELEMENT_CONTAINER);
  model_element_container.add_children(&[element_name, element_type, variable_details]);
  model_element_container
}

/// Creates element containing input/output variable details.
fn create_variable_details(variable: &InformationItem, heading: &str) -> HtmlElement {
  let mut variable_details_heading = HtmlElement::div().class("variable-details-heading");
  variable_details_heading.set_content(heading);

  let mut variable_details_properties = HtmlElement::div().class("variable-details-properties");

  if let Some(label) = variable.label() {
    let mut property_label = HtmlElement::div().class("variable-details-property-name");
    property_label.set_content("Label");
    let mut value_label = HtmlElement::div().class("variable-details-property-value");
    value_label.set_content(label);
    variable_details_properties.add_children(&[property_label, value_label]);
  }

  let mut property_name = HtmlElement::div().class("variable-details-property-name");
  property_name.set_content("Name");
  let mut value_name = HtmlElement::div().class("variable-details-property-value");
  value_name.set_content(variable.name());
  variable_details_properties.add_children(&[property_name, value_name]);

  if variable.description().is_some() {
    let mut property_description = HtmlElement::div().class("variable-details-property-name");
    property_description.set_content("Description");
    let mut value_description = HtmlElement::div().class("variable-details-property-value");
    value_description.add_child(create_description(variable.description()));
    variable_details_properties.add_children(&[property_description, value_description]);
  }

  let mut property_type = HtmlElement::div().class("variable-details-property-name");
  property_type.set_content("Type");
  let mut value_type = HtmlElement::div().class("variable-details-property-value-type");
  value_type.set_content(variable.type_ref());
  variable_details_properties.add_children(&[property_type, value_type]);

  let mut variable_details = HtmlElement::div().class("variable-details-container");
  variable_details.add_children(&[variable_details_heading, variable_details_properties]);
  variable_details
}

/// Creates element containing expression instance details.
fn create_model_expression_instance(opt_expression_instance: &Option<ExpressionInstance>) -> Option<HtmlElement> {
  if let Some(expression_instance) = opt_expression_instance {
    let mut container = HtmlElement::div().class(CLASS_EXPRESSION_INSTANCE_CONTAINER);
    match expression_instance {
      ExpressionInstance::Context(_) => {
        let mut variable_details_heading = HtmlElement::div().class("variable-details-heading");
        variable_details_heading.set_content("Decision Logic (Context)");
        container.add_child(variable_details_heading);
      }
      ExpressionInstance::DecisionTable(decision_table) => {
        let mut variable_details_heading = HtmlElement::div().class("variable-details-heading");
        variable_details_heading.set_content("Decision Logic (Decision Table)");
        container.add_child(variable_details_heading);
        container.add_child(create_horizontal_decision_table_elements(decision_table));
      }
      ExpressionInstance::FunctionDefinition(_) => {
        let mut variable_details_heading = HtmlElement::div().class("variable-details-heading");
        variable_details_heading.set_content("Decision Logic (Function Definition)");
        container.add_child(variable_details_heading);
      }
      ExpressionInstance::Invocation(_) => {
        let mut variable_details_heading = HtmlElement::div().class("variable-details-heading");
        variable_details_heading.set_content("Decision Logic (Invocation)");
        container.add_child(variable_details_heading);
      }
      ExpressionInstance::List(_) => {
        let mut variable_details_heading = HtmlElement::div().class("variable-details-heading");
        variable_details_heading.set_content("Decision Logic (List)");
        container.add_child(variable_details_heading);
      }
      ExpressionInstance::LiteralExpression(literal_expression) => {
        let mut variable_details_heading = HtmlElement::div().class("variable-details-heading");
        variable_details_heading.set_content("Decision Logic (Literal Expression)");
        container.add_child(variable_details_heading);
        let mut element_literal_expression = HtmlElement::div().class("literal-expression");
        let no_literal_expression = "(no literal expression specified)".to_string();
        let text = literal_expression.text().as_ref().unwrap_or(&no_literal_expression);
        element_literal_expression.set_content(text);
        container.add_child(element_literal_expression);
      }
      ExpressionInstance::Relation(_) => {
        let mut variable_details_heading = HtmlElement::div().class("variable-details-heading");
        variable_details_heading.set_content("Decision Logic (Relation)");
        container.add_child(variable_details_heading);
      }
      ExpressionInstance::Conditional(_) => {
        let mut variable_details_heading = HtmlElement::div().class("variable-details-heading");
        variable_details_heading.set_content("Decision Logic (Conditional)");
        container.add_child(variable_details_heading);
      }
      ExpressionInstance::Filter(_) => {
        let mut variable_details_heading = HtmlElement::div().class("variable-details-heading");
        variable_details_heading.set_content("Decision Logic (Filter)");
        container.add_child(variable_details_heading);
      }
      ExpressionInstance::For(_) => {
        let mut variable_details_heading = HtmlElement::div().class("variable-details-heading");
        variable_details_heading.set_content("Decision Logic (For)");
        container.add_child(variable_details_heading);
      }
      ExpressionInstance::Every(_) => {
        let mut variable_details_heading = HtmlElement::div().class("variable-details-heading");
        variable_details_heading.set_content("Decision Logic (Every)");
        container.add_child(variable_details_heading);
      }
      ExpressionInstance::Some(_) => {
        let mut variable_details_heading = HtmlElement::div().class("variable-details-heading");
        variable_details_heading.set_content("Decision Logic (Some)");
        container.add_child(variable_details_heading);
      }
    }
    return Some(container);
  }
  None
}

/// Creates a description element enclosed in description container.
fn create_description_in_container(optional_description: &Option<String>) -> HtmlElement {
  HtmlElement::div().class(CLASS_DESCRIPTION_CONTAINER).child(create_description(optional_description))
}

/// Creates a description element.
fn create_description(optional_description: &Option<String>) -> HtmlElement {
  if let Some(description) = optional_description {
    HtmlElement::div().class(CLASS_DESCRIPTION).content(&from_markdown(description))
  } else {
    HtmlElement::span()
  }
}

/// Creates SVG solid edge line with black-filled arrow at the end.
fn create_svg_edge_solid_with_black_arrow(way_points: &[DcPoint]) -> HtmlElement {
  // prepare line
  let points = way_points.iter().fold("".to_string(), |acc, w| format!("{}{},{} ", acc, w.x, w.y));
  let mut svg_edge = HtmlElement::new("polyline");
  svg_edge.set_attribute("points", points);
  svg_edge.set_attribute("stroke", "black");
  // prepare arrow
  let start_point = &way_points[way_points.len() - 2];
  let end_point = &way_points[way_points.len() - 1];
  let points = format!(
    "{},{} {},{} {},{}",
    end_point.x,
    end_point.y,
    end_point.x + 12.0,
    end_point.y - 4.0,
    end_point.x + 12.0,
    end_point.y + 4.0
  );
  let angle = get_angle(start_point, end_point);
  let rotate = format!("rotate({},{},{})", angle, end_point.x, end_point.y);
  let mut svg_arrow = HtmlElement::new("polygon");
  svg_arrow.set_attribute("points", points);
  svg_arrow.set_attribute("transform", rotate);
  svg_arrow.set_attribute("fill", "black");
  svg_arrow.set_attribute("stroke", "none");
  // return a group of arrow elements
  create_svg_group(vec![svg_edge, svg_arrow])
}

/// Creates SVG dashed edge line with thin arrow at the end.
fn create_svg_edge_dashed_with_thin_arrow(way_points: &[DcPoint]) -> HtmlElement {
  // prepare line
  let points = way_points.iter().fold("".to_string(), |acc, w| format!("{}{},{} ", acc, w.x, w.y));
  let mut svg_edge = HtmlElement::new("polyline");
  svg_edge.set_attribute("points", points);
  svg_edge.set_attribute("stroke-dasharray", "5 3");
  // prepare arrow
  let start_point = &way_points[way_points.len() - 2];
  let end_point = &way_points[way_points.len() - 1];
  let path = format!(
    "M {},{} l {},{} M {},{} l {}, {}",
    end_point.x, end_point.y, 12.0, -4.0, end_point.x, end_point.y, 12.0, 4.0
  );
  let angle = get_angle(start_point, end_point);
  let rotate = format!("rotate({},{},{})", angle, end_point.x, end_point.y);
  let mut svg_arrow = HtmlElement::new("path");
  svg_arrow.set_attribute("d", path);
  svg_arrow.set_attribute("transform", rotate);
  svg_arrow.set_attribute("fill", "none");
  svg_arrow.set_attribute("stroke", "black");
  // return a group of arrow elements
  create_svg_group(vec![svg_edge, svg_arrow])
}

/// Creates SVG dashed edge line with black end-point at the end.
fn create_svg_edge_dashed_with_end_point(way_points: &[DcPoint]) -> HtmlElement {
  // prepare line
  let points = way_points.iter().fold("".to_string(), |acc, w| format!("{}{},{} ", acc, w.x, w.y));
  let mut svg_edge = HtmlElement::new("polyline");
  svg_edge.set_attribute("points", points);
  svg_edge.set_attribute("stroke-dasharray", "5 3");
  // prepare arrow (ending point)
  let end_point = &way_points[way_points.len() - 1];
  let mut svg_arrow = HtmlElement::new("circle");
  svg_arrow.set_attribute("cx", end_point.x);
  svg_arrow.set_attribute("cy", end_point.y);
  svg_arrow.set_attribute("r", "4");
  svg_arrow.set_attribute("fill", "black");
  svg_arrow.set_attribute("stroke", "none");
  // return a group of arrow elements
  create_svg_group(vec![svg_edge, svg_arrow])
}

/// Prepares SVG object containing multiline text.
///
/// Text given in argument `text` is placed in the following construct:
///
/// ```text
/// <foreignObject>
///   <div>
///     <span>...text...</span>
///   </div>
/// </foreignObject>
/// ```
/// The `div` and `span` elements have such styles set, that the content is displayed as a table cell.
/// Text in a table cell is centered horizontally and aligned vertically in the middle.
///
fn create_svg_text(bounds: &DcBounds, style: &Style, text: &str) -> HtmlElement {
  // create span with content
  let mut span = HtmlElement::new("span");
  span.set_content(text);
  span.set_attribute("style", format!("display:table-cell;{}", style.svg_text_span_style()));
  // create div for above created span
  let mut div = HtmlElement::new("div");
  div.set_attribute("style", format!("display:table;height:100%;width:100%;{}color:blue;", style.svg_text_div_style()));
  div.add_child(span);
  // create foreignObject
  let mut foreign_object = HtmlElement::new("foreignObject");
  foreign_object.set_attribute("x", bounds.x + 4.0);
  foreign_object.set_attribute("y", bounds.y);
  foreign_object.set_attribute("width", bounds.width - 8.0);
  foreign_object.set_attribute("height", bounds.height - 2.0);
  foreign_object.add_child(div);
  foreign_object
}

/// Returns the text of the label associated with the shape,
/// when no label is present then the specified name is returned.
fn get_label_text(shape: &DmnShape, name: &str) -> String {
  if let Some(label) = &shape.label
    && let Some(label_text) = &label.text
  {
    return label_text.to_string();
  }
  name.to_string()
}

/// Returns the rotation angle of an arrow.
fn get_angle(start: &DcPoint, end: &DcPoint) -> f64 {
  let x = end.x - start.x;
  let y = end.y - start.y;
  if x == 0.0 {
    return if y >= 0.0 { -90.0 } else { 90.0 };
  }
  let angle = ((y / x).atan() * 360.0) / PI_2;
  if x > 0.0 { if y >= 0.0 { angle - 180.0 } else { angle + 180.0 } } else { angle }
}

/// Returns the path for drawing knowledge source symbol.
fn build_knowledge_source_path(bounds: &DcBounds) -> String {
  let period_div_2 = AMPLITUDE / 2.0;
  let curve_base_height = bounds.y + bounds.height - period_div_2;
  let width_div_4: f64 = bounds.width / 4.0;
  let width_div_2: f64 = bounds.width / 2.0;

  let mut path = format!("M {} {}", bounds.x, bounds.y);
  path = format!("{} L {} {}", path, bounds.x + bounds.width, bounds.y);
  path = format!("{} L {} {}", path, bounds.x + bounds.width, curve_base_height);
  path = format!(
    "{} C {},{} {},{} {},{}",
    path,
    bounds.x + bounds.width,
    curve_base_height,
    bounds.x + bounds.width - width_div_4,
    curve_base_height - AMPLITUDE,
    bounds.x + bounds.width - width_div_2,
    curve_base_height
  );
  path = format!(
    "{} C {},{} {},{} {},{}",
    path,
    bounds.x + bounds.width - width_div_2,
    curve_base_height,
    bounds.x + width_div_4,
    curve_base_height + AMPLITUDE,
    bounds.x,
    curve_base_height
  );
  path = format!("{} L {} {} Z", path, bounds.x, bounds.y);
  path
}

/// Creates `svg` tag with specified dimension and content.
pub fn create_svg_tag(title: &str, dimension: DcDimension, elements: Vec<HtmlElement>) -> HtmlElement {
  let mut svg = HtmlElement::new("svg");
  let width = dimension.width.ceil();
  let height = dimension.height.ceil();
  svg.set_attribute("viewBox", format!("0 0 {width} {height}"));
  svg.set_attribute("width", format!("{}", width));
  for element in elements {
    svg.add_child(element);
  }
  let mut diagram_title = HtmlElement::div().class(CLASS_DIAGRAM_TITLE);
  diagram_title.set_content(title);
  let mut diagram_container = HtmlElement::div().class(CLASS_DIAGRAM_CONTAINER);
  diagram_container.add_child(diagram_title);
  diagram_container.add_child(svg);
  diagram_container
}

/// Creates `g` tag containing specified elements.
fn create_svg_group(elements: Vec<HtmlElement>) -> HtmlElement {
  let mut group = HtmlElement::new("g");
  for element in elements {
    group.add_child(element);
  }
  group
}

/// Converts markdown content into HTML content.
fn from_markdown(input: &str) -> String {
  let trimmed_input = input.lines().map(|line| line.trim().to_string()).collect::<Vec<String>>().join("\n");
  markdown::to_html(&trimmed_input)
    .trim()
    .replace("&lt;", "<")
    .replace("&gt;", ">")
    .replace("&amp;", "&")
    .replace("&quot;", "\"")
    .replace("&apos;", "\'")
}
