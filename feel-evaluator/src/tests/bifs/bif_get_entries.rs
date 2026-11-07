use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_be_value(
        false,
        &scope!(),
        r#"get entries({key1: "value1", key2: "value2"})"#,
        r#"[{key: "key1", value: "value1"},{key: "key2", value: "value2"}]"#,
    );
}

#[test]
fn _0002() {
    te_be_value(
        false,
        &scope!(),
        r#"get entries(m: {key1: "value1", key2: "value2"})"#,
        r#"[{key: "key1", value: "value1"},{key: "key2", value: "value2"}]"#,
    );
}

#[test]
fn _0003() {
    te_null(
        false,
        &scope!(),
        r#"get entries(10)"#,
        r#"[core::get entries] invalid argument type, expected context, actual type is number"#,
    );
}

#[test]
fn _0004() {
    te_null(
        false,
        &scope!(),
        r#"get entries()"#,
        r#"expected 1 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"get entries(1,2)"#,
        r#"expected 1 parameters, actual number of parameters is 2"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"get entries(entries: {key1: "value1", key2: "value2"})"#,
        r#"parameter 'm' not found"#,
    );
}
