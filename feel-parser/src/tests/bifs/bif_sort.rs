use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "sort(metricsTable, function(x,y) x < y)",
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `sort`
       └─ PositionalParameters
          ├─ Name
          │  └─ `metricsTable`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  ├─ FormalParameter
             │  │  ├─ ParameterName
             │  │  │  └─ `x`
             │  │  └─ FeelType
             │  │     └─ Any
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `y`
             │     └─ FeelType
             │        └─ Any
             └─ FunctionBody
                └─ Lt
                   ├─ Name
                   │  └─ `x`
                   └─ Name
                      └─ `y`
    "#,
        false,
    );
}
