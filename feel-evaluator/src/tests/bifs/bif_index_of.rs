use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_be_value(false, &scope!(), r#"index of([1,2,3,2],2)"#, r#"[2,4]"#);
}

#[test]
fn _0002() {
    te_be_value(false, &scope!(), r#"index of([1,2,null,2],null)"#, r#"[3]"#);
}

#[test]
fn _0003() {
    te_be_value(
        false,
        &scope!(),
        r#"index of([1,2,true,3,59,34,true,23,false,true],true)"#,
        r#"[3,7,10]"#,
    );
}

#[test]
fn _0004() {
    te_be_value(
        false,
        &scope!(),
        r#"index of(list: [1,2,3,2], match: 2)"#,
        r#"[2,4]"#,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"index of()"#,
        r#"expected 2 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"index of([1,2,3])"#,
        r#"expected 2 parameters, actual number of parameters is 1"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"index of([1,2,3],2,6)"#,
        r#"expected 2 parameters, actual number of parameters is 3"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"index of(10,2)"#,
        r#"[core::index of] invalid argument type, expected list, actual type is number"#,
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"index of(l:[10,11,12],match:11)"#,
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"index of(list:[10,11,12],m:11)"#,
        r#"parameter 'match' not found"#,
    );
}
