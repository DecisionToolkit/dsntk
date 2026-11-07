use super::super::*;
use crate::lalr::TokenType::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[]"#,
        r#"
       List
       └─ (empty)
    "#,
        false,
    );
}

#[test]
fn _0001_1() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        r#"[]"#,
        r#"
       List
       └─ (empty)
    "#,
        true,
    );
}

#[test]
fn _0002() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[[[]]]"#,
        r#"
       List
       └─ List
          └─ List
             └─ (empty)
    "#,
        false,
    );
}

#[test]
fn _0003() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[12.45]"#,
        r#"
       List
       └─ Numeric
          └─ `12.45`
    "#,
        false,
    );
}

#[test]
fn _0004() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"["family","home","job","car"]"#,
        r#"
       List
       ├─ String
       │  └─ `family`
       ├─ String
       │  └─ `home`
       ├─ String
       │  └─ `job`
       └─ String
          └─ `car`
    "#,
        false,
    );
}

#[test]
fn _0005() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[1.1,2.2]"#,
        r#"
       List
       ├─ Numeric
       │  └─ `1.1`
       └─ Numeric
          └─ `2.2`
    "#,
        false,
    );
}

#[test]
fn _0006() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"["a","b","c"]"#,
        r#"
       List
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

#[test]
fn _0007() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[[1]]"#,
        r#"
       List
       └─ List
          └─ Numeric
             └─ `1`
    "#,
        false,
    );
}

#[test]
fn _0008() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[[1],[2]]"#,
        r#"
       List
       ├─ List
       │  └─ Numeric
       │     └─ `1`
       └─ List
          └─ Numeric
             └─ `2`
    "#,
        false,
    );
}

#[test]
fn _0009() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[[1],[2],[3]]"#,
        r#"
       List
       ├─ List
       │  └─ Numeric
       │     └─ `1`
       ├─ List
       │  └─ Numeric
       │     └─ `2`
       └─ List
          └─ Numeric
             └─ `3`
    "#,
        false,
    );
}

#[test]
fn _00010() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[[[1]]]"#,
        r#"
       List
       └─ List
          └─ List
             └─ Numeric
                └─ `1`
    "#,
        false,
    );
}

#[test]
fn _00011() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[[[1],[2],[3]],[[4],[5]],[7,8,9]]"#,
        r#"
       List
       ├─ List
       │  ├─ List
       │  │  └─ Numeric
       │  │     └─ `1`
       │  ├─ List
       │  │  └─ Numeric
       │  │     └─ `2`
       │  └─ List
       │     └─ Numeric
       │        └─ `3`
       ├─ List
       │  ├─ List
       │  │  └─ Numeric
       │  │     └─ `4`
       │  └─ List
       │     └─ Numeric
       │        └─ `5`
       └─ List
          ├─ Numeric
          │  └─ `7`
          ├─ Numeric
          │  └─ `8`
          └─ Numeric
             └─ `9`
    "#,
        false,
    );
}
