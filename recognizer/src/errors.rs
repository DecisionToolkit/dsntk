//! # Error definitions for decision table recognizer

use crate::point::Point;
use crate::rect::Rect;
use dsntk_common::{DsntkError, ToErrorMessage};
use std::fmt::Write;

/// Recognizer errors.
#[derive(ToErrorMessage)]
struct RecognizerError(String);

pub fn err_canvas_expected_characters_not_found(searched: &[char]) -> DsntkError {
    RecognizerError(format!(
        "expected characters not found: {}",
        chars_to_list(searched)
    ))
    .into()
}

pub fn err_canvas_character_is_not_allowed(ch: char, allowed: &[char]) -> DsntkError {
    RecognizerError(format!(
        "character '{ch}' is not allowed in: {}",
        chars_to_list(allowed)
    ))
    .into()
}

pub fn err_canvas_rectangle_not_closed(p1: Point, p2: Point) -> DsntkError {
    RecognizerError(format!(
        "rectangle is not closed, start point: {p1}, end point: {p2}"
    ))
    .into()
}

pub fn err_canvas_region_not_found(rect: Rect) -> DsntkError {
    RecognizerError(format!("region not found, rect: {rect}")).into()
}

pub fn err_plane_is_empty() -> DsntkError {
    RecognizerError("plane is empty".to_string()).into()
}

pub fn err_plane_cell_is_not_region(details: &str) -> DsntkError {
    RecognizerError(format!("not a region cell in plane: {details}")).into()
}

pub fn err_plane_row_is_out_of_range() -> DsntkError {
    RecognizerError("plane row is out of range".to_string()).into()
}

pub fn err_plane_no_main_double_crossing() -> DsntkError {
    RecognizerError("plane no main double crossing".to_string()).into()
}

pub fn err_plane_column_is_out_of_range() -> DsntkError {
    RecognizerError("plane column is out of range".to_string()).into()
}

pub fn err_plane_invalid_rule_number(num: usize) -> DsntkError {
    RecognizerError(format!("plane invalid rule number: {num}")).into()
}

pub fn err_expected_no_rule_numbers_present() -> DsntkError {
    RecognizerError("expected no rule numbers present".to_string()).into()
}

pub fn err_invalid_input_expressions() -> DsntkError {
    RecognizerError("invalid input expressions".to_string()).into()
}

pub fn err_invalid_output_expressions() -> DsntkError {
    RecognizerError("invalid output expressions".to_string()).into()
}

pub fn err_no_output_clause() -> DsntkError {
    RecognizerError("no output clause".to_string()).into()
}

pub fn err_expected_right_after_rule_numbers_placement() -> DsntkError {
    RecognizerError("expected right-after rule numbers placement".to_string()).into()
}

pub fn err_expected_left_below_rule_numbers_placement() -> DsntkError {
    RecognizerError("expected left-below rule numbers placement".to_string()).into()
}

pub fn err_expected_bottom_left_hit_policy_placement() -> DsntkError {
    RecognizerError("expected bottom-left hit policy placement".to_string()).into()
}

pub fn err_expected_top_left_hit_policy_placement() -> DsntkError {
    RecognizerError("expected top-left hit policy placement".to_string()).into()
}

pub fn err_recognizing_cross_tab_not_supported_yet() -> DsntkError {
    RecognizerError("recognizing cross-tab decision tables is not yet implemented".to_string())
        .into()
}

pub fn err_too_many_rows_in_input_clause() -> DsntkError {
    RecognizerError("too many rows in input clause".to_string()).into()
}

pub fn err_too_many_rows_in_output_clause() -> DsntkError {
    RecognizerError("too many rows in output clause".to_string()).into()
}

pub fn err_invalid_size(details: &str) -> DsntkError {
    RecognizerError(format!("invalid size: {details}")).into()
}

pub fn err_invalid_hit_policy(hit_policy: &str) -> DsntkError {
    RecognizerError(format!("invalid hit policy: {hit_policy}")).into()
}

/// Error reported when the Markdown does not contain proper decision table.
pub fn err_md_no_decision_table() -> DsntkError {
    RecognizerError("no decision table".to_string()).into()
}

/// Error reported when the number of columns in the row differ from the number of columns in the table header.
pub fn err_md_invalid_number_of_column(expected: usize, actual: usize, row: &str) -> DsntkError {
    RecognizerError(format!(
        "invalid number of columns, expected {expected}, actual is {actual} in row '{row}'"
    ))
    .into()
}

/// Error reported when the number of rows or the number of columns in the decision table is less than minimal.
pub fn err_md_invalid_decision_table(reason: &str) -> DsntkError {
    RecognizerError(format!("invalid decision table, reason: {reason}")).into()
}

/// Error reported when the hit policy is not present in the Markdown decision table.
pub fn err_md_no_hit_policy() -> DsntkError {
    RecognizerError("no hit policy".to_string()).into()
}

/// Utility function that formats a slice of characters into a list of characters,
/// making it more readable in error messages.
fn chars_to_list(input: &[char]) -> String {
    let mut buffer = String::new();
    for (index, ch) in input.iter().enumerate() {
        if index > 0 {
            let _ = write!(&mut buffer, ", ");
        }
        if ch.is_whitespace() || *ch == ',' {
            let _ = write!(&mut buffer, "'{}'", ch);
        } else {
            let _ = write!(&mut buffer, "{}", ch);
        }
    }
    buffer
}
