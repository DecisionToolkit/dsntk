use super::*;

from_examples!(DMN_3_0093);

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
    eq(b, "test_001", &CTX, r#"null(invalid @ literal: foo)"#);
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(b, "date_001", &CTX, r#"true"#);
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(b, "date_002", &CTX, r#"2019-03-31"#);
}

#[bench]
fn _0004(b: &mut Bencher) {
    eq(b, "datetime_001", &CTX, r#"true"#);
}

#[bench]
fn _0005(b: &mut Bencher) {
    eq(b, "datetime_002", &CTX, r#"2018-12-08T10:30:11"#);
}

#[bench]
fn _0006(b: &mut Bencher) {
    eq(b, "datetime_003", &CTX, r#"true"#);
}

#[bench]
fn _0007(b: &mut Bencher) {
    eq(
        b,
        "datetime_004",
        &CTX,
        r#""2018-12-08T10:30:11@Australia/Melbourne""#,
    );
}

#[bench]
fn _0008(b: &mut Bencher) {
    eq(b, "datetime_005", &CTX, r#"true"#);
}

#[bench]
fn _0009(b: &mut Bencher) {
    eq(b, "datetime_006", &CTX, r#"2018-12-08T10:30:11+11:00"#);
}

#[bench]
fn _0010(b: &mut Bencher) {
    eq(b, "time_001", &CTX, r#"true"#);
}

#[bench]
fn _0011(b: &mut Bencher) {
    eq(b, "time_002", &CTX, r#"10:30:11"#);
}

#[bench]
fn _0012(b: &mut Bencher) {
    eq(b, "time_003", &CTX, r#"true"#);
}

#[bench]
fn _0013(b: &mut Bencher) {
    eq(b, "time_004", &CTX, r#"10:30:11+11:00"#);
}

#[bench]
fn _0014(b: &mut Bencher) {
    eq(b, "time_005", &CTX, r#"true"#);
}

#[bench]
fn _0015(b: &mut Bencher) {
    eq(b, "time_006", &CTX, r#""10:30:11@Australia/Melbourne""#);
}

#[bench]
fn _0016(b: &mut Bencher) {
    eq(b, "dt_duration_001", &CTX, r#"true"#);
}

#[bench]
fn _0017(b: &mut Bencher) {
    eq(b, "dt_duration_002", &CTX, r#"P10D"#);
}

#[bench]
fn _0018(b: &mut Bencher) {
    eq(b, "ym_duration_001", &CTX, r#"true"#);
}

#[bench]
fn _0019(b: &mut Bencher) {
    eq(b, "ym_duration_002", &CTX, r#"P10Y"#);
}
