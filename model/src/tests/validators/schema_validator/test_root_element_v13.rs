use super::test_files::*;
use super::*;

#[test]
fn _0001() {
    assert_eq!(
        "<ModelParserError> unexpected XML node, expected: definitions, actual: definitionen",
        expect_err_str(v13::_0001)
    );
}

#[test]
fn _0002() {
    assert_eq!(
        "<ModelError> no supported namespace found",
        expect_err_str(v13::_0002)
    );
}

#[test]
fn _0003() {
    assert_eq!(
        "<ModelError> no supported namespace found",
        expect_err_str(v13::_0003)
    );
}

#[test]
fn _0004() {
    assert_eq!(
        "<ModelParserError> expected value for mandatory attribute 'name' in node 'definitions' at [2:1]",
        expect_err_str(v13::_0004)
    );
}

#[test]
fn _0005() {
    assert_eq!(
        "<ModelParserError> expected value for mandatory attribute 'namespace' in node 'definitions' at [2:1]",
        expect_err_str(v13::_0005)
    );
}

#[test]
fn _0006() {
    assert_eq!(
        "<ModelError> not allowed attribute: 'revision' in node 'definitions' at [2:1]",
        expect_err_str(v13::_0006)
    );
}

#[test]
fn _0007() {
    assert_eq!(
        "<ModelError> not allowed attribute: 'revision' in node 'definitions' at [2:1]",
        expect_err_str(v13::_0007)
    );
}

#[test]
fn _0008() {
    assert_eq!(
        "<ModelError> not allowed child node: 'revisions' in node 'definitions' at [2:1]",
        expect_err_str(v13::_0008)
    );
}

#[test]
fn _0009() {
    assert_eq!(
        "<ModelError> not allowed child node: 'revisions' in node 'definitions' at [2:1]",
        expect_err_str(v13::_0009)
    );
}
