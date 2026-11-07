use super::*;

from_examples!(DMN_3_0030);

#[test]
fn _0001() {
    let ctx = context("{ stringInputA: \"feel\", stringInputB: \"#\" }");
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "simple function invocation",
        &ctx,
        "\"feel#feel#\"",
    );
}

#[test]
fn _0002() {
    let ctx = context("{ stringInputA: \"feel\", stringInputB: \"#\" }");
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "named function invocation",
        &ctx,
        "\"#feel#feel\"",
    );
}
