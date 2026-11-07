use dsntk_recognizer::from_markdown;

/// Rule numbers are not monotonic.
#[test]
fn _0001() {
    let markdown = r#"
    | U |    Customer type     | Order size | Discount |         Priority          | Description | Reference |
    |:-:|:--------------------:|:----------:|:--------:|:-------------------------:|:-----------:|:---------:|
    |   | "Business","Private" |            |          | "Normal",**"High"**,"Low" |             |           |
    |   |         `in`         |    `in`    |  `out`   |           `out`           |     `#`     |    `#`    |
    | 1 |      "Business"      |    <10     |   0.10   |         "Normal"          | Small order |   Ref 1   |
    | 3 |      "Business"      |    >=10    |   0.15   |          "High"           | Large order |   Ref 2   |
    | 4 |      "Private"       |     -      |   0.05   |           "Low"           | All orders  |   Ref 3   |
  "#;
    assert_eq!(
        "<RecognizerError> invalid decision table, reason: can not recognize the preferred orientation",
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}

/// Rule numbers are not monotonic.
#[test]
fn _0002() {
    let markdown = r#"
    | U                     |       |                             |   1    |    2     |    4     |    3     |   5    |
    |:----------------------|:-----:|:---------------------------:|:------:|:--------:|:--------:|:--------:|:------:|
    | Applicant age         | `In`  |     <25, [25..60], >60      |  <25   |   <25    | [25..60] |   >60    |  >60   |
    | Medical history       | `In`  |        "good", "bad"        | "good" |  "bad"   |    -     |  "good"  | "bad"  |
    | Applicant risk rating | `Out` | "Low", "Medium", **"High"** | "Low"  | "Medium" | "Medium" | "Medium" | "High" |
    | Special Discount      | `Out` |          0, 5, 10           |   10   |    5     |    5     |    5     |   0    |
    | Additional acceptance | `Ann` |                             |   No   |    No    |    No    |    No    |  Yes   |
    | Reference             | `Ann` |                             |  Rf 0  |   Rf 1   |   Rf 2   |   Rf 3   |  Rf 4  |
  "#;
    assert_eq!(
        "<RecognizerError> invalid decision table, reason: can not recognize the preferred orientation",
        from_markdown(markdown, false).unwrap_err().to_string()
    );
}
