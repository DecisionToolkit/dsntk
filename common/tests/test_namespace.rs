use dsntk_common::to_rdnn;
use url::Url;

fn assert_rdnn(expected: &str, actual: Option<String>) {
    let value = actual.unwrap();
    let url = format!("https://dsntk.io/{}", value);
    assert!(Url::parse(&url).is_ok());
    assert_eq!(expected, value);
}

#[test]
fn test_to_rdnn() {
    assert_rdnn(
        "au/com/montera/www/spec/DMN/0099-arithmetic-negation",
        to_rdnn("https://www.montera.com.au/spec/DMN/0099-arithmetic-negation"),
    );
    assert_rdnn(
        "au/com/montera/www/spec/DMN/0099-arithmetic-negation",
        to_rdnn(
            "https://www.montera.com.au/spec/DMN/0099-arithmetic-negation#_427d2e23-d096-47d5-b861-ebb7f37f461e",
        ),
    );
    assert_rdnn("au/com/montera/www", to_rdnn("https://www.montera.com.au"));
    assert_rdnn(
        "io/dsntk/model-2/decision-5",
        to_rdnn("https://dsntk.io/model-2/decision-5"),
    );
    assert_eq!(None, to_rdnn("https://www.montera.com.au|error"));
    assert_eq!(None, to_rdnn("https://127.0.0.1"));
    assert_eq!(None, to_rdnn("montera.com"));
    assert_eq!(None, to_rdnn("data:text/plain,HelloWorld"));
}
