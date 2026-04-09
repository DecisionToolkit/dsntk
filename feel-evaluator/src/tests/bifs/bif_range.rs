use super::super::*;
use dsntk_feel::scope;

fn te_range(trace: bool, scope: &FeelScope, input: &str, expected: &str) {
  match dsntk_feel_parser::parse_textual_expression(scope, input, trace) {
    Ok(node) => {
      let evaluator = build_evaluator(&node);
      let actual = format!("{}", evaluator(scope));
      assert_eq!(actual, expected, "ERROR\nexpected: {expected}\n  actual: {actual}\n");
    }
    Err(reason) => {
      panic!("parsing textual expression failed with reason: {}", reason);
    }
  }
}

#[test]
fn _0001() {
  te_be_value(false, &scope!(), r#"range("[1..2]")"#, "[1..2]");
}

#[test]
fn _0002() {
  te_be_value(false, &scope!(), r#"range("(1..2]")"#, "(1..2]");
}

#[test]
fn _0003() {
  te_be_value(false, &scope!(), r#"range("]1..2]")"#, "(1..2]");
}

#[test]
fn _0004() {
  te_be_value(false, &scope!(), r#"range("[1..2)")"#, "[1..2)");
}

#[test]
fn _0005() {
  te_be_value(false, &scope!(), r#"range("[1..2[")"#, "[1..2)");
}

#[test]
fn _0006() {
  te_be_value(false, &scope!(), r#"range("(1..2)")"#, "(1..2)");
}

#[test]
fn _0007() {
  te_be_value(false, &scope!(), r#"range("]1..2[")"#, "(1..2)");
}

#[test]
fn _0008() {
  te_range(false, &scope!(), r#"range("[1..)")"#, "[1..)");
}

#[test]
fn _0009() {
  te_range(false, &scope!(), r#"range("[1..[")"#, "[1..)");
}

#[test]
fn _0010() {
  te_null(false, &scope!(), r#"range("[1..]")"#, "invalid range literal");
}

#[test]
fn _0011() {
  te_range(false, &scope!(), r#"range("(..1]")"#, "(..1]");
}

#[test]
fn _0012() {
  te_range(false, &scope!(), r#"range("]..1]")"#, "(..1]");
}

#[test]
fn _0013() {
  te_null(false, &scope!(), r#"range("[..1]")"#, "invalid range literal");
}

#[test]
fn _0014() {
  te_null(false, &scope!(), r#"range()"#, "expected 1 parameters, actual number of parameters is 0");
}

#[test]
fn _0015() {
  te_null(false, &scope!(), r#"range("[1..2]","[3..4]")"#, "expected 1 parameters, actual number of parameters is 2");
}

#[test]
fn _0016() {
  te_null(
    false,
    &scope!(),
    r#"range(1)"#,
    "[core::range] invalid argument type, expected string, actual type is number",
  );
}

#[test]
fn _0017() {
  te_be_value(false, &scope!(), r#"range(from:"[1..2]")"#, "[1..2]");
}

#[test]
fn _0018() {
  te_be_value(false, &scope!(), r#"range(from:"(1..2]")"#, "(1..2]");
}

#[test]
fn _0019() {
  te_be_value(false, &scope!(), r#"range(from:"]1..2]")"#, "(1..2]");
}

#[test]
fn _0020() {
  te_be_value(false, &scope!(), r#"range(from:"[1..2)")"#, "[1..2)");
}

#[test]
fn _0021() {
  te_be_value(false, &scope!(), r#"range(from:"[1..2[")"#, "[1..2)");
}

#[test]
fn _0022() {
  te_be_value(false, &scope!(), r#"range(from:"(1..2)")"#, "(1..2)");
}

#[test]
fn _0023() {
  te_be_value(false, &scope!(), r#"range(from:"]1..2[")"#, "(1..2)");
}

#[test]
fn _0024() {
  te_range(false, &scope!(), r#"range(from:"[1..)")"#, "[1..)");
}

#[test]
fn _0025() {
  te_range(false, &scope!(), r#"range(from:"[1..[")"#, "[1..)");
}

#[test]
fn _0026() {
  te_null(false, &scope!(), r#"range(from:"[1..]")"#, "invalid range literal");
}

#[test]
fn _0027() {
  te_range(false, &scope!(), r#"range(from:"(..1]")"#, "(..1]");
}

#[test]
fn _0028() {
  te_range(false, &scope!(), r#"range(from:"]..1]")"#, "(..1]");
}

#[test]
fn _0029() {
  te_null(false, &scope!(), r#"range(from:"[..1]")"#, "invalid range literal");
}

#[test]
fn _0030() {
  te_null(false, &scope!(), r#"range(to:"[1..2]")"#, "parameter 'from' not found");
}

#[test]
fn _0031() {
  te_null(
    false,
    &scope!(),
    r#"range(from:"[1..2]",to:"[3..4]")"#,
    "expected 1 parameters, actual number of parameters is 2",
  );
}

#[test]
fn _0032() {
  te_null(
    false,
    &scope!(),
    r#"range(from:1)"#,
    "[core::range] invalid argument type, expected string, actual type is number",
  );
}
