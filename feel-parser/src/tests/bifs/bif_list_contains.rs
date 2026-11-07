use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "list contains(list1,list2)",
        r#"
       FunctionInvocation
       ├─ Name
       │  └─ `list contains`
       └─ PositionalParameters
          ├─ Name
          │  └─ `list1`
          └─ Name
             └─ `list2`
    "#,
        false,
    );
}
