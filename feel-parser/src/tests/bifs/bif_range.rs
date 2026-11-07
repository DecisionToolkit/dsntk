use super::super::*;

#[test]
fn _0001() {
    let input = r#"range(string("[1..3]"))"#;
    let expected = r#"
       FunctionInvocation
       ├─ Name
       │  └─ `range`
       └─ PositionalParameters
          └─ FunctionInvocation
             ├─ Name
             │  └─ `string`
             └─ PositionalParameters
                └─ String
                   └─ `[1..3]`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0002() {
    let input = r#"range("[date(\"1970-01-01\")..date(\"1970-01-02\")]")"#;
    let expected = r#"
       FunctionInvocation
       ├─ Name
       │  └─ `range`
       └─ PositionalParameters
          └─ String
             └─ `[date("1970-01-01")..date("1970-01-02")]`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}
