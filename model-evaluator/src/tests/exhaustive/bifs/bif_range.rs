use super::*;

from_examples!(EXHAUSTIVE_BIFS_BIF_RANGE);

static_context!(CTX, "{}");

fn eq(invocable_name: &str, input: &FeelContext, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        input,
        expected,
    );
}

#[test]
fn _0001() {
    eq(
        "decision_0001",
        &CTX,
        "[[1..3], [1..3), (1..3], (1..3), [1..3), (1..3], (1..3), [1..), (..3]]",
    );
}

#[test]
fn _0002() {
    eq("decision_0002", &CTX, r#"[["a".."c"]]"#);
}

#[test]
fn _0003() {
    eq("decision_0100", &CTX, "[true, true]");
}
