//! # FEEL types

use crate::context::FeelContext;
use crate::errors::*;
use crate::names::Name;
use crate::value_null;
use crate::values::Value;
use dsntk_common::{DsntkError, Result};
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

pub const FEEL_TYPE_NAME_ANY: &str = "Any";
pub const FEEL_TYPE_NAME_BOOLEAN: &str = "boolean";
pub const FEEL_TYPE_NAME_DATE: &str = "date";
pub const FEEL_TYPE_NAME_DATE_AND_TIME: &str = "date and time";
pub const FEEL_TYPE_NAME_DAYS_AND_TIME_DURATION: &str = "days and time duration";
pub const FEEL_TYPE_NAME_NULL: &str = "Null";
pub const FEEL_TYPE_NAME_NUMBER: &str = "number";
pub const FEEL_TYPE_NAME_STRING: &str = "string";
pub const FEEL_TYPE_NAME_TIME: &str = "time";
pub const FEEL_TYPE_NAME_YEARS_AND_MONTHS_DURATION: &str = "years and months duration";

#[derive(Debug, Clone, PartialEq)]
#[must_use]
pub enum FeelType {
  /// Type representing any valid FEEL type.
  Any,

  /// Type representing a `boolean` value.
  Boolean,

  /// Type representing a `context` value.
  Context(
    /// Types of context entries.
    BTreeMap<Name, FeelType>,
  ),

  /// Type representing a `date` value.
  Date,

  /// Type representing a`date and time` value.
  DateTime,

  /// Type representing a `days and time duration` value.
  DaysAndTimeDuration,

  /// Type representing a `function` value.
  Function(
    /// List of types of the function's parameters.
    Vec<FeelType>,
    /// Type of the function's result.
    Box<FeelType>,
  ),

  /// Type representing a `list` of values.
  List(Box<FeelType>),

  /// Type representing a `null` value.
  Null,

  /// Type representing a `number` value.
  Number,

  /// Type representing a `range` values.
  Range(Box<FeelType>),

  /// Type representing a `string` value.
  String,

  /// Type representing a `time` value.
  Time,

  /// Type representing a `years and months duration` value.
  YearsAndMonthsDuration,
}

impl fmt::Display for FeelType {
  /// Converts [FeelType] to into string.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      FeelType::Any => write!(f, "{FEEL_TYPE_NAME_ANY}"),
      FeelType::Boolean => write!(f, "{FEEL_TYPE_NAME_BOOLEAN}"),
      FeelType::Context(entries) => {
        let entries_str = entries
          .iter()
          .map(|(entry_name, entry_type)| format!("{entry_name}: {entry_type}"))
          .collect::<Vec<String>>()
          .join(", ");
        write!(f, "context<{entries_str}>",)
      }
      FeelType::Date => write!(f, "{FEEL_TYPE_NAME_DATE}"),
      FeelType::DateTime => write!(f, "{FEEL_TYPE_NAME_DATE_AND_TIME}"),
      FeelType::DaysAndTimeDuration => write!(f, "{FEEL_TYPE_NAME_DAYS_AND_TIME_DURATION}"),
      FeelType::Function(parameter_types, result_type) => {
        let parameter_types_str = parameter_types.iter().map(|parameter_type| format!("{parameter_type}")).collect::<Vec<String>>().join(", ");
        let result_type_str = result_type.to_string();
        write!(f, "function<{parameter_types_str}>->{result_type_str}")
      }
      FeelType::List(item_type) => {
        write!(f, "list<{item_type}>")
      }
      FeelType::Null => write!(f, "{FEEL_TYPE_NAME_NULL}"),
      FeelType::Number => write!(f, "{FEEL_TYPE_NAME_NUMBER}"),
      FeelType::Range(range_type) => {
        write!(f, "range<{range_type}>")
      }
      FeelType::String => write!(f, "{FEEL_TYPE_NAME_STRING}"),
      FeelType::Time => write!(f, "{FEEL_TYPE_NAME_TIME}"),
      FeelType::YearsAndMonthsDuration => write!(f, "{FEEL_TYPE_NAME_YEARS_AND_MONTHS_DURATION}"),
    }
  }
}

impl FromStr for FeelType {
  type Err = DsntkError;
  /// Converts a string to built-in type.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      FEEL_TYPE_NAME_ANY => Ok(Self::Any),
      FEEL_TYPE_NAME_BOOLEAN => Ok(Self::Boolean),
      FEEL_TYPE_NAME_DATE => Ok(Self::Date),
      FEEL_TYPE_NAME_DATE_AND_TIME => Ok(Self::DateTime),
      FEEL_TYPE_NAME_DAYS_AND_TIME_DURATION => Ok(Self::DaysAndTimeDuration),
      FEEL_TYPE_NAME_NULL => Ok(Self::Null),
      FEEL_TYPE_NAME_NUMBER => Ok(Self::Number),
      FEEL_TYPE_NAME_STRING => Ok(Self::String),
      FEEL_TYPE_NAME_TIME => Ok(Self::Time),
      FEEL_TYPE_NAME_YEARS_AND_MONTHS_DURATION => Ok(Self::YearsAndMonthsDuration),
      _ => Err(err_invalid_feel_type_name(s)),
    }
  }
}

impl From<&Name> for FeelType {
  /// Converts a FEEL name to built-in type.
  fn from(name: &Name) -> Self {
    if let Ok(feel_type) = Self::from_str(name.to_string().as_str()) {
      feel_type
    } else {
      FeelType::Any
    }
  }
}

/// Returns `true` when the specified type name is a built-in type.
pub fn is_built_in_type_name(name: &str) -> bool {
  matches!(
    name,
    FEEL_TYPE_NAME_ANY
      | FEEL_TYPE_NAME_BOOLEAN
      | FEEL_TYPE_NAME_DATE
      | FEEL_TYPE_NAME_DATE_AND_TIME
      | FEEL_TYPE_NAME_DAYS_AND_TIME_DURATION
      | FEEL_TYPE_NAME_NULL
      | FEEL_TYPE_NAME_NUMBER
      | FEEL_TYPE_NAME_STRING
      | FEEL_TYPE_NAME_TIME
      | FEEL_TYPE_NAME_YEARS_AND_MONTHS_DURATION
  )
}

impl FeelType {
  pub fn get_conformant_value(&self, actual_value: &Value) -> Value {
    if let FeelType::Null = self {
      return value_null!();
    }
    if actual_value.is_conformant(self) {
      self.get_value_checked(actual_value).unwrap()
    } else {
      value_null!("type '{}' is not conformant with value '{}'", self.to_string(), actual_value)
    }
  }

  /// Returns a new value cloned from provided value, and retrieved with type checking.
  pub fn get_value_checked(&self, value: &Value) -> Result<Value> {
    if let Value::Null(_) = value {
      // `null` value is conformant with all types
      return Ok(value_null!());
    }
    match self {
      FeelType::Any => {
        return Ok(value.clone());
      }
      FeelType::Boolean => {
        if let Value::Boolean(_) = value {
          return Ok(value.clone());
        }
      }
      FeelType::Context(entries) => {
        if let Value::Context(context) = value {
          let mut result = FeelContext::default();
          for (name, entry_type) in entries {
            if let Some(entry_value) = context.get_entry(name) {
              result.set_entry(name, entry_type.get_value_checked(entry_value)?);
            }
          }
          return Ok(Value::Context(result));
        }
      }
      FeelType::Date => {
        if let Value::Date(_) = value {
          return Ok(value.clone());
        }
      }
      FeelType::DateTime => {
        if let Value::DateTime(_) = value {
          return Ok(value.clone());
        }
      }
      FeelType::DaysAndTimeDuration => {
        if let Value::DaysAndTimeDuration(_) = value {
          return Ok(value.clone());
        }
      }
      FeelType::Function(_, _) => {
        if let Value::FunctionDefinition { .. } = value {
          return Ok(value.clone());
        }
      }
      FeelType::List(items_type) => {
        if let Value::List(items) = value {
          let mut result = vec![];
          for item in items {
            result.push(items_type.get_value_checked(item)?);
          }
          return Ok(Value::List(result));
        }
      }
      FeelType::Number => {
        if let Value::Number(_) = value {
          return Ok(value.clone());
        }
      }
      FeelType::Range(_) => {
        if let Value::Range(_, _, _, _) = value {
          return Ok(value.clone());
        }
      }
      FeelType::String => {
        if let Value::String(_) = value {
          return Ok(value.clone());
        }
      }
      FeelType::Time => {
        if let Value::Time(_) = value {
          return Ok(value.clone());
        }
      }
      FeelType::YearsAndMonthsDuration => {
        if let Value::YearsAndMonthsDuration(_) = value {
          return Ok(value.clone());
        }
      }
      _ => {}
    }
    Err(err_invalid_value_for_retrieving_using_feel_type(&self.to_string(), &value.to_string()))
  }

  /// Returns `true` when this type is a simple FEEL type.
  pub fn is_simple_built_in_type(&self) -> bool {
    matches!(
      self,
      Self::Any | Self::Boolean | Self::Date | Self::DateTime | Self::DaysAndTimeDuration | Self::Null | Self::Number | Self::String | Self::Time | Self::YearsAndMonthsDuration
    )
  }

  /// Returns `true` when this type is a Null type.
  pub fn is_null(&self) -> bool {
    matches!(self, Self::Null)
  }

  /// Creates a `list` type with specified items' type.
  pub fn list(items_type: &FeelType) -> FeelType {
    FeelType::List(Box::new(items_type.clone()))
  }

  /// Creates a `range` type with specified elements' type.
  pub fn range(elements_type: &FeelType) -> FeelType {
    FeelType::Range(Box::new(elements_type.clone()))
  }

  /// Creates a `context` type with specified entries.
  pub fn context(entries_types: &[(&Name, &FeelType)]) -> FeelType {
    FeelType::Context(entries_types.iter().map(|(name, typ)| ((*name).clone(), (*typ).clone())).collect())
  }

  /// Creates a `function` type with specified parameter types and result type.
  pub fn function(parameter_types: &[FeelType], result_type: &FeelType) -> FeelType {
    FeelType::Function(parameter_types.iter().map(|typ| (*typ).clone()).collect(), Box::new((*result_type).clone()))
  }

  /// Checks if this type is conformant with specified target type.
  pub fn is_conformant(&self, target_type: &FeelType) -> bool {
    if matches!(self, FeelType::Null) {
      return true;
    }
    match target_type {
      FeelType::Any => return true,
      FeelType::Null => return matches!(self, FeelType::Null),
      FeelType::Boolean => return matches!(self, FeelType::Boolean),
      FeelType::Number => return matches!(self, FeelType::Number),
      FeelType::String => return matches!(self, FeelType::String),
      FeelType::Date => return matches!(self, FeelType::Date),
      FeelType::Time => return matches!(self, FeelType::Time),
      FeelType::DateTime => return matches!(self, FeelType::DateTime),
      FeelType::DaysAndTimeDuration => return matches!(self, FeelType::DaysAndTimeDuration),
      FeelType::YearsAndMonthsDuration => return matches!(self, FeelType::YearsAndMonthsDuration),
      FeelType::Range(type_other) => {
        if let FeelType::Range(type_self) = self {
          return type_self.is_conformant(type_other);
        }
      }
      FeelType::List(type_other) => {
        if let FeelType::List(type_self) = self {
          return type_self.is_conformant(type_other);
        }
      }
      FeelType::Context(target_entries) => {
        if let FeelType::Context(self_entries) = self {
          for (name, type_target) in target_entries {
            if let Some(type_self) = self_entries.get(name) {
              if !type_self.is_conformant(type_target) {
                return false;
              }
            } else {
              return false;
            }
          }
          return true;
        }
        return false;
      }
      FeelType::Function(parameters_other, result_other) => {
        if let FeelType::Function(parameters_self, result_self) = self
          && parameters_self.len() == parameters_other.len()
        {
          for (i, parameter_other) in parameters_other.iter().enumerate() {
            if !parameter_other.is_conformant(&parameters_self[i]) {
              return false;
            }
            if !result_self.is_conformant(result_other) {
              return false;
            }
          }
          return true;
        }
        return false;
      }
    }
    false
  }

  /// Checks if this type is an instance of the other type.
  pub fn instance_of(&self, other: &FeelType) -> bool {
    // list
    if let FeelType::List(self_inner_type) = self
      && let FeelType::List(other_inner_type) = other
    {
      return self_inner_type == other_inner_type || **other_inner_type == FeelType::Any;
    }
    // context
    if let FeelType::Context(self_inner_type) = self
      && let FeelType::Context(other_inner_type) = other
    {
      if self_inner_type == other_inner_type {
        return true;
      }
      for (name, other_inner_feel_type) in other_inner_type {
        if let Some(self_inner_feel_type) = self_inner_type.get(name) {
          if !self_inner_feel_type.is_null() && self_inner_feel_type != other_inner_feel_type {
            return false;
          }
        } else {
          return false;
        }
      }
      return true;
    }
    false
  }
}
