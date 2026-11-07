use super::test_files::*;
use super::*;

#[test]
fn _0030() {
    assert_eq!(
        "<ModelParserError> expected value for mandatory attribute 'name' in node 'inputData' at [5:5]",
        expect_err_str(v13::_0030)
    );
}

#[test]
fn _0031() {
    assert_eq!(
        "<ModelParserError> expected value for mandatory attribute 'name' in node 'inputData' at [6:5]",
        expect_err_str(v13::_0031)
    );
}
