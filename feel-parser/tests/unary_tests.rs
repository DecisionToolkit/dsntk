use dsntk_feel::{FeelScope, scope};

#[test]
fn _0001() {
  let node = dsntk_feel_parser::parse_unary_tests(&scope!(), "<1, <=2, >3, >=4, =5, !=6", false).unwrap();
  let expected = r#"
       ExpressionList
       ├─ UnaryLt
       │  └─ Numeric
       │     └─ `1`
       ├─ UnaryLe
       │  └─ Numeric
       │     └─ `2`
       ├─ UnaryGt
       │  └─ Numeric
       │     └─ `3`
       ├─ UnaryGe
       │  └─ Numeric
       │     └─ `4`
       ├─ UnaryEq
       │  └─ Numeric
       │     └─ `5`
       └─ UnaryNe
          └─ Numeric
             └─ `6`
    "#;
  assert_eq!(expected, node.trace());
}
