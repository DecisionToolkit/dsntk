use super::*;

#[test]
fn test_0001() {
    let scope = &te_scope(r#"{ Customer: "Private" }"#);
    te_none(false, scope, "()");
    te_number(false, scope, "(1)", 1, 0);
    te_bool(false, scope, "(true)", true);
    te_bool(false, scope, "(false)", false);
    te_string(false, scope, r#"("beta")"#, "beta");
    te_string(false, scope, "(Customer)", "Private");
}
