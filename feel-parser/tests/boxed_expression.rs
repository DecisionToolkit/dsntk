use dsntk_feel::{FeelScope, scope};

#[test]
fn _0001() {
  let node = dsntk_feel_parser::parse_boxed_expression(&scope!(), "[1,2,3]", false).unwrap();
  let expected = r#"
       List
       ├─ Numeric
       │  └─ `1`
       ├─ Numeric
       │  └─ `2`
       └─ Numeric
          └─ `3`
    "#;
  assert_eq!(expected, node.trace());
}
