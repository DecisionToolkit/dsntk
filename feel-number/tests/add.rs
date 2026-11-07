mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_add_001() {
    eqs!("0.4", num!(0.1) + num!(0.3));
}

#[test]
fn test_add_002() {
    eqs!(
        "123.8347847",
        FeelNumber::new(12345, 2) + FeelNumber::new(3847847, 7)
    );
}

#[test]
fn test_add_003() {
    eqs!("8.93", FeelNumber::new(123, 2) + FeelNumber::new(77, 1));
}

#[test]
fn test_add_004() {
    let mut x = FeelNumber::new(1, 1);
    x += FeelNumber::new(3, 1);
    eqs!("0.4", x);
}

#[test]
fn test_add_005() {
    let mut x = FeelNumber::new(12345, 2);
    x += FeelNumber::new(3847847, 7);
    eqs!("123.8347847", x);
}

#[test]
fn test_add_006() {
    let mut x = FeelNumber::new(123, 2);
    x += FeelNumber::new(77, 1);
    eqs!("8.93", x);
}
