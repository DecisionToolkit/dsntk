//! # Test cases for cyclic dependencies between item definitions

use super::test_files::*;
use crate::parse;

#[test]
fn _0001() {
    assert!(parse(DMN_0001).is_ok());
}

#[test]
fn _0002() {
    assert_eq!(
        "<ModelValidatorError> cyclic dependency between item definitions",
        parse(DMN_1001).err().unwrap().to_string()
    );
}
