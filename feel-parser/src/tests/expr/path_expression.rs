use super::super::*;
use crate::context::ParsingContext;

#[test]
fn _0001() {
    let input = "1.first";
    let expected = r#"
       Path
       ├─ Numeric
       │  └─ `1`
       └─ Name
          └─ `first`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0002() {
    let scope = scope!();
    scope.set_entry_name("Manager".into());
    accept(
        &scope,
        StartExpression,
        r#"Manager.Name"#,
        r#"
       Path
       ├─ Name
       │  └─ `Manager`
       └─ Name
          └─ `Name`
    "#,
        false,
    );
}

#[test]
fn _0003() {
    let scope = scope!();
    scope.set_entry_name("Manager".into());
    accept(
        &scope,
        StartExpression,
        r#"Manager.Address.Street"#,
        r#"
       Path
       ├─ Name
       │  └─ `Manager`
       └─ Name
          └─ `Address.Street`
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
        r#"(Manager.Address).Street"#,
        r#"
       Path
       ├─ Name
       │  └─ `Manager.Address`
       └─ Name
          └─ `Street`
    "#,
        false,
    );
}

#[test]
fn _0005() {
    let scope = scope!();
    scope.set_entry_name("Manager".into());
    scope.set_entry_name("Address".into());
    accept(
        &scope,
        StartExpression,
        r#"Manager.Address.Street"#,
        r#"
       Path
       ├─ Path
       │  ├─ Name
       │  │  └─ `Manager`
       │  └─ Name
       │     └─ `Address`
       └─ Name
          └─ `Street`
    "#,
        false,
    );
}

#[test]
fn _0006() {
    let scope = scope!();
    scope.set_entry_name("Manager".into());
    scope.set_entry_name("Address".into());
    scope.set_entry_name("City".into());
    scope.set_entry_name("Street".into());
    accept(
        &scope,
        StartExpression,
        r#"Manager.Address.City.Street"#,
        r#"
       Path
       ├─ Path
       │  ├─ Path
       │  │  ├─ Name
       │  │  │  └─ `Manager`
       │  │  └─ Name
       │  │     └─ `Address`
       │  └─ Name
       │     └─ `City`
       └─ Name
          └─ `Street`
    "#,
        false,
    );
}

#[test]
fn _0007() {
    let scope = scope!();
    let mut ctx_a = ParsingContext::default();
    ctx_a.set_name("Street".into());
    let mut ctx_b = ParsingContext::default();
    ctx_b.set_context("Address".into(), ctx_a);
    scope.set_context("Manager".into(), ctx_b);
    accept(
        &scope,
        StartExpression,
        r#"Manager.Address.Street"#,
        r#"
       Path
       ├─ Path
       │  ├─ Name
       │  │  └─ `Manager`
       │  └─ Name
       │     └─ `Address`
       └─ Name
          └─ `Street`
    "#,
        false,
    );
}

#[test]
fn _0008() {
    let scope = scope!();
    let mut ctx_1 = ParsingContext::default();
    ctx_1.set_name("principal".into());
    scope.set_context("loan".into(), ctx_1);
    accept(
        &scope,
        StartExpression,
        r#"(loan.principal)"#,
        r#"
       Path
       ├─ Name
       │  └─ `loan`
       └─ Name
          └─ `principal`
    "#,
        false,
    );
}

#[test]
fn _0009() {
    let scope = scope!();
    scope.set_entry_name("principal".into());
    scope.set_entry_name("loan".into());
    scope.set_entry_name("id".into());
    accept(
        &scope,
        StartExpression,
        r#"(loan.principal.id)"#,
        r#"
       Path
       ├─ Path
       │  ├─ Name
       │  │  └─ `loan`
       │  └─ Name
       │     └─ `principal`
       └─ Name
          └─ `id`
    "#,
        false,
    );
}

#[test]
fn _0010() {
    let scope = scope!();
    scope.set_entry_name("principal".into());
    scope.set_entry_name("loan".into());
    scope.set_entry_name("id".into());
    scope.set_entry_name("type".into());
    accept(
        &scope,
        StartExpression,
        r#"(loan.principal.id.type - 1)"#,
        r#"
       Sub
       ├─ Path
       │  ├─ Path
       │  │  ├─ Path
       │  │  │  ├─ Name
       │  │  │  │  └─ `loan`
       │  │  │  └─ Name
       │  │  │     └─ `principal`
       │  │  └─ Name
       │  │     └─ `id`
       │  └─ Name
       │     └─ `type`
       └─ Numeric
          └─ `1`
    "#,
        false,
    );
}

#[test]
fn _0011() {
    let scope = scope!();
    let mut ctx_1 = ParsingContext::default();
    ctx_1.set_name("principal".into());
    ctx_1.set_name("rate".into());
    ctx_1.set_name("termMonths".into());
    scope.set_context("loan".into(), ctx_1);
    accept(
        &scope,
        StartExpression,
        r#"(loan.principal) + (loan.rate)"#,
        r#"
       Add
       ├─ Path
       │  ├─ Name
       │  │  └─ `loan`
       │  └─ Name
       │     └─ `principal`
       └─ Path
          ├─ Name
          │  └─ `loan`
          └─ Name
             └─ `rate`
    "#,
        false,
    );
}

#[test]
fn _0012() {
    let scope = scope!();
    let mut ctx_1 = ParsingContext::default();
    ctx_1.set_name("fromString".into());
    scope.set_context("Date".into(), ctx_1);
    accept(
        &scope,
        StartExpression,
        r#"Date.fromString.day"#,
        r#"
       Path
       ├─ Path
       │  ├─ Name
       │  │  └─ `Date`
       │  └─ Name
       │     └─ `fromString`
       └─ Name
          └─ `day`
    "#,
        false,
    );
}

#[test]
fn _0013() {
    let scope = scope!();
    let mut ctx_bounds = ParsingContext::default();
    let mut ctx_min = ParsingContext::default();
    ctx_min.set_name("Tenor".into());
    ctx_min.set_name("Rate".into());
    ctx_bounds.set_context("Min".into(), ctx_min);
    let mut ctx_max = ParsingContext::default();
    ctx_max.set_name("Tenor".into());
    ctx_max.set_name("Rate".into());
    ctx_bounds.set_context("Max".into(), ctx_max);
    scope.set_context("Bounds".into(), ctx_bounds);
    scope.set_entry_name("Days".into());
    let input = "((Bounds.Max.Rate - Bounds.Min.Rate) * (Days - Bounds.Min.Tenor) / (Bounds.Max.Tenor - Bounds.Min.Tenor)) + Bounds.Min.Rate";
    let expected = r#"
       Add
       ├─ Div
       │  ├─ Mul
       │  │  ├─ Sub
       │  │  │  ├─ Path
       │  │  │  │  ├─ Path
       │  │  │  │  │  ├─ Name
       │  │  │  │  │  │  └─ `Bounds`
       │  │  │  │  │  └─ Name
       │  │  │  │  │     └─ `Max`
       │  │  │  │  └─ Name
       │  │  │  │     └─ `Rate`
       │  │  │  └─ Path
       │  │  │     ├─ Path
       │  │  │     │  ├─ Name
       │  │  │     │  │  └─ `Bounds`
       │  │  │     │  └─ Name
       │  │  │     │     └─ `Min`
       │  │  │     └─ Name
       │  │  │        └─ `Rate`
       │  │  └─ Sub
       │  │     ├─ Name
       │  │     │  └─ `Days`
       │  │     └─ Path
       │  │        ├─ Path
       │  │        │  ├─ Name
       │  │        │  │  └─ `Bounds`
       │  │        │  └─ Name
       │  │        │     └─ `Min`
       │  │        └─ Name
       │  │           └─ `Tenor`
       │  └─ Sub
       │     ├─ Path
       │     │  ├─ Path
       │     │  │  ├─ Name
       │     │  │  │  └─ `Bounds`
       │     │  │  └─ Name
       │     │  │     └─ `Max`
       │     │  └─ Name
       │     │     └─ `Tenor`
       │     └─ Path
       │        ├─ Path
       │        │  ├─ Name
       │        │  │  └─ `Bounds`
       │        │  └─ Name
       │        │     └─ `Min`
       │        └─ Name
       │           └─ `Tenor`
       └─ Path
          ├─ Path
          │  ├─ Name
          │  │  └─ `Bounds`
          │  └─ Name
          │     └─ `Min`
          └─ Name
             └─ `Rate`
    "#;
    accept(&scope, StartExpression, input, expected, false);
}

#[test]
fn _0014() {
    let input = "{first: 10, second: 20}.first";
    let expected = r#"
       Path
       ├─ Context
       │  ├─ ContextEntry
       │  │  ├─ ContextEntryKey
       │  │  │  └─ `first`
       │  │  └─ Numeric
       │  │     └─ `10`
       │  └─ ContextEntry
       │     ├─ ContextEntryKey
       │     │  └─ `second`
       │     └─ Numeric
       │        └─ `20`
       └─ Name
          └─ `first`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}
