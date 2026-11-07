use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "true and false",
        r#"
       And
       ├─ Boolean
       │  └─ `true`
       └─ Boolean
          └─ `false`
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
        "true and false and true",
        r#"
       And
       ├─ And
       │  ├─ Boolean
       │  │  └─ `true`
       │  └─ Boolean
       │     └─ `false`
       └─ Boolean
          └─ `true`
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
        "(true and true) and false",
        r#"
       And
       ├─ And
       │  ├─ Boolean
       │  │  └─ `true`
       │  └─ Boolean
       │     └─ `true`
       └─ Boolean
          └─ `false`
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
        "true and (true and false)",
        r#"
       And
       ├─ Boolean
       │  └─ `true`
       └─ And
          ├─ Boolean
          │  └─ `true`
          └─ Boolean
             └─ `false`
    "#,
        false,
    );
}
