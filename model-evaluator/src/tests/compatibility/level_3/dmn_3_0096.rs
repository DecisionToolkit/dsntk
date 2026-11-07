use super::*;

from_examples!(DMN_3_0096);

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
        r#"{_2021-01-11: "Monday", _2021-01-12: "Tuesday", _2021-01-13: "Wednesday", _2021-01-14: "Thursday", _2021-01-15: "Friday", _2021-01-16: "Saturday", _2021-01-17: "Sunday"}"#,
    );
}

#[test]
fn _0002() {
    eq("date_002", &CTX, r#""Tuesday""#);
}

#[test]
fn _0003() {
    let ctx = context(r#"{ date_input_001: @"2021-01-11T10:10:10" }"#);
    eq(
        "date_001",
        &ctx,
        r#"{_2021-01-11: "Monday", _2021-01-12: "Tuesday", _2021-01-13: "Wednesday", _2021-01-14: "Thursday", _2021-01-15: "Friday", _2021-01-16: "Saturday", _2021-01-17: "Sunday"}"#,
    );
}

#[test]
fn _0004() {
    eq("datetime_002", &CTX, r#""Tuesday""#);
}

#[test]
fn _0005() {
    eq(
        "null_001",
        &CTX,
        r#"null([core::day of week] invalid argument type, expected date, date and time, actual type is Null)"#,
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
        r#"null([core::day of week] invalid argument type, expected date, date and time, actual type is string)"#,
    );
}

#[test]
fn _0008() {
    eq(
        "null_004",
        &CTX,
        r#"null([core::day of week] invalid argument type, expected date, date and time, actual type is Null)"#,
    );
}

#[test]
fn _0009() {
    eq(
        "null_005",
        &CTX,
        r#"null([core::day of week] invalid argument type, expected date, date and time, actual type is string)"#,
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
        r#"null([core::day of week] invalid argument type, expected date, date and time, actual type is string)"#,
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
