use super::*;

from_examples!(DMN_3_1145);

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
    eq(b, "decision001", &CTX, "{a: 1, b: 2}");
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(b, "decision002", &CTX, "{a: 1}");
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(b, "decision003", &CTX, "null(context: duplicated key: a)");
}

#[bench]
fn _0004(b: &mut Bencher) {
    eq(b, "decision004", &CTX, "true");
}

#[bench]
fn _0005(b: &mut Bencher) {
    eq(b, "decision005", &CTX, "{a: 1}");
}

#[bench]
fn _0006(b: &mut Bencher) {
    eq(
        b,
        "decision006",
        &CTX,
        "null(context: no 'key' entry in context {value: 1})",
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    eq(
        b,
        "decision007",
        &CTX,
        "null(context: 'key' entry is not a string, actual type is Null)",
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    eq(
        b,
        "decision008",
        &CTX,
        r#"null(context: no 'value' entry in context {key: "a"})"#,
    );
}

#[bench]
fn _0009(b: &mut Bencher) {
    eq(b, "decision009", &CTX, "{a: null}");
}

#[bench]
fn _0010(b: &mut Bencher) {
    eq(b, "decision010", &CTX, r#"{"": 1}"#);
}

#[bench]
fn _0011(b: &mut Bencher) {
    eq(b, "decision011", &CTX, "null");
}

#[bench]
fn _0012(b: &mut Bencher) {
    eq(
        b,
        "decision012",
        &CTX,
        "null(expected 1 parameters, actual number of parameters is 0)",
    );
}

#[bench]
fn _0013(b: &mut Bencher) {
    eq(
        b,
        "decision013",
        &CTX,
        "null(expected 1 parameters, actual number of parameters is 2)",
    );
}

#[bench]
fn _0014(b: &mut Bencher) {
    eq(b, "decision014_a", &CTX, "{a: 1}");
}

#[bench]
fn _0015(b: &mut Bencher) {
    eq(b, "decision014_b", &CTX, "{a: 1}");
}

#[bench]
fn _0016(b: &mut Bencher) {
    eq(
        b,
        "decision015",
        &CTX,
        "null(parameter 'entries' not found)",
    );
}

#[bench]
fn _0017(b: &mut Bencher) {
    eq(
        b,
        "decision016",
        &CTX,
        "null([core::context] invalid argument type, expected list or context, actual type is string)",
    );
}

#[bench]
fn _0018(b: &mut Bencher) {
    eq(b, "decision017", &CTX, "{a: 1}");
}
