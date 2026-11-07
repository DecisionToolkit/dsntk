use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "2**4",
        r#"
       Exp
       ├─ Numeric
       │  └─ `2`
       └─ Numeric
          └─ `4`
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
        "2 ** 4",
        r#"
       Exp
       ├─ Numeric
       │  └─ `2`
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
        "2 ** 4 ** 3",
        r#"
       Exp
       ├─ Exp
       │  ├─ Numeric
       │  │  └─ `2`
       │  └─ Numeric
       │     └─ `4`
       └─ Numeric
          └─ `3`
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
        "-3 ** 2",
        r#"
       Exp
       ├─ Neg
       │  └─ Numeric
       │     └─ `3`
       └─ Numeric
          └─ `2`
    "#,
        false,
    );
}
