use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "1+2",
        r#"
       Add
       ├─ Numeric
       │  └─ `1`
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
        "1 + 2",
        r#"
       Add
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `2`
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
        " 5 +2 +1",
        r#"
       Add
       ├─ Add
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
fn _0004() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "2+20+200",
        r#"
       Add
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `2`
       │  └─ Numeric
       │     └─ `20`
       └─ Numeric
          └─ `200`
    "#,
        false,
    );
}

#[test]
fn _0005() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "( 1 + 2 ) + ( 3 + 4 )",
        r#"
       Add
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `1`
       │  └─ Numeric
       │     └─ `2`
       └─ Add
          ├─ Numeric
          │  └─ `3`
          └─ Numeric
             └─ `4`
    "#,
        false,
    );
}

#[test]
fn _0006() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "( ( ( 1 + 2 ) ) )",
        r#"
       Add
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `2`
    "#,
        false,
    );
}

#[test]
fn _0007() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "(1+2)*(3+4)",
        r#"
       Mul
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `1`
       │  └─ Numeric
       │     └─ `2`
       └─ Add
          ├─ Numeric
          │  └─ `3`
          └─ Numeric
             └─ `4`
    "#,
        false,
    );
}

#[test]
fn _0008() {
    let scope = scope!();
    assert_eq!(
        "<ParserError> syntax error: +1",
        Parser::new(&scope, StartExpression, "+1", false)
            .parse()
            .err()
            .unwrap()
            .to_string()
            .as_str()
    );
}
