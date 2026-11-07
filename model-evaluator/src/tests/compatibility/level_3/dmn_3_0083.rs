use super::*;

from_examples!(DMN_3_0083);
static_context!(CTX, "{}");

fn test(invocable: &str, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
}

#[test]
fn _0001() {
    test("decision_001", "1");
}

#[test]
fn _0002() {
    test("decision_001_a", "1");
}

#[test]
fn _0003() {
    test("decision_001_b", "true");
}

#[test]
fn _0004() {
    test("decision_002", "6");
}

#[test]
fn _0005() {
    test("decision_003", "1");
}

#[test]
fn _0006() {
    test("decision_003_a", "1");
}

#[test]
fn _0007() {
    test("decision_004", "2");
}

#[test]
fn _0008() {
    test("decision_004_a", "2");
}

#[test]
fn _0009() {
    test("decision_005", "true");
}

#[test]
fn _0010() {
    test("decision_005_a", "true");
}

#[test]
fn _0011() {
    test("decision_006", r#"{🐎: "bar"}"#);
}

#[test]
fn _0012() {
    test("decision_007", r#"{🐎: "😀"}"#);
}

#[test]
fn _0013() {
    test("decision_008", "true");
}

#[test]
fn _0014() {
    test("endswith_001", "true");
}

#[test]
fn _0015() {
    test("substring_001", r#""😀""#);
}

#[test]
fn _0016() {
    test("substring_002", r#""🐎""#);
}

#[test]
fn _0017() {
    test("substring_003", r#""🐎bar""#);
}

#[test]
fn _0018() {
    test("substring_004", "true");
}
