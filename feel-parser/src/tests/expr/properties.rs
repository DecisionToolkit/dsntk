use super::super::*;

#[test]
fn _0001() {
  //---------------------------------------------------------------------------
  // Properties should be parsed as paths.
  //---------------------------------------------------------------------------
  accept(
    &scope!(),
    StartExpression,
    "[1..10].end included",
    r#"
       Path
       ├─ Range
       │  ├─ IntervalStart (closed)
       │  │  └─ Numeric
       │  │     └─ `1`
       │  └─ IntervalEnd (closed)
       │     └─ Numeric
       │        └─ `10`
       └─ Name
          └─ `end included`
    "#,
    false,
  );
}

#[test]
fn _0002() {
  //---------------------------------------------------------------------------
  // If the name is a built-in property it should be properly parsed
  // even if the shorter name already exists in the context.
  // In this test case, the whole `end included` name should be parsed,
  // not only the `end` name that exists in the parsing context.
  //---------------------------------------------------------------------------
  let scope = scope!();
  scope.set_entry_name("end".into());
  accept(
    &scope,
    StartExpression,
    "[1..10].end included",
    r#"
       Path
       ├─ Range
       │  ├─ IntervalStart (closed)
       │  │  └─ Numeric
       │  │     └─ `1`
       │  └─ IntervalEnd (closed)
       │     └─ Numeric
       │        └─ `10`
       └─ Name
          └─ `end included`
    "#,
    false,
  );
}
