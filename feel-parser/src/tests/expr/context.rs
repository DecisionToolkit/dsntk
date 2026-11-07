use super::super::*;
use crate::lalr::TokenType::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{}"#,
        r#"
       Context
       └─ (empty)
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
        " \n { \t } \r ",
        r#"
       Context
       └─ (empty)
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
        r#"{age:49}"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `age`
          └─ Numeric
             └─ `49`
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
        r#"{"age":49}"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `age`
          └─ Numeric
             └─ `49`
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
        r#"{Birth_date: date("1956-07-11")}"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `Birth_date`
          └─ FunctionInvocation
             ├─ Name
             │  └─ `date`
             └─ PositionalParameters
                └─ String
                   └─ `1956-07-11`
    "#,
        false,
    );
}

#[test]
fn _0006() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{name:"Dariusz",age:49}"#,
        r#"
       Context
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `name`
       │  └─ String
       │     └─ `Dariusz`
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `age`
          └─ Numeric
             └─ `49`
    "#,
        false,
    );
}

#[test]
fn _0007() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{"name": "Dariusz", "age": 49}"#,
        r#"
       Context
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `name`
       │  └─ String
       │     └─ `Dariusz`
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `age`
          └─ Numeric
             └─ `49`
    "#,
        false,
    );
}

#[test]
fn _0008() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{name:"Dariusz",age:49,car:{model:"Skoda",production year:2016}}"#,
        r#"
       Context
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `name`
       │  └─ String
       │     └─ `Dariusz`
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `age`
       │  └─ Numeric
       │     └─ `49`
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `car`
          └─ Context
             ├─ ContextEntry
             │  ├─ ContextEntryKey
             │  │  └─ `model`
             │  └─ String
             │     └─ `Skoda`
             └─ ContextEntry
                ├─ ContextEntryKey
                │  └─ `production year`
                └─ Numeric
                   └─ `2016`
    "#,
        false,
    );
}

#[test]
fn _0009() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{"name":"Dariusz","age":49,"car":{"model":"Skoda","production year":2016}}"#,
        r#"
       Context
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `name`
       │  └─ String
       │     └─ `Dariusz`
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `age`
       │  └─ Numeric
       │     └─ `49`
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `car`
          └─ Context
             ├─ ContextEntry
             │  ├─ ContextEntryKey
             │  │  └─ `model`
             │  └─ String
             │     └─ `Skoda`
             └─ ContextEntry
                ├─ ContextEntryKey
                │  └─ `production year`
                └─ Numeric
                   └─ `2016`
    "#,
        false,
    );
}

#[test]
fn _00010() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{ a: 1, b: 2, c: 1 + 2}"#,
        r#"
       Context
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `a`
       │  └─ Numeric
       │     └─ `1`
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `b`
       │  └─ Numeric
       │     └─ `2`
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `c`
          └─ Add
             ├─ Numeric
             │  └─ `1`
             └─ Numeric
                └─ `2`
    "#,
        false,
    );
}

#[test]
fn _00011() {
    let scope = scope!();
    scope.set_entry_name("d".into());
    scope.set_entry_name("e".into());
    accept(
        &scope,
        StartBoxedExpression,
        r#"{ a: 1, b: 2, c: d + e}"#,
        r#"
       Context
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `a`
       │  └─ Numeric
       │     └─ `1`
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `b`
       │  └─ Numeric
       │     └─ `2`
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `c`
          └─ Add
             ├─ Name
             │  └─ `d`
             └─ Name
                └─ `e`
    "#,
        false,
    );
}

#[test]
fn _00012() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{ a: 1, b: 2, c: a + b}"#,
        r#"
       Context
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `a`
       │  └─ Numeric
       │     └─ `1`
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `b`
       │  └─ Numeric
       │     └─ `2`
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `c`
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
fn _00013() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"
      {
        "Another Date": @"2018-07-31",
        "Another Date and Time": @"2018-07-31T17:13:00Z",
        "Another Days and Time Duration": @"PT12H",
        "Another String": "Hello",
        "Another Time": @"17:13:00",
        "Another Years and Months Duration": @"P8M",
        "Another boolean": false,
        "Another number": 15,
        Complex: {
          aBoolean: true,
          aDate: @"2018-07-30",
          aDateTime: @"2018-07-30T16:12:00Z",
          aDaysAndTimeDuration: @"PT10H",
          aNumber: 10,
          aString: "Hi",
          aTime: @"16:11:00",
          aYearsAndMonthsDuration: @"P5M"
        }
      }
    "#,
        r#"
       Context
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `Another Date`
       │  └─ At
       │     └─ `2018-07-31`
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `Another Date and Time`
       │  └─ At
       │     └─ `2018-07-31T17:13:00Z`
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `Another Days and Time Duration`
       │  └─ At
       │     └─ `PT12H`
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `Another String`
       │  └─ String
       │     └─ `Hello`
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `Another Time`
       │  └─ At
       │     └─ `17:13:00`
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `Another Years and Months Duration`
       │  └─ At
       │     └─ `P8M`
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `Another boolean`
       │  └─ Boolean
       │     └─ `false`
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `Another number`
       │  └─ Numeric
       │     └─ `15`
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `Complex`
          └─ Context
             ├─ ContextEntry
             │  ├─ ContextEntryKey
             │  │  └─ `aBoolean`
             │  └─ Boolean
             │     └─ `true`
             ├─ ContextEntry
             │  ├─ ContextEntryKey
             │  │  └─ `aDate`
             │  └─ At
             │     └─ `2018-07-30`
             ├─ ContextEntry
             │  ├─ ContextEntryKey
             │  │  └─ `aDateTime`
             │  └─ At
             │     └─ `2018-07-30T16:12:00Z`
             ├─ ContextEntry
             │  ├─ ContextEntryKey
             │  │  └─ `aDaysAndTimeDuration`
             │  └─ At
             │     └─ `PT10H`
             ├─ ContextEntry
             │  ├─ ContextEntryKey
             │  │  └─ `aNumber`
             │  └─ Numeric
             │     └─ `10`
             ├─ ContextEntry
             │  ├─ ContextEntryKey
             │  │  └─ `aString`
             │  └─ String
             │     └─ `Hi`
             ├─ ContextEntry
             │  ├─ ContextEntryKey
             │  │  └─ `aTime`
             │  └─ At
             │     └─ `16:11:00`
             └─ ContextEntry
                ├─ ContextEntryKey
                │  └─ `aYearsAndMonthsDuration`
                └─ At
                   └─ `P5M`
    "#,
        false,
    );
}

#[test]
fn _0014() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{Full Name: "John Doe"}"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `Full Name`
          └─ String
             └─ `John Doe`
    "#,
        false,
    );
}

#[test]
fn _0015() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{Full           Name: "John Doe"}"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `Full Name`
          └─ String
             └─ `John Doe`
    "#,
        false,
    );
}

#[test]
fn _0016() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{"Full Name": "John Doe"}"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `Full Name`
          └─ String
             └─ `John Doe`
    "#,
        false,
    );
}

#[test]
fn _0017() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{"Full           Name": "John Doe"}"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `Full           Name`
          └─ String
             └─ `John Doe`
    "#,
        false,
    );
}

#[test]
fn _0018() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{person: {name: "John", age: 27}}"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `person`
          └─ Context
             ├─ ContextEntry
             │  ├─ ContextEntryKey
             │  │  └─ `name`
             │  └─ String
             │     └─ `John`
             └─ ContextEntry
                ├─ ContextEntryKey
                │  └─ `age`
                └─ Numeric
                   └─ `27`
    "#,
        false,
    );
}

#[test]
fn _0019() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{Another Date: "2018-07-30"}"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `Another Date`
          └─ String
             └─ `2018-07-30`
    "#,
        false,
    );
}

#[test]
fn _0020() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{Another Date and Time: "2018-07-30"}"#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `Another Date and Time`
          └─ String
             └─ `2018-07-30`
    "#,
        false,
    );
}

#[test]
fn _0021() {
    let scope = scope!();
    accept(
        &scope,
        StartBoxedExpression,
        r#"{"Another Date": "2018-07-30", "Another Date and Time": "2018-07-30T16:00:00"}"#,
        r#"
       Context
       ├─ ContextEntry
       │  ├─ ContextEntryKey
       │  │  └─ `Another Date`
       │  └─ String
       │     └─ `2018-07-30`
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `Another Date and Time`
          └─ String
             └─ `2018-07-30T16:00:00`
    "#,
        false,
    );
}
