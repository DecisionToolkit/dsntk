use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "if 1 > 2 then 5 else 8",
        r#"
       If
       ├─ Gt
       │  ├─ Numeric
       │  │  └─ `1`
       │  └─ Numeric
       │     └─ `2`
       ├─ Numeric
       │  └─ `5`
       └─ Numeric
          └─ `8`
    "#,
        false,
    );
}

#[test]
fn _0002() {
    let scope = scope!();
    scope.set_entry_name("a".into());
    scope.set_entry_name("b".into());
    accept(
        &scope,
        StartExpression,
        "if a > b then a else b",
        r#"
       If
       ├─ Gt
       │  ├─ Name
       │  │  └─ `a`
       │  └─ Name
       │     └─ `b`
       ├─ Name
       │  └─ `a`
       └─ Name
          └─ `b`
    "#,
        false,
    );
}
