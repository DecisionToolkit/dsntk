use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "50 between 30 and 70",
        r#"
       Between
       ├─ Numeric
       │  └─ `50`
       ├─ Numeric
       │  └─ `30`
       └─ Numeric
          └─ `70`
    "#,
        false,
    );
}
