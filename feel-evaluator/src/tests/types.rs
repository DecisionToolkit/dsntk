use dsntk_feel::{FeelScope, FeelType, scope};
use dsntk_feel_parser::AstNode;

#[test]
fn _0001() {
  let node = AstNode::FunctionType(
    Box::new(AstNode::ParameterTypes(vec![AstNode::FeelType(FeelType::Number), AstNode::FeelType(FeelType::String)])),
    Box::new(AstNode::FeelType(FeelType::Boolean)),
  );
  assert_eq!(r#"type(function<number, string>->boolean)"#, crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0002() {
  let node = AstNode::FunctionType(
    Box::new(AstNode::ParameterTypes(vec![AstNode::FeelType(FeelType::Number), AstNode::FeelType(FeelType::String)])),
    Box::new(AstNode::Numeric("0".into(), "0".into(), '+', "".into())),
  );
  assert_eq!(r#"null(expected function's result type)"#, crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0003() {
  let node = AstNode::FunctionType(
    Box::new(AstNode::Numeric("0".into(), "0".into(), '+', "".into())),
    Box::new(AstNode::FeelType(FeelType::Boolean)),
  );
  assert_eq!(r#"null(expected function's parameter types)"#, crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0004() {
  let node = AstNode::ListType(Box::new(AstNode::FeelType(FeelType::Boolean)));
  assert_eq!(r#"type(list<boolean>)"#, crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0005() {
  let node = AstNode::ListType(Box::new(AstNode::Numeric("0".into(), "0".into(), '+', "".into())));
  assert_eq!(r#"null(expected a feel type)"#, crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0006() {
  let node = AstNode::RangeType(Box::new(AstNode::FeelType(FeelType::Number)));
  assert_eq!(r#"type(range<number>)"#, crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0007() {
  let node = AstNode::RangeType(Box::new(AstNode::Numeric("0".into(), "0".into(), '+', "".into())));
  assert_eq!(r#"null(expected a feel type)"#, crate::evaluate(&scope!(), &node).to_string());
}
