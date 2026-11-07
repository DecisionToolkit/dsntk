use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "calculate()",
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `calculate`
       └─ PositionalParameters
          └─ (empty)
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
        "calculate(a:2)",
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `calculate`
       └─ NamedParameters
          └─ NamedParameter
             ├─ ParameterName
             │  └─ `a`
             └─ Numeric
                └─ `2`
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
        "add(x:2,y:5)",
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `add`
       └─ NamedParameters
          ├─ NamedParameter
          │  ├─ ParameterName
          │  │  └─ `x`
          │  └─ Numeric
          │     └─ `2`
          └─ NamedParameter
             ├─ ParameterName
             │  └─ `y`
             └─ Numeric
                └─ `5`
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
        "calculate(56,34)",
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `calculate`
       └─ PositionalParameters
          ├─ Numeric
          │  └─ `56`
          └─ Numeric
             └─ `34`
    "#,
        false,
    );
}
