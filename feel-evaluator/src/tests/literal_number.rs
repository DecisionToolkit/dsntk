use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    let node = AstNode::Numeric("2".into(), "56".into(), '+', "".into());
    assert_eq!(r#"2.56"#, crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0002() {
    let node = AstNode::Numeric("2".into(), "56".into(), '+', "2".into());
    assert_eq!(r#"256"#, crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0003() {
    let node = AstNode::Numeric("256".into(), "7".into(), '-', "2".into());
    assert_eq!(r#"2.567"#, crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0004() {
    let node = AstNode::Numeric("256".into(), "".into(), '+', "1".into());
    assert_eq!(r#"2560"#, crate::evaluate(&scope!(), &node).to_string());
}

#[test]
fn _0005() {
    let node = AstNode::Numeric("2".into(), "5a6".into(), '+', "".into());
    assert_eq!(
        r#"null(failed to convert text '2.5a6' into number)"#,
        crate::evaluate(&scope!(), &node).to_string()
    );
}
