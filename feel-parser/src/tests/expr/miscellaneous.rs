//! Additional test for expressions found in compatibility tests.

use super::super::*;
use crate::context::ParsingContext;
use crate::lalr::TokenType::List;

#[test]
fn _0001() {
    let scope = scope!();
    let mut ctx = ParsingContext::default();
    ctx.set_name("principal".into());
    ctx.set_name("rate".into());
    ctx.set_name("termMonths".into());
    scope.set_context("loan".into(), ctx);
    accept(
        &scope,
        StartExpression,
        r#"(loan.principal*loan.rate/12)/(1-(1+loan.rate/12)**-loan.termMonths)"#,
        r#"
       Div
       ├─ Div
       │  ├─ Mul
       │  │  ├─ Path
       │  │  │  ├─ Name
       │  │  │  │  └─ `loan`
       │  │  │  └─ Name
       │  │  │     └─ `principal`
       │  │  └─ Path
       │  │     ├─ Name
       │  │     │  └─ `loan`
       │  │     └─ Name
       │  │        └─ `rate`
       │  └─ Numeric
       │     └─ `12`
       └─ Sub
          ├─ Numeric
          │  └─ `1`
          └─ Exp
             ├─ Add
             │  ├─ Numeric
             │  │  └─ `1`
             │  └─ Div
             │     ├─ Path
             │     │  ├─ Name
             │     │  │  └─ `loan`
             │     │  └─ Name
             │     │     └─ `rate`
             │     └─ Numeric
             │        └─ `12`
             └─ Neg
                └─ Path
                   ├─ Name
                   │  └─ `loan`
                   └─ Name
                      └─ `termMonths`
    "#,
        false,
    );
}

/// Tests how the parser behaves, when invalid starting point is given.
#[test]
fn _0002() {
    let scope = scope!();
    assert!(Parser::new(&scope, List, "[1,2,3]", false).parse().is_err());
}
