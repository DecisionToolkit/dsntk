use dsntk_feel::{FeelScope, scope};

#[test]
fn _0001() {
  let node = dsntk_feel_parser::parse_simple_expressions(&scope!(), "1 + 2, 3 - 4, 5 * 6, 7 / 8, 9 ** 2, -23", false).unwrap();
  let expected = r#"
       ExpressionList
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `1`
       │  └─ Numeric
       │     └─ `2`
       ├─ Sub
       │  ├─ Numeric
       │  │  └─ `3`
       │  └─ Numeric
       │     └─ `4`
       ├─ Mul
       │  ├─ Numeric
       │  │  └─ `5`
       │  └─ Numeric
       │     └─ `6`
       ├─ Div
       │  ├─ Numeric
       │  │  └─ `7`
       │  └─ Numeric
       │     └─ `8`
       ├─ Exp
       │  ├─ Numeric
       │  │  └─ `9`
       │  └─ Numeric
       │     └─ `2`
       └─ Neg
          └─ Numeric
             └─ `23`
    "#;
  assert_eq!(expected, node.trace());
}
