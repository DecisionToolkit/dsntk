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

/// This expression parses with an **empty** scope, and every `and` in it stays an operator.
///
/// With no scope to resolve names against, the lexer takes the **longest** candidate name
/// unless a reserved word cuts it first. Without that cut, `Original Flight.From and To`
/// would become a single member called `From and To`, and this expression would fail to
/// parse.
///
/// The same absorption produces a silent wrong answer elsewhere. `c.noAddresses or true`
/// would evaluate to `null` instead of `true`. That kills the count guard in two of the
/// twenty-four rules of the benchmark model.
///
/// The two expectations are mutually exclusive. The absorption in this expression is itself
/// after a dot. So no narrowing of the cut can both fix `c.noAddresses or true` and keep
/// this expression unparseable. The conformance signal is the TCK, which stays at 3538
/// passing, unchanged.
///
/// **Correction.** An earlier version of this comment claimed a reserved word is not
/// permitted inside a `FEEL` name by grammar rule 25. That is wrong. Clause 10.3.1.6 gives
/// `Profit and loss` as an example of a name that contains a keyword. It resolves the
/// ambiguity by matching name tokens against **the names in-scope**, longest match
/// preferred. So a reserved word inside a name is legal, and the cut is not derived from
/// the grammar. It is a decision taken in the gap the specification leaves. No candidate
/// matched the scope, so there is no longest match to prefer. The pre-existing behavior of
/// taking the longest lexical run was equally an invention. The cut is the reading that
/// keeps the operator, and it can only reach a name the scope does not know.
#[test]
fn _0003() {
  let input = r#"Flights[ From = Original Flight.From and To = Original Flight.To and Departure > Original Flight.Departure and Status = "scheduled" and has capacity(item, Reassigned Passengers List)][1]"#;
  let node = Parser::new(&scope!(), StartExpression, input, false).parse();
  let tree = node.expect("parses without a scope: `and` is an operator, not part of a name").trace();
  // The member names stop at the reserved word.
  assert!(tree.contains("`From`"), "expected a member named `From`:\n{tree}");
  assert!(tree.contains("`To`"), "expected a member named `To`:\n{tree}");
  assert!(tree.contains("`Departure`"), "expected a member named `Departure`:\n{tree}");
  // and the absorbed forms are gone.
  assert!(!tree.contains("From and To"), "`and` was absorbed into a name:\n{tree}");
  assert!(!tree.contains("Departure and Status"), "`and` was absorbed into a name:\n{tree}");
  // Each `and` survives as an operator: four of them join the five comparisons.
  assert_eq!(4, tree.matches("And").count(), "expected four `and` operators:\n{tree}");
}

/// A path member is followed by `in`, and the `in` stays the comparison operator of grammar
/// rule 49.d, `expression in ( positive unary tests )`.
///
/// The scope knows `t` but knows nothing about its members, which is what a `typeRef="Any"`
/// parameter gives the parser. Without the cut in [crate::lexer::Lexer::consume_name], the
/// member name would absorb `in` and become `type in`. The following `(` would then read
/// that absent member as the callee of a function invocation. So `t.type in ("a","b")`
/// would answer `null` instead of a boolean, and `count(X[type in ("a","b")])` would
/// answer 0.
///
/// The `till_in` branch above the fallback does not cover this. It only trims the **variable**
/// name of a `for` or quantified expression, so it never sees an `in` used as a comparison.
#[test]
fn _0004() {
  let scope = scope!();
  scope.set_context("t".into(), ParsingContext::default());
  accept(
    &scope,
    StartExpression,
    r#"t.type in ("a","b")"#,
    r#"
       In
       ├─ Path
       │  ├─ Name
       │  │  └─ `t`
       │  └─ Name
       │     └─ `type`
       └─ ExpressionList
          ├─ String
          │  └─ `a`
          └─ String
             └─ `b`
    "#,
    false,
  );
}
