use super::*;

from_examples!(DMN_3_0097);

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
    let ctx = context(r#"{ date_input_001: @"2021-01-11" }"#);
    eq(
        "date_001",
        &ctx,
        r#"{_2021-01-01: "January", _2021-02-01: "February", _2021-03-01: "March", _2021-04-01: "April", _2021-05-01: "May", _2021-06-01: "June", _2021-07-01: "July", _2021-08-01: "August", _2021-09-01: "September", _2021-10-01: "October", _2021-11-01: "November", _2021-12-01: "December"}"#,
    );
}

#[test]
fn _0002() {
    eq("date_002", &CTX, r#""September""#);
}

#[test]
fn _0003() {
    let ctx = context(r#"{ date_input_001: @"2021-01-11T10:10:10" }"#);
    eq(
        "date_001",
        &ctx,
        r#"{_2021-01-01: "January", _2021-02-01: "February", _2021-03-01: "March", _2021-04-01: "April", _2021-05-01: "May", _2021-06-01: "June", _2021-07-01: "July", _2021-08-01: "August", _2021-09-01: "September", _2021-10-01: "October", _2021-11-01: "November", _2021-12-01: "December"}"#,
    );
}

#[test]
fn _0004() {
    eq("datetime_002", &CTX, r#""September""#);
}

#[test]
fn _0005() {
    eq(
        "null_001",
        &CTX,
        r#"null([core::month of year] invalid argument type, expected date, date and time, actual type is Null)"#,
    );
}

#[test]
fn _0006() {
    eq(
        "null_002",
        &CTX,
        r#"null(expected 1 parameters, actual number of parameters is 0)"#,
    );
}

#[test]
fn _0007() {
    eq(
        "null_003",
        &CTX,
        r#"null([core::month of year] invalid argument type, expected date, date and time, actual type is string)"#,
    );
}

#[test]
fn _0008() {
    eq(
        "null_004",
        &CTX,
        r#"null([core::month of year] invalid argument type, expected date, date and time, actual type is Null)"#,
    );
}

#[test]
fn _0009() {
    eq(
        "null_005",
        &CTX,
        r#"null([core::month of year] invalid argument type, expected date, date and time, actual type is string)"#,
    );
}

#[test]
fn _0010() {
    eq("null_006", &CTX, r#"null(parameter 'date' not found)"#);
}

#[test]
fn _0011() {
    let ctx = context(r#"{ date_input_001: "foo" }"#);
    eq(
        "null_007",
        &ctx,
        r#"null([core::month of year] invalid argument type, expected date, date and time, actual type is string)"#,
    );
}

#[test]
fn _0012() {
    eq(
        "null_008",
        &CTX,
        r#"null(expected 1 parameters, actual number of parameters is 2)"#,
    );
}
