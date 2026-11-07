use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_be_value(
        false,
        &scope!(),
        r#"reverse([1,2,3,4,5])"#,
        r#"[5,4,3,2,1]"#,
    );
}

#[test]
fn _0002() {
    te_be_value(
        false,
        &scope!(),
        r#"reverse([1,true,3,"z",5])"#,
        r#"[5,"z",3,true,1]"#,
    );
}

#[test]
fn _0003() {
    te_be_value(false, &scope!(), r#"reverse([])"#, r#"[]"#);
}

#[test]
fn _0004() {
    te_be_value(
        false,
        &scope!(),
        r#"reverse(list: [1,2,3,4,5])"#,
        r#"[5,4,3,2,1]"#,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"reverse(10)"#,
        r#"[core::reverse] invalid argument type, expected list, actual type is number"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"reverse(list:true)"#,
        r#"[core::reverse] invalid argument type, expected list, actual type is boolean"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"reverse()"#,
        r#"expected 1 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"reverse(l:[10,11,12])"#,
        r#"parameter 'list' not found"#,
    );
}
