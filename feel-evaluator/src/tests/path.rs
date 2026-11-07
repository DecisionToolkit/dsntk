use super::*;

#[test]
fn _0001() {
    let node = AstNode::Path(
        Box::new(AstNode::Name("a".into())),
        Box::new(AstNode::Name("b".into())),
    );
    let scope = te_scope(r#"{ a: { b: 10 } }"#);
    assert_eq!(r#"10"#, crate::evaluate(&scope, &node).to_string());
}

#[test]
fn _0002() {
    let node = AstNode::Path(
        Box::new(AstNode::Name("a".into())),
        Box::new(AstNode::Name("b".into())),
    );
    let scope = te_scope(r#"{ a: { c: 10 } }"#);
    assert_eq!(
        r#"null(build_path: no entry b in context: {c: 10})"#,
        crate::evaluate(&scope, &node).to_string()
    );
}

#[test]
fn _0003() {
    let node = AstNode::Path(
        Box::new(AstNode::Name("a".into())),
        Box::new(AstNode::Name("b".into())),
    );
    let scope = te_scope(r#"{ a: [{ b: 10 }, { b: 11 }, { b: 12 }] }"#);
    assert_eq!(
        r#"[10, 11, 12]"#,
        crate::evaluate(&scope, &node).to_string()
    );
}

#[test]
fn _0004() {
    let node = AstNode::Path(
        Box::new(AstNode::Name("a".into())),
        Box::new(AstNode::Name("b".into())),
    );
    let scope = te_scope(r#"{ a: [{ b: 10 }, { b: 11 }, 12 ] }"#);
    assert_eq!(
        "[10, 11, null(unexpected type: number, for property: b)]",
        crate::evaluate(&scope, &node).to_string()
    );
}

#[test]
fn _0005() {
    let node = AstNode::Path(
        Box::new(AstNode::Path(
            Box::new(AstNode::Name("a".into())),
            Box::new(AstNode::Name("b".into())),
        )),
        Box::new(AstNode::Name("days".into())),
    );
    let scope = te_scope(r#"{ a: [{ b: @"P1DT5H" }, { b: @"P2DT6H" }, { b: @"P3DT7H" }] }"#);
    assert_eq!(r#"[1, 2, 3]"#, crate::evaluate(&scope, &node).to_string());
}

#[test]
fn _0006() {
    let node = AstNode::Path(
        Box::new(AstNode::Name("a".into())),
        Box::new(AstNode::Path(
            Box::new(AstNode::Boolean(false)),
            Box::new(AstNode::Name("days".into())),
        )),
    );
    let scope = te_scope(r#"{ a: [{ b: @"P1DT5H" }, { b: @"P2DT6H" }, { b: @"P3DT7H" }] }"#);
    assert_eq!(
        r#"null(expected name node, actual node is: Path(Boolean(false), Name(Name("days"))))"#,
        crate::evaluate(&scope, &node).to_string()
    );
}

#[test]
fn _0007() {
    let node = AstNode::Path(
        Box::new(AstNode::Name("a".into())),
        Box::new(AstNode::Boolean(false)),
    );
    let scope = te_scope(r#"{ a: [{ b: @"P1DT5H" }, { b: @"P2DT6H" }, { b: @"P3DT7H" }] }"#);
    assert_eq!(
        "null(expected name node, actual node is: Boolean(false))",
        crate::evaluate(&scope, &node).to_string()
    );
}
