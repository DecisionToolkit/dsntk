mod common;

use dsntk_feel_number::FeelNumber;

#[test]
fn test_from_str_001() {
    eqs!("12345.6789", "12345.6789".parse::<FeelNumber>().unwrap());
}

#[test]
fn test_from_str_002() {
    assert!("1234a5".parse::<FeelNumber>().is_err());
}

#[test]
fn test_from_str_003() {
    eqs!(
        "<FeelNumberError> invalid number literal '1234a5'",
        "1234a5".parse::<FeelNumber>().unwrap_err().to_string()
    );
}

#[test]
fn test_from_str_004() {
    eqs!("12300", "1.23E+4".parse::<FeelNumber>().unwrap());
}

#[test]
fn test_from_str_005() {
    eqs!(
        "0.00000000000000000000001",
        "1E-23".parse::<FeelNumber>().unwrap()
    );
}

#[test]
fn test_from_str_006() {
    eqs!(
        "0.00000000000000001234567",
        "1.234567E-17".parse::<FeelNumber>().unwrap()
    );
}
