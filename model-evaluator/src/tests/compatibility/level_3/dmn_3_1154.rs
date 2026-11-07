use super::*;

from_examples!(DMN_3_1154);

static_context!(CTX, "{}");

fn eq(invocable: &str, expected: &str) {
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
    eq("decision001", "true");
}

#[test]
fn _0002() {
    eq("decision002", "false");
}

#[test]
fn _0003() {
    eq("decision003", "null");
}

#[test]
fn _0004() {
    eq("decision004", "null");
}

#[test]
fn _0005() {
    eq("decision005", "null");
}

#[test]
fn _0006() {
    eq("decision006", "null");
}

#[test]
fn _0007() {
    eq("decision007", "true");
}
