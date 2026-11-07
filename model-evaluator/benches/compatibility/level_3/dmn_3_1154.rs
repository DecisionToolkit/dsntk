use super::*;

from_examples!(DMN_3_1154);

static_context!(CTX, "{}");

fn eq(b: &mut Bencher, invocable: &str, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        &CTX,
        expected,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX)
    );
}

#[bench]
fn _0001(b: &mut Bencher) {
    eq(b, "decision001", "true");
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(b, "decision002", "false");
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(b, "decision003", "null");
}

#[bench]
fn _0004(b: &mut Bencher) {
    eq(b, "decision004", "null");
}

#[bench]
fn _0005(b: &mut Bencher) {
    eq(b, "decision005", "null");
}

#[bench]
fn _0006(b: &mut Bencher) {
    eq(b, "decision006", "null");
}

#[bench]
fn _0007(b: &mut Bencher) {
    eq(b, "decision007", "true");
}
