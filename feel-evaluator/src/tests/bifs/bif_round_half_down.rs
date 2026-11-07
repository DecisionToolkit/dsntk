use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_number(false, &scope!(), "round half down(5.0, 0)", 5, 0);
}

#[test]
fn _0002() {
    te_number(false, &scope!(), "round half down(5.1, 0)", 5, 0);
}

#[test]
fn _0003() {
    te_number(false, &scope!(), "round half down(5.2, 0)", 5, 0);
}

#[test]
fn _0004() {
    te_number(false, &scope!(), "round half down(5.3, 0)", 5, 0);
}

#[test]
fn _0005() {
    te_number(false, &scope!(), "round half down(5.4, 0)", 5, 0);
}

#[test]
fn _0006() {
    te_number(false, &scope!(), "round half down(5.5, 0)", 5, 0);
}

#[test]
fn _0007() {
    te_number(false, &scope!(), "round half down(5.6, 0)", 6, 0);
}

#[test]
fn _0008() {
    te_number(false, &scope!(), "round half down(5.7, 0)", 6, 0);
}

#[test]
fn _0009() {
    te_number(false, &scope!(), "round half down(5.8, 0)", 6, 0);
}

#[test]
fn _0010() {
    te_number(false, &scope!(), "round half down(5.9, 0)", 6, 0);
}

#[test]
fn _0011() {
    te_number(false, &scope!(), "round half down(6.0, 0)", 6, 0);
}

#[test]
fn _0012() {
    te_number(false, &scope!(), "round half down(-5.0, 0)", -5, 0);
}

#[test]
fn _0013() {
    te_number(false, &scope!(), "round half down(-5.1, 0)", -5, 0);
}

#[test]
fn _0014() {
    te_number(false, &scope!(), "round half down(-5.2, 0)", -5, 0);
}

#[test]
fn _0015() {
    te_number(false, &scope!(), "round half down(-5.3, 0)", -5, 0);
}

#[test]
fn _0016() {
    te_number(false, &scope!(), "round half down(-5.4, 0)", -5, 0);
}

#[test]
fn _0017() {
    te_number(false, &scope!(), "round half down(-5.5, 0)", -5, 0);
}

#[test]
fn _0018() {
    te_number(false, &scope!(), "round half down(-5.6, 0)", -6, 0);
}

#[test]
fn _0019() {
    te_number(false, &scope!(), "round half down(-5.7, 0)", -6, 0);
}

#[test]
fn _0020() {
    te_number(false, &scope!(), "round half down(-5.8, 0)", -6, 0);
}

#[test]
fn _0021() {
    te_number(false, &scope!(), "round half down(-5.9, 0)", -6, 0);
}

#[test]
fn _0022() {
    te_number(false, &scope!(), "round half down(-6.0, 0)", -6, 0);
}

#[test]
fn _0023() {
    te_number(false, &scope!(), "round half down(1.121, 2)", 112, 2);
}

#[test]
fn _0024() {
    te_number(false, &scope!(), "round half down(1.122, 2)", 112, 2);
}

#[test]
fn _0025() {
    te_number(false, &scope!(), "round half down(1.123, 2)", 112, 2);
}

#[test]
fn _0026() {
    te_number(false, &scope!(), "round half down(1.124, 2)", 112, 2);
}

#[test]
fn _0027() {
    te_number(false, &scope!(), "round half down(1.125, 2)", 112, 2);
}

#[test]
fn _0028() {
    te_number(false, &scope!(), "round half down(1.126, 2)", 113, 2);
}

#[test]
fn _0029() {
    te_number(false, &scope!(), "round half down(1.127, 2)", 113, 2);
}

#[test]
fn _0030() {
    te_number(false, &scope!(), "round half down(1.128, 2)", 113, 2);
}

#[test]
fn _0031() {
    te_number(false, &scope!(), "round half down(1.129, 2)", 113, 2);
}

#[test]
fn _0032() {
    te_number(false, &scope!(), "round half down(-1.121, 2)", -112, 2);
}

#[test]
fn _0033() {
    te_number(false, &scope!(), "round half down(-1.122, 2)", -112, 2);
}

#[test]
fn _0034() {
    te_number(false, &scope!(), "round half down(-1.123, 2)", -112, 2);
}

#[test]
fn _0035() {
    te_number(false, &scope!(), "round half down(-1.124, 2)", -112, 2);
}

#[test]
fn _0036() {
    te_number(false, &scope!(), "round half down(-1.125, 2)", -112, 2);
}

#[test]
fn _0037() {
    te_number(false, &scope!(), "round half down(-1.126, 2)", -113, 2);
}

#[test]
fn _0038() {
    te_number(false, &scope!(), "round half down(-1.127, 2)", -113, 2);
}

#[test]
fn _0039() {
    te_number(false, &scope!(), "round half down(-1.128, 2)", -113, 2);
}

#[test]
fn _0040() {
    te_number(false, &scope!(), "round half down(-1.129, 2)", -113, 2);
}

#[test]
fn _0041() {
    te_number(
        false,
        &scope!(),
        "round half down(n: -1.129, scale: 2)",
        -113,
        2,
    );
}

#[test]
fn _0042() {
    te_null(
        false,
        &scope!(),
        "round half down(1.1, 0, 5)",
        "expected 2 parameters, actual number of parameters is 3",
    );
}

#[test]
fn _0043() {
    te_null(
        false,
        &scope!(),
        "round half down()",
        "expected 2 parameters, actual number of parameters is 0",
    );
}

#[test]
fn _0044() {
    te_null(
        false,
        &scope!(),
        "round half down(num: 1.23, scale: 1)",
        "parameter 'n' not found",
    );
}

#[test]
fn _0045() {
    te_null(
        false,
        &scope!(),
        "round half down(n: 1.23, s: 1)",
        "parameter 'scale' not found",
    );
}

#[test]
fn _0046() {
    te_null(
        false,
        &scope!(),
        "round half down(n: 1.23, scale: 1, foo: 11)",
        "expected 2 parameters, actual number of parameters is 3",
    );
}

#[test]
fn _0047() {
    //--------------------------------------------------------------------------------------------------------------------
    // Used maximum allowed scale.
    //--------------------------------------------------------------------------------------------------------------------
    te_number(false, &scope!(), "round half down(5.5, 6144)", 55, 1);
}

#[test]
fn _0048() {
    //--------------------------------------------------------------------------------------------------------------------
    // Used minimum allowed scale.
    //--------------------------------------------------------------------------------------------------------------------
    te_number(false, &scope!(), "round half down(5.5, -6111)", 0, 0);
}

#[test]
fn _0049() {
    //--------------------------------------------------------------------------------------------------------------------
    // Scale is out of range.
    //--------------------------------------------------------------------------------------------------------------------
    te_null(
        false,
        &scope!(),
        "round half down(5.5, 6145)",
        "[core::round_half_down] <FeelNumberError> invalid scale, allowed range is -6111..6144, actual is 6145",
    );
}

#[test]
fn _0050() {
    //--------------------------------------------------------------------------------------------------------------------
    // Scale is out of range.
    //--------------------------------------------------------------------------------------------------------------------
    te_null(
        false,
        &scope!(),
        "round half down(5.5, -6112)",
        "[core::round_half_down] <FeelNumberError> invalid scale, allowed range is -6111..6144, actual is -6112",
    );
}
