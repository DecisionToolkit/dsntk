use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        r#"date and time(date("2017-01-01"),time("23:59:01"))"#,
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date and time`
       └─ PositionalParameters
          ├─ FunctionInvocation
          │  ├─ Name
          │  │  └─ `date`
          │  └─ PositionalParameters
          │     └─ String
          │        └─ `2017-01-01`
          └─ FunctionInvocation
             ├─ Name
             │  └─ `time`
             └─ PositionalParameters
                └─ String
                   └─ `23:59:01`
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
        r#"date and time(d:date("2017-01-01"),t:time("23:59:01"))"#,
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date and time`
       └─ NamedParameters
          ├─ NamedParameter
          │  ├─ ParameterName
          │  │  └─ `d`
          │  └─ FunctionInvocation
          │     ├─ Name
          │     │  └─ `date`
          │     └─ PositionalParameters
          │        └─ String
          │           └─ `2017-01-01`
          └─ NamedParameter
             ├─ ParameterName
             │  └─ `t`
             └─ FunctionInvocation
                ├─ Name
                │  └─ `time`
                └─ PositionalParameters
                   └─ String
                      └─ `23:59:01`
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
        r#"date and time(date:date("2017-01-01"),time:time("23:59:01"))"#,
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date and time`
       └─ NamedParameters
          ├─ NamedParameter
          │  ├─ ParameterName
          │  │  └─ `date`
          │  └─ FunctionInvocation
          │     ├─ Name
          │     │  └─ `date`
          │     └─ PositionalParameters
          │        └─ String
          │           └─ `2017-01-01`
          └─ NamedParameter
             ├─ ParameterName
             │  └─ `time`
             └─ FunctionInvocation
                ├─ Name
                │  └─ `time`
                └─ PositionalParameters
                   └─ String
                      └─ `23:59:01`
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
        r#"date and time("2018-12-10T10:30:00.0001+05:00:01")"#,
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `date and time`
       └─ PositionalParameters
          └─ String
             └─ `2018-12-10T10:30:00.0001+05:00:01`
    "#,
        false,
    );
}
