use dsntk_feel_number::FeelNumber;

#[test]
fn test_is_one_001() {
    assert!(FeelNumber::new(1, 0).is_one());
}

#[test]
fn test_is_one_002() {
    assert!(FeelNumber::one().is_one());
}

#[test]
fn test_is_one_003() {
    assert!(!FeelNumber::new(11, 1).is_one());
}
