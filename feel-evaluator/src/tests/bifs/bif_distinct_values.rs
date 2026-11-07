use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_be_value(
        false,
        &scope!(),
        r#"distinct values([1,2,3,2,1])"#,
        r#"[1,2,3]"#,
    );
}

#[test]
fn _0002() {
    te_be_value(
        false,
        &scope!(),
        r#"distinct values([1,"A",true,2,1,"A",true])"#,
        r#"[1,"A",true,2]"#,
    );
}

#[test]
fn _0003() {
    te_be_value(false, &scope!(), r#"distinct values([])"#, r#"[]"#);
}

#[test]
fn _0004() {
    te_be_value(
        false,
        &scope!(),
        r#"distinct values(list: [1,2,3,2,1])"#,
        r#"[1,2,3]"#,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"distinct values(8)"#,
        r#"[core::distinct values] invalid argument type, expected list, actual type is number"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"distinct values(l:[1])"#,
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"distinct values()"#,
        r#"expected 1 parameters, actual number of parameters is 0"#,
    );
}
