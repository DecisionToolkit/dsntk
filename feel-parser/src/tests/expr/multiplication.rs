use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "4*2",
        r#"
       Mul
       ├─ Numeric
       │  └─ `4`
       └─ Numeric
          └─ `2`
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
        "5 * 3",
        r#"
       Mul
       ├─ Numeric
       │  └─ `5`
       └─ Numeric
          └─ `3`
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
        "5 * 3 * 2",
        r#"
       Mul
       ├─ Mul
       │  ├─ Numeric
       │  │  └─ `5`
       │  └─ Numeric
       │     └─ `3`
       └─ Numeric
          └─ `2`
    "#,
        false,
    );
}
