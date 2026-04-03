//! # FEEL values

use crate::FunctionBody;
use crate::bif::Bif;
use crate::closure::Closure;
use crate::context::FeelContext;
use crate::errors::*;
use crate::names::Name;
use crate::strings::ToFeelString;
use crate::types::FeelType;
use dsntk_common::{Jsonify, Result};
use dsntk_feel_number::FeelNumber;
use dsntk_feel_temporal::{FeelDate, FeelDateTime, FeelDaysAndTimeDuration, FeelTime, FeelYearsAndMonthsDuration};
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

/// Creates `Value::Null` with optional tracing message.
///
/// # Examples
///
/// ```
/// use crate::dsntk_feel::{value_null, values::Value};
///
/// let v = value_null!();
/// assert_eq!("null", v.to_string());
///
/// let v = value_null!("missing input parameter");
/// assert_eq!("null(missing input parameter)", v.to_string());
///
/// let v = value_null!("integer out of range {}..{}", 1, 100);
/// assert_eq!("null(integer out of range 1..100)", v.to_string());
/// ```
#[macro_export]
macro_rules! value_null {
  ($module:expr, $function:literal, $format:literal, $($arguments:tt)*) => {
    Value::Null(Some(format!("[{}::{}] {}", $module, $function, format!($format, $($arguments)*))))
  };
  ($format:literal, $($arguments:tt)*) => {
    Value::Null(Some(format!($format, $($arguments)*)))
  };
  ($argument:expr) => {
    Value::Null(Some(format!("{}", $argument)))
  };
  () => {
    Value::Null(None)
  };
}

#[macro_export]
macro_rules! value_number {
  ($n:expr) => {{ Value::Number($n.into()) }};
  ($n:expr, $s:expr) => {
    Value::Number(FeelNumber::new($n, $s))
  };
}

#[macro_export]
macro_rules! value_string {
  ($s:literal) => {{ Value::String($s.to_string()) }};
  ($s:expr) => {{ Value::String($s) }};
}

/// Utility constant for value `true `of type `Boolean`.
pub const VALUE_TRUE: Value = Value::Boolean(true);

/// Utility constant for value `false` of type `Boolean`.
pub const VALUE_FALSE: Value = Value::Boolean(false);

/// Constant indicating invalid coercion result.
const INVALID_COERCION: &str = "after coercion";

/// `FEEL` value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  /// Value representing `FEEL` boolean type.
  Boolean(bool),

  /// Value for storing built-in function definition.
  BuiltInFunction(Bif),

  /// Value representing a context.
  Context(FeelContext),

  /// Value representing a context entry.
  ContextEntry(Name, Box<Value>),

  /// Value representing a key of the context entry.
  ContextEntryKey(Name),

  /// Value representing the context type.
  ContextType(FeelType),

  /// Value representing a context entry in context type definition.
  ContextTypeEntry(Name, FeelType),

  /// Value representing a key of the context entry in context type definition.
  ContextTypeEntryKey(Name),

  /// Value for storing dates as [FeelDate].
  Date(FeelDate),

  /// Value for storing date and time as [FeelDateTime].
  DateTime(FeelDateTime),

  /// Value for days and time durations.
  DaysAndTimeDuration(FeelDaysAndTimeDuration),

  /// Value representing a collection of comma-separated list of expressions.
  ExpressionList(Values),

  /// Value representing a mapping to externally defined `Java` function.
  ExternalJavaFunction(
    /// Class name.
    String,
    /// Method signature.
    String,
  ),

  /// Value representing a mapping to externally defined `PMML` function.
  ExternalPmmlFunction(
    /// Document.
    String,
    /// Model name.
    String,
  ),

  /// Value representing the `FEEL` type of a value.
  FeelType(FeelType),

  /// Value representing function's formal parameter with name and type.
  FormalParameter(Name, FeelType),

  /// List of formal parameters.
  FormalParameters(Vec<(Name, FeelType)>),

  /// Definition of the function body.
  FunctionBody(
    /// Body of the function.
    FunctionBody,
    /// Flag indicating if the function's body is an external function, `true` == external.
    bool,
  ),

  /// Value representing the function definition.
  /// This value holds the list of function's formal parameters, the function's body, closure for lambdas and expected result type.
  FunctionDefinition(
    /// Formal parameters of the function.
    Vec<(Name, FeelType)>,
    /// Body of the function.
    FunctionBody,
    /// Flag indicating if the function's body is an external function, `true` == external.
    bool,
    /// Closed names from function context (closure names).
    Closure,
    /// Values of the closed names (closure values).
    FeelContext,
    /// Return type of the function.
    FeelType,
  ),

  /// Value representing interval end.
  IntervalEnd(Box<Value>, bool),

  /// Value representing interval start.
  IntervalStart(Box<Value>, bool),

  /// Value representing `FEEL` `irrelevant` value.
  Irrelevant,

  /// Value representing a list of values.
  List(Values),

  /// Named parameter.
  NamedParameter(Box<Value>, Box<Value>),

  /// Value representing a collection of named parameters.
  NamedParameters(BTreeMap<Name, (Value, usize)>),

  /// Value representing a collection of values representing a negated comma-separated list of expressions.
  NegatedCommaList(Values),

  /// Null value with optional tracing message.
  Null(Option<String>),

  /// Value representing `FEEL` number type.
  Number(FeelNumber),

  /// Name of the parameter.
  ParameterName(Name),

  /// Value representing a list of function's parameter types.
  ParameterTypes(Vec<Value>),

  /// List of positional parameters.
  PositionalParameters(Values),

  /// Value representing a segment of a qualified name.
  QualifiedNameSegment(Name),

  /// Value representing a `range`.
  Range(Box<Value>, bool, Box<Value>, bool),

  /// `String` value...
  String(String),

  /// Value for storing time as [FeelTime].
  Time(FeelTime),

  /// Value representing unary `>`.
  UnaryGreater(
    /// Value on the right side of the unary `>` operator.
    Box<Value>,
  ),

  /// Value representing unary `>=`.
  UnaryGreaterOrEqual(
    /// Value on the right side of the unary `>=` operator.
    Box<Value>,
  ),

  /// Value representing unary `<`.
  UnaryLess(
    /// Value on the right side of the unary `<` operator.
    Box<Value>,
  ),

  /// Value representing unary `<=`.
  UnaryLessOrEqual(
    /// Value on the right side of the unary `<=` operator.
    Box<Value>,
  ),

  /// Value representing unary `=`.
  UnaryEqual(
    /// Value on the right side of the unary `=` operator.
    Box<Value>,
  ),

  /// Value representing unary `!=`.
  UnaryNotEqual(
    /// Value on the right side of the unary `!=` operator.
    Box<Value>,
  ),

  /// Value that is "bubbled-up" without any processing.
  Transparent(Box<Value>),

  /// Value for storing years and months duration.
  YearsAndMonthsDuration(FeelYearsAndMonthsDuration),
}

impl Display for Value {
  /// Implements [Display] for a [Value].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Boolean(value) => write!(f, "{value}"),
      Value::BuiltInFunction(_) => write!(f, "BuiltInFunction"),
      Value::Context(context) => write!(f, "{context}"),
      Value::ContextEntry(_, _) => write!(f, "ContextEntry"),
      Value::ContextEntryKey(name) => write!(f, "{name}"),
      Value::ContextType(_) => write!(f, "ContextType"),
      Value::ContextTypeEntry(name, feel_type) => write!(f, "{name}: {feel_type}"),
      Value::ContextTypeEntryKey(name) => write!(f, "{name}"),
      Value::Date(date) => write!(f, "{date}"),
      Value::DateTime(date_time) => write!(f, "{date_time}"),
      Value::DaysAndTimeDuration(dt_duration) => write!(f, "{dt_duration}"),
      Value::ExpressionList(items) => write!(f, "{}", values_to_string(items)),
      Value::ExternalJavaFunction(class_name, method_signature) => write!(f, "ExternalJavaFunction({class_name}, {method_signature})"),
      Value::ExternalPmmlFunction(iri, model_name) => write!(f, "ExternalPmmlFunction({iri}, {model_name})"),
      Value::FeelType(feel_type) => write!(f, "type({feel_type})"),
      Value::FormalParameter(_, _) => write!(f, "FormalParameter"),
      Value::FormalParameters(_) => write!(f, "FormalParameters"),
      Value::FunctionBody(_, external) => write!(f, "FunctionBody{}", if *external { " (external)" } else { "" }),
      Value::FunctionDefinition(parameters, _body, external, closure, closure_ctx, return_type) => {
        write!(f, "FunctionDefinition({parameters:?},_,{external},{closure},{closure_ctx},{return_type})")
      }
      Value::IntervalEnd(_, _) => write!(f, "IntervalEnd"),
      Value::IntervalStart(_, _) => write!(f, "IntervalStart"),
      Value::Irrelevant => write!(f, "Irrelevant"),
      Value::List(items) => write!(f, "{}", values_to_string(items)),
      Value::NamedParameter(_, _) => write!(f, "NamedParameter"),
      Value::NamedParameters(_) => write!(f, "NamedParameters"),
      Value::NegatedCommaList(_) => write!(f, "NegatedCommaList"),
      Value::Number(value) => write!(f, "{value}"),
      Value::Null(trace) => write!(f, "null{}", trace.as_ref().map_or("".to_string(), |s| format!("({s})"))),
      Value::ParameterName(_) => write!(f, "ParameterName"),
      Value::ParameterTypes(_) => write!(f, "ParameterTypes"),
      Value::PositionalParameters(_) => write!(f, "PositionalParameters"),
      Value::QualifiedNameSegment(_) => write!(f, "QualifiedNameSegment"),
      Value::Range(v1, c1, v2, c2) => write!(
        f,
        "{}{}..{}{}",
        if *c1 { '[' } else { '(' },
        if v1.is_null() { "".to_string() } else { v1.to_string() },
        if v2.is_null() { "".to_string() } else { v2.to_string() },
        if *c2 { ']' } else { ')' }
      ),
      Value::String(s) => write!(f, "\"{s}\""),
      Value::Time(time) => write!(f, "{time}"),
      Value::UnaryGreater(value) => write!(f, "UnaryGreater({value})"),
      Value::UnaryGreaterOrEqual(value) => write!(f, "UnaryGreaterOrEqual({value})"),
      Value::UnaryLess(value) => write!(f, "UnaryLess({value})"),
      Value::UnaryLessOrEqual(value) => write!(f, "UnaryLessOrEqual({value})"),
      Value::UnaryEqual(value) => write!(f, "UnaryEqual({value})"),
      Value::UnaryNotEqual(value) => write!(f, "UnaryNotEqual({value})"),
      Value::Transparent(value) => write!(f, "Transparent({value})"),
      Value::YearsAndMonthsDuration(ym_duration) => write!(f, "{ym_duration}"),
    }
  }
}

impl ToFeelString for Value {
  /// Converts [Value] into `FEEL` string.
  fn to_feel_string(&self) -> String {
    match self {
      Value::Context(context) => context.to_feel_string(),
      Value::List(items) => values_to_feel_string(items),
      Value::String(value) => format!("\"{}\"", value.replace('"', "\\\"")),
      other => other.to_string(),
    }
  }
}

impl Jsonify for Value {
  /// Converts a [Value] into `JSON`.
  fn jsonify(&self) -> String {
    match self {
      Value::Boolean(value) => format!("{value}"),
      Value::Number(value) => value.jsonify(),
      Value::String(s) => format!(r#""{s}""#),
      Value::Date(date) => format!(r#""{}""#, date),
      Value::Time(time) => format!(r#""{}""#, time),
      Value::DateTime(date_time) => format!(r#""{}""#, date_time),
      Value::DaysAndTimeDuration(dt_duration) => format!(r#""{}""#, dt_duration),
      Value::YearsAndMonthsDuration(ym_duration) => format!(r#""{}""#, ym_duration),
      Value::ExpressionList(items) => values_to_jsonify(items),
      Value::Context(ctx) => ctx.jsonify(),
      Value::ContextEntryKey(name) => name.to_string(),
      Value::List(items) => values_to_jsonify(items),
      range @ Value::Range(..) => format!(r#""{}""#, range),
      Value::Null(message) => {
        if let Some(details) = message {
          format!(r#""null({details})""#)
        } else {
          "null".to_string()
        }
      }
      _ => format!("jsonify trait not implemented for value: {self}"),
    }
  }
}

impl Value {
  /// Returns `true` when the value is of type [Value::Null].
  pub fn is_null(&self) -> bool {
    matches!(self, Value::Null(_))
  }

  /// Returns `true` when the value is of type [Value::Boolean] and is equal to `true`.
  pub fn is_true(&self) -> bool {
    matches!(self, Value::Boolean(true))
  }

  /// Returns `true` when the value is of type [Value::Boolean] and is equal to `false`.
  pub fn is_false(&self) -> bool {
    matches!(self, Value::Boolean(false))
  }

  /// Returns `true` when the value is of type [Value::Number].
  pub fn is_number(&self) -> bool {
    matches!(self, Value::Number(_))
  }

  /// Returns `true` when the value is of type [Value::String].
  pub fn is_string(&self) -> bool {
    matches!(self, Value::String(_))
  }

  /// Returns `true` when the value is of type [Value::List].
  pub fn is_list(&self) -> bool {
    matches!(self, Value::List(_))
  }

  /// Returns `true` when the value is of type [Value::Range].
  pub fn is_range(&self) -> bool {
    matches!(self, Value::Range(_, _, _, _))
  }

  /// Returns `true` when the value is a valid range.
  pub fn is_valid_range(&self) -> bool {
    if let Value::Range(start, start_closed, end, end_closed) = self {
      match start.borrow() {
        Value::String(start) => match end.borrow() {
          Value::String(end) => start <= end,
          Value::Null(_) => !*end_closed,
          _ => false,
        },
        Value::Number(start) => match end.borrow() {
          Value::Number(end) => start <= end,
          Value::Null(_) => !*end_closed,
          _ => false,
        },
        Value::Date(start) => match end.borrow() {
          Value::Date(end) => start <= end,
          Value::Null(_) => !*end_closed,
          _ => false,
        },
        Value::DateTime(start) => match end.borrow() {
          Value::DateTime(end) => start <= end,
          Value::Null(_) => !*end_closed,
          _ => false,
        },
        Value::Time(start) => match end.borrow() {
          Value::Time(end) => start <= end,
          Value::Null(_) => !*end_closed,
          _ => false,
        },
        Value::YearsAndMonthsDuration(start) => match end.borrow() {
          Value::YearsAndMonthsDuration(end) => start <= end,
          Value::Null(_) => !*end_closed,
          _ => false,
        },
        Value::DaysAndTimeDuration(start) => match end.borrow() {
          Value::DaysAndTimeDuration(end) => start <= end,
          Value::Null(_) => !*end_closed,
          _ => false,
        },
        Value::Null(_) => match end.borrow() {
          Value::Null(_) => false,
          _ => !*start_closed,
        },
        _ => false,
      }
    } else {
      false
    }
  }

  /// Returns `true` when the value is of type [Value::Null] indicating invalid coercion.
  pub fn is_invalid_coercion(&self) -> bool {
    if let Value::Null(Some(message)) = self { message == INVALID_COERCION } else { false }
  }

  /// Returns the type of this [Value].
  pub fn type_of(&self) -> FeelType {
    match self {
      Value::Boolean(_) => FeelType::Boolean,
      Value::BuiltInFunction(_) => FeelType::Any,
      Value::Context(context) => {
        let mut entries = BTreeMap::new();
        for (name, value) in context.deref() {
          entries.insert(name.clone(), value.type_of());
        }
        FeelType::Context(entries)
      }
      Value::ContextEntry(_, _) => FeelType::Any,
      Value::ContextEntryKey(_) => FeelType::Any,
      Value::ContextType(feel_type) => feel_type.clone(),
      Value::ContextTypeEntry(_, feel_type) => feel_type.clone(),
      Value::ContextTypeEntryKey(_) => FeelType::Any,
      Value::Date(_) => FeelType::Date,
      Value::DateTime(_) => FeelType::DateTime,
      Value::DaysAndTimeDuration(_) => FeelType::DaysAndTimeDuration,
      Value::ExpressionList(_) => FeelType::Any,
      Value::ExternalJavaFunction(_, _) => FeelType::Any,
      Value::ExternalPmmlFunction(_, _) => FeelType::Any,
      Value::FeelType(feel_type) => feel_type.clone(),
      Value::FormalParameter(_, feel_type) => feel_type.clone(),
      Value::FormalParameters(_) => FeelType::Any,
      Value::FunctionBody(_, _) => FeelType::Any,
      Value::FunctionDefinition(parameters, _, _, _, _, result_type) => {
        let parameter_types = parameters.iter().map(|(_, feel_type)| feel_type.clone()).collect();
        FeelType::Function(parameter_types, Box::new(result_type.clone()))
      }
      Value::IntervalEnd(interval_end, _) => interval_end.type_of(),
      Value::IntervalStart(interval_start, _) => interval_start.type_of(),
      Value::Irrelevant => FeelType::Any,
      Value::List(values) => {
        if values.is_empty() {
          FeelType::List(Box::new(FeelType::Null))
        } else {
          let item_type = values[0].type_of();
          for item in values {
            if !item.type_of().is_conformant(&item_type) {
              return FeelType::List(Box::new(FeelType::Any));
            }
          }
          FeelType::List(Box::new(item_type))
        }
      }
      Value::NamedParameter(_, _) => FeelType::Any,
      Value::NamedParameters(_) => FeelType::Any,
      Value::NegatedCommaList(_) => FeelType::Any,
      Value::Null(_) => FeelType::Null,
      Value::Number(_) => FeelType::Number,
      Value::ParameterName(_) => FeelType::Any,
      Value::ParameterTypes(_) => FeelType::Any,
      Value::PositionalParameters(_) => FeelType::Any,
      Value::QualifiedNameSegment(_) => FeelType::Any,
      Value::Range(range_start, _, range_end, _) => {
        let range_start_type = range_start.type_of();
        let range_end_type = range_end.type_of();
        if range_start_type == range_end_type {
          return FeelType::Range(Box::new(range_start_type));
        }
        FeelType::Range(Box::new(FeelType::Any))
      }
      Value::String(_) => FeelType::String,
      Value::Time(_) => FeelType::Time,
      Value::UnaryGreater(_) => FeelType::Boolean,
      Value::UnaryGreaterOrEqual(_) => FeelType::Boolean,
      Value::UnaryLess(_) => FeelType::Boolean,
      Value::UnaryLessOrEqual(_) => FeelType::Boolean,
      Value::UnaryEqual(_) => FeelType::Boolean,
      Value::UnaryNotEqual(_) => FeelType::Boolean,
      Value::Transparent(value) => value.type_of(),
      Value::YearsAndMonthsDuration(_) => FeelType::YearsAndMonthsDuration,
    }
  }

  /// Checks if a value is conformant with specified target type.
  pub fn is_conformant(&self, target_type: &FeelType) -> bool {
    if matches!(target_type, FeelType::Any) {
      return true;
    }
    match self {
      Value::Null(_) => true,
      Value::Boolean(_) => matches!(target_type, FeelType::Boolean),
      Value::Number(_) => matches!(target_type, FeelType::Number),
      Value::String(_) => matches!(target_type, FeelType::String),
      Value::Date(_) => matches!(target_type, FeelType::Date),
      Value::Time(_) => matches!(target_type, FeelType::Time),
      Value::DateTime(_) => matches!(target_type, FeelType::DateTime),
      Value::DaysAndTimeDuration(_) => matches!(target_type, FeelType::DaysAndTimeDuration),
      Value::YearsAndMonthsDuration(_) => matches!(target_type, FeelType::YearsAndMonthsDuration),
      Value::Range(r_start, _, r_end, _) => {
        if let FeelType::Range(range_type) = target_type {
          return r_start.is_conformant(range_type) && r_end.is_conformant(range_type);
        }
        false
      }
      Value::List(items) => {
        if let FeelType::List(list_type) = target_type {
          for item in items {
            if !item.is_conformant(list_type) {
              return false;
            }
          }
          return true;
        }
        false
      }
      Value::Context(context) => {
        if let FeelType::Context(type_context) = target_type {
          for (name, entry_type) in type_context.iter() {
            if let Some(entry_value) = context.get(name) {
              if !entry_value.is_conformant(entry_type) {
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
      Value::FunctionDefinition(parameters, _, _, _, _, result_type) => {
        if let FeelType::Function(t_parameters, t_result) = target_type {
          if parameters.len() != t_parameters.len() {
            return false;
          }
          if !result_type.is_conformant(t_result) {
            return false;
          }
          for (i, (_, parameter_type)) in parameters.iter().enumerate() {
            if !parameter_type.is_conformant(&t_parameters[i]) {
              return false;
            }
          }
          return true;
        }
        false
      }
      _ => false,
    }
  }

  /// Returns value coerced to specified type.
  /// When a value appears in a certain context, it must be compatible
  /// with a type expected in that context, called the target type.
  /// After the type of the value is known, an implicit conversion
  /// from the type of the value to the target type can be performed.
  /// If an implicit conversion is mandatory but it cannot be performed,
  /// the result is null.
  ///
  /// There are several possible type conversions:
  ///
  /// - to singleton list:
  ///
  ///  When the type of the value is `T` and the target type is `List<T>`,
  ///  the simple value is converted to a singleton list.
  ///
  /// - from singleton list:
  ///
  ///  When the type of the value is `List<T>`, and the value is a singleton list
  ///  and the target type is T, the value is converted by unwrapping the first element.
  ///
  /// - conforms to:
  ///
  ///  When the type of the value is T1, the target type is T2, and T1 conforms to T2,
  ///  the value remains unchanged, otherwise the result is null.
  ///
  /// All these conversion rules are implemented in this function.
  ///
  pub fn coerced(&self, target_type: &FeelType) -> Value {
    if let Value::FunctionDefinition(_, _, _, _, _, _) = self {
      return self.clone();
    }
    if let Value::BuiltInFunction(bif) = self
      && bif.feel_type().is_conformant(target_type)
    {
      return self.clone();
    }
    if self.is_conformant(target_type) {
      return self.clone();
    }
    match self {
      // from singleton list
      Value::List(items) => {
        if items.len() == 1 {
          let value = items[0].clone();
          if value.is_conformant(target_type) {
            return value;
          }
        }
      }
      // to singleton list
      value => {
        if let FeelType::List(list_type) = target_type
          && value.is_conformant(list_type)
        {
          return Value::List(vec![value.clone()]);
        }
      }
    }
    value_null!(INVALID_COERCION)
  }

  /// Tries to convert `xsd:integer` string into valid [Value] representing a number.
  pub fn try_from_xsd_integer(text: &str) -> Result<Self> {
    let value = text.parse::<FeelNumber>().map_err(|_| err_invalid_xsd_integer(text))?;
    Ok(Value::Number(value))
  }

  /// Tries to convert `xsd:decimal` string into valid [Value] representing a number.
  pub fn try_from_xsd_decimal(text: &str) -> Result<Self> {
    let value = text.parse::<FeelNumber>().map_err(|_| err_invalid_xsd_decimal(text))?;
    Ok(Value::Number(value))
  }

  /// Tries to convert `xsd:double` string into valid [Value] representing a number.
  pub fn try_from_xsd_double(text: &str) -> Result<Self> {
    let value = text.parse::<FeelNumber>().map_err(|_| err_invalid_xsd_double(text))?;
    Ok(Value::Number(value))
  }

  /// Tries to convert `xsd:boolean` string into valid [Value] representing a boolean.
  pub fn try_from_xsd_boolean(text: &str) -> Result<Self> {
    match text {
      "true" | "1" => Ok(Value::Boolean(true)),
      "false" | "0" => Ok(Value::Boolean(false)),
      _ => Err(err_invalid_xsd_boolean(text)),
    }
  }

  /// Tries to convert `xsd:date` string into valid [Value] representing a date.
  /// FEEL date format is fully conformant with `xsd:date`.
  pub fn try_from_xsd_date(text: &str) -> Result<Self> {
    if let Ok(feel_date) = FeelDate::from_str(text) {
      return Ok(Value::Date(feel_date));
    }
    Err(err_invalid_xsd_date(text))
  }

  /// Tries to convert `xsd:time` string into valid [Value] representing a time.
  /// FEEL time format is fully conformant with `xsd:time`.
  pub fn try_from_xsd_time(text: &str) -> Result<Self> {
    if let Ok(feel_time) = FeelTime::from_str(text) {
      return Ok(Value::Time(feel_time));
    }
    Err(err_invalid_xsd_time(text))
  }

  /// Tries to convert `xsd:dateTime` string into valid [Value] representing a date and time.
  /// FEEL date and time format is fully conformant with `xsd:dateTime`.
  pub fn try_from_xsd_date_time(text: &str) -> Result<Self> {
    Ok(Value::DateTime(FeelDateTime::try_from(text).map_err(|_| err_invalid_xsd_date_time(text))?))
  }

  /// Tries to convert `xsd:duration` string into valid [Value] representing a date and time.
  /// FEEL durations are conformant with `xsd:duration` but spit into two ranges.
  pub fn try_from_xsd_duration(text: &str) -> Result<Self> {
    if let Ok(ym_duration) = FeelYearsAndMonthsDuration::try_from(text) {
      return Ok(Value::YearsAndMonthsDuration(ym_duration));
    }
    if let Ok(dt_duration) = FeelDaysAndTimeDuration::try_from(text) {
      return Ok(Value::DaysAndTimeDuration(dt_duration));
    }
    Err(err_invalid_xsd_duration(text))
  }
}

/// Type alias to a collection of values.
pub type Values = Vec<Value>;

/// Converts a collection of values into string.
pub fn values_to_string(values: &Values) -> String {
  format!("[{}]", values.iter().map(|value| value.to_string()).collect::<Vec<String>>().join(", "))
}

/// Converts a collection of values into `FEEL` string.
pub fn values_to_feel_string(values: &Values) -> String {
  format!("[{}]", values.iter().map(|value| value.to_feel_string()).collect::<Vec<String>>().join(", "))
}

/// Converts a collection of values into `JSON` string.
pub fn values_to_jsonify(values: &Values) -> String {
  format!("[{}]", values.iter().map(|value| value.jsonify()).collect::<Vec<String>>().join(", "))
}
