//! ???

use super::super::*;

const DMN_0002: &str = include_str!("_0002.dmn");

model_evaluator!(DMN_0002);

const MODEL_NAMESPACE: &str = "https://dsntk.io/coercion";
const MODEL_NAME: &str = "coercion";

#[test]
fn _0001() {
    let ctx = context(
        r#"{ 
    Bounds: [
      { Max: 360, Min: 12 }
    ]
  }"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "Echo",
        &ctx,
        r#"[{Max: 360, Min: 12}]"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(
        r#"{ 
    Bounds: [
      { Max: 360, Min: 102 },
      { Max: 267, Min: 108 }
    ]
  }"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "Echo",
        &ctx,
        r#"[{Max: 360, Min: 102}, {Max: 267, Min: 108}]"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(
        r#"{ 
    Bounds: [
      { Max: 360, Min: 102 },
      { Max: 267, Min: null }
    ]
  }"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "Echo",
        &ctx,
        r#"[{Max: 360, Min: 102}, {Max: 267, Min: null}]"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(
        r#"{ 
    Bounds: [
      { Max: null, Min: 102 },
      { Max: 267, Min: null }
    ]
  }"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "Echo",
        &ctx,
        r#"[{Max: null, Min: 102}, {Max: 267, Min: null}]"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(
        r#"{ 
    Bounds: [
      { Max: null, Min: null },
      { Max: 267, Min: null },
      { Max: null, Min: 84 },
      { Max: 12, Min: 3 }
    ]
  }"#,
    );
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "Echo",
        &ctx,
        r#"[{Max: null, Min: null}, {Max: 267, Min: null}, {Max: null, Min: 84}, {Max: 12, Min: 3}]"#,
    );
}
