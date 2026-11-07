use super::super::*;
use crate::context::ParsingContext;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "[1..10]",
        r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `10`
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
        "(1..10]",
        r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `10`
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
        "[1..10)",
        r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `10`
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
        "(1..10)",
        r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `10`
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
        "]1..10]",
        r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `10`
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
        "[1..10[",
        r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `10`
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
        "]1..10[",
        r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `10`
    "#,
        false,
    );
}

#[test]
fn _0008() {
    let scope = scope!();
    scope.set_entry_name("a".into());
    scope.set_entry_name("b".into());
    accept(
        &scope,
        StartExpression,
        "[a..b]",
        r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Name
       │     └─ `a`
       └─ IntervalEnd (closed)
          └─ Name
             └─ `b`
    "#,
        false,
    );
}

#[test]
fn _0009() {
    let scope = scope!();
    let mut ctx = ParsingContext::default();
    ctx.set_name("start".into());
    ctx.set_name("end".into());
    scope.set_context("r".into(), ctx);
    accept(
        &scope,
        StartExpression,
        "[r.start..r.end]",
        r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Path
       │     ├─ Name
       │     │  └─ `r`
       │     └─ Name
       │        └─ `start`
       └─ IntervalEnd (closed)
          └─ Path
             ├─ Name
             │  └─ `r`
             └─ Name
                └─ `end`
    "#,
        false,
    );
}

#[test]
fn _00010() {
    let scope = scope!();
    scope.set_entry_name("r".into());
    scope.set_entry_name("start".into());
    scope.set_entry_name("end".into());
    accept(
        &scope,
        StartExpression,
        "[r.start..r.end]",
        r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Path
       │     ├─ Name
       │     │  └─ `r`
       │     └─ Name
       │        └─ `start`
       └─ IntervalEnd (closed)
          └─ Path
             ├─ Name
             │  └─ `r`
             └─ Name
                └─ `end`
    "#,
        false,
    );
}

#[test]
fn _00011() {
    let scope = scope!();
    scope.set_entry_name("r".into());
    scope.set_entry_name("s".into());
    scope.set_entry_name("start".into());
    scope.set_entry_name("end".into());
    accept(
        &scope,
        StartExpression,
        "[r.start..r.s.end]",
        r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Path
       │     ├─ Name
       │     │  └─ `r`
       │     └─ Name
       │        └─ `start`
       └─ IntervalEnd (closed)
          └─ Path
             ├─ Path
             │  ├─ Name
             │  │  └─ `r`
             │  └─ Name
             │     └─ `s`
             └─ Name
                └─ `end`
    "#,
        false,
    );
}
