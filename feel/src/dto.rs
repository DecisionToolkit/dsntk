//! # Data transfer objects for FEEL values

use crate::context::FeelContext;
use crate::errors::*;
use crate::values::Value;
use crate::{Name, value_null};
use dsntk_common::DsntkError;
use serde::{Deserialize, Serialize};

pub const XSD_STRING: &str = "xsd:string";
pub const XSD_INTEGER: &str = "xsd:integer";
pub const XSD_DECIMAL: &str = "xsd:decimal";
pub const XSD_DOUBLE: &str = "xsd:double";
pub const XSD_BOOLEAN: &str = "xsd:boolean";
pub const XSD_DATE: &str = "xsd:date";
pub const XSD_DATE_TIME: &str = "xsd:dateTime";
pub const XSD_TIME: &str = "xsd:time";
pub const XSD_DURATION: &str = "xsd:duration";

/// Type alias for an object value represented as a vector of components.
type ObjectDto = Vec<ComponentDto>;

/// DTO representing a single FEEL value.
#[derive(Default, Serialize, Deserialize)]
pub struct ValueDto {
  /// Simple value.
  #[serde(rename = "simple")]
  simple: Option<SimpleDto>,
  /// Object value.
  #[serde(rename = "components")]
  object: Option<ObjectDto>,
  /// List value.
  #[serde(rename = "list")]
  list: Option<ListDto>,
}

/// DTO representing a simple value.
#[derive(Default, Serialize, Deserialize)]
pub struct SimpleDto {
  /// Name of the value's type.
  #[serde(rename = "type")]
  typ: Option<String>,
  /// Value represented as text.
  #[serde(rename = "text")]
  text: Option<String>,
  /// Flag indicating if the value is `nil`.
  #[serde(rename = "isNil")]
  nil: bool,
}

impl SimpleDto {
  /// Creates a [SimpleDto] with initial type and value.
  fn new(typ: &str, text: String) -> Option<Self> {
    Some(Self {
      typ: Some(typ.to_string()),
      text: Some(text),
      nil: false,
    })
  }

  /// Creates a [SimpleDto] with `nil` value.
  fn new_nil() -> Option<Self> {
    Some(Self { typ: None, text: None, nil: true })
  }
}

/// DTO representing a component of the object value.
#[derive(Default, Serialize, Deserialize)]
pub struct ComponentDto {
  /// Name of the value's component.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Value of the component.
  #[serde(rename = "value")]
  value: Option<ValueDto>,
  /// Flag indicating if the value of the component is `nil`.
  #[serde(rename = "isNil")]
  nil: bool,
}

/// DTO representing a list of values.
#[derive(Default, Serialize, Deserialize)]
pub struct ListDto {
  /// Items in the list ov values.
  #[serde(rename = "items")]
  items: Vec<ValueDto>,
  /// Flag indicating if the value is of the list is `nil`.
  #[serde(rename = "isNil")]
  nil: bool,
}

impl ListDto {
  /// Creates a [ListDto] with initial items.
  fn new(items: Vec<ValueDto>) -> Option<Self> {
    Some(Self { items, nil: false })
  }
}

impl TryFrom<&Value> for ValueDto {
  type Error = DsntkError;
  /// Converts a [Value] to [ValueDto].
  fn try_from(value: &Value) -> Result<Self, Self::Error> {
    match value {
      Value::String(inner) => Ok(ValueDto {
        simple: SimpleDto::new(XSD_STRING, inner.to_string()),
        ..Default::default()
      }),
      v @ Value::Number(_) => Ok(ValueDto {
        simple: SimpleDto::new(XSD_DECIMAL, v.to_string()),
        ..Default::default()
      }),
      v @ Value::Boolean(_) => Ok(ValueDto {
        simple: SimpleDto::new(XSD_BOOLEAN, v.to_string()),
        ..Default::default()
      }),
      v @ Value::Date(_) => Ok(ValueDto {
        simple: SimpleDto::new(XSD_DATE, v.to_string()),
        ..Default::default()
      }),
      v @ Value::DateTime(_) => Ok(ValueDto {
        simple: SimpleDto::new(XSD_DATE_TIME, v.to_string()),
        ..Default::default()
      }),
      v @ Value::Time(_) => Ok(ValueDto {
        simple: SimpleDto::new(XSD_TIME, v.to_string()),
        ..Default::default()
      }),
      v @ Value::YearsAndMonthsDuration(_) => Ok(ValueDto {
        simple: SimpleDto::new(XSD_DURATION, v.to_string()),
        ..Default::default()
      }),
      v @ Value::DaysAndTimeDuration(_) => Ok(ValueDto {
        simple: SimpleDto::new(XSD_DURATION, v.to_string()),
        ..Default::default()
      }),
      Value::Null(_) => Ok(ValueDto {
        simple: SimpleDto::new_nil(),
        ..Default::default()
      }),
      Value::Context(ctx) => {
        let mut components = vec![];
        for (name, value) in ctx.iter() {
          components.push(ComponentDto {
            name: Some(name.to_string()),
            value: Some(value.try_into()?),
            nil: false,
          });
        }
        Ok(ValueDto {
          object: Some(components),
          ..Default::default()
        })
      }
      Value::List(list) => {
        let mut items = vec![];
        for value in list {
          items.push(value.try_into()?);
        }
        Ok(ValueDto {
          list: ListDto::new(items),
          ..Default::default()
        })
      }
      _ => Ok(Default::default()),
    }
  }
}

impl TryFrom<&ValueDto> for Value {
  type Error = DsntkError;
  /// Converts a [ValueDto] to [Value].
  fn try_from(value: &ValueDto) -> Result<Self, Self::Error> {
    if let Some(value_dto) = &value.simple {
      return Value::try_from(value_dto);
    }
    if let Some(components) = &value.object {
      return Value::try_from(components);
    }
    if let Some(list) = &value.list {
      return Value::try_from(list);
    }
    Err(err_missing_attribute("no 'simple', 'components' or 'list' attribute"))
  }
}

impl TryFrom<&SimpleDto> for Value {
  type Error = DsntkError;
  /// Converts [SimpleDto] into [Value].
  fn try_from(value: &SimpleDto) -> Result<Self, Self::Error> {
    if value.nil {
      return Ok(value_null!());
    }
    if let Some(typ) = &value.typ {
      if let Some(text) = &value.text {
        match typ.as_str() {
          XSD_STRING => Ok(Value::String(text.clone())),
          XSD_INTEGER => Ok(Value::try_from_xsd_integer(text)?),
          XSD_DECIMAL => Ok(Value::try_from_xsd_decimal(text)?),
          XSD_DOUBLE => Ok(Value::try_from_xsd_double(text)?),
          XSD_BOOLEAN => Ok(Value::try_from_xsd_boolean(text)?),
          XSD_DATE => Ok(Value::try_from_xsd_date(text)?),
          XSD_TIME => Ok(Value::try_from_xsd_time(text)?),
          XSD_DATE_TIME => Ok(Value::try_from_xsd_date_time(text)?),
          XSD_DURATION => Ok(Value::try_from_xsd_duration(text)?),
          _ => Err(err_invalid_attribute(&format!("invalid type '{typ}'"))),
        }
      } else {
        Err(err_missing_attribute("simple value must have 'text' attribute"))
      }
    } else {
      Err(err_missing_attribute("simple value must have 'type' attribute"))
    }
  }
}

impl TryFrom<&ObjectDto> for Value {
  type Error = DsntkError;
  fn try_from(object: &ObjectDto) -> Result<Self, Self::Error> {
    let mut ctx: FeelContext = Default::default();
    for item in object {
      let item_name = item.name.as_ref().ok_or_else(|| err_invalid_attribute("component must have a name"))?.to_string();
      let value = Value::try_from(item)?;
      let key: Name = item_name.into();
      ctx.set_entry(&key, value);
    }
    Ok(ctx.into())
  }
}

impl TryFrom<&ComponentDto> for Value {
  type Error = DsntkError;
  /// Converts a [ComponentDto] to [Value].
  fn try_from(value: &ComponentDto) -> Result<Self, Self::Error> {
    if value.nil {
      return Ok(value_null!());
    }
    if let Some(v) = &value.value {
      Value::try_from(v)
    } else {
      Err(err_invalid_attribute("component must have a value"))
    }
  }
}

impl TryFrom<&ListDto> for Value {
  type Error = DsntkError;
  /// Converts a [ListDto] to [Value].
  fn try_from(value: &ListDto) -> Result<Self, Self::Error> {
    if value.nil {
      return Ok(value_null!());
    }
    let mut values = vec![];
    for item in &value.items {
      values.push(Value::try_from(item)?);
    }
    Ok(Value::List(values))
  }
}
