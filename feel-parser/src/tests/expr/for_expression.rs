use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        r#"for n in 1..10 return n + 1"#,
        r#"
       For
       ├─ IterationContexts
       │  └─ IterationContextInterval
       │     ├─ Name
       │     │  └─ `n`
       │     ├─ Numeric
       │     │  └─ `1`
       │     └─ Numeric
       │        └─ `10`
       └─ EvaluatedExpression
          └─ Add
             ├─ Name
             │  └─ `n`
             └─ Numeric
                └─ `1`
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
        r#"for n in [1,2,3] return n + 1"#,
        r#"
       For
       ├─ IterationContexts
       │  └─ IterationContextSingle
       │     ├─ Name
       │     │  └─ `n`
       │     └─ List
       │        ├─ Numeric
       │        │  └─ `1`
       │        ├─ Numeric
       │        │  └─ `2`
       │        └─ Numeric
       │           └─ `3`
       └─ EvaluatedExpression
          └─ Add
             ├─ Name
             │  └─ `n`
             └─ Numeric
                └─ `1`
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
        r#"for n in 1..10, m in 100..200 return n + m"#,
        r#"
       For
       ├─ IterationContexts
       │  ├─ IterationContextInterval
       │  │  ├─ Name
       │  │  │  └─ `n`
       │  │  ├─ Numeric
       │  │  │  └─ `1`
       │  │  └─ Numeric
       │  │     └─ `10`
       │  └─ IterationContextInterval
       │     ├─ Name
       │     │  └─ `m`
       │     ├─ Numeric
       │     │  └─ `100`
       │     └─ Numeric
       │        └─ `200`
       └─ EvaluatedExpression
          └─ Add
             ├─ Name
             │  └─ `n`
             └─ Name
                └─ `m`
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
        r#"for n in [1,5,8], m in [15,18,-3] return n * m"#,
        r#"
       For
       ├─ IterationContexts
       │  ├─ IterationContextSingle
       │  │  ├─ Name
       │  │  │  └─ `n`
       │  │  └─ List
       │  │     ├─ Numeric
       │  │     │  └─ `1`
       │  │     ├─ Numeric
       │  │     │  └─ `5`
       │  │     └─ Numeric
       │  │        └─ `8`
       │  └─ IterationContextSingle
       │     ├─ Name
       │     │  └─ `m`
       │     └─ List
       │        ├─ Numeric
       │        │  └─ `15`
       │        ├─ Numeric
       │        │  └─ `18`
       │        └─ Neg
       │           └─ Numeric
       │              └─ `3`
       └─ EvaluatedExpression
          └─ Mul
             ├─ Name
             │  └─ `n`
             └─ Name
                └─ `m`
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
        r#"for n in [1,5,8], m in [15,18,-3] return (n + 2) * (m -4)"#,
        r#"
       For
       ├─ IterationContexts
       │  ├─ IterationContextSingle
       │  │  ├─ Name
       │  │  │  └─ `n`
       │  │  └─ List
       │  │     ├─ Numeric
       │  │     │  └─ `1`
       │  │     ├─ Numeric
       │  │     │  └─ `5`
       │  │     └─ Numeric
       │  │        └─ `8`
       │  └─ IterationContextSingle
       │     ├─ Name
       │     │  └─ `m`
       │     └─ List
       │        ├─ Numeric
       │        │  └─ `15`
       │        ├─ Numeric
       │        │  └─ `18`
       │        └─ Neg
       │           └─ Numeric
       │              └─ `3`
       └─ EvaluatedExpression
          └─ Mul
             ├─ Add
             │  ├─ Name
             │  │  └─ `n`
             │  └─ Numeric
             │     └─ `2`
             └─ Sub
                ├─ Name
                │  └─ `m`
                └─ Numeric
                   └─ `4`
    "#,
        false,
    );
}

#[test]
fn _0006() {
    let scope = scope!();
    scope.set_entry_name("N".into());
    accept(
        &scope,
        StartExpression,
        r#"for i in 0..N return if i = 0 then 1 else i * partial[-1]"#,
        r#"
       For
       ├─ IterationContexts
       │  └─ IterationContextInterval
       │     ├─ Name
       │     │  └─ `i`
       │     ├─ Numeric
       │     │  └─ `0`
       │     └─ Name
       │        └─ `N`
       └─ EvaluatedExpression
          └─ If
             ├─ Eq
             │  ├─ Name
             │  │  └─ `i`
             │  └─ Numeric
             │     └─ `0`
             ├─ Numeric
             │  └─ `1`
             └─ Mul
                ├─ Name
                │  └─ `i`
                └─ Filter
                   ├─ Name
                   │  └─ `partial`
                   └─ Neg
                      └─ Numeric
                         └─ `1`
    "#,
        false,
    );
}

#[test]
fn _0007() {
    // This test is not correct syntactically, because the variable `partial` is a list,
    // but only this way it is well tested, if this variable is properly defined
    // in local context during parsing the `return` expression.
    let scope = scope!();
    scope.set_entry_name("N".into());
    accept(
        &scope,
        StartExpression,
        r#"for i in 0..N return if i = 0 then 1 else i * (partial + 1)"#,
        r#"
       For
       ├─ IterationContexts
       │  └─ IterationContextInterval
       │     ├─ Name
       │     │  └─ `i`
       │     ├─ Numeric
       │     │  └─ `0`
       │     └─ Name
       │        └─ `N`
       └─ EvaluatedExpression
          └─ If
             ├─ Eq
             │  ├─ Name
             │  │  └─ `i`
             │  └─ Numeric
             │     └─ `0`
             ├─ Numeric
             │  └─ `1`
             └─ Mul
                ├─ Name
                │  └─ `i`
                └─ Add
                   ├─ Name
                   │  └─ `partial`
                   └─ Numeric
                      └─ `1`
    "#,
        false,
    );
}
