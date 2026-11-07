use super::super::*;
use crate::lalr::TokenType::StartTextualExpressions;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartTextualExpressions,
        r#"1,2,3"#,
        r#"
       ExpressionList
       ├─ Numeric
       │  └─ `1`
       ├─ Numeric
       │  └─ `2`
       └─ Numeric
          └─ `3`
    "#,
        false,
    );
}

#[test]
fn _0002() {
    let scope = scope!();
    scope.set_entry_name("a".into());
    scope.set_entry_name("b".into());
    scope.set_entry_name("c".into());
    accept(
        &scope,
        StartTextualExpressions,
        r#"a,b,c"#,
        r#"
       ExpressionList
       ├─ Name
       │  └─ `a`
       ├─ Name
       │  └─ `b`
       └─ Name
          └─ `c`
    "#,
        false,
    );
}

#[test]
fn _0003() {
    let scope = scope!();
    scope.set_entry_name("a".into());
    scope.set_entry_name("b".into());
    scope.set_entry_name("c".into());
    accept(
        &scope,
        StartTextualExpressions,
        r#"a+b,b-c,c**a,(["a","b","c"])"#,
        r#"
       ExpressionList
       ├─ Add
       │  ├─ Name
       │  │  └─ `a`
       │  └─ Name
       │     └─ `b`
       ├─ Sub
       │  ├─ Name
       │  │  └─ `b`
       │  └─ Name
       │     └─ `c`
       ├─ Exp
       │  ├─ Name
       │  │  └─ `c`
       │  └─ Name
       │     └─ `a`
       └─ List
          ├─ String
          │  └─ `a`
          ├─ String
          │  └─ `b`
          └─ String
             └─ `c`
    "#,
        false,
    );
}
