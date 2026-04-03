use dsntk_feel::{FeelScope, scope};

#[test]
fn _0001() {
  let node = dsntk_feel_parser::parse_simple_expression(&scope!(), "1 + 2", false).unwrap();
  let expected = r#"
       Add
       ├─ Numeric
       │  └─ `1`
       └─ Numeric
          └─ `2`
    "#;
  assert_eq!(expected, node.trace());
}
