use super::super::*;
use crate::lalr::TokenType::StartRangeLiteral;

#[test]
fn _0001() {
  let input = "[1..2]";
  let expected = r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `2`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0002() {
  let input = "(1..2]";
  let expected = r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `2`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0003() {
  let input = "]1..2]";
  let expected = r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `2`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0004() {
  let input = "[1..2)";
  let expected = r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `2`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0005() {
  let input = "[1..2[";
  let expected = r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `2`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0006() {
  let input = "(1..2)";
  let expected = r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `2`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0007() {
  let input = "]1..2[";
  let expected = r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `1`
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `2`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0008() {
  let input = r#"["a".."z"]"#;
  let expected = r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ String
       │     └─ `a`
       └─ IntervalEnd (closed)
          └─ String
             └─ `z`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0009() {
  let input = r#"[date("2012-01-01")..date("2021-12-31")]"#;
  let expected = r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ FunctionInvocation
       │     ├─ Name
       │     │  └─ `date`
       │     └─ PositionalParameters
       │        └─ String
       │           └─ `2012-01-01`
       └─ IntervalEnd (closed)
          └─ FunctionInvocation
             ├─ Name
             │  └─ `date`
             └─ PositionalParameters
                └─ String
                   └─ `2021-12-31`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, true);
}

#[test]
fn _0010() {
  let input = r#"[@"2012-01-01"..@"2021-12-31"]"#;
  let expected = r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ At
       │     └─ `2012-01-01`
       └─ IntervalEnd (closed)
          └─ At
             └─ `2021-12-31`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0011() {
  let input = r#"[false..true]"#;
  Parser::new(&scope!(), StartRangeLiteral, input, false).parse().unwrap_err();
}

#[test]
fn _0012() {
  // Opened start may not have a left bracket as opening character.
  let input = "[..10]";
  Parser::new(&scope!(), StartRangeLiteral, input, false).parse().unwrap_err();
}

#[test]
fn _0013() {
  let input = "(..10)";
  let expected = r#"
       Range
       ├─ IntervalStart (opened,undefined)
       │  └─ Null
       └─ IntervalEnd (opened)
          └─ Numeric
             └─ `10`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0014() {
  let input = "]..10]";
  let expected = r#"
       Range
       ├─ IntervalStart (opened,undefined)
       │  └─ Null
       └─ IntervalEnd (closed)
          └─ Numeric
             └─ `10`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0015() {
  // Opened start may not have a left bracket as opening character.
  let input = "[..10)";
  Parser::new(&scope!(), StartRangeLiteral, input, false).parse().unwrap_err();
}

#[test]
fn _0016() {
  // Opened end may not have a right bracket as closing character.
  let input = "[10..]";
  Parser::new(&scope!(), StartRangeLiteral, input, false).parse().unwrap_err();
}

#[test]
fn _0017() {
  let input = "(10..)";
  let expected = r#"
       Range
       ├─ IntervalStart (opened)
       │  └─ Numeric
       │     └─ `10`
       └─ IntervalEnd (opened,undefined)
          └─ Null
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0018() {
  // Opened end may not have a right bracket as closing character.
  let input = "]10..]";
  Parser::new(&scope!(), StartRangeLiteral, input, false).parse().unwrap_err();
}

#[test]
fn _0019() {
  let input = "[10..)";
  let expected = r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ Numeric
       │     └─ `10`
       └─ IntervalEnd (opened,undefined)
          └─ Null
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0020() {
  // The argument to `date` function must be a string literal.
  let input = r#"[date(string("1970-01-01"))..date("1970-01-02")]"#;
  Parser::new(&scope!(), StartRangeLiteral, input, false).parse().unwrap_err();
}

#[test]
fn _0021() {
  let input = r#"["a".."z"]"#;
  let expected = r#"
       Range
       ├─ IntervalStart (closed)
       │  └─ String
       │     └─ `a`
       └─ IntervalEnd (closed)
          └─ String
             └─ `z`
    "#;
  accept(&scope!(), StartRangeLiteral, input, expected, false);
}

#[test]
fn _0022() {
  // The `string` function is not allowed as range endpoint definition,
  // only temporal functions: `date`, `date and time`, `time` and `duration` are allowed.
  let input = r#"[string("a").."z"]"#;
  Parser::new(&scope!(), StartRangeLiteral, input, false).parse().unwrap_err();
}
