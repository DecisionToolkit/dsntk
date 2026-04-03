use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_value(false, &scope!(), r#"list replace([1, 7, 3], 2, 4)"#, "[1, 4, 3]");
}

#[test]
fn _0002() {
  te_value(false, &scope!(), r#"list replace(list: [1, 7, 3], position: 2, newItem: 4)"#, "[1, 4, 3]");
}

#[test]
fn _0003() {
  te_null(
    false,
    &scope!(),
    r#"list replace(null, 2, 4)"#,
    "[core::list replace] invalid argument type, expected list, actual type is Null",
  );
}

#[test]
fn _0004() {
  te_value(false, &scope!(), r#"list replace(2, 1, 4)"#, "[4]");
}

#[test]
fn _0005() {
  te_null(false, &scope!(), r#"list replace(2, 2, 4)"#, "position exceeds list bounds");
}

#[test]
fn _0006() {
  te_value(false, &scope!(), r#"list replace([1, 2, 3, 4, 5, 6], -2, 9)"#, "[1, 2, 3, 4, 9, 6]");
}

#[test]
fn _0007() {
  te_null(false, &scope!(), r#"list replace([1, 2, 3, 4, 5, 6], -7, 9)"#, "position exceeds list bounds");
}

#[test]
fn _0008() {
  te_null(false, &scope!(), r#"list replace(2, 0, 4)"#, "position must not be zero");
}

#[test]
fn _0009() {
  te_null(false, &scope!(), r#"list replace(2, 32e+144, 4)"#, "position exceeds integer range");
}

#[test]
fn _0010() {
  te_null(false, &scope!(), r#"list replace(2, "1", 4)"#, "position must be number or function");
}

#[test]
fn _0011() {
  te_value(false, &scope!(), r#"list replace([2, 4, 7, 8], function(item, newItem) item < newItem, 5)"#, "[5, 5, 7, 8]");
}

#[test]
fn _0012() {
  te_value(
    false,
    &scope!(),
    r#"list replace(list: [2, 4, 7, 8], match: function(item, newItem) item < newItem, newItem: 5)"#,
    "[5, 5, 7, 8]",
  );
}

#[test]
fn _0013() {
  te_null(
    false,
    &scope!(),
    r#"list replace([2, 4, 7, 8], function(item, newItem, param) item < newItem, 5)"#,
    "list replace: matching function must have exactly two arguments",
  );
}

#[test]
fn _0014() {
  te_null(
    false,
    &scope!(),
    r#"list replace([2, 4, 7, 8], function(item) item  = 5, 5)"#,
    "list replace: matching function must have exactly two arguments",
  );
}

#[test]
fn _0015() {
  te_null(
    false,
    &scope!(),
    r#"list replace([2, 4, 7, 8], function(item, newItem) 17, 5)"#,
    "list replace: matching function must return boolean value",
  );
}

#[test]
fn _0016() {
  te_null(false, &scope!(), r#"list replace(lists: [1, 7, 3], position: 2, newItem: 4)"#, "parameter 'list' not found");
}

#[test]
fn _0017() {
  te_null(
    false,
    &scope!(),
    r#"list replace(list: [1, 7, 3], positions: 2, newItem: 4)"#,
    "parameter 'position' or 'match' not found",
  );
}

#[test]
fn _0018() {
  te_null(
    false,
    &scope!(),
    r#"list replace(list: [1, 7, 3], position: 2, newItems: 4)"#,
    "parameter 'newItem' not found",
  );
}

#[test]
fn _0019() {
  te_null(
    false,
    &scope!(),
    r#"list replace(list: [2, 4, 7, 8], matches: function(item, newItem) item < newItem, newItem: 5)"#,
    "parameter 'position' or 'match' not found",
  );
}

#[test]
fn _0020() {
  te_null(
    false,
    &scope!(),
    r#"list replace(list: [2, 4, 7, 8], matche: function(item, newItem) item < newItem, newItem: 5, oldItem: 12)"#,
    "expected 3 parameters, actual number of parameters is 4",
  );
}
