use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        r#"date("2012-12-25")"#,
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date`
       └─ PositionalParameters
          └─ String
             └─ `2012-12-25`
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
        r#"date(from:"2012-12-25")"#,
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date`
       └─ NamedParameters
          └─ NamedParameter
             ├─ ParameterName
             │  └─ `from`
             └─ String
                └─ `2012-12-25`
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
        r#"date(string("2012-12-25"))"#,
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date`
       └─ PositionalParameters
          └─ FunctionInvocation
             ├─ Name
             │  └─ `string`
             └─ PositionalParameters
                └─ String
                   └─ `2012-12-25`
    "#,
        false,
    );
}
