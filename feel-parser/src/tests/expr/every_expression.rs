use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        r#"every n in [1,2,3] satisfies n > 1.5"#,
        r#"
       Every
       ├─ QuantifiedContexts
       │  └─ QuantifiedContext
       │     ├─ Name
       │     │  └─ `n`
       │     └─ List
       │        ├─ Numeric
       │        │  └─ `1`
       │        ├─ Numeric
       │        │  └─ `2`
       │        └─ Numeric
       │           └─ `3`
       └─ Satisfies
          └─ Gt
             ├─ Name
             │  └─ `n`
             └─ Numeric
                └─ `1.5`
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
        r#"every n in [1,2,3] satisfies n + 1 > 1.5"#,
        r#"
       Every
       ├─ QuantifiedContexts
       │  └─ QuantifiedContext
       │     ├─ Name
       │     │  └─ `n`
       │     └─ List
       │        ├─ Numeric
       │        │  └─ `1`
       │        ├─ Numeric
       │        │  └─ `2`
       │        └─ Numeric
       │           └─ `3`
       └─ Satisfies
          └─ Gt
             ├─ Add
             │  ├─ Name
             │  │  └─ `n`
             │  └─ Numeric
             │     └─ `1`
             └─ Numeric
                └─ `1.5`
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
        r#"every n in [1,2,3], m in <= 100 satisfies n > 1.5 * m"#,
        r#"
       Every
       ├─ QuantifiedContexts
       │  ├─ QuantifiedContext
       │  │  ├─ Name
       │  │  │  └─ `n`
       │  │  └─ List
       │  │     ├─ Numeric
       │  │     │  └─ `1`
       │  │     ├─ Numeric
       │  │     │  └─ `2`
       │  │     └─ Numeric
       │  │        └─ `3`
       │  └─ QuantifiedContext
       │     ├─ Name
       │     │  └─ `m`
       │     └─ UnaryLe
       │        └─ Numeric
       │           └─ `100`
       └─ Satisfies
          └─ Gt
             ├─ Name
             │  └─ `n`
             └─ Mul
                ├─ Numeric
                │  └─ `1.5`
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
        r#"every n in [1..3] satisfies n > 1.5"#,
        r#"
       Every
       ├─ QuantifiedContexts
       │  └─ QuantifiedContext
       │     ├─ Name
       │     │  └─ `n`
       │     └─ Range
       │        ├─ IntervalStart (closed)
       │        │  └─ Numeric
       │        │     └─ `1`
       │        └─ IntervalEnd (closed)
       │           └─ Numeric
       │              └─ `3`
       └─ Satisfies
          └─ Gt
             ├─ Name
             │  └─ `n`
             └─ Numeric
                └─ `1.5`
    "#,
        false,
    );
}
