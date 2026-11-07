use super::*;

from_examples!(DMN_3_1101);

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
    eq(b, "decision001", &CTX, "1");
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(b, "decision002", &CTX, "-2");
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(b, "decision003", &CTX, "0");
}

#[bench]
fn _0004(b: &mut Bencher) {
    eq(b, "decision004", &CTX, "1.5");
}

#[bench]
fn _0005(b: &mut Bencher) {
    eq(b, "decision005", &CTX, "-1.6");
}

#[bench]
fn _0006(b: &mut Bencher) {
    eq(
        b,
        "decision006",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is Null)",
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    eq(
        b,
        "decision007",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is Null)",
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    eq(
        b,
        "decision008",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is Null)",
    );
}

#[bench]
fn _0009(b: &mut Bencher) {
    eq(
        b,
        "decision009",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is Null)",
    );
}

#[bench]
fn _0010(b: &mut Bencher) {
    eq(
        b,
        "decision010",
        &CTX,
        "null(expected 1,2 parameters, actual number of parameters is 0)",
    );
}

#[bench]
fn _0011(b: &mut Bencher) {
    eq(
        b,
        "decision011",
        &CTX,
        "null(expected 1,2 parameters, actual number of parameters is 3)",
    );
}

#[bench]
fn _0012(b: &mut Bencher) {
    eq(
        b,
        "decision012",
        &CTX,
        "null(expected 1,2 parameters, actual number of parameters is 3)",
    );
}

#[bench]
fn _0013(b: &mut Bencher) {
    eq(b, "decision013", &CTX, "null(parameter 'scale' not found)");
}

#[bench]
fn _0014(b: &mut Bencher) {
    eq(
        b,
        "decision014",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is string)",
    );
}

#[bench]
fn _0015(b: &mut Bencher) {
    eq(
        b,
        "decision015",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is string)",
    );
}

#[bench]
fn _0016(b: &mut Bencher) {
    eq(
        b,
        "decision016",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is string)",
    );
}

#[bench]
fn _0017(b: &mut Bencher) {
    eq(
        b,
        "decision017",
        &CTX,
        "null([core::floor] invalid argument type, expected number, actual type is string)",
    );
}
