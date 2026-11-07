use super::*;

from_examples!(DMN_3_1106);

#[test]
fn _0001() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-lower-case-function_001_f6ff05bcfa",
        &ctx,
        r#""a""#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-lower-case-function_002_0ecb21e1d8",
        &ctx,
        r#""abc""#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-lower-case-function_003_af9f3a8dab",
        &ctx,
        r#""abc""#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-lower-case-function_004_2dbf99e8c0",
        &ctx,
        r#""abc4""#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-lower-case-function_005_ad33325968",
        &ctx,
        r#""""#,
    );
}

#[test]
fn _0006() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-lower-case-function_006_d8257b3b92",
        &ctx,
        r#""?@{""#,
    );
}

#[test]
fn _0007() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-lower-case-function_007_a2ce59499a",
        &ctx,
        r#""abdcf""#,
    );
}

#[test]
fn _0008() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-lower-case-function_008_9913ad454f",
        &ctx,
        r#""xyz ""#,
    );
}

#[test]
fn _0009() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "feel-lower-case-function_009_78e6b2969b",
        &ctx,
        r#""123abc""#,
    );
}
