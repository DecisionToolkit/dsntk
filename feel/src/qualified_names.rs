//! # FEEL qualified names

use crate::Name;
use std::fmt;
use std::fmt::Display;
use std::ops::Deref;

/// FEEL qualified name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct QualifiedName(Vec<Name>);

impl QualifiedName {
    /// Creates a [QualifiedName] from [Names](Name).
    pub fn new(names: &[&Name]) -> Self {
        Self(names.iter().map(|&v| v.clone()).collect::<Vec<Name>>())
    }
}

impl Display for QualifiedName {
    /// Implements [Display] trait for [QualifiedName].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(".")
        )
    }
}

impl Deref for QualifiedName {
    type Target = Vec<Name>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl QualifiedName {
    /// Appends this [QualifiedName] with a given [Name].
    pub fn push(&mut self, name: Name) {
        self.0.push(name);
    }

    /// Inserts a given [Name] at specified position in [QualifiedName].
    pub fn insert(&mut self, index: usize, name: Name) {
        self.0.insert(index, name);
    }

    /// Returns the last [Name] from [QualifiedName].
    pub fn pop(&mut self) -> Option<Name> {
        self.0.pop()
    }
}

impl From<Name> for QualifiedName {
    /// Converts a name into qualified name.
    fn from(value: Name) -> Self {
        Self(value.to_string().split('.').map(Name::from).collect())
    }
}

impl From<Vec<Name>> for QualifiedName {
    /// Converts a vector of names into qualified name.
    fn from(value: Vec<Name>) -> Self {
        let mut names = vec![];
        for name in value {
            let mut sub_names = name
                .to_string()
                .split('.')
                .map(Name::from)
                .collect::<Vec<Name>>();
            names.append(&mut sub_names);
        }
        Self(names)
    }
}
