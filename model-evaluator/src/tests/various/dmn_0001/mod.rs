//! ???

use super::super::*;

const DMN_0001: &str = include_str!("_0001.dmn");

model_evaluator!(DMN_0001);

const MODEL_NAMESPACE: &str = "https://dsntk.io/selector";
const MODEL_NAME: &str = "selector";

#[test]
fn _0001() {
    let ctx = context(r#"{ Days: 6, Bounds: { Min: null, Max: 360 } }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "Selector",
        &ctx,
        r#""7a""#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{ Days: 7, Bounds: { Min: 7, Max: 360 } }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "Selector",
        &ctx,
        r#""7b""#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{ Days: 8, Bounds: { Min: 7, Max: 360 } }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "Selector",
        &ctx,
        r#""(7..360)""#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{ Days: 360, Bounds: { Min: 7, Max: 360 } }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "Selector",
        &ctx,
        r#""360+""#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{ Days: 365, Bounds: { Min: 7, Max: 360 } }"#);
    assert_decision(
        &MODEL_EVALUATOR,
        MODEL_NAMESPACE,
        MODEL_NAME,
        "Selector",
        &ctx,
        r#""360+""#,
    );
}
