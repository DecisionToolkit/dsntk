use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_be_value(
        false,
        &scope!(),
        r#"flatten([["w","x"],["y"],["z"]])"#,
        r#"["w","x","y","z"]"#,
    );
}

#[test]
fn _0002() {
    te_be_value(
        false,
        &scope!(),
        r#"flatten(list: [["w","x"],["y"],["z"]])"#,
        r#"["w","x","y","z"]"#,
    );
}

#[test]
fn _0003() {
    te_be_value(
        false,
        &scope!(),
        r#"flatten([["w","x"],["y",["a","b","c"]],["z"]])"#,
        r#"["w","x","y","a","b","c","z"]"#,
    );
}

#[test]
fn _0004() {
    te_null(
        false,
        &scope!(),
        r#"flatten()"#,
        r#"expected 1 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0005() {
    te_null(
        false,
        &scope!(),
        r#"flatten(10)"#,
        r#"[core::flatten] invalid argument type, expected list, actual type is number"#,
    );
}

#[test]
fn _0006() {
    te_null(
        false,
        &scope!(),
        r#"flatten(l:[1])"#,
        r#"parameter 'list' not found"#,
    );
}
