use super::*;

#[test]
fn test_0001() {
    let scope = &te_scope("{}");
    te_number(
        false,
        scope,
        r#"  1  // eol comment
         + 1"#,
        2,
        0,
    );
}

#[test]
fn test_0002() {
    let scope = &te_scope("{}");
    te_number(
        false,
        scope,
        r#" 1
          /*
          some intro waffle
          */
          + 1"#,
        2,
        0,
    );
}

#[test]
fn test_0003() {
    let scope = &te_scope("{}");
    te_number(false, scope, r#"1 + /* 1 + */ 1"#, 2, 0);
}

#[test]
fn test_0004() {
    let scope = &te_scope("{}");
    te_number(
        false,
        scope,
        r#" 1
          /*
          some intro waffle
          */
          + 1 // and stuff
          + 2"#,
        4,
        0,
    );
}
