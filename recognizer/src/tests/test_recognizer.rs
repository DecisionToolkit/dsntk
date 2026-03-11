use super::*;
use crate::model::{BuiltinAggregator, DecisionTableOrientation};
use crate::recognizer::Recognizer;
use crate::HitPolicy;
use dsntk_examples::decision_tables::H_110010;

const EMPTY_VECTOR: &[&str] = &[];
const EMPTY_OPT_VECTOR: &[Option<&str>] = &[];
const EMPTY_MATRIX: &[&[&str]] = &[];

fn no_information_item_name(recognizer: &Recognizer) {
  assert!(recognizer.information_item_name.is_none());
}

fn eq_information_item_name(recognizer: &Recognizer, expected: &str) {
  assert!(recognizer.information_item_name.is_some());
  assert_eq!(recognizer.information_item_name.as_ref().unwrap(), expected);
}

fn eq_hit_policy(recognizer: &Recognizer, expected: HitPolicy) {
  assert_eq!(recognizer.hit_policy, expected);
}

fn eq_orientation(recognizer: &Recognizer, expected: DecisionTableOrientation) {
  assert_eq!(recognizer.orientation, expected);
}

fn eq_output_label(recognizer: &Recognizer, expected: Option<String>) {
  assert_eq!(recognizer.output_label, expected);
}

fn eq_input_expressions(recognizer: &Recognizer, expected: &[&str]) {
  eq_vectors(&recognizer.input_expressions, expected);
}

fn eq_input_values(recognizer: &Recognizer, expected: &[Option<&str>]) {
  eq_opt_vectors(&recognizer.allowed_input_values, expected);
}

fn eq_input_entries(recognizer: &Recognizer, expected: &[&[&str]]) {
  eq_matrices(&recognizer.input_entries, expected);
}

fn eq_output_components(recognizer: &Recognizer, expected: &[Option<&str>]) {
  eq_opt_vectors(&recognizer.output_components, expected);
}

fn eq_output_values(recognizer: &Recognizer, expected: &[Option<&str>]) {
  eq_opt_vectors(&recognizer.allowed_output_values, expected);
}

fn eq_output_entries(recognizer: &Recognizer, expected: &[&[&str]]) {
  eq_matrices(&recognizer.output_entries, expected);
}

fn eq_annotations(recognizer: &Recognizer, expected: &[&str]) {
  eq_vectors(&recognizer.annotations, expected);
}

fn eq_annotation_entries(recognizer: &Recognizer, expected: &[&[&str]]) {
  eq_matrices(&recognizer.annotation_entries, expected);
}

#[test]
fn test_invalid_0001() {
  let input = r#"
    // This decision table is invalid, because the top-left corner character
    // of the information item name should be '┌'.

    │──────────┐
    │ Weekdays │
    ├───╥──────┴──────┐
    │ C ║   Weekday   │
    ╞═══╬═════════════╡
    │ 1 ║  "Monday"   │
    ├───╫─────────────┤
    │ 2 ║  "Tuesday"  │
    └───╨─────────────┘
  "#;
  assert_eq!(
    "<RecognizerError> expected characters not found: ┌",
    Recognizer::recognize(input, false).unwrap_err().to_string()
  );
}

#[test]
fn test_invalid_0002() {
  let input = r#"
    // This decision table is invalid, because the top-left corner character
    // of the decision table (when information item name is not present) should be '┌'.

    │───╥─────────────┐
    │ C ║   Weekday   │
    ╞═══╬═════════════╡
    │ 1 ║  "Monday"   │
    ├───╫─────────────┤
    │ 2 ║  "Tuesday"  │
    └───╨─────────────┘
  "#;
  assert_eq!(
    "<RecognizerError> expected characters not found: ┌",
    Recognizer::recognize(input, false).unwrap_err().to_string()
  );
}

#[test]
fn test_dt_0001() {
  let recognizer = &Recognizer::recognize(H_110010, false).unwrap();
  eq_orientation(recognizer, DecisionTableOrientation::RulesAsRows);
  eq_information_item_name(recognizer, "Days of week");
  eq_hit_policy(recognizer, HitPolicy::Collect(BuiltinAggregator::List));
  eq_input_expressions(recognizer, EMPTY_VECTOR);
  eq_input_values(recognizer, EMPTY_OPT_VECTOR);
  eq_input_entries(
    recognizer,
    &[EMPTY_VECTOR, EMPTY_VECTOR, EMPTY_VECTOR, EMPTY_VECTOR, EMPTY_VECTOR, EMPTY_VECTOR, EMPTY_VECTOR],
  );
  eq_output_label(recognizer, Some("Weekday".to_string()));
  eq_output_components(recognizer, EMPTY_OPT_VECTOR);
  eq_output_values(recognizer, EMPTY_OPT_VECTOR);
  eq_output_entries(
    recognizer,
    &[
      &[r#""Monday""#],
      &[r#""Tuesday""#],
      &[r#""Wednesday""#],
      &[r#""Thursday""#],
      &[r#""Friday""#],
      &[r#""Saturday""#],
      &[r#""Sunday""#],
    ],
  );
  eq_annotations(recognizer, EMPTY_VECTOR);
  eq_annotation_entries(recognizer, EMPTY_MATRIX);
}

#[test]
fn test_example_01() {
  let rec = &Recognizer::recognize(EX_01, false).unwrap();
  no_information_item_name(rec);
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RulesAsRows);
  eq_input_expressions(rec, &["Customer", "Order"]);
  eq_input_values(rec, EMPTY_OPT_VECTOR);
  eq_input_entries(rec, &[&[r#""Business""#, "<10"], &[r#""Business""#, ">=10"], &[r#""Private""#, "-"]]);
  eq_output_label(rec, None);
  eq_output_components(rec, EMPTY_OPT_VECTOR);
  eq_output_values(rec, EMPTY_OPT_VECTOR);
  eq_output_entries(rec, &[&["0.10"], &["0.15"], &["0.05"]]);
  eq_annotations(rec, EMPTY_VECTOR);
  eq_annotation_entries(rec, EMPTY_MATRIX);
}

#[test]
fn test_example_02() {
  let rec = &Recognizer::recognize(EX_02, false).unwrap();
  no_information_item_name(rec);
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RulesAsRows);
}

#[test]
fn test_example_03() {
  let rec = &Recognizer::recognize(EX_03, false).unwrap();
  no_information_item_name(rec);
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RulesAsRows);
}

#[test]
fn ex_0033() {
  let rec = &Recognizer::recognize(EX_05, false).unwrap();
  no_information_item_name(rec);
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RulesAsColumns);
}

#[test]
fn ex_0044() {
  let rec = &Recognizer::recognize(EX_06, false).unwrap();
  no_information_item_name(rec);
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RulesAsColumns);
  eq_input_expressions(rec, &["Applicant age", "Medical history"]);
  eq_input_values(rec, EMPTY_OPT_VECTOR);
  eq_input_entries(
    rec,
    &[&["<25", r#""good""#], &["<25", r#""bad""#], &["[25..60]", "-"], &[">60", r#""good""#], &[">60", r#""bad""#]],
  );
  eq_output_label(rec, Some("Sell options".to_string()));
  eq_output_components(rec, &[Some("Applicant risk rating"), Some("Special Discount")]);
  eq_output_values(rec, EMPTY_OPT_VECTOR);
  eq_output_entries(
    rec,
    &[
      &[r#""Low""#, "10"],
      &[r#""Medium""#, "5"],
      &[r#""Medium""#, "5"],
      &[r#""Medium""#, "5"],
      &[r#""High""#, "0"],
    ],
  );
  eq_annotations(rec, &["Additional acceptance", "Reference"]);
  eq_annotation_entries(rec, &[&["No", "Rf 0"], &["No", "Rf 1"], &["No", "Rf 2"], &["No", "Rf 3"], &["Yes", "Rf 4"]]);
}

#[test]
fn ex_0064() {
  let rec = &Recognizer::recognize(EX_07, false).unwrap();
  eq_information_item_name(rec, "Sell options");
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RulesAsColumns);
  eq_input_expressions(rec, &["Applicant age", "Medical history"]);
  eq_input_values(rec, &[Some("<25,[25..60],>60"), Some(r#""good","bad""#)]);
  eq_input_entries(
    rec,
    &[&["<25", r#""good""#], &["<25", r#""bad""#], &["[25..60]", "-"], &[">60", r#""good""#], &[">60", r#""bad""#]],
  );
  eq_output_label(rec, Some("Sell options".to_string()));
  eq_output_components(rec, &[Some("Applicant risk rating"), Some("Special Discount")]);
  eq_output_entries(
    rec,
    &[
      &[r#""Low""#, "10"],
      &[r#""Medium""#, "5"],
      &[r#""Medium""#, "5"],
      &[r#""Medium""#, "5"],
      &[r#""High""#, "0"],
    ],
  );
  eq_annotations(rec, &["Additional acceptance", "Reference"]);
  eq_annotation_entries(rec, &[&["No", "Rf 0"], &["No", "Rf 1"], &["No", "Rf 2"], &["No", "Rf 3"], &["Yes", "Rf 4"]]);
}

#[test]
fn general_horizontal() {
  let rec = &Recognizer::recognize(EX_08, false).unwrap();
  eq_information_item_name(rec, "information item name");
  eq_hit_policy(rec, HitPolicy::Collect(BuiltinAggregator::List));
  eq_orientation(rec, DecisionTableOrientation::RulesAsRows);
  eq_input_expressions(rec, &["input expression 1", "input expression 2"]);
  eq_input_values(rec, &[Some("input value 1a, input value 1b"), Some("input value 2a, input value 2b")]);
  eq_input_entries(
    rec,
    &[
      &["input entry 1.1", "input entry 2.1"],
      &["input entry 1.1", "input entry 2.2"],
      &["input entry 1.2", "-"],
      &["input entry 1.3", "input entry 2.3"],
    ],
  );
  eq_output_label(rec, Some("output label".to_string()));
  eq_output_components(rec, EMPTY_OPT_VECTOR);
  eq_output_values(rec, &[Some("output value 1a, output value 1b")]);
  eq_output_entries(rec, &[&["output entry 1.1"], &["output entry 1.2"], &["output entry 1.3"], &["output entry 1.4"]]);
  eq_annotations(rec, EMPTY_VECTOR);
  eq_annotation_entries(rec, EMPTY_MATRIX);
}

#[test]
fn general_vertical() {
  let rec = &Recognizer::recognize(EX_09, false).unwrap();
  eq_information_item_name(rec, "information item name");
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RulesAsColumns);
}

#[test]
fn general_cross_tab() {
  assert!(&Recognizer::recognize(EX_10, false).is_err());
  // eq_information_item_name(rec, " information item name                                          ");
  // eq_hit_policy(rec, HitPolicy::Unique);
  // eq_orientation(rec, Orientation::Crosstab);
  // eq_input_expressions(rec, EMPTY_VECTOR);
  // eq_input_values(rec, EMPTY_VECTOR);
  // eq_input_entries(rec, EMPTY_MATRIX);
  // no_output_label(rec);
  // eq_output_components(rec, EMPTY_VECTOR);
  // eq_output_values(rec, EMPTY_VECTOR);
  // eq_output_entries(rec, EMPTY_MATRIX);
  // eq_annotations(rec, EMPTY_VECTOR);
  // eq_annotation_entries(rec, EMPTY_MATRIX);
}

#[test]
fn test_err_01() {
  assert_eq!(
    "<RecognizerError> expected characters not found: ╬",
    Recognizer::recognize(EX_ERR_01, false).err().unwrap().to_string()
  );
}

#[test]
fn test_err_02() {
  assert_eq!(
    "<RecognizerError> character ' ' is not allowed in: ─, ┴",
    Recognizer::recognize(EX_ERR_02, false).err().unwrap().to_string()
  );
}

#[test]
fn test_err_03() {
  assert_eq!(
    "<RecognizerError> rectangle is not closed, start point: (0,1), end point: (0,0)",
    Recognizer::recognize(EX_ERR_03, false).err().unwrap().to_string()
  );
}

#[test]
fn test_err_04() {
  assert_eq!(
    "<RecognizerError> too many rows in input clause",
    Recognizer::recognize(EX_ERR_04, false).err().unwrap().to_string()
  );
}

#[test]
#[ignore]
fn test_err_05() {
  assert_eq!(
    "<RecognizerError> too many rows in output clause",
    Recognizer::recognize(EX_ERR_05, false).err().unwrap().to_string()
  );
}

#[test]
#[ignore]
fn test_err_06() {
  assert_eq!(
    "<RecognizerError> too many rows in input clause",
    Recognizer::recognize(EX_ERR_06, false).err().unwrap().to_string()
  );
}

#[test]
fn test_err_07() {
  assert_eq!(
    "<RecognizerError> expected right-after rule numbers placement",
    Recognizer::recognize(EX_ERR_07, false).err().unwrap().to_string()
  );
}

#[test]
fn test_err_08() {
  assert_eq!(
    "<RecognizerError> expected no rule numbers present",
    Recognizer::recognize(EX_ERR_08, false).err().unwrap().to_string()
  );
}

#[test]
fn test_age_calculation_01() {
  let rec = &Recognizer::recognize(EX_AGE_CALCULATION, false).unwrap();
  no_information_item_name(rec);
  eq_hit_policy(rec, HitPolicy::First);
  eq_orientation(rec, DecisionTableOrientation::RulesAsRows);
  eq_input_expressions(rec, &["Age"]);
  eq_input_values(rec, EMPTY_OPT_VECTOR);
  eq_input_entries(
    rec,
    &[
      &["<13"],
      &["<18"],
      &["<65"],
      &[">=65"],
    ],
  );
  eq_output_label(rec, Some("Age Group".to_string()));
  eq_output_components(rec, EMPTY_OPT_VECTOR);
  eq_output_values(rec, EMPTY_OPT_VECTOR);
  eq_output_entries(
    rec,
    &[
      &[r#""child""#],
      &[r#""teenager""#],
      &[r#""adult""#],
      &[r#""senior""#],
    ],
  );
  eq_annotations(rec, EMPTY_VECTOR);
  eq_annotation_entries(rec, EMPTY_MATRIX);
}

#[test]
fn test_temporal_functions() {
  let rec = &Recognizer::recognize(EX_TEMPORAL_FUNCTIONS, false).unwrap();
  no_information_item_name(rec);
  eq_hit_policy(rec, HitPolicy::Unique);
  eq_orientation(rec, DecisionTableOrientation::RulesAsRows);
  eq_input_expressions(rec, &["Function"]);
  eq_input_values(rec, EMPTY_OPT_VECTOR);
  eq_input_entries(
    rec,
    &[
      &["now()"],
      &["today()"],
    ],
  );
  eq_output_label(rec, Some("Result".to_string()));
  eq_output_components(rec, EMPTY_OPT_VECTOR);
  eq_output_values(rec, EMPTY_OPT_VECTOR);
  eq_output_entries(
    rec,
    &[
      &["now()"],
      &["today()"],
    ],
  );
  eq_annotations(rec, EMPTY_VECTOR);
  eq_annotation_entries(rec, EMPTY_MATRIX);
}
