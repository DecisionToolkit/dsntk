use super::*;

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    "null in null",
    r#"
       In
       ├─ Null
       └─ Null
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "null in null",
    r#"
       In
       ├─ Null
       └─ Null
    "#,
    false,
  );
}
