use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    scope.set_entry_name("Date".into());
    scope.set_entry_name("fromString".into());
    accept(
        &scope,
        StartExpression,
        r#"Date.fromString.day"#,
        r#"
       Path
       ├─ Path
       │  ├─ Name
       │  │  └─ `Date`
       │  └─ Name
       │     └─ `fromString`
       └─ Name
          └─ `day`
    "#,
        false,
    );
}
