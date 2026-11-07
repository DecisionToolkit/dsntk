use dsntk_common::HRef;

#[test]
fn _0001() {
    let href = HRef::try_from("#_id").unwrap();
    assert_eq!(None, href.namespace());
    assert_eq!("_id", href.id());
}

#[test]
fn _0002() {
    let href = HRef::try_from("documents#_id").unwrap();
    assert_eq!("documents", href.namespace().unwrap());
    assert_eq!("_id", href.id());
}

#[test]
fn _0003() {
    assert_eq!(
        r#"<HRefError> fragment is missing in reference: ''"#,
        HRef::try_from("").unwrap_err().to_string()
    );
}

#[test]
fn _0004() {
    assert_eq!(
        r#"<HRefError> fragment is missing in reference: 'documents'"#,
        HRef::try_from("documents").unwrap_err().to_string()
    );
}

#[test]
fn _0005() {
    assert_eq!(
        r#"<HRefError> query is not allowed in reference: 'documents?name=Introduction#_b51ac78b-fd76-42fc-a12d-aad7150c9278'"#,
        HRef::try_from("documents?name=Introduction#_b51ac78b-fd76-42fc-a12d-aad7150c9278")
            .unwrap_err()
            .to_string()
    );
}

#[test]
fn _0006() {
    assert_eq!(
        r#"<HRefError> invalid reference '                                               ', reason: invalid path character"#,
        HRef::try_from("                                               ")
            .unwrap_err()
            .to_string()
    );
}

#[test]
fn _0007() {
    let href =
        HRef::try_from("https://dsntk.io/documents#_b51ac78b-fd76-42fc-a12d-aad7150c9278").unwrap();
    assert_eq!("https://dsntk.io/documents", href.namespace().unwrap());
    assert_eq!("_b51ac78b-fd76-42fc-a12d-aad7150c9278", href.id());
}

#[test]
fn _0008() {
    assert_eq!(
        r#"<HRefError> fragment is missing in reference: 'https://dsntk.io/documents'"#,
        HRef::try_from("https://dsntk.io/documents")
            .unwrap_err()
            .to_string()
    );
}

#[test]
fn _0009() {
    assert_eq!(
        r#"<HRefError> invalid reference 'https::\/dsntk.io/documents#id', reason: invalid path character"#,
        HRef::try_from(r#"https::\/dsntk.io/documents#id"#)
            .unwrap_err()
            .to_string()
    );
}

#[test]
fn _0010() {
    assert_eq!(
        r#"<HRefError> query is not allowed in reference: 'https://dsntk.io/documents?name=john'"#,
        HRef::try_from(r#"https://dsntk.io/documents?name=john"#)
            .unwrap_err()
            .to_string()
    );
}

#[test]
fn _0011() {
    assert_eq!(
        r#"<HRefError> invalid reference 'https://127.0.0.1:a/alfa#_id', reason: invalid port character"#,
        HRef::try_from(r#"https://127.0.0.1:a/alfa#_id"#)
            .unwrap_err()
            .to_string()
    );
}
