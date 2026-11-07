use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "not(true)",
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `not`
       └─ PositionalParameters
          └─ Boolean
             └─ `true`
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
        " not (  false ) ",
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `not`
       └─ PositionalParameters
          └─ Boolean
             └─ `false`
    "#,
        false,
    );
}
