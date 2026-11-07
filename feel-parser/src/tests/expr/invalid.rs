//! Testing parsing of invalid statements.

use super::super::*;
use dsntk_common::DsntkError;
use std::str::from_utf8;

/// Utility function to shorten repeatable code in tests.
fn te(input: &str, source: &str, message: &str) {
    assert_eq!(
        Err(DsntkError::new(source, message)),
        Parser::new(&scope!(), StartExpression, input, false).parse()
    )
}

#[test]
fn _0001() {
    // Caret character is not valid at the beginning of any FEEL statement.
    te(r#"^123"#, "ParserError", r#"syntax error: ^123"#);
}

#[test]
fn _0002() {
    // Vertical space is not allowed inside FEEL string.
    let input = from_utf8(&[34, 49, 50, 10, 51, 34]).unwrap(); // "12\n3"
    te(input, "ParserError", "syntax error: \"12\n3\"");
}

#[test]
fn _0003() {
    // Unexpected end of input before the FEEL string is closed with quotation mark `"`.
    te(r#""123"#, "ParserError", "syntax error: \"123");
}

#[test]
fn _0004() {
    // Unexpected end of input when parsing hex digits.
    te(r#""a\u4B"#, "LexerError", "unexpected end of file");
}

#[test]
fn _0005() {
    // After decimal point must always be a digit.
    te(r#"1. + 2"#, "ParserError", "syntax error: 1. + 2");
}

#[test]
fn _0006() {
    // After decimal point must be a digit.
    te(r#"1.^ + 2"#, "ParserError", "syntax error: 1.^ + 2");
}

#[test]
fn _0007() {
    // End of input after decimal point.
    te(r#"1."#, "ParserError", "syntax error: 1.");
}

#[test]
fn _0008() {
    // Keyword `function` interpreted as a name. This is ok, when such name is in scope.
    let scope = scope!();
    accept(
        &scope,
        StartExpression,
        "function ",
        r#"
       Name
       └─ `function`
    "#,
        false,
    );
}

#[test]
fn _0009() {
    // Invalid UTF-16 surrogate.
    te(
        r#""\uD800\uE000""#,
        "LexerError",
        "surrogate value is out of allowed range 0xD800..0xDFFF : E000",
    );
}

#[test]
fn _0010() {
    // Invalid UTF value.
    te(
        r#""\U110000""#,
        "LexerError",
        "value is out of allowed Unicode range 0x0000..0x10FFFF : 110000",
    );
}

#[test]
fn _0011() {
    // There is no `+=` operator in FEEL, but this expression parses properly (will return null on execution).
    let input = "1 += 2";
    let expected = r#"
       Add
       ├─ Numeric
       │  └─ `1`
       └─ UnaryEq
          └─ Numeric
             └─ `2`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}
