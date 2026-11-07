use super::*;

from_examples!(DMN_3_0095);

static_context!(CTX, "{}");

fn eq(invocable: &str, ctx: &FeelContext, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        ctx,
        expected,
    );
}

#[test]
fn _0001() {
    eq("date_001", &CTX, r#"1"#);
}

#[test]
fn _0002() {
    eq("date_002", &CTX, r#"365"#);
}

#[test]
fn _0003() {
    eq("date_003", &CTX, r#"366"#);
}

#[test]
fn _0004() {
    eq("date_004", &CTX, r#"260"#);
}

#[test]
fn _0005() {
    let ctx = context(r#"{ date_input_001: @"1970-01-01" }"#);
    eq("date_005", &ctx, r#"1"#);
}

#[test]
fn _0006() {
    eq("date_006", &CTX, r#"260"#);
}

#[test]
fn _0007() {
    eq("datetime_001", &CTX, r#"1"#);
}

#[test]
fn _0008() {
    eq("datetime_002", &CTX, r#"365"#);
}

#[test]
fn _0009() {
    eq("datetime_003", &CTX, r#"366"#);
}

#[test]
fn _0010() {
    let ctx = context(r#"{ date_input_001: @"1970-01-01" }"#);
    eq("datetime_004", &ctx, r#"1"#);
}

#[test]
fn _0011() {
    eq("datetime_005", &CTX, r#"260"#);
}

#[test]
fn _0012() {
    eq(
        "null_001",
        &CTX,
        r#"null([core::day of year] invalid argument type, expected date, date and time, actual type is Null)"#,
    );
}

#[test]
fn _0013() {
    eq(
        "null_002",
        &CTX,
        r#"null(expected 1 parameters, actual number of parameters is 0)"#,
    );
}

#[test]
fn _0014() {
    eq(
        "null_003",
        &CTX,
        r#"null([core::day of year] invalid argument type, expected date, date and time, actual type is string)"#,
    );
}

#[test]
fn _0015() {
    eq(
        "null_004",
        &CTX,
        r#"null([core::day of year] invalid argument type, expected date, date and time, actual type is Null)"#,
    );
}

#[test]
fn _0016() {
    eq(
        "null_005",
        &CTX,
        r#"null([core::day of year] invalid argument type, expected date, date and time, actual type is string)"#,
    );
}

#[test]
fn _0017() {
    eq("null_006", &CTX, r#"null(parameter 'date' not found)"#);
}

#[test]
fn _0018() {
    let ctx = context(r#"{ date_input_001: "foo" }"#);
    eq(
        "null_007",
        &ctx,
        r#"null([core::day of year] invalid argument type, expected date, date and time, actual type is string)"#,
    );
}

#[test]
fn _0019() {
    eq(
        "null_008",
        &CTX,
        r#"null(expected 1 parameters, actual number of parameters is 2)"#,
    );
}
