//! Implementation of the parsing context.

use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::{FeelType, Name};
use std::borrow::Borrow;
use std::collections::{BTreeMap, HashSet};
use std::fmt;

/// Types of entries in parsing context.
pub enum ParsingEntry {
    /// Parsing entry representing a context.
    Context(ParsingContext),
    /// Parsing entry representing a variable.
    Variable,
}

impl fmt::Display for ParsingEntry {
    /// Converts parsing entry into text representation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParsingEntry::Context(ctx) => ctx.to_string(),
                ParsingEntry::Variable => "<v>".to_string(),
            }
        )
    }
}

impl From<FeelType> for ParsingEntry {
    /// Converts a reference to feel type into parsing entry.
    fn from(value: FeelType) -> Self {
        Self::from(&value)
    }
}

impl From<&FeelType> for ParsingEntry {
    /// Converts a feel type into parsing entry.
    fn from(value: &FeelType) -> Self {
        match value {
            FeelType::Context(feel_type_ctx) => {
                let mut entries = BTreeMap::new();
                for (name, feel_type) in feel_type_ctx {
                    entries.insert(name.to_owned(), feel_type.into());
                }
                ParsingEntry::Context(ParsingContext(entries))
            }
            FeelType::List(feel_type_items) => {
                let feel_type = feel_type_items.borrow();
                if let FeelType::Context(_) = feel_type {
                    Self::from(feel_type)
                } else {
                    ParsingEntry::Variable
                }
            }
            _ => ParsingEntry::Variable,
        }
    }
}

/// Parsing context.
#[derive(Default)]
pub struct ParsingContext(BTreeMap<Name, ParsingEntry>);

impl fmt::Display for ParsingContext {
    /// Converts parsing context into text representation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|(name, entry)| { format!(r#"{name}: {entry}"#) })
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl From<FeelContext> for ParsingContext {
    /// Converts feel context into parsing context.
    fn from(ctx: FeelContext) -> Self {
        Self::from(&ctx)
    }
}

impl From<&FeelContext> for ParsingContext {
    /// Converts a reference to feel context into parsing context.
    fn from(value: &FeelContext) -> Self {
        let mut entries = BTreeMap::new();
        for (name, value) in value.iter() {
            match value {
                Value::Context(ctx) => {
                    entries.insert(name.to_owned(), ParsingEntry::Context(ctx.into()));
                }
                list @ Value::List(_) => {
                    entries.insert(name.to_owned(), list.type_of().into());
                }
                Value::FeelType(feel_type) => {
                    entries.insert(name.to_owned(), feel_type.into());
                }
                _ => {
                    entries.insert(name.to_owned(), ParsingEntry::Variable);
                }
            }
        }
        Self(entries)
    }
}

impl ParsingContext {
    /// Places a specified name in this parsing context.
    pub fn set_name(&mut self, name: Name) {
        self.0.insert(name, ParsingEntry::Variable);
    }

    /// Places parsing context under specified name.
    pub fn set_context(&mut self, name: Name, ctx: ParsingContext) {
        self.0.insert(name, ParsingEntry::Context(ctx));
    }

    /// Returns a list of flattened keys for this parsing context.
    pub fn flattened_keys(&self) -> HashSet<String> {
        let mut keys: HashSet<String> = HashSet::new();
        for (key, value) in self.0.iter() {
            keys.insert(key.into());
            if let ParsingEntry::Context(sub_ctx) = value {
                let sub_keys = sub_ctx.flattened_keys();
                if !sub_keys.is_empty() {
                    for sub_key in sub_keys {
                        keys.insert(sub_key.clone());
                        keys.insert(format!("{key} . {sub_key}"));
                    }
                }
            }
        }
        keys
    }
}
