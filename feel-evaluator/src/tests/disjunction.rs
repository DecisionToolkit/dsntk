use super::*;

#[test]
fn test_0001() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "true or true", true);
}

#[test]
fn test_0002() {
    let scope = &te_scope("{}");
    super::te_bool(false, scope, "true or not(false)", true);
}

#[test]
fn test_0003() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "true or true or true", true);
}

#[test]
fn test_0004() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "true or true or false", true);
}

#[test]
fn test_0005() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "true or false", true);
}

#[test]
fn test_0006() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "true or 1", true);
}

#[test]
fn test_0007() {
    let scope = &te_scope("{}");
    te_bool(false, scope, r#" true or "true" "#, true);
}

#[test]
fn test_0008() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "true or null", true);
}

#[test]
fn test_0009() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "false or true", true);
}

#[test]
fn test_0010() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "false or false", false);
}

#[test]
fn test_0011() {
    let scope = &te_scope("{}");
    te_null(false, scope, "false or 1", "");
}

#[test]
fn test_0012() {
    let scope = &te_scope("{}");
    te_null(false, scope, "false or null", "");
}

#[test]
fn test_0013() {
    let scope = &te_scope("{}");
    te_null(false, scope, r#" false or "true" "#, "");
}

#[test]
fn test_0014() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "1 or true", true);
}

#[test]
fn test_0015() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "null or true", true);
}

#[test]
fn test_0016() {
    let scope = &te_scope("{}");
    te_null(false, scope, "1 or false", "");
}

#[test]
fn test_0017() {
    let scope = &te_scope("{}");
    te_null(false, scope, "null or false", "");
}

#[test]
fn test_0018() {
    let scope = &te_scope("{}");
    te_null(false, scope, "1 or 1", "");
}

#[test]
fn test_0019() {
    let scope = &te_scope("{}");
    te_null(false, scope, "null or null", "");
}

#[test]
fn test_0020() {
    let scope = &te_scope("{}");
    te_null(false, scope, "10.2 or 10.2", "");
}

#[test]
fn test_0021() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "(1 < 2) or (3 > 1)", true);
}

#[test]
fn test_0022() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "((1 < 2) or (3 > 1)) and false", false);
}

#[test]
fn test_0023() {
    let scope = &te_scope("{}");
    te_bool(false, scope, "((1 < 2) or (3 > 1)) or false", true);
}
