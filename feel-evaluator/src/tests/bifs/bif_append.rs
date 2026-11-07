use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_be_value(false, &scope!(), r#"append([1],2,3)"#, r#"[1,2,3]"#);
}

#[test]
fn _0002() {
    te_be_value(false, &scope!(), r#"append([8],4)"#, r#"[8,4]"#);
}

#[test]
fn _0003() {
    te_be_value(false, &scope!(), r#"append([8],"A")"#, r#"[8,"A"]"#);
}

#[test]
fn _0004() {
    te_be_value(false, &scope!(), r#"append([1],null)"#, r#"[1,null]"#);
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"append(1,2,3)"#,
        r#"[core::append] invalid argument type, expected list, actual type is number"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"append()"#,
        r#"expected 2+ parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0007() {
    te_null(
        false,
        &scope!(),
        r#"append([1])"#,
        r#"expected 2+ parameters, actual number of parameters is 1"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"append(list: [1])"#,
        r#"[named::append] this function has no version with named parameters"#,
    );
}

#[test]
fn _0009() {
    te_null(false, &scope!(), r#"append(null, "A")"#, r#""#);
}
