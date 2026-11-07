use super::*;

from_examples!(DMN_3_1153);

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
    eq(b, "decision002", &CTX, "false");
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(b, "decision003", &CTX, "null");
}

#[bench]
fn _0004(b: &mut Bencher) {
    eq(b, "decision004", &CTX, "null");
}

#[bench]
fn _0005(b: &mut Bencher) {
    eq(b, "decision005", &CTX, "null");
}

#[bench]
fn _0006(b: &mut Bencher) {
    eq(b, "decision006", &CTX, "null");
}

#[bench]
fn _0007(b: &mut Bencher) {
    eq(b, "decision007", &CTX, "false");
}
