use dsntk_feel_grammar::lalr_rust_tables;
use std::process::Command;

fn main() {
    if std::env::var("CARGO_FEATURE_PARSING_TABLES").is_ok() {
        // generate new LALR parsing tables for FEEL grammar
        // when executed with --features=parsing-tables
        lalr_rust_tables("./src/lalr.rs");
        // reformat the generated code
        let status = Command::new("cargo")
            .args(["+nightly", "fmt", "-p", "dsntk-feel-parser"])
            .status()
            .expect("failed to run code formatter");
        if !status.success() {
            panic!("code formatter failed");
        }
    }
}
