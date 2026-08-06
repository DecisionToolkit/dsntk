//! Runs one of the scaling shapes once, at any size, and reports the time.
//!
//! The benchmarks next door stop at a few thousand items, because a cost that grows with the
//! square of the input makes larger sizes take minutes per iteration and `cargo bench` runs
//! many iterations. This runs the same shapes exactly once, so the large sizes stay usable.
//!
//! ```text
//! cargo run --release --example scaling -- for  200000
//! cargo run --release --example scaling -- list 200000
//! ```
//!
//! For peak memory, wrap it in the system timer rather than adding a dependency:
//!
//! ```text
//! /usr/bin/time -l cargo run --release --example scaling -- for 200000   # macOS
//! /usr/bin/time -v cargo run --release --example scaling -- for 200000   # Linux
//! ```
//!
//! `for` evaluates `count(for item in 1..N return item + 1)`, which runs N iterations. `list`
//! parses a literal holding N numbers. Compare the same command across two revisions, and
//! compare each revision against itself at two sizes: four times the items should cost about
//! four times the time.

use dsntk_feel::values::Value;
use dsntk_feel::{FeelScope, scope};
use dsntk_feel_evaluator::prepare;
use std::time::Instant;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let shape = args.get(1).cloned().unwrap_or_else(|| "for".to_string());
  let count: usize = args.get(2).and_then(|a| a.parse().ok()).unwrap_or(200_000);
  let scope = scope!();
  match shape.as_str() {
    "for" => {
      let input = format!("count(for item in 1..{count} return item + 1)");
      let started = Instant::now();
      let node = dsntk_feel_parser::parse_expression(&scope, &input, false).expect("parsing failed");
      let evaluator = prepare(&node);
      let result = evaluator(&scope);
      let elapsed = started.elapsed();
      assert_eq!(Value::Number(count.into()), result, "the loop did not run {count} times");
      println!("for   items {count:>9}   {:>10.3} s", elapsed.as_secs_f64());
    }
    "list" => {
      let mut input = String::with_capacity(count * 8 + 2);
      input.push('[');
      for index in 0..count {
        if index > 0 {
          input.push(',');
        }
        input.push_str(&index.to_string());
      }
      input.push(']');
      let started = Instant::now();
      let node = dsntk_feel_parser::parse_expression(&scope, &input, false).expect("parsing failed");
      let elapsed = started.elapsed();
      assert!(matches!(node, dsntk_feel_parser::AstNode::List(_)), "expected a list");
      println!("list  items {count:>9}   {:>10.3} s", elapsed.as_secs_f64());
    }
    other => {
      eprintln!("unknown shape `{other}`, expected `for` or `list`");
      std::process::exit(1);
    }
  }
}
