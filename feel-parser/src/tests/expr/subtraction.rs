use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "3-2",
        r#"
       Sub
       ├─ Numeric
       │  └─ `3`
       └─ Numeric
          └─ `2`
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
        "-3.65-5.24-6.257",
        r#"
       Sub
       ├─ Sub
       │  ├─ Neg
       │  │  └─ Numeric
       │  │     └─ `3.65`
       │  └─ Numeric
       │     └─ `5.24`
       └─ Numeric
          └─ `6.257`
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
        "1 - 2",
        r#"
       Sub
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `2`
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
        " 5 -2 -1",
        r#"
       Sub
       ├─ Sub
       │  ├─ Numeric
       │  │  └─ `5`
       │  └─ Numeric
       │     └─ `2`
       └─ Numeric
          └─ `1`
    "#,
        false,
    );
}

#[test]
fn _0005() {
    let scope = scope!();
    scope.set_entry_name("Date-Time".into());
    scope.set_entry_name("Date-Time2".into());
    accept(
        &scope,
        StartExpression,
        "Date-Time - Date-Time2",
        r#"
       Sub
       ├─ Name
       │  └─ `Date-Time`
       └─ Name
          └─ `Date-Time2`
    "#,
        false,
    );
}

#[test]
fn _0006() {
    let scope = scope!();
    scope.set_entry_name("Date  -  Time".into());
    scope.set_entry_name("Date     -   Time2".into());
    accept(
        &scope,
        StartExpression,
        "(Date    -   Time) - (  Date     -      Time2)",
        r#"
       Sub
       ├─ Name
       │  └─ `Date-Time`
       └─ Name
          └─ `Date-Time2`
    "#,
        false,
    );
}
