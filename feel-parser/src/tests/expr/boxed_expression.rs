use super::super::*;
use crate::lalr::TokenType::StartBoxedExpression;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"function () "the body" "#,
        r#"
       FunctionDefinition
       ├─ FormalParameters
       │  └─ (empty)
       └─ FunctionBody
          └─ String
             └─ `the body`
    "#,
        false,
    );
}

#[test]
fn _0002() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"function () 1.414926 "#,
        r#"
       FunctionDefinition
       ├─ FormalParameters
       │  └─ (empty)
       └─ FunctionBody
          └─ Numeric
             └─ `1.414926`
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
        r#"function () external { java: {class: "java.lang.Math", method signature: "min"} }"#,
        r#"
       FunctionDefinition
       ├─ FormalParameters
       │  └─ (empty)
       └─ FunctionBody (external)
          └─ Context
             └─ ContextEntry
                ├─ ContextEntryKey
                │  └─ `java`
                └─ Context
                   ├─ ContextEntry
                   │  ├─ ContextEntryKey
                   │  │  └─ `class`
                   │  └─ String
                   │     └─ `java.lang.Math`
                   └─ ContextEntry
                      ├─ ContextEntryKey
                      │  └─ `method signature`
                      └─ String
                         └─ `min`
    "#,
        false,
    );
}

#[test]
#[should_panic]
fn _0004() {
    let scope = scope!();
    accept(&scope, StartBoxedExpression, r#""#, r#""#, false);
}

#[test]
fn _0005() {
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
fn _0006() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"[1]"#,
        r#"
       List
       └─ Numeric
          └─ `1`
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
        r#" [ 1 , 2 , 3 ] "#,
        r#"
       List
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
fn _0008() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#" {} "#,
        r#"
       Context
       └─ (empty)
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
        r#" { age: 50 } "#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `age`
          └─ Numeric
             └─ `50`
    "#,
        false,
    );
}

#[test]
fn _0010() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#" { name: "John", address: { street: "Bourbon Street" , house no: 15} , married: false } "#,
        r#"
       Context
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `name`
       │  └─ String
       │     └─ `John`
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `address`
       │  └─ Context
       │     ├─ ContextEntry
       │     │  ├─ ContextEntryKey
       │     │  │  └─ `street`
       │     │  └─ String
       │     │     └─ `Bourbon Street`
       │     └─ ContextEntry
       │        ├─ ContextEntryKey
       │        │  └─ `house no`
       │        └─ Numeric
       │           └─ `15`
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `married`
          └─ Boolean
             └─ `false`
    "#,
        false,
    );
}
