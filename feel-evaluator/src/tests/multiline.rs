use super::*;

#[test]
fn test_multiline() {
    let scope = &te_scope("{}");
    te_number(
        false,
        scope,
        r#"1 + 
    2
  + 3"#,
        6,
        0,
    );
}
