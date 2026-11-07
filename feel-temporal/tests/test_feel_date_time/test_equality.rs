use dsntk_feel_temporal::FeelDateTime;

#[test]
fn _0001() {
    let a: FeelDateTime = "2025-02-09T00:00:00Z".try_into().unwrap();
    let b: FeelDateTime = "2025-02-09T00:00:00Z".try_into().unwrap();
    assert_eq!(a, b);
}

#[test]
fn _0002() {
    let a: FeelDateTime = "2025-02-09T00:00:00Z".try_into().unwrap();
    let b: FeelDateTime = "2025-02-09T00:00:00@Etc/UTC".try_into().unwrap();
    assert_eq!(a, b);
}

#[test]
fn _0003() {
    let a: FeelDateTime = "2025-02-09T00:00:00@Etc/UTC".try_into().unwrap();
    let b: FeelDateTime = "2025-02-09T00:00:00Z".try_into().unwrap();
    assert_eq!(a, b);
}

#[test]
fn _0004() {
    let a: FeelDateTime = "2025-02-09T00:00:00Z".try_into().unwrap();
    let b: FeelDateTime = "2025-02-09T00:00:00+00:00".try_into().unwrap();
    assert_eq!(a, b);
}

#[test]
fn _0005() {
    let a: FeelDateTime = "2025-02-09T00:00:00Z".try_into().unwrap();
    let b: FeelDateTime = "2025-02-09T00:00:00+01:00".try_into().unwrap();
    assert_ne!(a, b);
}

#[test]
fn _0006() {
    let a: FeelDateTime = "2025-02-09T00:00:00+01:00".try_into().unwrap();
    let b: FeelDateTime = "2025-02-09T00:00:00Z".try_into().unwrap();
    assert_ne!(a, b);
}

#[test]
fn _0007() {
    let a: FeelDateTime = "2025-02-09T00:00:00".try_into().unwrap();
    let b: FeelDateTime = "2025-02-09T00:00:00".try_into().unwrap();
    assert_eq!(a, b);
}

#[test]
fn _0008() {
    let a: FeelDateTime = "2025-02-09T00:00:00+01:00".try_into().unwrap();
    let b: FeelDateTime = "2025-02-09T00:00:00+01:00".try_into().unwrap();
    assert_eq!(a, b);
}

#[test]
fn _0009() {
    let a: FeelDateTime = "2018-10-08T00:00:00+02:00".try_into().unwrap();
    let b: FeelDateTime = "2018-10-08T00:00:00@Europe/Paris".try_into().unwrap();
    assert_eq!(a, b);
}

#[test]
fn _0010() {
    let a: FeelDateTime = "2018-10-08T00:00:00@Europe/Paris".try_into().unwrap();
    let b: FeelDateTime = "2018-10-08T00:00:00+02:00".try_into().unwrap();
    assert_eq!(a, b);
}

#[test]
fn _0011() {
    let a: FeelDateTime = "2025-02-09T00:00:00+01:00".try_into().unwrap();
    let b: FeelDateTime = "2025-02-09T00:00:00@Europe/Paris".try_into().unwrap();
    assert_eq!(a, b);
}

#[test]
fn _0012() {
    let a: FeelDateTime = "2025-02-09T00:00:00@Europe/Paris".try_into().unwrap();
    let b: FeelDateTime = "2025-02-09T00:00:00+01:00".try_into().unwrap();
    assert_eq!(a, b);
}

#[test]
fn _0013() {
    let a: FeelDateTime = "2021-04-02T23:00:00@Australia/Melbourne"
        .try_into()
        .unwrap();
    let b: FeelDateTime = "2021-04-02T23:00:00+11:00".try_into().unwrap();
    assert_eq!(a, b);
}

#[test]
fn _0014() {
    let a: FeelDateTime = "2021-10-02T23:00:00@Australia/Melbourne"
        .try_into()
        .unwrap();
    let b: FeelDateTime = "2021-10-02T23:00:00+10:00".try_into().unwrap();
    assert_eq!(a, b);
}
