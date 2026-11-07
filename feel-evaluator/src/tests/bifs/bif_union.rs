use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_be_value(false, &scope!(), r#"union([1,2],[2,3])"#, r#"[1,2,3]"#);
}

#[test]
fn _0002() {
    te_be_value(false, &scope!(), r#"union([1,2],[3,4])"#, r#"[1,2,3,4]"#);
}

#[test]
fn _0003() {
    te_be_value(
        false,
        &scope!(),
        r#"union([1,2,"a",true],[3,4,"a",false])"#,
        r#"[1,2,"a",true,3,4,false]"#,
    );
}

#[test]
fn _0004() {
    te_be_value(
        false,
        &scope!(),
        r#"union([[1,2],[2,3]])"#,
        r#"[[1,2],[2,3]]"#,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"union(1,2,3)"#,
        r#"[core::union] invalid argument type, expected list, actual type is number"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"union()"#,
        r#"expected 1+ parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"union(list: [1,2])"#,
        r#"[named::union] this function has no implementation with named parameters"#,
    );
}
