use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_be_value(
        false,
        &scope!(),
        r#"sublist([1,2,3,4,5,6,7,8,9,10],3,4)"#,
        r#"[3,4,5,6]"#,
    );
}

#[test]
fn _0002() {
    te_be_value(
        false,
        &scope!(),
        r#"sublist([1,2,3,4,5,6,7,8,9,10],3)"#,
        r#"[3,4,5,6,7,8,9,10]"#,
    );
}

#[test]
fn _0003() {
    te_be_value(
        false,
        &scope!(),
        r#"sublist([1,2,3,4,5,6,7,8,9,10],-1,1)"#,
        r#"[10]"#,
    );
}

#[test]
fn _0004() {
    te_be_value(
        false,
        &scope!(),
        r#"sublist([1,2,3,4,5,6,7,8,9,10],-6,4)"#,
        r#"[5,6,7,8]"#,
    );
}

#[test]
fn _0005() {
    te_be_value(
        false,
        &scope!(),
        r#"sublist(list: [1,2,3,4,5,6,7,8,9,10], start position: -6, length: 4)"#,
        r#"[5,6,7,8]"#,
    );
}

#[test]
fn _0006() {
    te_be_value(
        false,
        &scope!(),
        r#"sublist(list: [1,2,3,4,5,6,7,8,9,10], start position: -6)"#,
        r#"[5,6,7,8,9,10]"#,
    );
}

#[test]
fn _0007() {
    te_be_value(
        false,
        &scope!(),
        r#"sublist(list: [1,2,3,4,5,6,7,8,9,10], start position: 3)"#,
        r#"[3,4,5,6,7,8,9,10]"#,
    );
}

#[test]
fn _0008() {
    te_null(
        false,
        &scope!(),
        r#"sublist()"#,
        r#"expected 2,3 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"sublist([])"#,
        r#"expected 2,3 parameters, actual number of parameters is 1"#,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"sublist([],1,1,1)"#,
        r#"expected 2,3 parameters, actual number of parameters is 4"#,
    );
}

#[test]
fn _0011() {
    te_null(
        false,
        &scope!(),
        r#"sublist(l:[1,2,3], start position: 1, length: 1)"#,
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0012() {
    te_null(
        false,
        &scope!(),
        r#"sublist(list:[1,2,3], sp: 1, length: 1)"#,
        r#"parameter 'start position' not found"#,
    );
}

#[test]
fn _0013() {
    te_null(
        false,
        &scope!(),
        r#"sublist(list:[1,2,3], start position: 1, l: 1)"#,
        r#"parameter 'length' not found"#,
    );
}

#[test]
fn _0014() {
    te_null(
        false,
        &scope!(),
        r#"sublist(list:[1,2,3], sp: 1)"#,
        r#"parameter 'start position' not found"#,
    );
}

#[test]
fn _0015() {
    te_null(
        false,
        &scope!(),
        r#"sublist(lista:[1,2,3], start position: 1)"#,
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0016() {
    te_null(
        false,
        &scope!(),
        r#"sublist(list:[1,2,3], start position: 1, l: 1, k:5)"#,
        r#"expected 2,3 parameters, actual number of parameters is 4"#,
    );
}

#[test]
fn _0017() {
    te_null(
        false,
        &scope!(),
        r#"sublist([1,2,3], 5)"#,
        r#"sublist: position is out of range, len = 3, position = 5"#,
    );
}

#[test]
fn _0018() {
    te_null(
        false,
        &scope!(),
        r#"sublist([1,2,3], -5)"#,
        r#"sublist: position is out of range, len = 3, position = -5"#,
    );
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        r#"sublist([1,2,3], 18446744073709551616)"#,
        r#"sublist: invalid position value: 18446744073709551616"#,
    );
}

#[test]
fn _0020() {
    te_null(
        false,
        &scope!(),
        r#"sublist([1,2,3], -18446744073709551616)"#,
        r#"sublist: invalid position value: -18446744073709551616"#,
    );
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        r#"sublist([1,2,3], 0)"#,
        r#"sublist: position must not be zero"#,
    );
}

#[test]
fn _0022() {
    te_null(
        false,
        &scope!(),
        r#"sublist([1,2,3], "a")"#,
        r#"sublist: expected number, actual position value type is string"#,
    );
}

#[test]
fn _0023() {
    te_null(
        false,
        &scope!(),
        r#"sublist("[1,2,3]", 2)"#,
        r#"sublist: expected list, actual value type is string"#,
    );
}

#[test]
fn _0024() {
    te_null(
        false,
        &scope!(),
        r#"sublist([1,2,3], 2, 5)"#,
        r#"sublist: invalid range, len = 3, start position = 2, end position = 7"#,
    );
}

#[test]
fn _0025() {
    te_null(
        false,
        &scope!(),
        r#"sublist([1,2,3], -2, 5)"#,
        r#"sublist: invalid range, len = 3, start position = 2, end position = 7"#,
    );
}

#[test]
fn _0026() {
    te_null(
        false,
        &scope!(),
        r#"sublist([1,2,3,4,5], 18446744073709551616, 2)"#,
        r#"sublist: invalid position value: 18446744073709551616"#,
    );
}

#[test]
fn _0027() {
    te_null(
        false,
        &scope!(),
        r#"sublist([1,2,3,4,5], -18446744073709551616, 2)"#,
        r#"sublist: invalid position value: -18446744073709551616"#,
    );
}

#[test]
fn _0028() {
    te_null(
        false,
        &scope!(),
        r#"sublist([1,2,3,4,5], 0, 2)"#,
        r#"sublist: position must not be zero"#,
    );
}

#[test]
fn _0029() {
    te_null(
        false,
        &scope!(),
        r#"sublist([1,2,3,4,5], "0", 2)"#,
        r#"sublist: expected number, actual position value type is string"#,
    );
}

#[test]
fn _0030() {
    te_null(
        false,
        &scope!(),
        r#"sublist([1,2,3,4,5], 1, true)"#,
        r#"sublist: expected number, actual length value type is boolean"#,
    );
}

#[test]
fn _0031() {
    te_null(
        false,
        &scope!(),
        r#"sublist([1,2,3,4,5], 1, 18446744073709551616)"#,
        r#"sublist: invalid length value: 18446744073709551616"#,
    );
}

#[test]
fn _0032() {
    te_null(
        false,
        &scope!(),
        r#"sublist("[1,2,3,4,5]", 2, 3)"#,
        r#"sublist: expected list, actual value type is string"#,
    );
}
