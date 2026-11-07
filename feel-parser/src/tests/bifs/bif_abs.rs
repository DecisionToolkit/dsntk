use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "abs(1)",
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `abs`
       └─ PositionalParameters
          └─ Numeric
             └─ `1`
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
        "abs(-1)",
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `abs`
       └─ PositionalParameters
          └─ Neg
             └─ Numeric
                └─ `1`
    "#,
        false,
    );
}
