use super::*;

from_examples!(DMN_3_1150);

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
    eq(b, "decision001", &CTX, r#""then""#);
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(b, "decision002", &CTX, r#""else""#);
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(
        b,
        "decision003",
        &CTX,
        "null(condition is not a boolean expression)",
    );
}
