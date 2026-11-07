use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    let (year, month, day) = FeelDate::today().as_tuple();
    te_date(false, &scope!(), "today()", year, month, day);
}

#[test]
fn _0002() {
    te_null(
        false,
        &scope!(),
        r#"today(when: 1)"#,
        r#"[named::today] this function has no implementation with named parameters"#,
    );
}
