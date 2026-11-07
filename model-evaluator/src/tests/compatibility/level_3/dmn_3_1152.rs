use super::*;

from_examples!(DMN_3_1152);

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
    eq("decision001", &CTX, "[2, 4, 6, 8, 10]");
}

#[test]
fn _0002() {
    eq("decision002", &CTX, "[]");
}

#[test]
fn _0003() {
    eq("decision003", &CTX, "[]");
}

#[test]
fn _0004() {
    eq("decision004", &CTX, "[]");
}
