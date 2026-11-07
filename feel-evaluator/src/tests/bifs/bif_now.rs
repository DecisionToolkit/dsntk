use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_date_time_local_after(false, &scope!(), "now()", FeelDateTime::now(), 1);
}

#[test]
fn _0002() {
    te_null(
        false,
        &scope!(),
        r#"now(when: 1)"#,
        r#"[named::now] this function has no implementation with named parameters"#,
    );
}
