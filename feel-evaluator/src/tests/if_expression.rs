use super::*;

#[test]
fn _0001() {
    let scope = &te_scope(r#"{N:9}"#);
    te_number(false, scope, r#"if N < 10 then 1 else 2"#, 1, 0);
}

#[test]
fn _0002() {
    let scope = &te_scope(r#"{N:10}"#);
    te_number(false, scope, r#"if N < 10 then 1 else 2"#, 2, 0);
}

#[test]
fn _0003() {
    let scope = &te_scope(r#"{aDate: @"2017-01-02", aString: "Hello World"}"#);
    te_string(
        false,
        scope,
        r#"if aDate > date("2017-01-01") then substring before(aString, " ") else substring after(aString, " ")"#,
        r#"Hello"#,
    );
}

#[test]
fn _0004() {
    let scope = &te_scope(r#"{aDate: @"2017-01-01", aString: "Hello World"}"#);
    te_string(
        false,
        scope,
        r#"if aDate > date("2017-01-01") then substring before(aString, " ") else substring after(aString, " ")"#,
        r#"World"#,
    );
}

#[test]
fn _0005() {
    let scope = &te_scope(r#"{N:9}"#);
    te_null(
        false,
        scope,
        r#"if N then 1 else 2"#,
        r#"condition in 'if' expression is not a boolean value"#,
    );
}
