use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_context(
        false,
        &scope!(),
        r#"context([{key:"a", value:1}, {key:"b", value:2}])"#,
        r#"{a:1, b:2}"#,
    );
}

#[test]
fn _0002() {
    te_context(
        false,
        &scope!(),
        r#"context([{key:"a", value:1}])"#,
        r#"{a:1}"#,
    );
}

#[test]
fn _0003() {
    te_null(
        false,
        &scope!(),
        r#"context([{key:"a", value:1},{key:"a", value:2}])"#,
        "context: duplicated key: a",
    );
}

#[test]
fn _0004() {
    te_context(false, &scope!(), r#"context([])"#, "{}");
}

#[test]
fn _0005() {
    te_bool(false, &scope!(), r#"context([]) = {}"#, true);
}

#[test]
fn _0006() {
    te_context(false, &scope!(), r#"context({key:"a", value:1})"#, "{a:1}");
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"context({value:1})"#,
        "context: no 'key' entry in context {value: 1}",
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"context({key: null, value:1})"#,
        "context: 'key' entry is not a string, actual type is Null",
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"context({key: "a"})"#,
        r#"context: no 'value' entry in context {key: "a"}"#,
    );
}

#[test]
fn _0010() {
    te_context(
        false,
        &scope!(),
        r#"context({key:"a", value:null})"#,
        r#"{a: null}"#,
    );
}

#[test]
fn _0011() {
    te_context(
        false,
        &scope!(),
        r#"context({key:"", value:1})"#,
        r#"{"": 1}"#,
    );
}

#[test]
fn _0012() {
    te_null(false, &scope!(), r#"context(null)"#, "");
}

#[test]
fn _0013() {
    te_null(
        false,
        &scope!(),
        r#"context()"#,
        "expected 1 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0014() {
    te_null(
        false,
        &scope!(),
        r#"context([], "foo")"#,
        "expected 1 parameters, actual number of parameters is 2",
    );
}

#[test]
fn _0015() {
    te_context(
        false,
        &scope!(),
        r#"context(entries: [{key:"a", value:1}])"#,
        "{a: 1}",
    );
}

#[test]
fn _0016() {
    te_context(
        false,
        &scope!(),
        r#"context(entries: {key:"a", value:1})"#,
        "{a: 1}",
    );
}

#[test]
fn _0017() {
    te_null(
        false,
        &scope!(),
        r#"context(entris: {key:"a", value:1})"#,
        "parameter 'entries' not found",
    );
}

#[test]
fn _0018() {
    te_null(
        false,
        &scope!(),
        r#"context("foo")"#,
        "[core::context] invalid argument type, expected list or context, actual type is string",
    );
}

#[test]
fn _0019() {
    te_context(
        false,
        &scope!(),
        r#"context(entries: [{key:"a", value:1, ignored:"foo"}])"#,
        "{a: 1}",
    );
}

#[test]
fn _0020() {
    te_context(
        false,
        &scope!(),
        r#"context({key:"a", value:1, ignored:"foo"})"#,
        "{a: 1}",
    );
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        "context({key: 1, value: 1})",
        "context: 'key' entry is not a string, actual type is number",
    );
}

#[test]
fn _0022() {
    te_null(
        false,
        &scope!(),
        "context(entries: [21])",
        "context: invalid entry, expected context, actual type is number",
    );
}

#[test]
fn _0023() {
    te_null(
        false,
        &scope!(),
        r#"context(entries: [{key: 1, value: 1}])"#,
        "context: 'key' entry is not a string, actual type is number",
    );
}
