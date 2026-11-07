use super::*;

from_examples!(DMN_3_0094);

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
    eq(b, "decision001", &CTX, r#"5832"#);
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(b, "decision002", &CTX, r#"null"#);
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(
        b,
        "decision003",
        &CTX,
        r#"null(expected 1+ parameters, actual number of parameters is 0)"#,
    );
}

#[bench]
fn _0004(b: &mut Bencher) {
    eq(
        b,
        "decision003_a",
        &CTX,
        r#"null([core::product] invalid argument type, expected number, actual type is Null)"#,
    );
}

#[bench]
fn _0005(b: &mut Bencher) {
    eq(
        b,
        "decision004",
        &CTX,
        r#"null([core::product] invalid argument type, expected number, actual type is Null)"#,
    );
}

#[bench]
fn _0006(b: &mut Bencher) {
    eq(
        b,
        "decision005",
        &CTX,
        r#"null([core::product] invalid argument type, expected number, actual type is string)"#,
    );
}

#[bench]
fn _0007(b: &mut Bencher) {
    eq(b, "decision006", &CTX, r#"18.75"#);
}

#[bench]
fn _0008(b: &mut Bencher) {
    eq(b, "decision007", &CTX, r#"24"#);
}

#[bench]
fn _0009(b: &mut Bencher) {
    eq(b, "decision008", &CTX, r#"6"#);
}

#[bench]
fn _0010(b: &mut Bencher) {
    eq(b, "decision008_a", &CTX, r#"6"#);
}

#[bench]
fn _0011(b: &mut Bencher) {
    eq(b, "decision011", &CTX, r#"5832"#);
}

#[bench]
fn _0012(b: &mut Bencher) {
    eq(
        b,
        "decision012",
        &CTX,
        r#"null(parameter 'list' not found)"#,
    );
}

#[bench]
fn _0013(b: &mut Bencher) {
    eq(
        b,
        "decision013",
        &CTX,
        r#"null(parameter 'list' not found)"#,
    );
}
