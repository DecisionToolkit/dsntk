use super::*;

from_examples!(DMN_2_0105);

#[test]
fn _0001() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision1",
        &ctx,
        r#"15"#,
    );
}

#[test]
fn _0002() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision2",
        &ctx,
        r#"-15"#,
    );
}

#[test]
fn _0003() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision3",
        &ctx,
        r#"-15"#,
    );
}

#[test]
fn _0004() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision4",
        &ctx,
        r#"5"#,
    );
}

#[test]
fn _0005() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision5",
        &ctx,
        r#"-5"#,
    );
}

#[test]
fn _0006() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision6",
        &ctx,
        r#"-5"#,
    );
}

#[test]
fn _0007() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision7",
        &ctx,
        r#"32"#,
    );
}

#[test]
fn _0008() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision8",
        &ctx,
        r#"50"#,
    );
}

#[test]
fn _0009() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision9",
        &ctx,
        r#"50"#,
    );
}

#[test]
fn _0010() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision10",
        &ctx,
        r#"50"#,
    );
}

#[test]
fn _0011() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision11",
        &ctx,
        r#"-225"#,
    );
}

#[test]
fn _0012() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision12",
        &ctx,
        r#"2"#,
    );
}

#[test]
fn _0013() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision13",
        &ctx,
        r#"2"#,
    );
}

#[test]
fn _0014() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision14",
        &ctx,
        r#"2"#,
    );
}

#[test]
fn _0015() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision15",
        &ctx,
        r#"-2"#,
    );
}

#[test]
fn _0016() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision16",
        &ctx,
        r#"null([division] division by zero)"#,
    );
}

#[test]
fn _0017() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision17",
        &ctx,
        r#"100000"#,
    );
}

#[test]
fn _0018() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision18",
        &ctx,
        r#"0.00001"#,
    );
}

#[test]
fn _0019() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision19",
        &ctx,
        r#"16807"#,
    );
}

#[test]
fn _0020() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision20",
        &ctx,
        r#"37"#,
    );
}

#[test]
fn _0021() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision21",
        &ctx,
        r#"40"#,
    );
}

#[test]
fn _0022() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision22",
        &ctx,
        r#"261"#,
    );
}

#[test]
fn _0023() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision23",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0024() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision24",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0025() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision25",
        &ctx,
        r#"null([subtraction] incompatible types: 10 - null)"#,
    );
}

#[test]
fn _0026() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision26",
        &ctx,
        r#"null([subtraction] incompatible types: null - 10)"#,
    );
}

#[test]
fn _0027() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision27",
        &ctx,
        r#"null([multiplication] incompatible types: 10 * null)"#,
    );
}

#[test]
fn _0028() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision28",
        &ctx,
        r#"null"#,
    );
}

#[test]
fn _0029() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision29",
        &ctx,
        r#"null([division] incompatible types: 10 / null)"#,
    );
}

#[test]
fn _0030() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision30",
        &ctx,
        r#"null([division] incompatible types: null / 10)"#,
    );
}

#[test]
fn _0031() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision31",
        &ctx,
        r#"3"#,
    );
}

#[test]
fn _0032() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision32",
        &ctx,
        r#"7.5"#,
    );
}

#[test]
fn _0033() {
    let ctx = context(r#"{}"#);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Decision33",
        &ctx,
        r#"1200"#,
    );
}
