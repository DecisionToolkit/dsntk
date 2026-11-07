use super::super::*;
use crate::lalr::TokenType::*;

#[test]
fn _0001() {
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        r#"  1  // eol comment
         + 1"#,
        r#"
       Add
       ├─ Numeric
       │  └─ `1`
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
        r#" 1
          /*
          some intro waffle
          */
          + 1"#,
        r#"
       Add
       ├─ Numeric
       │  └─ `1`
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
        r#"1 + /* 1 + */ 1"#,
        r#"
       Add
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `1`
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
        r#" 1
          /*
          some intro waffle
          */
          + 1 // and stuff
          + 2"#,
        r#"
       Add
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `1`
       │  └─ Numeric
       │     └─ `1`
       └─ Numeric
          └─ `2`
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
        r#"
            // Some multi-line comment
            // composed from
            // multiple single-line
            // comments
            1 + 2 
    "#,
        r#"
       Add
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `2`
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
        r#"
            /*
             * Some multi-line comment
             * composed from...
             */
             
            /*
             *
             * ...multiple multi-line
             * comments
             */ 
             
            5 * 8 
    "#,
        r#"
       Mul
       ├─ Numeric
       │  └─ `5`
       └─ Numeric
          └─ `8`
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
        r#"
      /// Maybe this comment may be used
      /// for some documentation, like in Rust?
      
      //! Or for some global documentation too, like in Rust?
      {
        /// Maybe this comment may be used for some documentation, like in Rust?
        
        /*
         * We will see.
         */
        A: /* This is cool :-), not like in JSON :-( */ 15 // Just set fiveteen.
      }
    "#,
        r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `A`
          └─ Numeric
             └─ `15`
    "#,
        false,
    );
}
