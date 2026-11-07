use super::*;

from_examples!(DMN_3_1150);

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
    eq("decision001", &CTX, r#""then""#);
}

#[test]
fn _0002() {
    eq("decision002", &CTX, r#""else""#);
}

#[test]
fn _0003() {
    eq(
        "decision003",
        &CTX,
        "null(condition is not a boolean expression)",
    );
}
