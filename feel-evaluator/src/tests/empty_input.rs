use super::*;

#[test]
fn test_empty_input() {
    let scope = &te_scope("{}");
    te_none(false, scope, "");
    te_none(false, scope, "    ");
    te_none(false, scope, "\n");
    te_none(false, scope, "\u{000A}\u{000B}\u{000C}\u{000D}");
}
