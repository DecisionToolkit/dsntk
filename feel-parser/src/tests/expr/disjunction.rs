use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "true or false",
        r#"
       Or
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
        "true or false or true",
        r#"
       Or
       ├─ Or
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
        "(false or false) or true",
        r#"
       Or
       ├─ Or
       │  ├─ Boolean
       │  │  └─ `false`
       │  └─ Boolean
       │     └─ `false`
       └─ Boolean
          └─ `true`
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
        "false or (false or true)",
        r#"
       Or
       ├─ Boolean
       │  └─ `false`
       └─ Or
          ├─ Boolean
          │  └─ `false`
          └─ Boolean
             └─ `true`
    "#,
        false,
    );
}
