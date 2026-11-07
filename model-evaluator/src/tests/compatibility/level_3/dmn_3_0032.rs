use super::*;

from_examples!(DMN_3_0032);

#[test]
fn _0001() {
    let ctx = context(r#"{bool: true,num: 100}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "simpleIf",
        &ctx,
        r#"110"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{bool: false,num: 100}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "simpleIf",
        &ctx,
        r#"90"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{bool: null,num: 100}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "simpleIf",
        &ctx,
        r#"90"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{aDate: @"2017-01-02",aString: "Hello World"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "conditionWithFunctions",
        &ctx,
        r#""Hello""#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{aDate: @"2017-01-01",aString: "Hello World"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "conditionWithFunctions",
        &ctx,
        r#""World""#,
    );
}

#[test]
fn _0006() {
    let ctx = context(r#"{aDate: null,aString: "Hello World"}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "conditionWithFunctions",
        &ctx,
        r#""World""#,
    );
}
