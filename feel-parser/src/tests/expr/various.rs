use super::super::*;
use crate::context::ParsingContext;

#[test]
fn _0001() {
    let scope = scope!();
    let mut ctx_1 = ParsingContext::default();
    ctx_1.set_name("tenor".into());
    ctx_1.set_name("rate".into());
    scope.set_context("offer".into(), ctx_1);
    let mut ctx_2 = ParsingContext::default();
    ctx_2.set_name("tenor".into());
    ctx_2.set_name("rate".into());
    scope.set_context("bid".into(), ctx_2);
    accept(
        &scope,
        StartExpression,
        r#"for i in 1..6 return ((log(1 + (offer[i].rate / 100) * (offer[i].tenor / 365))) / (offer[i].tenor / 365) + (log(1 + (bid[i].rate / 100) * (bid[i].tenor / 365))) / (bid[i].tenor / 365)) / 2"#,
        r#"
       For
       ├─ IterationContexts
       │  └─ IterationContextInterval
       │     ├─ Name
       │     │  └─ `i`
       │     ├─ Numeric
       │     │  └─ `1`
       │     └─ Numeric
       │        └─ `6`
       └─ EvaluatedExpression
          └─ Div
             ├─ Add
             │  ├─ Div
             │  │  ├─ FunctionInvocation
             │  │  │  ├─ Name
             │  │  │  │  └─ `log`
             │  │  │  └─ PositionalParameters
             │  │  │     └─ Add
             │  │  │        ├─ Numeric
             │  │  │        │  └─ `1`
             │  │  │        └─ Mul
             │  │  │           ├─ Div
             │  │  │           │  ├─ Path
             │  │  │           │  │  ├─ Filter
             │  │  │           │  │  │  ├─ Name
             │  │  │           │  │  │  │  └─ `offer`
             │  │  │           │  │  │  └─ Name
             │  │  │           │  │  │     └─ `i`
             │  │  │           │  │  └─ Name
             │  │  │           │  │     └─ `rate`
             │  │  │           │  └─ Numeric
             │  │  │           │     └─ `100`
             │  │  │           └─ Div
             │  │  │              ├─ Path
             │  │  │              │  ├─ Filter
             │  │  │              │  │  ├─ Name
             │  │  │              │  │  │  └─ `offer`
             │  │  │              │  │  └─ Name
             │  │  │              │  │     └─ `i`
             │  │  │              │  └─ Name
             │  │  │              │     └─ `tenor`
             │  │  │              └─ Numeric
             │  │  │                 └─ `365`
             │  │  └─ Div
             │  │     ├─ Path
             │  │     │  ├─ Filter
             │  │     │  │  ├─ Name
             │  │     │  │  │  └─ `offer`
             │  │     │  │  └─ Name
             │  │     │  │     └─ `i`
             │  │     │  └─ Name
             │  │     │     └─ `tenor`
             │  │     └─ Numeric
             │  │        └─ `365`
             │  └─ Div
             │     ├─ FunctionInvocation
             │     │  ├─ Name
             │     │  │  └─ `log`
             │     │  └─ PositionalParameters
             │     │     └─ Add
             │     │        ├─ Numeric
             │     │        │  └─ `1`
             │     │        └─ Mul
             │     │           ├─ Div
             │     │           │  ├─ Path
             │     │           │  │  ├─ Filter
             │     │           │  │  │  ├─ Name
             │     │           │  │  │  │  └─ `bid`
             │     │           │  │  │  └─ Name
             │     │           │  │  │     └─ `i`
             │     │           │  │  └─ Name
             │     │           │  │     └─ `rate`
             │     │           │  └─ Numeric
             │     │           │     └─ `100`
             │     │           └─ Div
             │     │              ├─ Path
             │     │              │  ├─ Filter
             │     │              │  │  ├─ Name
             │     │              │  │  │  └─ `bid`
             │     │              │  │  └─ Name
             │     │              │  │     └─ `i`
             │     │              │  └─ Name
             │     │              │     └─ `tenor`
             │     │              └─ Numeric
             │     │                 └─ `365`
             │     └─ Div
             │        ├─ Path
             │        │  ├─ Filter
             │        │  │  ├─ Name
             │        │  │  │  └─ `bid`
             │        │  │  └─ Name
             │        │  │     └─ `i`
             │        │  └─ Name
             │        │     └─ `tenor`
             │        └─ Numeric
             │           └─ `365`
             └─ Numeric
                └─ `2`
    "#,
        false,
    );
}

#[test]
fn _0002() {
    let scope = scope!();
    scope.set_entry_name("Days".into());
    let mut ctx_1 = ParsingContext::default();

    let mut ctx_2 = ParsingContext::default();
    ctx_2.set_name("Tenor".into());
    ctx_2.set_name("Rate".into());

    ctx_1.set_context("Min".into(), ctx_2);

    let mut ctx_3 = ParsingContext::default();
    ctx_3.set_name("Tenor".into());
    ctx_3.set_name("Rate".into());

    ctx_1.set_context("Max".into(), ctx_3);

    scope.set_context("Bounds".into(), ctx_1);

    let input = r#"(((Bounds.Max.Rate - Bounds.Min.Rate) * (Days - Bounds.Min.Tenor)) / (Bounds.Max.Tenor - Bounds.Min.Tenor)) + Bounds.Min.Rate"#;
    accept(
        &scope,
        StartExpression,
        input,
        r#"
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
    "#,
        false,
    );
}

#[test]
fn _0003() {
    let input = r#"Flights[ From = Original Flight.From and To = Original Flight.To and Departure > Original Flight.Departure and Status = "scheduled" and has capacity(item, Reassigned Passengers List)][1]"#;
    // this test should fail without properly set scope
    let node = Parser::new(&scope!(), StartExpression, input, false).parse();
    assert!(node.is_err());
}
