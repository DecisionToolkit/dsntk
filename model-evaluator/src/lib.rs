#[macro_use]
extern crate dsntk_macros;

mod boxed_expressions;
mod business_knowledge_model;
mod decision;
mod decision_service;
mod decision_table;
mod errors;
mod input_data;
mod input_data_context;
mod item_definition;
mod item_definition_context;
mod item_definition_type;
mod model_builder;
mod model_definitions;
mod model_evaluator;
mod type_ref;
mod variable;

#[cfg(test)]
mod tests;

pub use decision_table::build_decision_table_evaluator;
pub use model_evaluator::ModelEvaluator;

#[cfg(test)]
mod utilities {
  use antex::{ColorMode, StyledText, Text};
  use std::collections::{BTreeMap, BTreeSet};
  use std::fs;
  use walkdir::WalkDir;

  /// This is a utility function for comparing the number of compatibility tests
  /// with the number of compatibility benchmarks, used to verify if all tests
  /// are also benchmarked.
  #[test]
  fn compare_the_number_of_tests_and_benchmarks() {
    let (tests, test_count) = count_number_of_artefacts("src/tests/compatibility", "#[test]");
    let (benches, bench_count) = count_number_of_artefacts("benches/compatibility", "#[bench]");
    println!("---------------------------------------------------------------");
    println!("      tests: {}", test_count);
    println!(" benchmarks: {}", bench_count);
    println!("---------------------------------------------------------------");
    println!(" {:29} {:^9} {:^6} {:^6}", "File name", "Tests", "Benchmarks", "Difference");
    println!("---------------------------------------------------------------");
    let mut file_names = BTreeSet::new();
    file_names.append(&mut tests.keys().map(|key| key.to_string()).collect::<BTreeSet<String>>());
    file_names.append(&mut benches.keys().map(|key| key.to_string()).collect::<BTreeSet<String>>());
    for file_name in &file_names {
      let t_count = tests.get(file_name).unwrap_or(&0_usize).to_owned();
      let b_count = benches.get(file_name).unwrap_or(&0_usize).to_owned();
      println!("{:30} {:>6} {:>12} {}", file_name, t_count, b_count, diff(t_count, b_count));
    }
    println!("---------------------------------------------------------------");
    println!("{:30} {:>6} {:>12} {}", "Total", test_count, bench_count, diff(test_count, bench_count));
  }

  /// Counts the number of specific words in a file.
  fn count_number_of_artefacts(root_dir: &str, artefact: &str) -> (BTreeMap<String, usize>, usize) {
    let mut results = BTreeMap::new();
    let mut total = 0_usize;
    for entry_result in WalkDir::new(root_dir).into_iter() {
      match entry_result {
        Ok(entry) => {
          let path = entry.path();
          if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
            let content = fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to load file: {} with reason: {}", path.display(), e));
            let mut count = 0_usize;
            for line in content.lines() {
              if line.trim() == artefact {
                count += 1;
              }
            }
            if count > 1 {
              results.insert(path.strip_prefix(root_dir).unwrap().display().to_string(), count);
              total += count;
            }
          }
        }
        Err(reason) => eprintln!("{}", reason),
      }
    }
    (results, total)
  }

  /// Prepares the difference in text format.
  fn diff(a: usize, b: usize) -> String {
    let diff = a.abs_diff(b);
    if diff > 0 {
      Text::new(ColorMode::On).red().s(format!("{:>9}", diff)).clear().to_string()
    } else {
      "".to_string()
    }
  }
}
