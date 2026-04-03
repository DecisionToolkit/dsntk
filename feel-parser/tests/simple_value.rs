use dsntk_feel::{FeelScope, scope};

#[test]
fn _0001() {
  let node = dsntk_feel_parser::parse_simple_value(&scope!(), "1", false).unwrap();
  let expected = r#"
       Numeric
       └─ `1`
    "#;
  assert_eq!(expected, node.trace());
}
