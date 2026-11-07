use super::*;

from_examples!(DMN_3_1147);

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
    eq(b, "decision001", &CTX, "{a: 1}");
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(b, "decision001_a", &CTX, "true");
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(b, "decision002", &CTX, "{a: 1, b: 2}");
}

#[bench]
fn _0004(b: &mut Bencher) {
    eq(b, "decision003", &CTX, "{a: 2}");
}

#[bench]
fn _0005(b: &mut Bencher) {
    eq(b, "decision004", &CTX, "{a: {bb: 2}}");
}

#[bench]
fn _0006(b: &mut Bencher) {
    eq(b, "decision005", &CTX, "null");
}

#[bench]
fn _0007(b: &mut Bencher) {
    eq(
        b,
        "decision006",
        &CTX,
        "null(expected 1 parameters, actual number of parameters is 0)",
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    eq(
        b,
        "decision007",
        &CTX,
        "null(expected 1 parameters, actual number of parameters is 2)",
    );
}

#[bench]
fn _0009(b: &mut Bencher) {
    eq(b, "decision008", &CTX, "{a: 1}");
}

#[bench]
fn _0010(b: &mut Bencher) {
    eq(
        b,
        "decision009",
        &CTX,
        "null(parameter 'contexts' not found)",
    );
}

#[bench]
fn _0011(b: &mut Bencher) {
    eq(
        b,
        "decision010",
        &CTX,
        "null([core::context merge] invalid argument type, expected context, actual type is number)",
    );
}

#[bench]
fn _0012(b: &mut Bencher) {
    eq(
        b,
        "decision011",
        &CTX,
        "null([core::context merge] invalid argument type, expected context, actual type is number)",
    );
}

#[bench]
fn _0013(b: &mut Bencher) {
    eq(b, "decision012", &CTX, "{a: 1}");
}

#[bench]
fn _0014(b: &mut Bencher) {
    eq(b, "decision013", &CTX, "{a: 1}");
}
