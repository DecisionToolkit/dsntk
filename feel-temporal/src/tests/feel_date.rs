use crate::FeelDate;

#[test]
fn _0001() {
    let date_a = &FeelDate::new(2023, 2, 8).unwrap();
    let date_b = &FeelDate::new(2021, 2, 8).unwrap();
    let result = date_a - date_b;
    assert_eq!("P2Y", result.to_string());
}

#[test]
fn _0002() {
    let date_a = &FeelDate::new(2021, 2, 8).unwrap();
    let date_b = &FeelDate::new(2023, 2, 8).unwrap();
    let result = date_a - date_b;
    assert_eq!("-P2Y", result.to_string());
}

#[test]
fn _0003() {
    let date_a = &FeelDate::new(2021, 2, 9).unwrap();
    let date_b = &FeelDate::new(2023, 3, 8).unwrap();
    let result = date_a - date_b;
    assert_eq!("-P2Y", result.to_string());
}
