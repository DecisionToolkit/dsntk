use super::*;

from_examples!(DMN_3_1151);

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
    eq(b, "decision001", &CTX, "[3, 4, 5]");
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(b, "decision002", &CTX, "[]");
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(b, "decision003", &CTX, r#"["not a list"]"#);
}

#[bench]
fn _0004(b: &mut Bencher) {
    eq(
        b,
        "decision004",
        &CTX,
        "null(only number or boolean indexes are allowed in filters)",
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    eq(
        b,
        "decision005",
        &CTX,
        "null(only number or boolean indexes are allowed in filters)",
    );
}

#[bench]
fn _0006(b: &mut Bencher) {
    eq(
        b,
        "decision006",
        &CTX,
        "null(only number or boolean indexes are allowed in filters)",
    );
}
