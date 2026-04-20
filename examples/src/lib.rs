#![doc = include_str!("../docs/README.md")]
#![deny(rustdoc::broken_intra_doc_links)]

mod compatibility;
pub mod decision_logic;
pub mod decision_tables;
mod diagrams;
mod examples;
mod exhaustive;
mod full_model;
pub mod input_data;
pub mod item_definition;

pub use compatibility::*;
pub use diagrams::*;
pub use examples::valid::*;
pub use examples::*;
pub use exhaustive::*;
pub use full_model::*;

#[cfg(test)]
mod utilities {
  use std::collections::BTreeSet;
  use std::fmt::Write;
  use walkdir::WalkDir;

  /// Generates multiple decision table variants.
  #[test]
  #[rustfmt::skip]
  pub fn generate_decision_table_variants() {
    let mut buffer = String::new();
    let orientation = ["H", "V", "C"];
    let information_item = ["0", "1" ];
    let output_label = ["0", "1" ];
    let allowed_values = ["0", "1"];
    let inputs = ["0", "1", "2"];
    let outputs = ["1", "2"];
    let annotations = ["0", "1", "2"];
    let _ = writeln!(&mut buffer, "┌──────┬─────────────┬─────────────┬─────────┬─────────┬──────────┬──────────┬─────────────┬───────────┬────────┐");
    let _ = writeln!(&mut buffer, "│  No. │  Preferred  │ Information │ Output  │ Allowed │  Inputs  │ Outputs  │ Annotations │   Name    │ Status │");
    let _ = writeln!(&mut buffer, "│      │ orientation │  item name  │  label  │ values  │          │          │             │           │        │");
    let _ = writeln!(&mut buffer, "├──────┼─────────────┼─────────────┼─────────┼─────────┼──────────┼──────────┼─────────────┼───────────┼────────┤");
    let mut counter = 1;
    for v_decision_table_orientation in orientation {
      for v_information_item_name in information_item {
        for v_output_label in output_label {
          for v_allowed_values in allowed_values {
            for v_inputs in inputs {
              for v_outputs in outputs {
                for v_annotations in annotations {
                  let dt_name = format!("{}_{}{}{}{}{}{}", v_decision_table_orientation, v_information_item_name, v_output_label, v_allowed_values, v_inputs, v_outputs, v_annotations);
                  let _ = writeln!(&mut buffer, "│ {counter:>4} │{v_decision_table_orientation:^13}│{v_information_item_name:^13}│{v_output_label:^9}│{v_allowed_values:^9}│{v_inputs:^10}│{v_outputs:^10}│{v_annotations:^13}│ {dt_name:9} │        │");
                  counter += 1;
                }
              }
            }
          }
        }
      }
    }
    let _ = writeln!(&mut buffer, "└──────┴─────────────┴─────────────┴─────────┴─────────┴──────────┴──────────┴─────────────┴───────────┴────────┘");
    println!("{}", buffer);
    assert_eq!(437, buffer.lines().count());
  }

  /// This utility function compares the number of compatibility test models in this crate
  /// with the number of compatibility test models in TCK repository.
  #[test]
  fn compare_the_number_of_models() {
    let tck_models = count_models("../../tck/TestCases");
    let tck_adjusted_models = tck_models
      .iter()
      .filter_map(|s| {
        let segments = s.split('/').collect::<Vec<&str>>();
        let first_segment = segments[0]
          .replace("compliance-level-2", "level_2")
          .replace("compliance-level-3", "level_3")
          .replace("non-compliant", "non_compliant");
        let last_segment = segments[2][0..4].to_string();
        if last_segment.chars().next().unwrap().is_ascii_digit() {
          Some(format!("{}/{}", first_segment, last_segment))
        } else {
          None
        }
      })
      .collect::<BTreeSet<String>>();

    let dsntk_models = count_models("src/compatibility");
    let dsntk_adjusted_models = dsntk_models
      .iter()
      .map(|s| {
        let segments = s.split('/').collect::<Vec<&str>>();
        let first_segment = segments[0];
        let last_segment = segments[1][2..6].to_string();
        format!("{}/{}", first_segment, last_segment)
      })
      .collect::<BTreeSet<String>>();

    let mut all_keys = BTreeSet::new();
    all_keys.append(&mut tck_adjusted_models.clone());
    all_keys.append(&mut dsntk_adjusted_models.clone());

    println!("────────────────────────────────");
    println!(" Model               TCK  DSNTK");
    println!("────────────────────────────────");
    for key in &all_keys {
      println!(
        "{:20}  {:>2}     {:>2}",
        key,
        if tck_adjusted_models.contains(key) { "OK" } else { "-" },
        if dsntk_adjusted_models.contains(key) { "OK" } else { "-" }
      )
    }
  }

  /// Counts DMN models residing in the specified directory.
  fn count_models(root_dir: &str) -> BTreeSet<String> {
    let mut results = BTreeSet::new();
    for entry_result in WalkDir::new(root_dir).into_iter() {
      let entry = entry_result.unwrap();
      let path = entry.path();
      if path.is_file() && path.extension().is_some_and(|ext| ext == "dmn") {
        results.insert(path.strip_prefix(root_dir).unwrap().display().to_string());
      }
    }
    results
  }
}
