use dsntk_recognizer::from_markdown;

#[test]
fn _0001() {
    let markdown = r#"
    | C |           |
    |:-:|:---------:|
    |   |    Out    |
    | 1 | "Monday"  |
    | 2 | "Tuesday" | 2 |
  "#;
    assert_eq!(
        r#"<RecognizerError> invalid number of columns, expected 2, actual is 3 in row '| 2 | "Tuesday" | 2 |'"#,
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0002() {
    let markdown = r#"
    | C |           |
    |:-:|:---------:|
    | 1 | "Monday"  |
    | 2 | "Tuesday" |
  "#;
    assert_eq!(
        r#"<RecognizerError> invalid decision table, reason: no markers row before the first rule"#,
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0003() {
    let markdown = r#"
    | C |           |
    |:-:|:---------:|
    |   | 1,2       |
    |   | Out       |
    |   | invalid   |
    | 1 | "Monday"  |
    | 2 | "Tuesday" |
  "#;
    assert_eq!(
        r#"<RecognizerError> invalid decision table, reason: too many rows before the first rule"#,
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0004() {
    let markdown = r#"
    | U |    1     |
    |:--|:--------:|
    |   | "Monday" |
  "#;
    assert_eq!(
        r#"<RecognizerError> invalid decision table, reason: no markers column before the first rule"#,
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0005() {
    let markdown = r#"
    | U |     |     |         |    1     |     |
    |:--|:---:|:---:|:-------:|:--------:|:---:|
    |   | A   | Out | Invalid | "Monday" |     |
  "#;
    assert_eq!(
        r#"<RecognizerError> invalid decision table, reason: too many columns before the first rule"#,
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0006() {
    let markdown = r#"
    | U | Customer type |            | Discount | Priority | Description | Reference |
    |:-:|:-------------:|:----------:|:--------:|:--------:|:-----------:|:---------:|
    |   |     `>>>`     |   `>>>`    |  `<<<`   |  `<<<`   |     `#`     |    `#`    |
    | 1 |  "Business"   |    <10     |   0.11   | "Normal" | Small order |   Ref 1   |
    | 2 |  "Business"   |    >=10    |   0.16   |  "High"  | Large order |   Ref 2   |
    | 3 |   "Private"   |     -      |   0.06   |  "Low"   | All orders  |   Ref 3   |
  "#;
    assert_eq!(
        r#"<RecognizerError> invalid decision table, reason: no input clause"#,
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0007() {
    let markdown = r#"
    | U | Customer type | Order size | Discount | Priority | Description |           |
    |:-:|:-------------:|:----------:|:--------:|:--------:|:-----------:|:---------:|
    |   |     `>>>`     |   `>>>`    |  `<<<`   |  `<<<`   |     `#`     |    `#`    |
    | 1 |  "Business"   |    <10     |   0.11   | "Normal" | Small order |   Ref 1   |
    | 2 |  "Business"   |    >=10    |   0.16   |  "High"  | Large order |   Ref 2   |
    | 3 |   "Private"   |     -      |   0.06   |  "Low"   | All orders  |   Ref 3   |
  "#;
    assert_eq!(
        r#"<RecognizerError> invalid decision table, reason: no annotation name"#,
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0008() {
    let markdown = r#"
    | U | Customer type | Order size | Discount | Priority | Description | Reference |
    |:-:|:-------------:|:----------:|:--------:|:--------:|:-----------:|:---------:|
    |   |     `>>>`     |   `>>>`    |  `<<<`   |  `<<<`   |     `#`     |    `#`    |
    | 1 |  "Business"   |    <10     |   0.11   | "Normal" | Small order |   Ref 1   |
    | 2 |  "Business"   |            |   0.16   |  "High"  | Large order |   Ref 2   |
    | 3 |   "Private"   |     -      |   0.06   |  "Low"   | All orders  |   Ref 3   |
  "#;
    assert_eq!(
        r#"<RecognizerError> invalid decision table, reason: no input entry"#,
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0009() {
    let markdown = r#"
    | U | Customer type | Order size | Discount | Priority | Description | Reference |
    |:-:|:-------------:|:----------:|:--------:|:--------:|:-----------:|:---------:|
    |   |     `>>>`     |   `>>>`    |  `<<<`   |  `<<<`   |     `#`     |    `#`    |
    | 1 |  "Business"   |    <10     |   0.11   | "Normal" | Small order |   Ref 1   |
    | 2 |  "Business"   |    >=10    |   0.16   |  "High"  | Large order |   Ref 2   |
    | 3 |   "Private"   |     -      |          |  "Low"   | All orders  |   Ref 3   |
  "#;
    assert_eq!(
        r#"<RecognizerError> invalid decision table, reason: no output entry"#,
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0010() {
    let markdown = r#"
    | C  |           |
    |:--:|:---------:|
    | In |   Out     |
    | 1  | "Monday"  |
    | 2  | "Tuesday" |
  "#;
    assert_eq!(
        r#"<RecognizerError> invalid decision table, reason: unexpected marker"#,
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0011() {
    let markdown = r#"
    | C  |            |
    |:--:|:----------:|
    |    | Annotation |
    | 1  | "Monday"   |
    | 2  | "Tuesday"  |
  "#;
    assert_eq!(
        r#"<RecognizerError> invalid decision table, reason: expected input or output marker"#,
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0012() {
    let markdown = r#"
    | U | Customer type | Order size | Discount | Priority | Description | Reference |
    |:-:|:-------------:|:----------:|:--------:|:--------:|:-----------:|:---------:|
    |   |     `>>>`     |   `>>>`    |  `<<<`   |  `>>>`   |     `#`     |    `#`    |
    | 1 |  "Business"   |    <10     |   0.11   | "Normal" | Small order |   Ref 1   |
    | 2 |  "Business"   |    >=10    |   0.16   |  "High"  | Large order |   Ref 2   |
    | 3 |   "Private"   |     -      |   0.06   |  "Low"   | All orders  |   Ref 3   |
  "#;
    assert_eq!(
        r#"<RecognizerError> invalid decision table, reason: expected output or annotation marker"#,
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

#[test]
fn _0013() {
    let markdown = r#"
    | U | Customer type | Order size | Discount | Priority | Description | Reference |
    |:-:|:-------------:|:----------:|:--------:|:--------:|:-----------:|:---------:|
    |   |     `>>>`     |   `>>>`    |  `<<<`   |  `<<<`   |     `#`     |    In     |
    | 1 |  "Business"   |    <10     |   0.11   | "Normal" | Small order |   Ref 1   |
    | 2 |  "Business"   |    >=10    |   0.16   |  "High"  | Large order |   Ref 2   |
    | 3 |   "Private"   |     -      |   0.06   |  "Low"   | All orders  |   Ref 3   |
  "#;
    assert_eq!(
        r#"<RecognizerError> invalid decision table, reason: expected annotation marker"#,
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}
