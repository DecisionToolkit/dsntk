use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "-5",
        r#"
       Neg
       └─ Numeric
          └─ `5`
    "#,
        false,
    );
}

#[test]
fn _0002() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        " - 4 ",
        r#"
       Neg
       └─ Numeric
          └─ `4`
    "#,
        false,
    );
}

#[test]
fn _0003() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "--4",
        r#"
       Neg
       └─ Neg
          └─ Numeric
             └─ `4`
    "#,
        false,
    );
}

#[test]
fn _0004() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        " - - 23.24 ",
        r#"
       Neg
       └─ Neg
          └─ Numeric
             └─ `23.24`
    "#,
        false,
    );
}
