use dsntk_recognizer::{DecisionTableOrientation, HitPolicy, from_markdown};

/// There is some additional text after decision table.
#[test]
fn _0001() {
  let markdown = r#"
    | U |             |
    |:-:|:-----------:|
    |   |    Out      |
    | 1 |  "Monday"   |
    > Other text after decision table
  "#;
  let dt = from_markdown(markdown, false).unwrap();
  assert_eq!(None, dt.information_item_name);
  assert_eq!(None, dt.output_label);
  assert_eq!(HitPolicy::Unique, dt.hit_policy);
  assert_eq!(DecisionTableOrientation::RulesAsRows, dt.preferred_orientation);
}

/// Only the information item name is present.
#[test]
fn _0002() {
  let markdown = r#"
    > # Weekdays sorted
    | U |             |
    |:-:|:-----------:|
    |   |    Out      |
    | 1 |  "Monday"   |
    > Other text after decision table
  "#;
  let dt = from_markdown(markdown, false).unwrap();
  assert_eq!("Weekdays sorted", dt.information_item_name.unwrap());
  assert_eq!(None, dt.output_label);
  assert_eq!(HitPolicy::Unique, dt.hit_policy);
  assert_eq!(DecisionTableOrientation::RulesAsRows, dt.preferred_orientation);
}

/// The information item name is present and the output label.
#[test]
fn _0003() {
  let markdown = r#"
    > # Weekdays sorted
    > Weekday
    | U |             |
    |:-:|:-----------:|
    |   |    Out      |
    | 1 |  "Monday"   |
    > Other text after decision table
  "#;
  let dt = from_markdown(markdown, false).unwrap();
  assert_eq!("Weekdays sorted", dt.information_item_name.unwrap());
  assert_eq!("Weekday", dt.output_label.unwrap());
  assert_eq!(HitPolicy::Unique, dt.hit_policy);
  assert_eq!(DecisionTableOrientation::RulesAsRows, dt.preferred_orientation);
}

/// Only the output label is present before the table,
/// but it is also defined in a first row.
#[test]
fn _0004() {
  let markdown = r#"
    > Weekday
    | U | Day of week |
    |:-:|:-----------:|
    |   |    Out      |
    | 1 |  "Monday"   |
    > Other text after decision table
  "#;
  let dt = from_markdown(markdown, false).unwrap();
  assert_eq!(None, dt.information_item_name);
  assert_eq!("Weekday", dt.output_label.unwrap());
  assert_eq!(HitPolicy::Unique, dt.hit_policy);
  assert_eq!(DecisionTableOrientation::RulesAsRows, dt.preferred_orientation);
}
