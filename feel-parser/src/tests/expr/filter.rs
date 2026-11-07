use super::super::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "[1,2,3][2]",
        r#"
       Filter
       ├─ List
       │  ├─ Numeric
       │  │  └─ `1`
       │  ├─ Numeric
       │  │  └─ `2`
       │  └─ Numeric
       │     └─ `3`
       └─ Numeric
          └─ `2`
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
        "[1,2,3][item >= 2]",
        r#"
       Filter
       ├─ List
       │  ├─ Numeric
       │  │  └─ `1`
       │  ├─ Numeric
       │  │  └─ `2`
       │  └─ Numeric
       │     └─ `3`
       └─ Ge
          ├─ Name
          │  └─ `item`
          └─ Numeric
             └─ `2`
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
        "EmployeeTable[name=LastName]",
        r#"
       Filter
       ├─ Name
       │  └─ `EmployeeTable`
       └─ Eq
          ├─ Name
          │  └─ `name`
          └─ Name
             └─ `LastName`
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
        "EmployeeTable[1].deptNum",
        r#"
       Path
       ├─ Filter
       │  ├─ Name
       │  │  └─ `EmployeeTable`
       │  └─ Numeric
       │     └─ `1`
       └─ Name
          └─ `deptNum`
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
        r#"DeptTable[number=EmployeeTable[name=LastName].deptNum[1]].manager[1]"#,
        r#"
       Filter
       ├─ Path
       │  ├─ Filter
       │  │  ├─ Name
       │  │  │  └─ `DeptTable`
       │  │  └─ Eq
       │  │     ├─ Name
       │  │     │  └─ `number`
       │  │     └─ Filter
       │  │        ├─ Path
       │  │        │  ├─ Filter
       │  │        │  │  ├─ Name
       │  │        │  │  │  └─ `EmployeeTable`
       │  │        │  │  └─ Eq
       │  │        │  │     ├─ Name
       │  │        │  │     │  └─ `name`
       │  │        │  │     └─ Name
       │  │        │  │        └─ `LastName`
       │  │        │  └─ Name
       │  │        │     └─ `deptNum`
       │  │        └─ Numeric
       │  │           └─ `1`
       │  └─ Name
       │     └─ `manager`
       └─ Numeric
          └─ `1`
    "#,
        false,
    );
}

#[test]
fn _0006() {
    let scope = scope!();
    scope.set_entry_name("?".into());
    scope.set_entry_name("Lender Name".into());
    accept(
        &scope,
        StartExpression,
        r#"count(Ratings[Lender Name = ?.Lender Name and Customer Rating > 4]) > 0"#,
        r#"
       Gt
       ├─ FunctionInvocation
       │  ├─ Name
       │  │  └─ `count`
       │  └─ PositionalParameters
       │     └─ Filter
       │        ├─ Name
       │        │  └─ `Ratings`
       │        └─ And
       │           ├─ Eq
       │           │  ├─ Name
       │           │  │  └─ `Lender Name`
       │           │  └─ Path
       │           │     ├─ Name
       │           │     │  └─ `?`
       │           │     └─ Name
       │           │        └─ `Lender Name`
       │           └─ Gt
       │              ├─ Name
       │              │  └─ `Customer Rating`
       │              └─ Numeric
       │                 └─ `4`
       └─ Numeric
          └─ `0`
    "#,
        false,
    );
}
