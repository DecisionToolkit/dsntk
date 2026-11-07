use super::*;

from_examples!(DMN_3_0030);

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context("{ stringInputA: \"feel\", stringInputB: \"#\" }");
    let invocable_name = "simple function invocation";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        "\"feel#feel#\"",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}

#[bench]
fn _0002(b: &mut Bencher) {
    let ctx = context("{ stringInputA: \"feel\", stringInputB: \"#\" }");
    let invocable_name = "named function invocation";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        "\"#feel#feel\"",
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
