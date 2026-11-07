use dsntk_recognizer::from_markdown;

#[test]
fn _0001() {
    let markdown = r#""#;
    assert_eq!(
        "<RecognizerError> no decision table",
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0002() {
    let markdown = r#"
    > # Offered selling options
  "#;
    assert_eq!(
        "<RecognizerError> no decision table",
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0003() {
    let markdown = r#"
    > # Offered selling options

    some text that is not a decision table
  "#;
    assert_eq!(
        "<RecognizerError> no decision table",
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0004() {
    let markdown = r#"
    > # Offered selling options
    > Sell options
  "#;
    assert_eq!(
        "<RecognizerError> no decision table",
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0005() {
    let markdown = r#"
    > # Offered selling options
    > Sell options

    some text that is not a decision table
  "#;
    assert_eq!(
        "<RecognizerError> no decision table",
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0006() {
    let markdown = r#"
    > Sell options
  "#;
    assert_eq!(
        "<RecognizerError> no decision table",
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0007() {
    let markdown = r#"
    > Sell options

    some text that is not a decision table
  "#;
    assert_eq!(
        "<RecognizerError> no decision table",
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0008() {
    let markdown = r#"
    | U |
    |:-:|
    |   |
    | 1 |
  "#;
    assert_eq!(
        "<RecognizerError> invalid decision table, reason: number of columns is less than 2",
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0009() {
    let markdown = r#"
    | U |     |    1     |
    |:--|:---:|:--------:|
  "#;
    assert_eq!(
        "<RecognizerError> invalid decision table, reason: number of rows is less than 2",
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}
