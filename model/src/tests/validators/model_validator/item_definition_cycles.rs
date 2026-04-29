//! # Test cases for cyclic dependencies between item definitions

use super::test_files::*;
use crate::from_xml;

#[test]
fn _0001() {
  assert!(from_xml(DMN_0001).is_ok());
}

#[test]
fn _0002() {
  assert_eq!(
    "<ModelValidatorError> cyclic dependency between item definitions",
    from_xml(DMN_1001).err().unwrap().to_string()
  );
}
