use super::*;

from_examples!(DMN_3_1149);

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
    eq(
        "decision002",
        &CTX,
        "null(expected 0 parameters, actual number of parameters is 1)",
    );
}
