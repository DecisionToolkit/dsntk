use super::*;

from_examples!(DMN_3_0092);

static_context!(CTX, r#"{}"#);

fn eq(invocable: &str, ctx: &FeelContext, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        ctx,
        expected,
    );
}

#[test]
fn _0001() {
    eq("decision_001_1", &CTX, r#"3"#);
}

#[test]
fn _0002() {
    eq("decision_002_1", &CTX, r#"4"#);
}

#[test]
fn _0003() {
    eq("decision_003_1", &CTX, r#"5"#);
}

#[test]
fn _0004() {
    eq("decision_004_1", &CTX, r#"6"#);
}

#[test]
fn _0005() {
    eq("decision_005_1", &CTX, r#"20"#);
}

#[test]
fn _0006() {
    eq("decision_006_1", &CTX, r#"30"#);
}

#[test]
fn _0007() {
    let ctx = context(r#"{input_007_1: 20}"#);
    eq("decision_007_1", &ctx, r#"100"#);
}

#[test]
fn _0008() {
    eq("decision_008_1", &CTX, r#"6"#);
}

#[test]
fn _0009() {
    eq("decision_009_1", &CTX, r#"200"#);
}

#[test]
fn _0010() {
    eq("decision_010_1", &CTX, r#"120"#);
}

#[test]
fn _0011() {
    eq("decision_010_1_a", &CTX, r#"120"#);
}

#[test]
fn _0012() {
    let ctx = context(r#"{input_011_1: 10}"#);
    eq("decision_011_1", &ctx, r#"5000"#);
}

#[test]
fn _0013() {
    eq("decision_012_1", &CTX, r#"5000"#);
}

#[test]
fn _0014() {
    eq("decision_013_1", &CTX, r#"5000"#);
}

#[test]
fn _0015() {
    eq("decision_014_1", &CTX, r#"25"#);
}

#[test]
fn _0016() {
    eq("decision_015_1", &CTX, r#"18"#);
}

#[test]
fn _0017() {
    eq(
        "decision_016_1",
        &CTX,
        r#"null(expected 1 parameters, actual number of parameters is 2)"#,
    );
}

#[test]
fn _0018() {
    let ctx = context(r#"{input_017_1: "a"}"#);
    eq("decision_017_1", &ctx, r#"["a", "a", "z", "z"]"#);
}

#[test]
fn _0019() {
    eq("decision_018", &CTX, r#"["a", "a", "z", "z"]"#);
}
