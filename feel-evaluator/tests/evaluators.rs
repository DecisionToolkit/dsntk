use dsntk_feel::values::Value;
use dsntk_feel::{FeelNumber, FeelScope, scope};
use dsntk_feel_evaluator::*;
use dsntk_feel_parser::AstNode;

#[test]
fn _0001() {
  assert_eq!("{}", evaluate_context(&scope!(), r#"{}"#).unwrap().to_string());
}

#[test]
fn _0002() {
  let node = AstNode::Context(vec![AstNode::ContextEntry(
    Box::new(AstNode::ContextEntryKey("alpha".into())),
    Box::new(AstNode::Boolean(true)),
  )]);
  assert_eq!("{alpha: true}", evaluate_context_node(&scope!(), &node).unwrap().to_string());
}

#[test]
fn _0003() {
  let node = AstNode::Boolean(true);
  assert_eq!(
    "<FeelEvaluatorError> expected FEEL context on input",
    evaluate_context_node(&scope!(), &node).err().unwrap().to_string()
  );
}

#[test]
fn _0004() {
  let node = AstNode::Add(
    Box::new(AstNode::Numeric("1".to_string(), "23".to_string(), '+', "".into())),
    Box::new(AstNode::Numeric("1".to_string(), "77".to_string(), '+', "".into())),
  );
  let evaluator = prepare(&node);
  assert_eq!("3", evaluator(&scope!()).to_string());
}

#[test]
fn _0005() {
  let value = evaluate_sum(vec![Value::Number(FeelNumber::new(123, 2)), Value::Number(FeelNumber::new(177, 2))]);
  assert_eq!("3", value.to_string());
}

#[test]
fn _0006() {
  let value = evaluate_min(vec![Value::Number(FeelNumber::new(123, 2)), Value::Number(FeelNumber::new(177, 2))]);
  assert_eq!("1.23", value.to_string());
}

#[test]
fn _0007() {
  let value = evaluate_max(vec![Value::Number(FeelNumber::new(123, 2)), Value::Number(FeelNumber::new(177, 2))]);
  assert_eq!("1.77", value.to_string());
}

#[test]
fn _0008() {
  let node = AstNode::FunctionInvocation(Box::new(AstNode::Name("calculate".into())), Box::new(AstNode::Boolean(true)));
  assert_eq!(
    "null(expected positional or named parameter, actual AST node is Boolean(true))",
    evaluate(&scope!(), &node).to_string()
  );
}

#[test]
fn _0009() {
  let node = AstNode::FunctionInvocation(
    Box::new(AstNode::Name("calculate".into())),
    Box::new(AstNode::NamedParameters(vec![AstNode::NamedParameter(
      Box::new(AstNode::Boolean(true)),
      Box::new(AstNode::Boolean(false)),
    )])),
  );
  assert_eq!(
    "null(expected built-in function name or function definition, actual is null(context has no value for key 'calculate'))",
    evaluate(&scope!(), &node).to_string()
  );
}

#[test]
fn _0010() {
  let node = AstNode::Every(Box::new(AstNode::Boolean(true)), Box::new(AstNode::Boolean(false)));
  assert_eq!(
    "null(expected AST node QuantifiedContexts, actual AST node is Boolean(true))",
    evaluate(&scope!(), &node).to_string()
  );
}

#[test]
fn _0011() {
  let node = AstNode::Some(Box::new(AstNode::Boolean(true)), Box::new(AstNode::Boolean(false)));
  assert_eq!(
    r#"null(expected AST node QuantifiedContexts, actual AST node is Boolean(true))"#,
    evaluate(&scope!(), &node).to_string()
  );
}

#[test]
fn _0012() {
  let node = AstNode::CommaList(vec![]);
  assert_eq!("null(unexpected AST node: CommaList([]))", evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0013() {
  assert_eq!(
    "<ParserError> syntax error: [false..true]",
    evaluate_range_literal(&scope!(), "[false..true]").err().unwrap().to_string()
  );
}

#[test]
fn _0014() {
  let node = AstNode::Boolean(true);
  assert_eq!(
    "<FeelEvaluatorError> expected FEEL range on input",
    evaluate_range_literal_node(&scope!(), &node).err().unwrap().to_string()
  );
}
