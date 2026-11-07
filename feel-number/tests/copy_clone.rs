mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_copy_001() {
    let x = FeelNumber::new(12345, 2);
    let y = x;
    eqs!("123.45", y);
}

#[test]
#[allow(clippy::clone_on_copy)]
fn test_clone_001() {
    let x = FeelNumber::new(12345, 2);
    let y = x.clone();
    eqs!("123.45", y);
}
