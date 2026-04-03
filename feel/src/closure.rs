//! # Closure

use crate::{Name, QualifiedName};
use std::collections::BTreeSet;
use std::collections::btree_set::Iter;
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Closure(BTreeSet<QualifiedName>);

impl fmt::Display for Closure {
  /// Converts a closure to string.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{}]", self.0.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(","))
  }
}

impl From<Vec<QualifiedName>> for Closure {
  /// Creates a [Closure] from a vector of [QualifiedNames](QualifiedName).
  fn from(value: Vec<QualifiedName>) -> Self {
    Self(value.iter().cloned().collect())
  }
}

impl Closure {
  /// Returns an iterator over closure items.
  pub fn iter(&self) -> Iter<'_, QualifiedName> {
    self.0.iter()
  }

  /// Removes a closure item with specified name.
  pub fn remove(&mut self, name: Name) {
    let qname: QualifiedName = name.into();
    self.0.remove(&qname);
  }
}
