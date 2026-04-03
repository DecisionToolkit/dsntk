use dsntk_recognizer::{HitPolicy, from_markdown};

/// Minimal horizontal decision table defined in Markdown:
/// - hit policy,
/// - one required output clause,
/// - one row with input/output/annotation markers,
/// - minimum one rule as row.
///
/// The parsed table must contain minimum 3 rows and 2 columns.
#[test]
fn test_minimal_horizontal_decision_table() {
  let markdown = r#"
    | U |             |
    |:-:|:-----------:|
    |   |    Out      |
    | 1 |  "Monday"   |
  "#;
  let dt = from_markdown(markdown, false).unwrap();
  assert_eq!(None, dt.information_item_name);
  assert_eq!(None, dt.output_label);
  assert_eq!(HitPolicy::Unique, dt.hit_policy);
}

/// Minimal vertical decision table defined in Markdown:
/// - hit policy,
/// - one required output clause,
/// - one column with input/output/annotation markers,
/// - minimum one rule as column.
///
/// The parsed table must contain minimum 2 rows and 3 columns.
#[test]
fn test_minimal_vertical_decision_table() {
  let markdown = r#"
    | U |     |    1     |
    |:--|:---:|:--------:|
    |   | Out | "Monday" |
  "#;
  let dt = from_markdown(markdown, false).unwrap();
  assert_eq!(None, dt.information_item_name);
  assert_eq!(None, dt.output_label);
  assert_eq!(HitPolicy::Unique, dt.hit_policy);
}

#[test]
fn test_empty_rows_should_be_skipped() {
  let markdown = r#"
    | U |             |
    |:-:|:-----------:|
    |   |             |
    |   |    Out      |
    |   |             |
    | 1 |  "Monday"   |
    |   |             |
  "#;
  let dt = from_markdown(markdown, false).unwrap();
  assert_eq!(None, dt.information_item_name);
  assert_eq!(None, dt.output_label);
  assert_eq!(HitPolicy::Unique, dt.hit_policy);
}

#[test]
fn test_empty_columns_should_be_skipped() {
  let markdown = r#"
    | U |     |     |     |    1     |     |
    |:--|:---:|:---:|:---:|:--------:|:---:|
    |   |     | Out |     | "Monday" |     |
  "#;
  let dt = from_markdown(markdown, false).unwrap();
  assert_eq!(None, dt.information_item_name);
  assert_eq!(None, dt.output_label);
  assert_eq!(HitPolicy::Unique, dt.hit_policy);
}
