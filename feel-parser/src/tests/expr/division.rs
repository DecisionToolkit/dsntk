use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "1/2",
        r#"
       Div
       ├─ Numeric
       │  └─ `1`
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
        "5 / 3",
        r#"
       Div
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
        "5 / 3 / 2",
        r#"
       Div
       ├─ Div
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
