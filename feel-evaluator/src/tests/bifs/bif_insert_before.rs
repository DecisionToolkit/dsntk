use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_be_value(
        false,
        &scope!(),
        r#"insert before([2,3,4,5],1,1)"#,
        r#"[1,2,3,4,5]"#,
    );
}

#[test]
fn _0002() {
    te_be_value(
        false,
        &scope!(),
        r#"insert before(list: [2,3,4,5], position: 1, newItem: 1)"#,
        r#"[1,2,3,4,5]"#,
    );
}

#[test]
fn _0003() {
    te_be_value(
        false,
        &scope!(),
        r#"insert before([1,2,3,5],4,4)"#,
        r#"[1,2,3,4,5]"#,
    );
}

#[test]
fn _0004() {
    te_null(
        false,
        &scope!(),
        r#"insert before([2,3,4,5],0,1)"#,
        "index is out of range",
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"insert before([2,3,4,5],5,1)"#,
        "index is out of range",
    );
}

#[test]
fn _0006() {
    te_be_value(
        false,
        &scope!(),
        r#"insert before([1,2,3,5],-1,4)"#,
        r#"[1,2,3,4,5]"#,
    );
}

#[test]
fn _0007() {
    te_be_value(
        false,
        &scope!(),
        r#"insert before([2,3,4,5],-4,1)"#,
        r#"[1,2,3,4,5]"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"insert before([1,2,3,5],0,4)"#,
        "index is out of range",
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"insert before([1,2,3,5],-5,4)"#,
        "index is out of range",
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"insert before()"#,
        "expected 3 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0011() {
    te_null(
        false,
        &scope!(),
        r#"insert before([1,2,3])"#,
        "expected 3 parameters, actual number of parameters is 1",
    );
}

#[test]
fn _0012() {
    te_null(
        false,
        &scope!(),
        r#"insert before([1,2,3],1)"#,
        "expected 3 parameters, actual number of parameters is 2",
    );
}

#[test]
fn _0013() {
    te_null(
        false,
        &scope!(),
        r#"insert before([1,2,3],0,1,2)"#,
        "expected 3 parameters, actual number of parameters is 4",
    );
}

#[test]
fn _0014() {
    te_null(
        false,
        &scope!(),
        r#"insert before(l: [2,3,4,5], position: 1, newItem: 1)"#,
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0015() {
    te_null(
        false,
        &scope!(),
        r#"insert before(list: [2,3,4,5], p: 1, newItem: 1)"#,
        r#"parameter 'position' not found"#,
    );
}

#[test]
fn _0016() {
    te_null(
        false,
        &scope!(),
        r#"insert before(list: [2,3,4,5], position: 1, ni: 1)"#,
        r#"parameter 'newItem' not found"#,
    );
}
