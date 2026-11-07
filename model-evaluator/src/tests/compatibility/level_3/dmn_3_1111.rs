use super::*;

from_examples!(DMN_3_1111);

static_context!(CTX, "{}");

fn eq(invocable: &str, ctx: &FeelContext, expected: &str) {
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable,
        ctx,
        expected,
    );
}

#[test]
fn _0001() {
    let invocable = "decision001";
    let expected =
        "null([core::matches] invalid argument type, expected string, actual type is Null)";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0002() {
    let invocable = "decision002";
    let expected =
        "null([core::matches] invalid argument type, expected string, actual type is Null)";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0003() {
    let invocable = "decision003";
    let expected = "true";
    eq(invocable, &CTX, expected);
}
#[test]
fn _0004() {
    let invocable = "fn-matches2args-1";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0005() {
    let invocable = "fn-matchesErr-1";
    let expected = "null(<FeelRegexError> invalid regex flag: 'p')";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0006() {
    let invocable = "fn-matches-27";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0007() {
    let invocable = "fn-matches-28";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
#[ignore]
fn _0008() {
    let invocable = "fn-matches-32";
    let expected = "?";
    eq(invocable, &CTX, expected);
}

#[test]
#[ignore]
fn _0009() {
    let invocable = "fn-matches-33";
    let expected = "?";
    eq(invocable, &CTX, expected);
}

#[test]
#[ignore]
fn _0010() {
    let invocable = "fn-matches-34";
    let expected = "?";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0011() {
    let invocable = "fn-matches-45";
    let expected = "false";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0012() {
    let invocable = "caselessmatch01";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0013() {
    let invocable = "caselessmatch02";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0014() {
    let invocable = "caselessmatch03";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0015() {
    let invocable = "caselessmatch07";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0016() {
    let invocable = "caselessmatch08";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0017() {
    let invocable = "caselessmatch09";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0018() {
    let invocable = "caselessmatch10";
    let expected = "false";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0019() {
    let invocable = "caselessmatch11";
    let expected = "false";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0020() {
    let invocable = "caselessmatch12";
    let expected = "false";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0021() {
    let invocable = "caselessmatch13";
    let expected = "false";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0022() {
    let invocable = "K-MatchesFunc-1";
    let expected =
        "null([core::matches] invalid argument type, expected string, actual type is list<Null>)";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0023() {
    let invocable = "K-MatchesFunc-2";
    let expected = "null(expected 2,3 parameters, actual number of parameters is 1)";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0024() {
    let invocable = "K-MatchesFunc-3";
    let expected =
        "null([core::matches] invalid argument type, expected string, actual type is list<Null>)";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0025() {
    let invocable = "K-MatchesFunc-4";
    let expected = "null(expected 2,3 parameters, actual number of parameters is 4)";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0026() {
    let invocable = "K-MatchesFunc-5";
    let expected = "null(<FeelRegexError> empty regex flags)";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0027() {
    let invocable = "K-MatchesFunc-6";
    let expected = "null(<FeelRegexError> invalid regex flag: 'X')";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0028() {
    let invocable = "K2-MatchesFunc-1";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0029() {
    let invocable = "K2-MatchesFunc-2";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0030() {
    let invocable = "K2-MatchesFunc-3";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0031() {
    let invocable = "K2-MatchesFunc-4";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0032() {
    let invocable = "K2-MatchesFunc-5";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0033() {
    let invocable = "K2-MatchesFunc-6";
    let expected = "true";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0034() {
    let invocable = "K2-MatchesFunc-7";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '\p{ IsBasicLatin}+')"#;
    eq(invocable, &CTX, expected);
}

#[test]
fn _0035() {
    let invocable = "K2-MatchesFunc-8";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '(.)\3')"#;
    eq(invocable, &CTX, expected);
}

#[test]
fn _0036() {
    let invocable = "K2-MatchesFunc-9";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '(.)\2')"#;
    eq(invocable, &CTX, expected);
}

#[test]
fn _0037() {
    let invocable = "K2-MatchesFunc-10";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '\3')"#;
    eq(invocable, &CTX, expected);
}

#[test]
fn _0038() {
    let invocable = "K2-MatchesFunc-11";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '(asd)[\1]')"#;
    eq(invocable, &CTX, expected);
}

#[test]
fn _0039() {
    let invocable = "K2-MatchesFunc-12";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '(asd)[asd\1]')"#;
    eq(invocable, &CTX, expected);
}

#[test]
fn _0040() {
    let invocable = "K2-MatchesFunc-13";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '(asd)[asd\0]')"#;
    eq(invocable, &CTX, expected);
}

#[test]
fn _0041() {
    let invocable = "K2-MatchesFunc-14";
    let expected = r#"null(<FeelRegexError> invalid regex pattern: '1[asd\0]')"#;
    eq(invocable, &CTX, expected);
}

#[test]
fn _0042() {
    let invocable = "K2-MatchesFunc-15";
    let expected = "[false, true]";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0043() {
    let invocable = "K2-MatchesFunc-16";
    let expected = "false";
    eq(invocable, &CTX, expected);
}

#[test]
fn _0044() {
    let invocable = "K2-MatchesFunc-17";
    let expected = "true";
    eq(invocable, &CTX, expected);
}
