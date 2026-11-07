mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_exp_001() {
    eqs!(
        "2.718281828459045235360287471352662",
        num!(1).exp().unwrap()
    );
}

#[test]
fn test_exp_002() {
    eqs!(
        "54.59815003314423907811026120286088",
        num!(4).exp().unwrap()
    );
}
