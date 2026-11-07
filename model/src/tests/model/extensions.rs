//! # Tests for extension elements and attributes.
//!
//! These elements are currently ignored, so tests are just for code coverage.

use crate::model::{ExtensionAttribute, ExtensionElement};

#[test]
#[allow(clippy::redundant_clone)]
fn test_extension_element() {
    let actual = ExtensionElement;
    assert_eq!("ExtensionElement", format!("{:?}", actual.clone()));
}

#[test]
#[allow(clippy::redundant_clone)]
fn test_extension_attribute() {
    let actual = ExtensionAttribute;
    assert_eq!("ExtensionAttribute", format!("{:?}", actual.clone()));
}
