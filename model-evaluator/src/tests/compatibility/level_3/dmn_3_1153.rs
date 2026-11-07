use super::*;

from_examples!(DMN_3_1153);

static_context!(CTX, "{}");

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
    eq("decision001", &CTX, "true");
}

#[test]
fn _0002() {
    eq("decision002", &CTX, "false");
}

#[test]
fn _0003() {
    eq("decision003", &CTX, "null");
}

#[test]
fn _0004() {
    eq("decision004", &CTX, "null");
}

#[test]
fn _0005() {
    eq("decision005", &CTX, "null");
}

#[test]
fn _0006() {
    eq("decision006", &CTX, "null");
}

#[test]
fn _0007() {
    eq("decision007", &CTX, "false");
}
