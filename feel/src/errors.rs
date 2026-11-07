//! # Error definitions

use crate::values::Value;
use dsntk_common::{DsntkError, ToErrorMessage};

//-----------------------------------------------------------------------------
// TypesError
//-----------------------------------------------------------------------------

/// Definition of errors used in module [types](crate::types).
#[derive(ToErrorMessage)]
struct TypesError(String);

/// Creates an invalid `FEEL` type name error.
pub fn err_invalid_feel_type_name(s: &str) -> DsntkError {
    TypesError(format!("invalid FEEL type name: {s}")).into()
}

/// Creates an error indicating value non conformant with type.
pub fn err_invalid_value_for_retrieving_using_feel_type(s1: &str, s2: &str) -> DsntkError {
    TypesError(format!(
        "invalid value for retrieving with type check, type = '{s1}', value = '{s2}'"
    ))
    .into()
}

//-----------------------------------------------------------------------------
// ValueError
//-----------------------------------------------------------------------------

/// Value errors.
#[derive(ToErrorMessage)]
struct ValueError(String);

/// Error used when parsed text is not acceptable `xsd:integer` representation.
pub fn err_invalid_xsd_integer(text: &str) -> DsntkError {
    ValueError(format!("'{text}' is not valid xsd:integer representation")).into()
}
/// Error used when parsed text is not acceptable `xsd:decimal` representation.
pub fn err_invalid_xsd_decimal(text: &str) -> DsntkError {
    ValueError(format!("'{text}' is not valid xsd:decimal representation")).into()
}
/// Error used when parsed text is not acceptable `xsd:double` representation.
pub fn err_invalid_xsd_double(text: &str) -> DsntkError {
    ValueError(format!("'{text}' is not valid xsd:double representation")).into()
}
/// Error used when parsed text is not acceptable `xsd:boolean` representation.
pub fn err_invalid_xsd_boolean(text: &str) -> DsntkError {
    ValueError(format!("'{text}' is not valid xsd:boolean representation")).into()
}
/// Error used when parsed text is not acceptable `xsd:date` representation.
pub fn err_invalid_xsd_date(text: &str) -> DsntkError {
    ValueError(format!("'{text}' is not valid xsd:date representation")).into()
}
/// Error used when parsed text is not acceptable `xsd:time` representation.
pub fn err_invalid_xsd_time(text: &str) -> DsntkError {
    ValueError(format!("'{text}' is not valid xsd:time representation")).into()
}
/// Error used when parsed text is not acceptable `xsd:dateTime` representation.
pub fn err_invalid_xsd_date_time(text: &str) -> DsntkError {
    ValueError(format!("'{text}' is not valid xsd:dateTime representation")).into()
}
/// Error used when parsed text is not acceptable `xsd:duration` representation.
pub fn err_invalid_xsd_duration(text: &str) -> DsntkError {
    ValueError(format!("'{text}' is not valid xsd:duration representation")).into()
}

//-----------------------------------------------------------------------------
// ContextError
//-----------------------------------------------------------------------------

/// Context errors.
#[derive(ToErrorMessage)]
struct ContextError(String);

/// Creates an instance of `value is not a context` error.
pub fn err_value_is_not_a_context(value: &Value) -> DsntkError {
    ContextError(format!("'{value}' is not a value containing context")).into()
}

//-----------------------------------------------------------------------------
// BifError
//-----------------------------------------------------------------------------

/// Built-in functions errors.
#[derive(ToErrorMessage)]
struct BifError(String);

/// Creates an instance of `UnknownFunctionName` error.
pub fn err_unknown_function_name(name: &str) -> DsntkError {
    BifError(format!("unknown built-in function name: {name}")).into()
}

//-----------------------------------------------------------------------------
// DtoError
//-----------------------------------------------------------------------------

/// DTO errors.
#[derive(ToErrorMessage)]
struct DtoError(String);

pub fn err_invalid_attribute(description: &str) -> DsntkError {
    DtoError(format!("invalid attribute: {description}")).into()
}

pub fn err_missing_attribute(name: &str) -> DsntkError {
    DtoError(format!("missing attribute: {name}")).into()
}
