use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_null(
        false,
        &scope!(),
        "median()",
        r#"expected 1+ parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0002() {
    te_null(false, &scope!(), "median([])", "");
}

#[test]
fn _0003() {
    te_null(
        false,
        &scope!(),
        "median(l:[])",
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0004() {
    te_null(
        false,
        &scope!(),
        "median(l:[1,2,3])",
        r#"parameter 'list' not found"#,
    );
}

#[test]
fn _0005() {
    te_null(false, &scope!(), "median([true,false])", r#"median"#);
}

#[test]
fn _0006() {
    te_number(false, &scope!(), "median([8, 2, 5, 7])", 6, 0);
}

#[test]
fn _0007() {
    te_number(false, &scope!(), "median(list:[8, 2, 5, 7])", 6, 0);
}

#[test]
fn _0008() {
    te_number(false, &scope!(), "median([8,2,5,3,4])", 4, 0);
}

#[test]
fn _0009() {
    te_number(false, &scope!(), "median(list:[8,2,5,3,4])", 4, 0);
}

#[test]
fn _0010() {
    te_number(false, &scope!(), "median(8,2,5,3,4)", 4, 0);
}

#[test]
fn _0011() {
    te_number(false, &scope!(), "median([8,2,5,3,4.25])", 425, 2);
}

#[test]
fn _0012() {
    te_number(false, &scope!(), "median(list:[8,2,5,3,4.25])", 425, 2);
}

#[test]
fn _0013() {
    te_number(false, &scope!(), "median(8,2,5,3,4.25)", 425, 2);
}

#[test]
fn _0014() {
    te_number(false, &scope!(), "median([6,1,2,3])", 25, 1);
}

#[test]
fn _0015() {
    te_number(false, &scope!(), "median(list:[6,1,2,3])", 25, 1);
}

#[test]
fn _0016() {
    te_number(false, &scope!(), "median(6,1,2,3)", 25, 1);
}

#[test]
fn _0017() {
    te_number(false, &scope!(), "median([2021])", 2021, 0);
}

#[test]
fn _0018() {
    te_number(false, &scope!(), "median(list:[2021])", 2021, 0);
}

#[test]
fn _0019() {
    te_number(false, &scope!(), "median(2021)", 2021, 0);
}

#[test]
fn _0020() {
    te_number(false, &scope!(), "median([1999,2999])", 2499, 0);
}

#[test]
fn _0021() {
    te_number(false, &scope!(), "median(list:[1999,2999])", 2499, 0);
}

#[test]
fn _0022() {
    te_number(false, &scope!(), "median(1999,2999)", 2499, 0);
}
