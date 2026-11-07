use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    let node = AstNode::Out(
        Box::new(AstNode::Numeric(
            "21".to_string(),
            "".to_string(),
            '+',
            "".into(),
        )),
        Box::new(AstNode::Numeric(
            "21".to_string(),
            "".to_string(),
            '+',
            "".into(),
        )),
    );
    assert_eq!(r#"21"#, crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0002() {
    let node = AstNode::Out(
        Box::new(AstNode::Numeric(
            "21".to_string(),
            "".to_string(),
            '+',
            "".into(),
        )),
        Box::new(AstNode::FormalParameters(vec![])),
    );
    assert_eq!(r#"null"#, crate::evaluate(&scope!(), &node).to_string());
}
