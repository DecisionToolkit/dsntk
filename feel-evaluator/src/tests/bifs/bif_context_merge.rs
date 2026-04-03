use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_value(false, &scope!(), r#"context merge([{x:1}, {y:2}])"#, "{x:1, y:2}");
}

#[test]
fn _0002() {
  te_value(false, &scope!(), r#"context merge(contexts: [{x:1}, {y:2}])"#, "{x:1, y:2}");
}

#[test]
fn _0003() {
  te_null(false, &scope!(), r#"context merge(context: [{x:1}, {y:2}])"#, "parameter 'contexts' not found");
}

#[test]
fn _0004() {
  te_null(
    false,
    &scope!(),
    r#"context merge([1,{}])"#,
    "[core::context merge] invalid argument type, expected context, actual type is number",
  );
}

#[test]
fn _0005() {
  te_null(
    false,
    &scope!(),
    r#"context merge(1)"#,
    "[core::context merge] invalid argument type, expected list<context>, actual type is number",
  );
}
