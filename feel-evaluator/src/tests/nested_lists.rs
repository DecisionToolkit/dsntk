use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    let input = r#"
    [{a: {b: 1}}, {a: {b: [2.1, 2.2]}}, {a: {b: 3}}, {a: {b: 4}}, {a: {b: 5}}].a.b
     = 
    [{b: 1}, {b: [2.1, 2.2]}, {b: 3}, {b: 4}, {b: 5}].b
  "#;
    te_bool(false, &scope!(), input, true);
}

#[test]
fn _0002() {
    let input = r#"
    [{b: 1}, {b: [2.1, 2.2]}, {b: 3}, {b: 4}, {b: 5}].b
     =
    [1, [2.1, 2.2], 3, 4, 5]
  "#;
    te_bool(false, &scope!(), input, true);
}

#[test]
fn _0003() {
    let input = r#"
    [{a: {b: [1]}}, {a: {b: [2.1, 2.2]}}, {a: {b: [3]}}, {a: {b: [4, 5]}}].a.b
     =
    [{b: [1]}, {b: [2.1,2.2]}, {b: [3]}, {b: [4, 5]}].b
  "#;
    te_bool(false, &scope!(), input, true);
}

#[test]
fn _0004() {
    let input = r#"
    [{b: [1]}, {b: [2.1,2.2]}, {b: [3]}, {b: [4, 5]}].b
     =
    [[1], [2.1, 2.2], [3], [4, 5]]
  "#;
    te_bool(false, &scope!(), input, true);
}
