use super::*;

#[test]
fn test_0001() {
    let scope = &te_scope(r#"{}"#);
    te_value(false, scope, r#"@"2021-01-28""#, r#"date("2021-01-28")"#);
}

#[test]
fn test_0002() {
    let scope = &te_scope(r#"{}"#);
    te_value(
        false,
        scope,
        r#"@"2021-01-28T19:15:00""#,
        r#"date and time("2021-01-28T19:15:00")"#,
    );
}

#[test]
fn test_0003() {
    let scope = &te_scope(r#"{}"#);
    te_value(false, scope, r#"@"-PT1H""#, r#"duration("-PT1H")"#);
}

#[test]
fn test_0004() {
    let scope = &te_scope(r#"{}"#);
    te_value(false, scope, r#"@"P2Y""#, r#"duration("P2Y")"#);
}

#[test]
fn test_0005() {
    let scope = &te_scope(r#"{}"#);
    te_null(false, scope, r#"@"airbus""#, r#"invalid @ literal: airbus"#);
}
