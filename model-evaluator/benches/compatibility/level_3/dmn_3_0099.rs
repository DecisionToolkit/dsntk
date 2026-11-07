use super::*;

from_examples!(DMN_3_0099);

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
    eq(b, "decision_001", &CTX, r#"-10"#);
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(b, "decision_002", &CTX, r#"10"#);
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(b, "decision_003", &CTX, r#"-P1D"#);
}

#[bench]
fn _0004(b: &mut Bencher) {
    eq(b, "decision_003_a", &CTX, r#"P1D"#);
}

#[bench]
fn _0005(b: &mut Bencher) {
    eq(b, "decision_004", &CTX, r#"-P1Y"#);
}

#[bench]
fn _0006(b: &mut Bencher) {
    eq(b, "decision_004_a", &CTX, r#"P1Y"#);
}

#[bench]
fn _0007(b: &mut Bencher) {
    eq(
        b,
        "decision_005",
        &CTX,
        r#"null(unexpected type in arithmetic negation: date)"#,
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    eq(
        b,
        "decision_006",
        &CTX,
        r#"null(unexpected type in arithmetic negation: date and time)"#,
    );
}

#[bench]
fn _0009(b: &mut Bencher) {
    eq(
        b,
        "decision_007",
        &CTX,
        r#"null(unexpected type in arithmetic negation: time)"#,
    );
}

#[bench]
fn _0010(b: &mut Bencher) {
    eq(
        b,
        "decision_008",
        &CTX,
        r#"null(unexpected type in arithmetic negation: context<a: number>)"#,
    );
}

#[bench]
fn _0011(b: &mut Bencher) {
    eq(
        b,
        "decision_009",
        &CTX,
        r#"null(unexpected type in arithmetic negation: string)"#,
    );
}

#[bench]
fn _0012(b: &mut Bencher) {
    eq(
        b,
        "decision_010",
        &CTX,
        r#"null(unexpected type in arithmetic negation: list<number>)"#,
    );
}

#[bench]
fn _0013(b: &mut Bencher) {
    eq(
        b,
        "decision_011",
        &CTX,
        r#"null(unexpected type in arithmetic negation: range<number>)"#,
    );
}

#[bench]
fn _0014(b: &mut Bencher) {
    eq(b, "decision_012", &CTX, r#"-10"#);
}
