//! # Markdown decision table recognizer

use crate::errors::{err_md_invalid_decision_table, err_md_invalid_number_of_column, err_md_no_decision_table, err_md_no_hit_policy};
use crate::utils::{EMPHASES, get_allowed_and_default_output_values};
use crate::{AnnotationClause, AnnotationEntry, DecisionRule, DecisionTable, DecisionTableOrientation, HitPolicy, InputClause, InputEntry, OutputClause, OutputEntry};
use dsntk_common::Result;

/// Character used in Markdown as a vertical line that forms table columns.
const TABLE_VERT_LINE: &str = "|";

/// Characters used to denote the information item name of the decision table.
const INFORMATION_ITEM_NAME_START: &str = "> #";

/// Optional characters in the information item name of the decision table after start.
const INFORMATION_ITEM_NAME_HEADING: &str = "#";

/// Characters used to denote the output label of the decision table.
const OUTPUT_LABEL_START: &str = ">";

/// Minimum number of rows in any decision table.
/// Do NOT change this.
const MIN_ROWS: usize = 2;

/// Minimum number of columns in every decision table.
/// Do NOT change this.
const MIN_COLUMNS: usize = 2;

type Table = Vec<Vec<Option<String>>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Marker {
  Input,
  Output,
  Annotation,
}

type Markers = Vec<Option<Marker>>;

/// Recognizes a decision table defined as Markdown text.
pub fn from_markdown(markdown: &str, trace: bool) -> Result<DecisionTable> {
  // Locate and retrieve lines containing the decision table from provided Markdown content.
  let (information_item_name, mut output_label, lines) = markdown_lines(markdown);
  // Convert lines into table.
  let table = markdown_table(lines)?;
  // Get the hit policy and aggregation.
  let hit_policy = get_hit_policy(&table)?;
  let aggregation = hit_policy.aggregation();
  // Get preferred orientation.
  let (preferred_orientation, table, empty_count, _rule_count) = get_preferred_orientation(table)?;
  if empty_count < 1 {
    match preferred_orientation {
      DecisionTableOrientation::RulesAsRows => return Err(err_md_invalid_decision_table("no markers row before the first rule")),
      DecisionTableOrientation::RulesAsColumns => return Err(err_md_invalid_decision_table("no markers column before the first rule")),
      DecisionTableOrientation::CrossTable => unreachable!(),
    }
  }
  if empty_count > 2 {
    match preferred_orientation {
      DecisionTableOrientation::RulesAsRows => return Err(err_md_invalid_decision_table("too many rows before the first rule")),
      DecisionTableOrientation::RulesAsColumns => return Err(err_md_invalid_decision_table("too many columns before the first rule")),
      DecisionTableOrientation::CrossTable => unreachable!(),
    }
  }
  // Prepare a flag indicating if allowed values are present in the decision table.
  let allowed_values = empty_count == 2;
  // Get the input/output/annotation markers.
  let markers = get_markers(table[empty_count].iter())?;
  let output_count = markers.iter().filter(|marker| marker.is_some_and(|m| m == Marker::Output)).count();
  // Prepare the input, output and annotation clauses with optional allowed values.
  let mut input_clauses = vec![];
  let mut output_clauses = vec![];
  let mut annotation_clauses = vec![];
  for (index, marker) in markers.iter().enumerate() {
    match marker {
      Some(Marker::Input) => {
        let input_expression = table[0][index].as_ref().ok_or(err_md_invalid_decision_table("no input clause"))?.to_string();
        let allowed_input_values = if allowed_values { table[1][index].as_ref().map(|text| text.to_string()) } else { None };
        input_clauses.push(InputClause {
          input_expression,
          allowed_input_values,
        });
      }
      Some(Marker::Output) => {
        let name = table[0][index].clone();
        // If there is only one output with the name, then it is also the default output label.
        if output_count == 1 && output_label.is_none() {
          output_label = name.clone();
        }
        let (allowed_output_values, default_output_value) = if allowed_values {
          get_allowed_and_default_output_values(&table[1][index])
        } else {
          (None, None)
        };
        output_clauses.push(OutputClause {
          output_component_name: name,
          allowed_output_values,
          default_output_value,
        });
      }
      Some(Marker::Annotation) => {
        let name = table[0][index].as_ref().ok_or(err_md_invalid_decision_table("no annotation name"))?.to_string();
        annotation_clauses.push(AnnotationClause { name });
      }
      _ => {}
    }
  }
  // Prepare the rules.
  let mut rules = vec![];
  for row in table.iter().skip(empty_count + 1) {
    let mut input_entries = vec![];
    let mut output_entries = vec![];
    let mut annotation_entries = vec![];
    for (index, marker) in markers.iter().enumerate() {
      match marker {
        Some(Marker::Input) => input_entries.push(InputEntry {
          text: row[index].as_ref().ok_or(err_md_invalid_decision_table("no input entry"))?.to_string(),
        }),
        Some(Marker::Output) => output_entries.push(OutputEntry {
          text: row[index].as_ref().ok_or(err_md_invalid_decision_table("no output entry"))?.to_string(),
        }),
        Some(Marker::Annotation) => annotation_entries.push(AnnotationEntry {
          text: row[index].as_ref().unwrap_or(&"".to_string()).to_string(),
        }),
        _ => {}
      }
    }
    rules.push(DecisionRule {
      input_entries,
      output_entries,
      annotation_entries,
    });
  }
  // Display tracing report when requested.
  if trace {
    println!("Preferred orientation: {}", preferred_orientation);
    println!("Information item name: {}", information_item_name.as_ref().unwrap_or(&"(none)".to_string()));
    println!("Hip policy: {}", hit_policy);
    println!("Output label: {}", output_label.as_ref().unwrap_or(&"(none)".to_string()));
    println!("Allowed values: {}", allowed_values);
    println!("Markers: {}", markers_to_string(&markers));
    println!("Rows:");
    for row in &table {
      println!(
        "| {} |",
        row
          .iter()
          .map(|column| { if let Some(text) = column { text.to_string() } else { "(none)".to_string() } })
          .collect::<Vec<String>>()
          .join(" | ")
      );
    }
  }
  // Return the recognized decision table.
  Ok(DecisionTable::new(
    information_item_name,
    input_clauses,
    output_clauses,
    annotation_clauses,
    rules,
    hit_policy,
    aggregation,
    preferred_orientation,
    output_label,
  ))
}

/// Returns the lines of the Markdown table, with optional information item name and output label.
fn markdown_lines(text: &str) -> (Option<String>, Option<String>, Vec<String>) {
  enum State {
    BeforeTable,
    BlockQuoteFirst,
    BlockQuoteSecond,
    TableLines,
  }
  let mut buffer = vec![];
  let mut information_item_name = None;
  let mut output_label = None;
  let mut state = State::BeforeTable;
  for line in text.lines().filter_map(|line| {
    let trimmed_line = line.trim();
    if !trimmed_line.is_empty() { Some(trimmed_line) } else { None }
  }) {
    match state {
      State::BeforeTable => {
        if line.starts_with(INFORMATION_ITEM_NAME_START) {
          buffer.push(
            line
              .trim_start_matches(INFORMATION_ITEM_NAME_START)
              .trim_start_matches(INFORMATION_ITEM_NAME_HEADING)
              .trim()
              .to_string(),
          );
          state = State::BlockQuoteFirst;
        } else if line.starts_with(OUTPUT_LABEL_START) {
          buffer.push(line.trim_start_matches(OUTPUT_LABEL_START).trim().to_string());
          state = State::BlockQuoteSecond;
        } else if line.starts_with(TABLE_VERT_LINE) {
          buffer.push(line.to_string());
          state = State::TableLines;
        }
      }
      State::BlockQuoteFirst => {
        if line.starts_with(OUTPUT_LABEL_START) {
          buffer.push(line.trim_start_matches(OUTPUT_LABEL_START).trim().to_string());
          state = State::BlockQuoteSecond;
        } else if line.starts_with(TABLE_VERT_LINE) {
          information_item_name = buffer.pop();
          buffer.push(line.to_string());
          state = State::TableLines;
        } else {
          buffer.pop();
          state = State::BeforeTable;
        }
      }
      State::BlockQuoteSecond => {
        if line.starts_with(TABLE_VERT_LINE) {
          output_label = buffer.pop();
          information_item_name = buffer.pop();
          buffer.push(line.to_string());
          state = State::TableLines;
        } else {
          buffer.pop();
          buffer.pop();
          state = State::BeforeTable;
        }
      }
      State::TableLines => {
        if line.starts_with(TABLE_VERT_LINE) {
          buffer.push(line.to_string());
        }
      }
    }
  }
  (information_item_name, output_label, buffer)
}

fn markdown_table(lines: Vec<String>) -> Result<Table> {
  let mut table: Table = vec![];
  let Some(first_line) = lines.iter().take(1).next() else {
    return Err(err_md_no_decision_table());
  };
  let columns = markdown_columns(first_line);
  if columns.is_empty() {
    return Err(err_md_no_decision_table());
  }
  let column_count = columns.len();
  table.push(columns);
  for line in lines.iter().skip(2) {
    let columns = markdown_columns(line);
    if columns.len() != column_count {
      return Err(err_md_invalid_number_of_column(column_count, columns.len(), line));
    }
    if columns.iter().any(|column| column.is_some()) {
      table.push(columns);
    }
  }
  let filtered_table = remove_empty_columns(table);
  if filtered_table.len() < MIN_ROWS {
    return Err(err_md_invalid_decision_table(&format!("number of rows is less than {MIN_ROWS}")));
  }
  if filtered_table[0].len() < MIN_COLUMNS {
    return Err(err_md_invalid_decision_table(&format!("number of columns is less than {MIN_COLUMNS}")));
  }
  Ok(filtered_table)
}

/// Converts Markdown line into a vector of columns containing optional text from decision table.
fn markdown_columns(line: &str) -> Vec<Option<String>> {
  let mut row = line
    .split(TABLE_VERT_LINE)
    .skip(1)
    .map(|text| {
      let trimmed_text = text.trim();
      if trimmed_text.is_empty() { None } else { Some(trimmed_text.to_string()) }
    })
    .collect::<Vec<Option<String>>>();
  row.pop();
  row
}

/// Removes column from each row, for which all columns in all rows are empty.
fn remove_empty_columns(table: Table) -> Table {
  let column_count = table[0].len(); // Safe, table has at least MIN_ROWS
  let mut keep_column = vec![false; column_count];
  for row in &table {
    for (index, column) in row.iter().enumerate() {
      keep_column[index] |= column.is_some();
    }
  }
  table
    .iter()
    .map(|row| {
      row
        .iter()
        .enumerate()
        .filter_map(|(index, column)| if keep_column[index] { Some(column.clone()) } else { None })
        .collect()
    })
    .collect()
}

/// Returns the hit policy.
fn get_hit_policy(table: &Table) -> Result<HitPolicy> {
  let top_row = &table[0]; // Safe, table has at least MIN_ROWS
  let top_left_column = &top_row[0]; // Safe, table has at least MIN_COLUMNS
  top_left_column.as_ref().ok_or(err_md_no_hit_policy())?.try_into()
}

/// Discovers and returns the orientation of the decision table.
fn get_preferred_orientation(table: Table) -> Result<(DecisionTableOrientation, Table, usize, usize)> {
  match monotonic_numbers(get_rule_numbers(table[0].iter().skip(1))) {
    (true, empty_count, rule_count) => {
      // Decision table is vertical, so the preferred orientation is rule-as-column.
      // But the returned table is pivoted, to have rules as rows.
      Ok((DecisionTableOrientation::RulesAsColumns, pivot(table), empty_count, rule_count))
    }
    (false, _, _) => {
      // Pivot the table to have rules as columns.
      let table = pivot(table);
      match monotonic_numbers(get_rule_numbers(table[0].iter().skip(1))) {
        (true, empty_count, rule_count) => {
          // Decision table is horizontal, so the preferred orientation is rule-as-row.
          // But the returned table is pivoted again, to have rules as rows again.
          Ok((DecisionTableOrientation::RulesAsRows, pivot(table), empty_count, rule_count))
        }
        (false, _, _) => {
          // Preferred orientation could not be recognized.
          Err(err_md_invalid_decision_table("can not recognize the preferred orientation"))
        }
      }
    }
  }
}

/// Returns a vector of numbers converted from optional texts.
fn get_rule_numbers<'a>(columns: impl Iterator<Item = &'a Option<String>>) -> Vec<usize> {
  let zero = "0".to_string();
  columns.map(|text| text.as_ref().unwrap_or(&zero).parse::<usize>().unwrap_or(0)).collect()
}

/// Checks if rule numbers are monotonic, skipping leading zeros.
/// Returns (`true`, `zeros count`, `non-zeros count`) when numbers are monotonic,
/// otherwise returns (`false`, 0, 0).
fn monotonic_numbers(list: Vec<usize>) -> (bool, usize, usize) {
  let position = list.iter().position(|&x| x != 0);
  match position {
    Some(index) if list[index..].iter().enumerate().all(|(i, &value)| value == i + 1) => (true, index, list[index..].len()),
    _ => (false, 0, 0),
  }
}

/// Pivots the table.
fn pivot(table: Table) -> Table {
  let column_count = table[0].len();
  let mut pivot_table: Table = vec![vec![]; column_count];
  for row in table {
    for (index, column) in row.iter().enumerate() {
      pivot_table[index].push(column.clone());
    }
  }
  pivot_table
}

/// Returns a vector of optional markers converted from optional texts.
fn get_markers<'a>(columns: impl Iterator<Item = &'a Option<String>>) -> Result<Markers> {
  validate_markers(columns.map(get_marker).collect())
}

/// Converts optional text into optional marker.
fn get_marker(text: &Option<String>) -> Option<Marker> {
  let Some(text) = text else {
    return None;
  };
  let text = strip_emphasis(text.to_lowercase());
  if "input".starts_with(&text) || text == ">>>" || text == ">>" {
    return Some(Marker::Input);
  }
  if "output".starts_with(&text) || text == "<<<" || text == "<<" {
    return Some(Marker::Output);
  }
  if "annotation".starts_with(&text) || text == "###" || text == "##" || text == "#" {
    return Some(Marker::Annotation);
  }
  None
}

/// Validates the markers.
fn validate_markers(markers: Markers) -> Result<Markers> {
  enum State {
    Start,
    Input,
    Output,
    Annotation,
  }
  let mut state = State::Start;
  for marker in &markers {
    match state {
      State::Start => {
        if marker.is_none() {
          state = State::Input;
        } else {
          return Err(err_md_invalid_decision_table("unexpected marker"));
        }
      }
      State::Input => match marker {
        Some(Marker::Input) => {}
        Some(Marker::Output) => state = State::Output,
        _ => return Err(err_md_invalid_decision_table("expected input or output marker")),
      },
      State::Output => match marker {
        Some(Marker::Output) => {}
        Some(Marker::Annotation) => state = State::Annotation,
        _ => return Err(err_md_invalid_decision_table("expected output or annotation marker")),
      },
      State::Annotation => match marker {
        Some(Marker::Annotation) => {}
        _ => return Err(err_md_invalid_decision_table("expected annotation marker")),
      },
    }
  }
  Ok(markers)
}

/// Removes the Markdown emphasis from text if present.
fn strip_emphasis(text: String) -> String {
  for emphasis in EMPHASES {
    if text.starts_with(emphasis) && text.ends_with(emphasis) {
      return text.trim_start_matches(emphasis).trim_end_matches(emphasis).to_string();
    }
  }
  text
}

/// Converts a collection of markers into user-readable list.
fn markers_to_string(markers: &Markers) -> String {
  format!(
    "[{}]",
    markers
      .iter()
      .map(|marker| {
        if let Some(marker) = marker {
          match marker {
            Marker::Input => "input",
            Marker::Output => "output",
            Marker::Annotation => "annotation",
          }
          .to_string()
        } else {
          "(none)".to_string()
        }
      })
      .collect::<Vec<String>>()
      .join(", ")
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_monotonic() {
    assert_eq!((false, 0, 0), monotonic_numbers(vec![0, 0, 0, 0, 0, 0, 0]));
    assert_eq!((false, 0, 0), monotonic_numbers(vec![1, 0, 0, 0, 0, 0, 0]));
    assert_eq!((true, 0, 5), monotonic_numbers(vec![1, 2, 3, 4, 5]));
    assert_eq!((true, 1, 6), monotonic_numbers(vec![0, 1, 2, 3, 4, 5, 6]));
    assert_eq!((true, 2, 7), monotonic_numbers(vec![0, 0, 1, 2, 3, 4, 5, 6, 7]));
    assert_eq!((true, 3, 4), monotonic_numbers(vec![0, 0, 0, 1, 2, 3, 4]));
  }

  #[test]
  fn test_strip_emphasis() {
    assert_eq!("text", strip_emphasis("text".to_string()));
    assert_eq!("__text", strip_emphasis("__text".to_string()));
    assert_eq!("text__", strip_emphasis("text__".to_string()));
    assert_eq!("_text", strip_emphasis("_text".to_string()));
    assert_eq!("text_", strip_emphasis("text_".to_string()));
    assert_eq!("**text", strip_emphasis("**text".to_string()));
    assert_eq!("text**", strip_emphasis("text**".to_string()));
    assert_eq!("*text", strip_emphasis("*text".to_string()));
    assert_eq!("text*", strip_emphasis("text*".to_string()));
    assert_eq!("`text", strip_emphasis("`text".to_string()));
    assert_eq!("text`", strip_emphasis("text`".to_string()));
    assert_eq!("_text*", strip_emphasis("_text*".to_string()));
    assert_eq!("**text__", strip_emphasis("**text__".to_string()));
    assert_eq!("text", strip_emphasis("__text__".to_string()));
    assert_eq!("text", strip_emphasis("**text**".to_string()));
    assert_eq!("text", strip_emphasis("_text_".to_string()));
    assert_eq!("text", strip_emphasis("*text*".to_string()));
    assert_eq!("text", strip_emphasis("`text`".to_string()));
    assert_eq!("text", strip_emphasis("**text*".to_string()));
    assert_eq!("text", strip_emphasis("*text**".to_string()));
    assert_eq!("text", strip_emphasis("__text_".to_string()));
    assert_eq!("text", strip_emphasis("_text__".to_string()));
  }

  #[test]
  fn test_get_marker() {
    assert_eq!(None, get_marker(&None));
    assert_eq!(Marker::Input, get_marker(&Some("i".to_string())).unwrap());
    assert_eq!(Marker::Input, get_marker(&Some("I".to_string())).unwrap());
    assert_eq!(Marker::Input, get_marker(&Some("in".to_string())).unwrap());
    assert_eq!(Marker::Input, get_marker(&Some("inp".to_string())).unwrap());
    assert_eq!(Marker::Input, get_marker(&Some("inpu".to_string())).unwrap());
    assert_eq!(Marker::Input, get_marker(&Some("input".to_string())).unwrap());
    assert_eq!(Marker::Input, get_marker(&Some(">>>".to_string())).unwrap());
    assert_eq!(Marker::Input, get_marker(&Some(">>".to_string())).unwrap());
    assert_eq!(None, get_marker(&Some("inputa".to_string())));
    assert_eq!(None, get_marker(&Some(">".to_string())));
    assert_eq!(Marker::Output, get_marker(&Some("o".to_string())).unwrap());
    assert_eq!(Marker::Output, get_marker(&Some("O".to_string())).unwrap());
    assert_eq!(Marker::Output, get_marker(&Some("ou".to_string())).unwrap());
    assert_eq!(Marker::Output, get_marker(&Some("out".to_string())).unwrap());
    assert_eq!(Marker::Output, get_marker(&Some("outp".to_string())).unwrap());
    assert_eq!(Marker::Output, get_marker(&Some("outpu".to_string())).unwrap());
    assert_eq!(Marker::Output, get_marker(&Some("output".to_string())).unwrap());
    assert_eq!(Marker::Output, get_marker(&Some("<<<".to_string())).unwrap());
    assert_eq!(Marker::Output, get_marker(&Some("<<".to_string())).unwrap());
    assert_eq!(None, get_marker(&Some("outputa".to_string())));
    assert_eq!(None, get_marker(&Some("<".to_string())));
    assert_eq!(Marker::Annotation, get_marker(&Some("a".to_string())).unwrap());
    assert_eq!(Marker::Annotation, get_marker(&Some("A".to_string())).unwrap());
    assert_eq!(Marker::Annotation, get_marker(&Some("an".to_string())).unwrap());
    assert_eq!(Marker::Annotation, get_marker(&Some("ann".to_string())).unwrap());
    assert_eq!(Marker::Annotation, get_marker(&Some("anno".to_string())).unwrap());
    assert_eq!(Marker::Annotation, get_marker(&Some("annot".to_string())).unwrap());
    assert_eq!(Marker::Annotation, get_marker(&Some("annota".to_string())).unwrap());
    assert_eq!(Marker::Annotation, get_marker(&Some("annotat".to_string())).unwrap());
    assert_eq!(Marker::Annotation, get_marker(&Some("annotati".to_string())).unwrap());
    assert_eq!(Marker::Annotation, get_marker(&Some("annotatio".to_string())).unwrap());
    assert_eq!(Marker::Annotation, get_marker(&Some("annotation".to_string())).unwrap());
    assert_eq!(Marker::Annotation, get_marker(&Some("###".to_string())).unwrap());
    assert_eq!(Marker::Annotation, get_marker(&Some("##".to_string())).unwrap());
    assert_eq!(Marker::Annotation, get_marker(&Some("#".to_string())).unwrap());
    assert_eq!(None, get_marker(&Some("annotationa".to_string())));
    assert_eq!(None, get_marker(&Some("@".to_string())));
  }
}
