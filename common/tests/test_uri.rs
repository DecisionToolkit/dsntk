use dsntk_common::{encode_segments, to_uri};

#[test]
fn _0001() {
    assert_eq!(
        "https://decision-toolkit.org/guide/overview",
        to_uri("https://decision-toolkit.org/guide/overview")
            .unwrap()
            .to_string()
    )
}

#[test]
fn _0002() {
    assert_eq!(
        "<UriError> invalid URI: ''",
        to_uri("").unwrap_err().to_string()
    )
}

#[test]
fn _0003() {
    assert_eq!(
        "<UriError> invalid URI: 'http://document?name=john'",
        to_uri("http://document?name=john").unwrap_err().to_string()
    )
}

#[test]
fn _0004() {
    assert_eq!(
        "<UriError> invalid URI: 'http://document#_id'",
        to_uri("http://document#_id").unwrap_err().to_string()
    )
}

#[test]
fn _0005() {
    assert_eq!(
        "<UriError> invalid URI: 'https://127.0.0.1:a/document#_id'",
        to_uri("https://127.0.0.1:a/document#_id")
            .unwrap_err()
            .to_string()
    )
}

#[test]
fn encoding_path_segments_should_work() {
    assert_eq!("a%20b/c%20d/e%20f", encode_segments("a b/c d/e f"));
}
