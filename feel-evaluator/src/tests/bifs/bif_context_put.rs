use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_context(false, &scope!(), r#"context put({}, "a", 1)"#, "{a: 1}");
}

#[test]
fn _0002() {
    te_be_value(
        false,
        &scope!(),
        r#"get entries(context put({"a": 1}, "b", 2))"#,
        r#"[{key: "a", value: 1}, {key: "b", value: 2}]"#,
    );
}

#[test]
fn _0003() {
    te_context(
        false,
        &scope!(),
        r#"context put({"a": 1}, "a", 2)"#,
        "{a: 2}",
    );
}

#[test]
fn _0004() {
    te_context(
        false,
        &scope!(),
        r#"context put({"a":1, "b":2, "c":3 }, "b", 3)"#,
        "{a: 1, b: 3, c: 3}",
    );
}

#[test]
fn _0005() {
    te_context(false, &scope!(), r#"context put({}, "", 1)"#, r#"{"": 1}"#);
}

#[test]
fn _0006() {
    te_null(false, &scope!(), "context put({}, null, 1)", "");
}

#[test]
fn _0007() {
    te_null(false, &scope!(), r#"context put(null, "a", 1)"#, "");
}

#[test]
fn _0008() {
    te_context(
        false,
        &scope!(),
        r#"context put({}, "a", null)"#,
        "{a: null}",
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"context put({}, "a")"#,
        "expected 3 parameters, actual number of parameters is 2",
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"context put({}, "a", 1, 1)"#,
        "expected 3 parameters, actual number of parameters is 4",
    );
}

#[test]
fn _0011() {
    te_context(
        false,
        &scope!(),
        r#"context put(context: {}, key: "a", value: 1)"#,
        "{a: 1}",
    );
}

#[test]
fn _0012() {
    te_null(
        false,
        &scope!(),
        r#"context put(context: {}, ky: "a", value: 1)"#,
        "parameter 'key' or 'keys' not found",
    );
}

#[test]
fn _0013() {
    te_null(
        false,
        &scope!(),
        r#"context put(context: {}, key: "a", v: 1)"#,
        "parameter 'value' not found",
    );
}

#[test]
fn _0014() {
    te_null(
        false,
        &scope!(),
        r#"context put(c: {}, key: "a", value: 1)"#,
        "parameter 'context' not found",
    );
}

#[test]
fn _0015() {
    te_null(
        false,
        &scope!(),
        r#"context put(context: {}, keys: "a", value: 1)"#,
        "",
    );
}

#[test]
fn _0016() {
    te_null(
        false,
        &scope!(),
        r#"context put([], "a", 1)"#,
        "[core::context put] invalid argument type, expected context, actual type is list<Null>",
    );
}

#[test]
fn _0017() {
    te_null(
        false,
        &scope!(),
        r#"context put({}, 1, 1)"#,
        "[core::context put] invalid argument type, expected string or list<string>, actual type is number",
    );
}

#[test]
fn _0018() {
    te_context(
        false,
        &scope!(),
        r#"context put({x:1, y: {a: 0} }, ["y", "a"], 2)"#,
        "{x: 1, y: {a: 2}}",
    );
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        r#"context put(context: {}, keys: [1], value: 1)"#,
        "[core::context put] invalid argument type, expected string, actual type is number",
    );
}
