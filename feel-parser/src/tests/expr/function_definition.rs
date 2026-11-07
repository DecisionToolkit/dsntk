use super::super::*;
use crate::context::ParsingContext;
use crate::lalr::TokenType::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{msg: function () "hello!" }"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `msg`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  └─ (empty)
             └─ FunctionBody
                └─ String
                   └─ `hello!`
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
        r#"{pow: function (x: number) x * x }"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `pow`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `x`
             │     └─ FeelType
             │        └─ number
             └─ FunctionBody
                └─ Mul
                   ├─ Name
                   │  └─ `x`
                   └─ Name
                      └─ `x`
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
        r#"{add: function (x: number, y: number) x + y }"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `add`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  ├─ FormalParameter
             │  │  ├─ ParameterName
             │  │  │  └─ `x`
             │  │  └─ FeelType
             │  │     └─ number
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `y`
             │     └─ FeelType
             │        └─ number
             └─ FunctionBody
                └─ Add
                   ├─ Name
                   │  └─ `x`
                   └─ Name
                      └─ `y`
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
        r#"{add3: function(x:number,y:number,z:number)x+y+z}"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `add3`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  ├─ FormalParameter
             │  │  ├─ ParameterName
             │  │  │  └─ `x`
             │  │  └─ FeelType
             │  │     └─ number
             │  ├─ FormalParameter
             │  │  ├─ ParameterName
             │  │  │  └─ `y`
             │  │  └─ FeelType
             │  │     └─ number
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `z`
             │     └─ FeelType
             │        └─ number
             └─ FunctionBody
                └─ Add
                   ├─ Add
                   │  ├─ Name
                   │  │  └─ `x`
                   │  └─ Name
                   │     └─ `y`
                   └─ Name
                      └─ `z`
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
        r#"{inc: function (state) state + 1 }"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `inc`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `state`
             │     └─ FeelType
             │        └─ Any
             └─ FunctionBody
                └─ Add
                   ├─ Name
                   │  └─ `state`
                   └─ Numeric
                      └─ `1`
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
        r#"function(a,b) a+b"#,
        r#"
       FunctionDefinition
       ├─ FormalParameters
       │  ├─ FormalParameter
       │  │  ├─ ParameterName
       │  │  │  └─ `a`
       │  │  └─ FeelType
       │  │     └─ Any
       │  └─ FormalParameter
       │     ├─ ParameterName
       │     │  └─ `b`
       │     └─ FeelType
       │        └─ Any
       └─ FunctionBody
          └─ Add
             ├─ Name
             │  └─ `a`
             └─ Name
                └─ `b`
    "#,
        false,
    );
}

#[test]
fn _0007() {
    let scope = scope!();
    scope.set_entry_name("a".into());
    accept(
        &scope,
        StartExpression,
        r#"function(b) a * b"#,
        r#"
       FunctionDefinition
       ├─ FormalParameters
       │  └─ FormalParameter
       │     ├─ ParameterName
       │     │  └─ `b`
       │     └─ FeelType
       │        └─ Any
       └─ FunctionBody
          └─ Mul
             ├─ Name
             │  └─ `a`
             └─ Name
                └─ `b`
    "#,
        false,
    );
}

#[test]
fn _0008() {
    let scope = scope!();
    let mut ctx_inner = ParsingContext::default();
    ctx_inner.set_name("c".into());
    let mut ctx_outer = ParsingContext::default();
    ctx_outer.set_context("b".into(), ctx_inner);
    scope.set_context("a".into(), ctx_outer);
    accept(
        &scope,
        StartExpression,
        r#"function(x) a.b.c * x"#,
        r#"
       FunctionDefinition
       ├─ FormalParameters
       │  └─ FormalParameter
       │     ├─ ParameterName
       │     │  └─ `x`
       │     └─ FeelType
       │        └─ Any
       └─ FunctionBody
          └─ Mul
             ├─ Path
             │  ├─ Path
             │  │  ├─ Name
             │  │  │  └─ `a`
             │  │  └─ Name
             │  │     └─ `b`
             │  └─ Name
             │     └─ `c`
             └─ Name
                └─ `x`
    "#,
        false,
    );
}

#[test]
fn _0009() {
    let scope = scope!();
    scope.set_entry_name("a".into());
    accept(
        &scope,
        StartExpression,
        r#"function (b) function(c) function(d) a * b * c * d"#,
        r#"
       FunctionDefinition
       ├─ FormalParameters
       │  └─ FormalParameter
       │     ├─ ParameterName
       │     │  └─ `b`
       │     └─ FeelType
       │        └─ Any
       └─ FunctionBody
          └─ FunctionDefinition
             ├─ FormalParameters
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `c`
             │     └─ FeelType
             │        └─ Any
             └─ FunctionBody
                └─ FunctionDefinition
                   ├─ FormalParameters
                   │  └─ FormalParameter
                   │     ├─ ParameterName
                   │     │  └─ `d`
                   │     └─ FeelType
                   │        └─ Any
                   └─ FunctionBody
                      └─ Mul
                         ├─ Mul
                         │  ├─ Mul
                         │  │  ├─ Name
                         │  │  │  └─ `a`
                         │  │  └─ Name
                         │  │     └─ `b`
                         │  └─ Name
                         │     └─ `c`
                         └─ Name
                            └─ `d`
    "#,
        false,
    );
}
