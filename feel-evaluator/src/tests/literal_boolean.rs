use super::*;

#[test]
fn test_0001() {
    let scope = &te_scope(r#"{}"#);
    te_value(false, scope, r#"true"#, r#"true"#);
}

#[test]
fn test_0002() {
    let scope = &te_scope(r#"{}"#);
    te_value(false, scope, r#"false"#, r#"false"#);
}
