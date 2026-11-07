use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_be_value(
        false,
        &scope!(),
        r#"concatenate([1],[2],[3])"#,
        r#"[1,2,3]"#,
    );
}

#[test]
fn _0002() {
    te_be_value(false, &scope!(), r#"concatenate([1,2],[3])"#, r#"[1,2,3]"#);
}

#[test]
fn _0003() {
    te_be_value(
        false,
        &scope!(),
        r#"concatenate([1,2],["A","B","C"])"#,
        r#"[1,2,"A","B","C"]"#,
    );
}

#[test]
fn _0004() {
    te_null(
        false,
        &scope!(),
        r#"concatenate(1,[2])"#,
        r#"[core::concatenate] invalid argument type, expected list, actual type is number"#,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"concatenate()"#,
        r#"expected 1+ parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"concatenate(list: [1,2,3])"#,
        r#"[named::concatenate] this function has no version with named parameters"#,
    );
}
