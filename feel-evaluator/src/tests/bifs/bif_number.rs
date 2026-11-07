use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), r#"number("1",",",".")"#, 1, 0);
}

#[test]
fn _0002() {
    te_number(
        false,
        &scope!(),
        r#"number(from: "1", grouping separator: ",", decimal separator: ".")"#,
        1,
        0,
    );
}

#[test]
fn _0003() {
    te_number(false, &scope!(), r#"number("1,000.21",",",".")"#, 100021, 2);
}

#[test]
fn _0004() {
    te_number(false, &scope!(), r#"number("1 000.21"," ",".")"#, 100021, 2);
}

#[test]
fn _0005() {
    te_number(false, &scope!(), r#"number("1.000,21",".",",")"#, 100021, 2);
}

#[test]
fn _0006() {
    te_number(false, &scope!(), r#"number("12345",null,null)"#, 12345, 0);
}

#[test]
fn _0007() {
    te_number(false, &scope!(), r#"number("12,345",",",null)"#, 12345, 0);
}

#[test]
fn _0008() {
    te_number(false, &scope!(), r#"number("123,45",null,",")"#, 12345, 2);
}

#[test]
fn _0009() {
    te_null(
        false,
        &scope!(),
        r#"number("1,000.21",".",".")"#,
        r#"[core::number] decimal separator must be different from grouping separator"#,
    );
}

#[test]
fn _0010() {
    te_null(
        false,
        &scope!(),
        r#"number("1$000.21","$",".")"#,
        r#"[core::number] grouping separator must be space, period, comma or null"#,
    );
}

#[test]
fn _0011() {
    te_null(
        false,
        &scope!(),
        r#"number("1,000$21",",","$")"#,
        r#"[core::number] decimal separator must be period, comma or null"#,
    );
}

#[test]
fn _0012() {
    te_null(
        false,
        &scope!(),
        r#"number("123a56",null,null)"#,
        r#"[core::number] <FeelNumberError> invalid number literal '123a56'"#,
    );
}

#[test]
fn _0013() {
    te_null(
        false,
        &scope!(),
        r#"number("1,000.21",2,".")"#,
        r#"[core::number] grouping separator must be space, period, comma or null"#,
    );
}

#[test]
fn _0014() {
    te_null(
        false,
        &scope!(),
        r#"number("1,000.21",",",true)"#,
        r#"[core::number] decimal separator must be period, comma or null"#,
    );
}

#[test]
fn _0015() {
    te_null(
        false,
        &scope!(),
        r#"number(1000,null,null)"#,
        r#"[core::number] invalid argument type, expected string, actual type is number"#,
    );
}

#[test]
fn _0016() {
    te_null(
        false,
        &scope!(),
        r#"number()"#,
        r#"expected 3 parameters, actual number of parameters is 0"#,
    );
}

#[test]
fn _0017() {
    te_null(
        false,
        &scope!(),
        r#"number(1000)"#,
        r#"expected 3 parameters, actual number of parameters is 1"#,
    );
}

#[test]
fn _0018() {
    te_null(
        false,
        &scope!(),
        r#"number(1000,",")"#,
        r#"expected 3 parameters, actual number of parameters is 2"#,
    );
}

#[test]
fn _0019() {
    te_null(
        false,
        &scope!(),
        r#"number(1000,",",".",",")"#,
        r#"expected 3 parameters, actual number of parameters is 4"#,
    );
}

#[test]
fn _0020() {
    te_null(
        false,
        &scope!(),
        r#"number(f: "1", grouping separator: ",", decimal separator: ".")"#,
        r#"parameter 'from' not found"#,
    );
}

#[test]
fn _0021() {
    te_null(
        false,
        &scope!(),
        r#"number(from: "1", grouping sep: ",", decimal separator: ".")"#,
        r#"parameter 'grouping separator' not found"#,
    );
}

#[test]
fn _0022() {
    te_null(
        false,
        &scope!(),
        r#"number(from: "1", grouping separator: ",", decimal sep: ".")"#,
        r#"parameter 'decimal separator' not found"#,
    );
}
