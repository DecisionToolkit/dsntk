use super::super::*;
use crate::context::ParsingContext;
use crate::lalr::TokenType::*;

#[test]
fn _0001() {
    let scope = scope!();
    scope.set_entry_name("name".into());
    accept(
        &scope,
        StartExpression,
        r#"{greeting: function (name) "Hello " + name + "!" }"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `greeting`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `name`
             │     └─ FeelType
             │        └─ Any
             └─ FunctionBody
                └─ Add
                   ├─ Add
                   │  ├─ String
                   │  │  └─ `Hello `
                   │  └─ Name
                   │     └─ `name`
                   └─ String
                      └─ `!`
    "#,
        false,
    );
}

#[test]
fn _0002() {
    let scope = scope!();
    scope.set_entry_name("suffix".into());
    scope.set_entry_name("other".into());
    scope.set_entry_name("name".into());
    accept(
        &scope,
        StartExpression,
        r#"{greeting: function (name) "Hello " + name + "! (" + suffix + ")" }"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `greeting`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `name`
             │     └─ FeelType
             │        └─ Any
             └─ FunctionBody
                └─ Add
                   ├─ Add
                   │  ├─ Add
                   │  │  ├─ Add
                   │  │  │  ├─ String
                   │  │  │  │  └─ `Hello `
                   │  │  │  └─ Name
                   │  │  │     └─ `name`
                   │  │  └─ String
                   │  │     └─ `! (`
                   │  └─ Name
                   │     └─ `suffix`
                   └─ String
                      └─ `)`
    "#,
        false,
    );
}

#[test]
fn _0003() {
    // prepare the context with names
    let mut ctx_inner = ParsingContext::default();
    ctx_inner.set_name("Surname".into());
    ctx_inner.set_name("City".into());
    ctx_inner.set_name("Street".into());
    ctx_inner.set_name(vec!["Marital", "status"].into());
    let mut ctx_outer = ParsingContext::default();
    ctx_outer.set_context("Person".into(), ctx_inner);
    let mut ctx = ParsingContext::default();
    ctx.set_context(vec!["Customer", "data"].into(), ctx_outer);
    // prepare scope
    let scope = scope!();
    scope.set_entry_name("x".into());
    scope.set_entry_name("y".into());
    scope.set_entry_name(vec!["Customer", "data"].into());
    scope.push(ctx);
    assert_eq!(
        "[{Customer data: <v>, x: <v>, y: <v>}, {Customer data: {Person: {City: <v>, Marital status: <v>, Street: <v>, Surname: <v>}}}]",
        scope.to_string()
    );
    accept(
        &scope,
        StartExpression,
        r#"{greeting: function (prefix) prefix + ", hello " + Customer    data.Person.Surname + " (" + Customer                     data.Person.City + ")" + x }"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `greeting`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `prefix`
             │     └─ FeelType
             │        └─ Any
             └─ FunctionBody
                └─ Add
                   ├─ Add
                   │  ├─ Add
                   │  │  ├─ Add
                   │  │  │  ├─ Add
                   │  │  │  │  ├─ Add
                   │  │  │  │  │  ├─ Name
                   │  │  │  │  │  │  └─ `prefix`
                   │  │  │  │  │  └─ String
                   │  │  │  │  │     └─ `, hello `
                   │  │  │  │  └─ Path
                   │  │  │  │     ├─ Path
                   │  │  │  │     │  ├─ Name
                   │  │  │  │     │  │  └─ `Customer data`
                   │  │  │  │     │  └─ Name
                   │  │  │  │     │     └─ `Person`
                   │  │  │  │     └─ Name
                   │  │  │  │        └─ `Surname`
                   │  │  │  └─ String
                   │  │  │     └─ ` (`
                   │  │  └─ Path
                   │  │     ├─ Path
                   │  │     │  ├─ Name
                   │  │     │  │  └─ `Customer data`
                   │  │     │  └─ Name
                   │  │     │     └─ `Person`
                   │  │     └─ Name
                   │  │        └─ `City`
                   │  └─ String
                   │     └─ `)`
                   └─ Name
                      └─ `x`
    "#,
        false,
    );
    //assert_eq!("[prefix, Customer data.Person.Surname, Customer data.Person.City, x]", closure.to_string());
}
