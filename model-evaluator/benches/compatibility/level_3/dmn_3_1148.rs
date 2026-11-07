use super::*;

from_examples!(DMN_3_1148);

static_context!(CTX, "{}");

fn eq(b: &mut Bencher, invocable: &str, ctx: &FeelContext, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        ctx,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, ctx)
    );
}

#[bench]
fn _0001(b: &mut Bencher) {
    eq(b, "decision001", &CTX, "true");
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(
        b,
        "decision002",
        &CTX,
        "null(expected 0 parameters, actual number of parameters is 1)",
    );
}
