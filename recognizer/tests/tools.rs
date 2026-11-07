//! # Tools for testing the recognizer

use std::fs;
use walkdir::WalkDir;

/// Checks if all tests are correct for horizontal tables.
#[test]
fn check_decision_tables_tests() {
    let mut ok = true;
    let dir_examples_horizontal = "../examples/src/decision_tables/horizontal";
    for entry_result in WalkDir::new(dir_examples_horizontal).into_iter() {
        let entry = entry_result.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            let file_name = path
                .strip_prefix(dir_examples_horizontal)
                .unwrap()
                .display()
                .to_string();
            if file_name.starts_with("H_") {
                let decision_table_name = file_name.trim_end_matches(".md").to_uppercase();
                let unicode_test = format!("from_unicode({}, false)", decision_table_name);
                let markdown_test = format!("from_markdown({}, false)", decision_table_name);
                let test_file_name = format!(
                    "./tests/valid_decision_tables/horizontal/{}.rs",
                    decision_table_name.to_lowercase()
                );
                ok &= if let Ok(content) = fs::read_to_string(test_file_name.clone()) {
                    match (content.find(&unicode_test), content.find(&markdown_test)) {
                        (Some(_), Some(_)) => true,
                        (None, Some(_)) => {
                            println!("{} NO UNICODE TEST", decision_table_name);
                            false
                        }
                        (Some(_), None) => {
                            println!("{} NO MARKDOWN TEST", decision_table_name);
                            false
                        }
                        (None, None) => {
                            println!("{} NO UNICODE AND MARKDOWN TESTS", decision_table_name);
                            false
                        }
                    }
                } else {
                    println!("{} NO TEST FILE {}", decision_table_name, test_file_name);
                    false
                }
            }
        }
    }
    assert!(ok, "ERROR: not all tests were found.");
}
