use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_string(false, &scope!(), r#"string join(["a","b","c"])"#, "abc");
}

#[test]
fn _0002() {
  te_string(false, &scope!(), r#"string join(["a","b","c"]," and ")"#, "a and b and c");
}

#[test]
fn _0003() {
  te_string(false, &scope!(), r#"string join(["a","b","c"], "")"#, "abc");
}

#[test]
fn _0004() {
  te_string(false, &scope!(), r#"string join(["a","b","c"], null)"#, "abc");
}

#[test]
fn _0005() {
  te_string(false, &scope!(), r#"string join(["a"])"#, "a");
}

#[test]
fn _0006() {
  te_string(false, &scope!(), r#"string join(["a"], "X")"#, "a");
}

#[test]
fn _0007() {
  te_string(false, &scope!(), r#"string join(["a",null,"c"])"#, "ac");
}

#[test]
fn _0008() {
  te_string(false, &scope!(), r#"string join(["a",null,"c"], "X")"#, "aXc");
}

#[test]
fn _0009() {
  te_string(false, &scope!(), r#"string join([])"#, "");
}

#[test]
fn _0010() {
  te_string(false, &scope!(), r#"string join([], "X")"#, "");
}

#[test]
fn _0011() {
  te_null(false, &scope!(), r#"string join()"#, "expected 1,2 parameters, actual number of parameters is 0");
}

#[test]
fn _0012() {
  te_null(
    false,
    &scope!(),
    r#"string join(["a","c"], "X", "foo")"#,
    "expected 1,2 parameters, actual number of parameters is 3",
  );
}

#[test]
fn _0013() {
  te_string(false, &scope!(), r#"string join(list: ["a","c"])"#, "ac");
}

#[test]
fn _0014() {
  te_string(false, &scope!(), r#"string join(delimiter: "X", list: ["a","c"])"#, "aXc");
}

#[test]
fn _0015() {
  te_null(false, &scope!(), r#"string join(list: ["a","c"], delimitr: "X")"#, "parameter 'delimiter' not found");
}

#[test]
fn _0016() {
  te_null(false, &scope!(), r#"string join(lst: ["a","c"], delimiter: "X")"#, "parameter 'list' not found");
}

#[test]
fn _0017() {
  te_null(false, &scope!(), r#"string join(null)"#, "string join: expected list or string, actual value type is Null");
}

#[test]
fn _0018() {
  te_null(
    false,
    &scope!(),
    r#"string join(null, "X")"#,
    "string join: expected list or string, actual value type is Null",
  );
}

#[test]
fn _0019() {
  te_null(
    false,
    &scope!(),
    r#"string join([1,2,3], "X")"#,
    "[core::string join] invalid argument type, expected string, actual type is number",
  );
}

#[test]
fn _0020() {
  te_null(
    false,
    &scope!(),
    r#"string join(123, "X")"#,
    "string join: expected list or string, actual value type is number",
  );
}

#[test]
fn _0021() {
  te_string(false, &scope!(), r#"string join("a", "X")"#, "a");
}

#[test]
fn _0022() {
  te_string(false, &scope!(), r#"string join(list: "a", delimiter: "X")"#, "a");
}

#[test]
fn _0023() {
  te_null(
    false,
    &scope!(),
    r#"string join(["a","b","c"],2)"#,
    "string join: invalid delimiter, expected string, actual value type is number",
  );
}

#[test]
fn _0024() {
  te_null(false, &scope!(), r#"string join(lst: ["a","c"])"#, "parameter 'list' not found");
}

#[test]
fn _0025() {
  te_null(
    false,
    &scope!(),
    r#"string join(list: ["a","c"], delimiter: "X", other: 1)"#,
    "expected 1,2 parameters, actual number of parameters is 3",
  );
}

#[test]
fn _0026() {
  te_string(false, &scope!(), r#"string join([null,"b"], "X")"#, "b");
}

#[test]
fn _0027() {
  te_string(false, &scope!(), r#"string join([null,"b"])"#, "b");
}

#[test]
fn _0028() {
  te_string(false, &scope!(), r#"string join([null,null,"c"], "X")"#, "c");
}

#[test]
fn _0029() {
  te_string(false, &scope!(), r#"string join([null,"a",null,"b"], "X")"#, "aXb");
}

#[test]
fn _0030() {
  te_string(false, &scope!(), r#"string join([null,null], "X")"#, "");
}

#[test]
fn _0031() {
  te_string(false, &scope!(), r#"string join(["a",null], "X")"#, "a");
}
