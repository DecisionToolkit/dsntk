//! Test for converting decision tables defined as text into HTML format.

use super::*;
use dsntk_examples::decision_tables::*;
use dsntk_model::DecisionTable;

/// Utility function for generating HTML file for decision table defined as text.
fn generate_html(dec_tab: &str, output_file_name: &str) {
    let decision_table: DecisionTable = dsntk_recognizer::from_unicode(dec_tab, false)
        .expect("building decision table failed")
        .into();
    let html = crate::decision_table_to_html(&decision_table);
    assert_eq!("<!DOCTYPE html>", &html[0..15]);
    fs::create_dir_all(TARGET_DIR).expect("creating target directories failed");
    let mut file = File::create(format!("{TARGET_DIR}/{output_file_name}.html"))
        .expect("creating HTML file failed");
    file.write_all(html.as_bytes())
        .expect("saving HTML file failed");
}

/// Utility macro for generating HTML file for decision table defined as text.
macro_rules! generate_html {
    ($t:tt) => {{
        generate_html($t, stringify!($t));
    }};
}

#[test]
fn _0001() {
    generate_html!(H_000010);
}

#[test]
fn _0002() {
    generate_html!(H_000011);
}

#[test]
fn _0003() {
    generate_html!(H_000020);
}

#[test]
fn _0004() {
    generate_html!(H_000021);
}

#[test]
fn _0005() {
    generate_html!(H_000210);
}

#[test]
fn _0006() {
    generate_html!(H_110010);
}

#[test]
fn _0007() {
    generate_html!(H_001010);
}

#[test]
fn _0008() {
    generate_html!(H_001020);
}

#[test]
fn _0009() {
    generate_html!(H_001210);
}

#[test]
fn _0010() {
    generate_html!(H_010010);
}

#[test]
fn _0011() {
    generate_html!(H_010210);
}

#[test]
fn _0012() {
    generate_html!(H_011221);
}

#[test]
fn _0013() {
    generate_html!(H_011222);
}

#[test]
fn _0014() {
    generate_html!(H_101222);
}

#[test]
fn _0015() {
    generate_html!(H_110010);
}

#[test]
fn _0016() {
    generate_html!(H_110222);
}

#[test]
fn _0017() {
    generate_html!(H_111222);
}
