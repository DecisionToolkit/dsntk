use dsntk_evaluator::prepare_test_cases;

#[test]
fn test_empty_input_file() {
    assert_eq!(0, prepare_test_cases("").unwrap().len())
}

#[test]
fn test_valid_input_file() {
    let input = r#"
      % { Customer:"Business",   Order:  -3.23 }, 0.10
      % { Customer:"Business",   Order:   9.00 }, 0.10
      % { Customer:"Business",   Order:  10.00 }, 0.15
      % { Customer:"Business",   Order: 120.00 }, 0.15
      % { Customer:"Private",    Order:  -2.34 }, 0.05
      % { Customer:"Private",    Order:  10.00 }, 0.05
      % { Customer:"Private",    Order: 101.00 }, 0.05
      % { Customer:"Government", Order:  10.00 }, null
  "#;
    assert_eq!(8, prepare_test_cases(input).unwrap().len())
}

#[test]
fn _0001() {
    let input = r#"
      % -
  "#;
    assert_eq!(
        "<EvaluatorError> expected expression list, but found ' Irrelevant\n'",
        prepare_test_cases(input).unwrap_err().to_string()
    );
}

#[test]
fn _0002() {
    let input = r#"  % { Customer: "Business"  "#;
    assert_eq!(
        r#"<ParserError> syntax error: { Customer: "Business""#,
        prepare_test_cases(input).unwrap_err().to_string()
    );
}

#[test]
fn _0003() {
    let input = r#"  % { Customer: "Business", Order: 1.12 }, 0.10, 0.20 "#;
    assert_eq!(
        "<EvaluatorError> expression list must have exactly 2 elements, found 3",
        prepare_test_cases(input).unwrap_err().to_string()
    );
}

#[test]
fn _0004() {
    let input = r#"  % 0.10, 0.20 "#;
    assert_eq!(
        "<FeelEvaluatorError> expected FEEL context on input",
        prepare_test_cases(input).unwrap_err().to_string()
    );
}
