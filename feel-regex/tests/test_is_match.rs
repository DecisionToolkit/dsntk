use dsntk_feel_regex::is_match;

#[test]
fn _0001() {
    assert!(is_match(r#"Mr\. Obama"#, ".+Obama").unwrap());
}

#[test]
fn _0002() {
    assert!(is_match(r#"banana"#, "[a-z]{3}").unwrap());
}
