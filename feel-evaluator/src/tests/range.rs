use super::*;
use dsntk_feel::{IntervalType, scope};

#[test]
fn _0001() {
  te_bool(false, &scope!(), r#"2 in [1..5]"#, true);
}

#[test]
fn _0002() {
  te_null(false, &scope!(), r#"2 in ["a".."z"]"#, "eval_in_range");
}

#[test]
fn _0003() {
  te_null(false, &scope!(), r#"2 in [1.."z"]"#, "eval_in_range");
}

#[test]
fn _0004() {
  te_bool(false, &scope!(), r#"0 in [1..5]"#, false);
}

#[test]
fn _0005() {
  te_bool(false, &scope!(), r#"6 in [1..5]"#, false);
}

#[test]
fn _0006() {
  te_bool(false, &scope!(), r#"1 in [1..5]"#, true);
}

#[test]
fn _0007() {
  te_bool(false, &scope!(), r#"5 in [1..5]"#, true);
}

#[test]
fn _0008() {
  te_bool(false, &scope!(), r#"1 in (1..5]"#, false);
}

#[test]
fn _0009() {
  te_bool(false, &scope!(), r#"5 in [1..5)"#, false);
}

#[test]
fn _0010() {
  te_bool(false, &scope!(), r#"1 in (1..5)"#, false);
}

#[test]
fn _0011() {
  te_bool(false, &scope!(), r#"1.01 in (1..5)"#, true);
}

#[test]
fn _0012() {
  te_bool(false, &scope!(), r#"5 in (1..5)"#, false);
}

#[test]
fn _0013() {
  te_bool(false, &scope!(), r#"4.99 in (1..5)"#, true);
}

#[test]
fn _0014() {
  te_bool(false, &scope!(), r#"1 in ]1..5]"#, false);
}

#[test]
fn _0015() {
  te_bool(false, &scope!(), r#"5 in [1..5["#, false);
}

#[test]
fn _0016() {
  te_bool(false, &scope!(), r#"1 in ]1..5["#, false);
}

#[test]
fn _0017() {
  te_bool(false, &scope!(), r#"5 in ]1..5["#, false);
}

#[test]
fn _0018() {
  te_bool(false, &scope!(), r#"1 in [1..5] and 5 in [1..5]"#, true);
}

#[test]
fn _0019() {
  te_bool(false, &scope!(), r#"1 in [1..5] or 6 in [1..5]"#, true);
}

#[test]
fn _0020() {
  te_bool(false, &scope!(), r#" @"P3D" in [@"P1D"..@"P5D"]"#, true);
}

#[test]
fn _0021() {
  te_bool(false, &scope!(), r#" @"P5D" in [@"P1D"..@"P5D")"#, false);
}

#[test]
fn _0022() {
  te_null(false, &scope!(), r#" "k" in [1.."z"]"#, "eval_in_range");
}

#[test]
fn _0023() {
  te_null(false, &scope!(), r#" "k" in ["a"..34]"#, "eval_in_range");
}

#[test]
fn _0024() {
  te_null(false, &scope!(), r#" @"2023-02-06" in ["2023-02-01"..@"2023-02-28"]"#, "eval_in_range");
}

#[test]
fn _0025() {
  te_null(false, &scope!(), r#" @"2023-02-06" in [@"2023-02-01".."2023-02-28"]"#, "eval_in_range");
}

#[test]
fn _0026() {
  te_null(
    false,
    &scope!(),
    r#" @"2023-02-06T10:11:12" in ["2023-02-01T00:00:00"..@"2023-02-28T23:59:59"]"#,
    "eval_in_range",
  );
}

#[test]
fn _0027() {
  te_null(
    false,
    &scope!(),
    r#" @"2023-02-06T10:11:12" in [@"2023-02-01T00:00:00".."2023-02-28T23:59:59"]"#,
    "eval_in_range",
  );
}

#[test]
fn _0028() {
  te_null(false, &scope!(), r#" @"10:11:12" in ["00:00:00"..@"23:59:59"]"#, "eval_in_range");
}

#[test]
fn _0029() {
  te_null(false, &scope!(), r#" @"10:11:12" in [@"00:00:00".."23:59:59"]"#, "eval_in_range");
}

#[test]
fn _0030() {
  te_null(false, &scope!(), r#" @"P5D" in [1..@"P5D"]"#, "eval_in_range");
}

#[test]
fn _0031() {
  te_null(false, &scope!(), r#" @"P5D" in [@"P1D"..5]"#, "eval_in_range");
}

#[test]
fn _0032() {
  te_null(false, &scope!(), r#" @"P5Y" in [1..@"P5Y"]"#, "eval_in_range");
}

#[test]
fn _0033() {
  te_null(false, &scope!(), r#" @"P5Y" in [@"P1Y"..5]"#, "eval_in_range");
}

#[test]
fn _0034() {
  te_null(false, &scope!(), r#" true in [false..true]"#, "eval_in_range");
}

#[test]
fn _0035() {
  let node = AstNode::Range(
    Box::new(AstNode::IntervalStart(
      Box::new(AstNode::Numeric("1".into(), "0".into(), '+', "".into())),
      IntervalType::Opened,
    )),
    Box::new(AstNode::IntervalEnd(
      Box::new(AstNode::Numeric("5".into(), "0".into(), '+', "".into())),
      IntervalType::Opened,
    )),
  );
  assert_eq!(r#"(1.0..5.0)"#, crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0036() {
  let node = AstNode::Range(
    Box::new(AstNode::IntervalEnd(
      Box::new(AstNode::Numeric("5".into(), "0".into(), '+', "".into())),
      IntervalType::Opened,
    )),
    Box::new(AstNode::IntervalStart(
      Box::new(AstNode::Numeric("1".into(), "0".into(), '+', "".into())),
      IntervalType::Opened,
    )),
  );
  assert_eq!(r#"null(expected interval start)"#, crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0037() {
  let node = AstNode::Range(
    Box::new(AstNode::IntervalStart(
      Box::new(AstNode::Numeric("1".into(), "0".into(), '+', "".into())),
      IntervalType::Opened,
    )),
    Box::new(AstNode::IntervalStart(
      Box::new(AstNode::Numeric("1".into(), "0".into(), '+', "".into())),
      IntervalType::Opened,
    )),
  );
  assert_eq!(r#"null(expected interval end)"#, crate::evaluate(&scope!(), &node).to_string());
}
