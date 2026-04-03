use super::*;
use dsntk_feel::{FeelType, scope};

#[test]
fn _0001() {
  let node = AstNode::FormalParameter(Box::new(AstNode::ParameterName("n".into())), Box::new(AstNode::FeelType(FeelType::Boolean)));
  assert_eq!("FormalParameter", crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0002() {
  let node = AstNode::FormalParameter(Box::new(AstNode::Name("n".into())), Box::new(AstNode::FeelType(FeelType::Boolean)));
  assert_eq!("null(expected name of the formal parameter)", crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0003() {
  let parameter_x = AstNode::FormalParameter(Box::new(AstNode::ParameterName("x".into())), Box::new(AstNode::FeelType(FeelType::Number)));
  let parameter_y = AstNode::FormalParameter(Box::new(AstNode::ParameterName("y".into())), Box::new(AstNode::FeelType(FeelType::Number)));
  let node = AstNode::FormalParameters(vec![parameter_x, parameter_y]);
  assert_eq!("FormalParameters", crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0004() {
  let parameter_x = AstNode::FormalParameter(Box::new(AstNode::ParameterName("x".into())), Box::new(AstNode::FeelType(FeelType::Number)));
  let parameter_y = AstNode::FormalParameter(Box::new(AstNode::ParameterName("y".into())), Box::new(AstNode::FeelType(FeelType::Number)));
  let parameter_invalid = AstNode::Numeric("1".into(), "0".into(), '+', "".into());
  let node = AstNode::FormalParameters(vec![parameter_x, parameter_y, parameter_invalid]);
  assert_eq!(
    "null(expected formal parameter, actual value type is: number)",
    crate::evaluate(&scope!(), &node).to_string()
  );
}
