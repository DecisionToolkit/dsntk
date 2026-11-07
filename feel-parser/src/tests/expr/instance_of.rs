use super::super::*;
use crate::context::ParsingContext;

#[test]
fn _0001() {
    let scope = scope!();
    scope.set_entry_name("Person".into());
    accept(
        &scope,
        StartExpression,
        "Person instance of number",
        r#"
       InstanceOf
       ├─ Name
       │  └─ `Person`
       └─ FeelType
          └─ number
    "#,
        false,
    );
}

#[test]
fn _0002() {
    let scope = scope!();
    scope.set_entry_name("Person".into());
    accept(
        &scope,
        StartExpression,
        "Person instance of function<string,string,number>->string",
        r#"
       InstanceOf
       ├─ Name
       │  └─ `Person`
       └─ FunctionType
          ├─ ParameterTypes
          │  ├─ FeelType
          │  │  └─ string
          │  ├─ FeelType
          │  │  └─ string
          │  └─ FeelType
          │     └─ number
          └─ FeelType
             └─ string
    "#,
        false,
    );
}

#[test]
fn _0003() {
    let scope = scope!();
    scope.set_entry_name("Person".into());
    accept(
        &scope,
        StartExpression,
        "Person instance of function<string>->string",
        r#"
       InstanceOf
       ├─ Name
       │  └─ `Person`
       └─ FunctionType
          ├─ ParameterTypes
          │  └─ FeelType
          │     └─ string
          └─ FeelType
             └─ string
    "#,
        false,
    );
}

#[test]
fn _0004() {
    let scope = scope!();
    scope.set_entry_name("Person".into());
    accept(
        &scope,
        StartExpression,
        "Person instance of function<>->number",
        r#"
       InstanceOf
       ├─ Name
       │  └─ `Person`
       └─ FunctionType
          ├─ ParameterTypes
          │  └─ (empty)
          └─ FeelType
             └─ number
    "#,
        false,
    );
}

#[test]
fn _0005() {
    let scope = scope!();
    scope.set_entry_name("Numbers".into());
    accept(
        &scope,
        StartExpression,
        "Numbers instance of list<number>",
        r#"
       InstanceOf
       ├─ Name
       │  └─ `Numbers`
       └─ ListType
          └─ FeelType
             └─ number
    "#,
        false,
    );
}

#[test]
fn _0006() {
    let scope = scope!();
    scope.set_entry_name("Person".into());
    accept(
        &scope,
        StartExpression,
        "Person instance of context<name:string,age:number>",
        r#"
       InstanceOf
       ├─ Name
       │  └─ `Person`
       └─ ContextType
          ├─ ContextTypeEntry
          │  ├─ Name
          │  │  └─ `name`
          │  └─ FeelType
          │     └─ string
          └─ ContextTypeEntry
             ├─ Name
             │  └─ `age`
             └─ FeelType
                └─ number
    "#,
        false,
    );
}

#[test]
fn _0006_1() {
    let scope = scope!();
    scope.set_entry_name("Person".into());
    accept(
        &scope,
        StartExpression,
        "  \nPerson \r instance of \t context  <   name:  string ,  age  : number >  ",
        r#"
       InstanceOf
       ├─ Name
       │  └─ `Person`
       └─ ContextType
          ├─ ContextTypeEntry
          │  ├─ Name
          │  │  └─ `name`
          │  └─ FeelType
          │     └─ string
          └─ ContextTypeEntry
             ├─ Name
             │  └─ `age`
             └─ FeelType
                └─ number
    "#,
        false,
    );
}

#[test]
fn _0007() {
    let scope = scope!();
    scope.set_entry_name("Numbers".into());
    accept(
        &scope,
        StartExpression,
        "Numbers instance of range<number>",
        r#"
       InstanceOf
       ├─ Name
       │  └─ `Numbers`
       └─ RangeType
          └─ FeelType
             └─ number
    "#,
        false,
    );
}

#[test]
fn _0008() {
    let scope = scope!();
    scope.set_entry_name("Power".into());
    let mut ctx = ParsingContext::default();
    ctx.set_name("power".into());
    scope.set_context("engine".into(), ctx);
    accept(
        &scope,
        StartExpression,
        "Power instance of engine.power",
        r#"
       InstanceOf
       ├─ Name
       │  └─ `Power`
       └─ QualifiedName
          ├─ Name
          │  └─ `engine`
          └─ Name
             └─ `power`
    "#,
        false,
    );
}

#[test]
fn _0009() {
    let scope = scope!();
    scope.set_entry_name("Power".into());
    let mut ctx = ParsingContext::default();
    ctx.set_name("power".into());
    scope.set_entry_name("engine".into());
    accept(
        &scope,
        StartExpression,
        "Power instance of engine.power",
        r#"
       InstanceOf
       ├─ Name
       │  └─ `Power`
       └─ QualifiedName
          ├─ Name
          │  └─ `engine`
          └─ Name
             └─ `power`
    "#,
        false,
    );
}

#[test]
fn _0010() {
    let scope = scope!();
    scope.set_entry_name("Items".into());
    accept(
        &scope,
        StartExpression,
        "Items instance of list<list<number>>",
        r#"
       InstanceOf
       ├─ Name
       │  └─ `Items`
       └─ ListType
          └─ ListType
             └─ FeelType
                └─ number
    "#,
        false,
    );
}

#[test]
fn _0011() {
    let scope = scope!();
    scope.set_entry_name("Numbers".into());
    accept(
        &scope,
        StartExpression,
        "Numbers instance of list <number>",
        r#"
       InstanceOf
       ├─ Name
       │  └─ `Numbers`
       └─ ListType
          └─ FeelType
             └─ number
    "#,
        false,
    );
}

#[test]
fn _0012() {
    let scope = scope!();
    scope.set_entry_name("Numbers".into());
    accept(
        &scope,
        StartExpression,
        "  Numbers    instance  of   list    <   number   >   ",
        r#"
       InstanceOf
       ├─ Name
       │  └─ `Numbers`
       └─ ListType
          └─ FeelType
             └─ number
    "#,
        false,
    );
}

#[test]
fn _0013() {
    let scope = scope!();
    scope.set_entry_name("Numbers".into());
    accept(
        &scope,
        StartExpression,
        " Numbers \t     instance  \t of  \t range \r <   \n number \t  >  ",
        r#"
       InstanceOf
       ├─ Name
       │  └─ `Numbers`
       └─ RangeType
          └─ FeelType
             └─ number
    "#,
        false,
    );
}
