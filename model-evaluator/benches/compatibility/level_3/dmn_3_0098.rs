use super::*;

from_examples!(DMN_3_0098);

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
    eq(b, "date_001", &CTX, r#"38"#);
}

#[bench]
fn _0002(b: &mut Bencher) {
    eq(b, "date_002", &CTX, r#"1"#);
}

#[bench]
fn _0003(b: &mut Bencher) {
    eq(b, "date_003", &CTX, r#"1"#);
}

#[bench]
fn _0004(b: &mut Bencher) {
    eq(b, "date_004", &CTX, r#"1"#);
}

#[bench]
fn _0005(b: &mut Bencher) {
    eq(b, "date_005", &CTX, r#"53"#);
}

#[bench]
fn _0006(b: &mut Bencher) {
    let ctx = context(r#"{date_input_001: @"1970-01-01"}"#);
    eq(b, "date_006", &ctx, r#"1"#);
}

#[bench]
fn _0007(b: &mut Bencher) {
    eq(b, "date_007", &CTX, r#"38"#);
}

#[bench]
fn _0008(b: &mut Bencher) {
    eq(b, "date_008", &CTX, r#"[53, 1, 1]"#);
}

#[bench]
fn _0009(b: &mut Bencher) {
    eq(b, "datetime_001", &CTX, r#"1"#);
}

#[bench]
fn _0010(b: &mut Bencher) {
    let ctx = context(r#"{date_input_001: @"1970-01-01T10:10:10"}"#);
    eq(b, "datetime_004", &ctx, r#"1"#);
}

#[bench]
fn _0011(b: &mut Bencher) {
    eq(b, "datetime_005", &CTX, r#"38"#);
}

#[bench]
fn _0012(b: &mut Bencher) {
    eq(
        b,
        "null_001",
        &CTX,
        r#"null([core::week of year] invalid argument type, expected date, date and time, actual type is Null)"#,
    );
}

#[bench]
fn _0013(b: &mut Bencher) {
    eq(
        b,
        "null_002",
        &CTX,
        r#"null(expected 1 parameters, actual number of parameters is 0)"#,
    );
}

#[bench]
fn _0014(b: &mut Bencher) {
    eq(
        b,
        "null_003",
        &CTX,
        r#"null([core::week of year] invalid argument type, expected date, date and time, actual type is string)"#,
    );
}

#[bench]
fn _0015(b: &mut Bencher) {
    eq(
        b,
        "null_004",
        &CTX,
        r#"null([core::week of year] invalid argument type, expected date, date and time, actual type is Null)"#,
    );
}

#[bench]
fn _0016(b: &mut Bencher) {
    eq(
        b,
        "null_005",
        &CTX,
        r#"null([core::week of year] invalid argument type, expected date, date and time, actual type is string)"#,
    );
}

#[bench]
fn _0017(b: &mut Bencher) {
    eq(b, "null_006", &CTX, r#"null(parameter 'date' not found)"#);
}

#[bench]
fn _0018(b: &mut Bencher) {
    let ctx = context(r#"{date_input_001: "foo"}"#);
    eq(
        b,
        "null_007",
        &ctx,
        r#"null([core::week of year] invalid argument type, expected date, date and time, actual type is string)"#,
    );
}

#[bench]
fn _0019(b: &mut Bencher) {
    eq(
        b,
        "null_008",
        &CTX,
        r#"null(expected 1 parameters, actual number of parameters is 2)"#,
    );
}
