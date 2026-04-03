//! # Recognizer

#![doc = include_str!("../docs/algorithm.md")]

use crate::HitPolicy;
use crate::canvas::Canvas;
use crate::errors::*;
use crate::model::DecisionTableOrientation;
use crate::plane::{HitPolicyPlacement, Plane, RuleNumbersPlacement};
use dsntk_common::Result;

/// Decision table recognizer.
#[derive(Debug)]
pub struct Recognizer {
  /// Plane used during recognition process.
  pub plane: Plane,
  /// Optional information item name.
  pub information_item_name: Option<String>,
  /// Placement of the hit policy.
  pub hit_policy_placement: HitPolicyPlacement,
  /// Detected hit policy.
  pub hit_policy: HitPolicy,
  /// Placement of rule numbers.
  pub rule_numbers_placement: RuleNumbersPlacement,
  /// Detected table orientation.
  pub orientation: DecisionTableOrientation,
  /// Number of recognized input clauses.
  pub input_clause_count: usize,
  /// List of input expressions.
  pub input_expressions: Vec<String>,
  /// List of allowed input values.
  pub allowed_input_values: Vec<Option<String>>,
  /// Matrix of input entries.
  pub input_entries: Vec<Vec<String>>,
  /// Number of recognized output clauses.
  pub output_clause_count: usize,
  /// Detected output label.
  pub output_label: Option<String>,
  /// List of output component names.
  pub output_components: Vec<Option<String>>,
  /// List of allowed output values.
  pub allowed_output_values: Vec<Option<String>>,
  /// Matrix of output entries.
  pub output_entries: Vec<Vec<String>>,
  /// Number of recognized annotation clauses.
  pub annotation_clause_count: usize,
  /// List of annotations.
  pub annotations: Vec<String>,
  /// Matrix of annotation entries.
  pub annotation_entries: Vec<Vec<String>>,
  /// Number of recognized rules.
  pub rule_count: usize,
}

impl Recognizer {
  /// Recognizes the decision table defined as plain Unicode text.
  pub fn recognize(text: &str, trace: bool) -> Result<Recognizer> {
    let mut canvas = Canvas::scan(text)?;
    if trace {
      canvas.display_text_layer();
      canvas.display_thin_layer();
      canvas.display_body_layer();
      canvas.display_grid_layer();
    }
    let information_item_name = canvas.information_item_name.clone();
    let plane = canvas.plane()?;
    if trace {
      println!("PLANE\n{plane}");
    }
    let mut recognizer = Recognizer {
      plane,
      information_item_name,
      hit_policy_placement: HitPolicyPlacement::NotPresent,
      hit_policy: HitPolicy::Unique,
      rule_numbers_placement: RuleNumbersPlacement::NotPresent,
      orientation: DecisionTableOrientation::CrossTable,
      input_clause_count: 0,
      input_expressions: vec![],
      allowed_input_values: vec![],
      input_entries: vec![],
      output_clause_count: 0,
      output_label: None,
      output_components: vec![],
      allowed_output_values: vec![],
      output_entries: vec![],
      annotation_clause_count: 0,
      annotations: vec![],
      annotation_entries: vec![],
      rule_count: 0,
    };
    recognizer.recognize_table_components()?;
    if trace {
      recognizer.trace();
    }
    Ok(recognizer)
  }

  /// Recognizes the decision table components based on table orientation.
  pub fn recognize_table_components(&mut self) -> Result<()> {
    self.recognize_orientation()?;
    match self.orientation {
      DecisionTableOrientation::RulesAsRows => {
        self.plane.remove_first_column();
        self.recognize_horizontal_table()?;
      }
      DecisionTableOrientation::RulesAsColumns => {
        self.plane.remove_last_row();
        self.plane.pivot();
        self.recognize_horizontal_table()?;
      }
      DecisionTableOrientation::CrossTable => {
        self.recognize_crosstab_table()?;
      }
    }
    Ok(())
  }

  /// Recognizes decision table components from horizontally oriented plane.
  /// Vertical decision tables are pivoted horizontal decision tables.
  fn recognize_horizontal_table(&mut self) -> Result<()> {
    // Retrieve the rectangle of the input clause region.
    let r_in = self.plane.horz_input_clause_rect()?;

    // Retrieve the number of recognized input clauses.
    self.input_clause_count = r_in.width();

    // Retrieve the rectangle of the output clause region.
    let r_out = self.plane.horz_output_clause_rect()?;

    // Retrieve the number of recognized output clauses.
    self.output_clause_count = r_out.width();

    // Detect if the allowed input and/or allowed output values are present.
    let allowed_values_present = match r_in.height() {
      1 => {
        // By a single row, there are no allowed values present, only input expressions.
        false
      }
      2 => {
        // By two rows, the logic is more complex.
        let equal_input_regions = self.plane.equal_regions_in_columns(&r_in)?;
        let inputs_present = self.input_clause_count > 0;
        let multiple_outputs = self.output_clause_count > 1;
        let equal_output_regions = self.plane.equal_regions_in_columns(&r_out)?;
        let unique_output_regions = self.plane.unique_regions(&r_out)?;
        matches!(
          (equal_input_regions, inputs_present, multiple_outputs, equal_output_regions, unique_output_regions),
          (false, _, _, _, _) | (true, false, true, true, _) | (true, false, true, false, true)
        )
      }
      3 => {
        // By three rows, allowed input or allowed output values are always provided.
        // Just check if the two bottom rows contain the same regions: input and output expressions.
        if !self.plane.unique_regions_in_columns(&r_in.offset_top(1))? {
          return Err(err_invalid_input_expressions());
        }
        if !self.plane.unique_regions_in_columns(&r_out.offset_top(1))? {
          return Err(err_invalid_output_expressions());
        }
        true
      }
      _ => {
        // There are to many rows in the input clause (above the double line), report an error.
        return Err(err_too_many_rows_in_input_clause());
      }
    };

    // Retrieve the input expressions from the plane.
    for col in r_in.left..r_in.right {
      self.input_expressions.push(self.plane.region_text(0, col)?);
    }

    // retrieve allowed input values (when present) from the plane
    if allowed_values_present {
      for col in r_in.left..r_in.right {
        self.allowed_input_values.push(self.opt_text(self.plane.region_text(r_in.bottom - 1, col)?));
      }
    }

    // retrieve input entries from the plane
    let r = self.plane.horz_input_entries_rect()?;
    for row in r.top..r.bottom {
      self.input_entries.push(vec![]);
      for col in r.left..r.right {
        self.input_entries.last_mut().unwrap().push(self.plane.region_text(row, col)?);
      }
    }

    // process output clause
    match r_out.width() {
      0 => {
        // if no output columns are present then report an error
        return Err(err_no_output_clause());
      }
      1 => {
        // single output
        match r_out.height() {
          1 => {
            // only output label is present
            self.output_label = self.opt_text(self.plane.region_text(r_out.top, r_out.left)?);
          }
          2 => {
            if !self.plane.equal_regions(&r_out)? {
              // output label is present
              self.output_label = self.opt_text(self.plane.region_text(r_out.top, r_out.left)?);
            } else {
              // output label is not present
              self.output_label = None;
            }
            self.allowed_output_values.push(self.opt_text(self.plane.region_text(r_out.top + 1, r_out.left)?))
          }
          _ => return Err(err_too_many_rows_in_output_clause()),
        }
      }
      _ => {
        // multiple outputs
        match r_out.height() {
          1 => {
            // only component names are present
            for col in r_out.left..r_out.right {
              self.output_components.push(self.opt_text(self.plane.region_text(r_out.top, col)?));
            }
          }
          2 => {
            println!("DDD: allowed_values_present: {}", allowed_values_present);

            if allowed_values_present {
              // component names and output values
              for col in r_out.left..r_out.right {
                self.output_components.push(self.opt_text(self.plane.region_text(r_out.top, col)?));
              }
              for col in r_out.left..r_out.right {
                self.allowed_output_values.push(self.opt_text(self.plane.region_text(r_out.top + 1, col)?));
              }
            } else {
              // output label is present and component names are present
              self.output_label = self.opt_text(self.plane.region_text(r_out.top, r_out.left)?);
              for col in r_out.left..r_out.right {
                self.output_components.push(self.opt_text(self.plane.region_text(r_out.top + 1, col)?));
              }
            }
          }
          3 => {
            // output label, component names and output values
            self.output_label = self.opt_text(self.plane.region_text(r_out.top, r_out.left)?);
            for col in r_out.left..r_out.right {
              self.output_components.push(self.opt_text(self.plane.region_text(r_out.top + 1, col)?));
            }
            for col in r_out.left..r_out.right {
              self.allowed_output_values.push(self.opt_text(self.plane.region_text(r_out.top + 2, col)?));
            }
          }
          _ => return Err(err_too_many_rows_in_output_clause()),
        }
      }
    }

    // retrieve output entries
    let r = self.plane.horz_output_entries_rect()?;
    for row in r.top..r.bottom {
      self.output_entries.push(vec![]);
      for col in r.left..r.right {
        self.output_entries.last_mut().unwrap().push(self.plane.region_text(row, col)?);
      }
    }

    // retrieve annotation clauses
    let r = self.plane.horz_annotation_clauses_rect()?;

    // assign the number of recognized annotation clauses
    self.annotation_clause_count = r.width();
    for col in r.left..r.right {
      self.annotations.push(self.plane.region_text(r.top, col)?);
    }

    // retrieve annotation entries
    let r = self.plane.horz_annotation_entries_rect()?;
    for row in r.top..r.bottom {
      self.annotation_entries.push(vec![]);
      for col in r.left..r.right {
        self.annotation_entries.last_mut().unwrap().push(self.plane.region_text(row, col)?);
      }
    }
    Ok(())
  }

  /// Returns text when not empty, otherwise returns [None].
  fn opt_text(&self, text: String) -> Option<String> {
    let s = text.trim().to_string();
    if s.is_empty() { None } else { Some(s) }
  }

  /// Recognizes decision table components from crosstab oriented plane.
  fn recognize_crosstab_table(&mut self) -> Result<()> {
    // TODO implement crosstab recognition
    self.rule_count = 0; // TODO properly recognize the total number of rules!
    Err(err_recognizing_cross_tab_not_supported_yet())
  }

  /// Recognizes the orientation of decision table.
  fn recognize_orientation(&mut self) -> Result<()> {
    self.hit_policy_placement = self.plane.recognize_hit_policy_placement()?;
    self.rule_numbers_placement = self.plane.recognize_rule_numbers_placement()?;
    if self.plane.horizontal_double_crossing().is_some() {
      // horizontal orientation
      if self.hit_policy_placement.is_top_left() {
        if self.rule_numbers_placement.is_left_below() {
          self.hit_policy = self.hit_policy_placement.hit_policy();
          self.orientation = DecisionTableOrientation::RulesAsRows;
          self.rule_count = self.rule_numbers_placement.rule_count();
          Ok(())
        } else {
          Err(err_expected_left_below_rule_numbers_placement())
        }
      } else {
        Err(err_expected_top_left_hit_policy_placement())
      }
    } else if self.plane.vertical_double_crossing().is_some() {
      // vertical orientation
      if self.hit_policy_placement.is_bottom_left() {
        if self.rule_numbers_placement.is_right_after() {
          self.hit_policy = self.hit_policy_placement.hit_policy();
          self.orientation = DecisionTableOrientation::RulesAsColumns;
          self.rule_count = self.rule_numbers_placement.rule_count();
          Ok(())
        } else {
          Err(err_expected_right_after_rule_numbers_placement())
        }
      } else {
        Err(err_expected_bottom_left_hit_policy_placement())
      }
    } else {
      // detect the orientation
      match self.hit_policy_placement {
        HitPolicyPlacement::TopLeft(_) => {
          if self.rule_numbers_placement.is_left_below() {
            self.hit_policy = self.hit_policy_placement.hit_policy();
            self.orientation = DecisionTableOrientation::RulesAsRows;
            self.rule_count = self.rule_numbers_placement.rule_count();
            Ok(())
          } else {
            Err(err_expected_left_below_rule_numbers_placement())
          }
        }
        HitPolicyPlacement::BottomLeft(_) => {
          if self.rule_numbers_placement.is_right_after() {
            self.hit_policy = self.hit_policy_placement.hit_policy();
            self.orientation = DecisionTableOrientation::RulesAsColumns;
            self.rule_count = self.rule_numbers_placement.rule_count();
            Ok(())
          } else {
            Err(err_expected_right_after_rule_numbers_placement())
          }
        }
        HitPolicyPlacement::NotPresent => {
          if self.rule_numbers_placement.is_not_present() {
            self.hit_policy = self.hit_policy_placement.hit_policy();
            self.orientation = DecisionTableOrientation::CrossTable;
            self.rule_count = 0; // will be recognized later
            Ok(())
          } else {
            Err(err_expected_no_rule_numbers_present())
          }
        }
      }
    }
  }

  /// Prints to standard output the result of decision table recognition.
  pub fn trace(&self) {
    print!("\n>> input expressions:\n|");
    for text in &self.input_expressions {
      self.trace_line(text);
    }
    print!("\n\n>> allowed input values:\n|");
    for opt_text in &self.allowed_input_values {
      if let Some(text) = opt_text {
        self.trace_line(text);
      } else {
        self.trace_line("");
      }
    }
    print!("\n\n>> input entries:\n");
    for row in &self.input_entries {
      print!("|");
      for text in row {
        self.trace_line(text);
      }
      println!()
    }
    print!("\n>> output label:\n|");
    if let Some(text) = &self.output_label {
      self.trace_line(text);
    }
    print!("\n\n>> output components:\n|");
    for opt_text in &self.output_components {
      if let Some(text) = opt_text {
        self.trace_line(text);
      } else {
        self.trace_line("");
      }
    }
    print!("\n\n>> allowed output values:\n|");
    for opt_text in &self.allowed_output_values {
      if let Some(text) = opt_text {
        self.trace_line(text);
      } else {
        self.trace_line("");
      }
    }
    print!("\n\n>> output entries:\n");
    for row in &self.output_entries {
      print!("|");
      for text in row {
        self.trace_line(text);
      }
      println!()
    }
    print!("\n\n>> annotations:\n|");
    for text in &self.annotations {
      self.trace_line(text);
    }
    print!("\n\n>> annotation entries:\n");
    for row in &self.annotation_entries {
      print!("|");
      for text in row {
        self.trace_line(text);
      }
      println!()
    }
  }

  /// Displays a single tracing line.
  fn trace_line(&self, text: &str) {
    print!("{}|", text.replace('\n', " ").trim());
  }
}
