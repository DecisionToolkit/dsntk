//! # Unit tests for data transfer objects for values

use crate::FeelType;
use crate::dto::ValueDto;
use crate::values::Value;

fn eq(expected: &str, actual: String) {
  let s = expected
    .lines()
    .filter_map(|line| if line.trim().is_empty() { None } else { Some(line[4..].to_string()) })
    .collect::<Vec<String>>()
    .join("\n");
  assert_eq!(s, actual);
}

#[test]
fn test_simple_nil() {
  let input = r#"
    {
      "simple": {
        "isNil": true
      }
    }
  "#;
  let expected = r#"
    {
      "simple": {
        "type": null,
        "text": null,
        "isNil": true
      },
      "components": null,
      "list": null
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::Null(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_simple_string() {
  let input = r#"
    {
      "simple": {
        "type": "xsd:string",
        "text": "Hello World!",
        "isNil": false
      }
    }
  "#;
  let expected = r#"
    {
      "simple": {
        "type": "xsd:string",
        "text": "Hello World!",
        "isNil": false
      },
      "components": null,
      "list": null
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::String(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_simple_integer() {
  let input = r#"
    {
      "simple": {
        "type": "xsd:integer",
        "text": "52",
        "isNil": false
      }
    }
  "#;
  let expected = r#"
    {
      "simple": {
        "type": "xsd:decimal",
        "text": "52",
        "isNil": false
      },
      "components": null,
      "list": null
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::Number(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_simple_decimal() {
  let input = r#"
    {
      "simple": {
        "type": "xsd:decimal",
        "text": "52",
        "isNil": false
      }
    }
  "#;
  let expected = r#"
    {
      "simple": {
        "type": "xsd:decimal",
        "text": "52",
        "isNil": false
      },
      "components": null,
      "list": null
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::Number(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_simple_double() {
  let input = r#"
    {
      "simple": {
        "type": "xsd:double",
        "text": "52.1",
        "isNil": false
      }
    }
  "#;
  let expected = r#"
    {
      "simple": {
        "type": "xsd:decimal",
        "text": "52.1",
        "isNil": false
      },
      "components": null,
      "list": null
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::Number(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_simple_boolean() {
  let input = r#"
    {
      "simple": {
        "type": "xsd:boolean",
        "text": "true",
        "isNil": false
      }
    }
  "#;
  let expected = r#"
    {
      "simple": {
        "type": "xsd:boolean",
        "text": "true",
        "isNil": false
      },
      "components": null,
      "list": null
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::Boolean(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_simple_date() {
  let input = r#"
    {
      "simple": {
        "type": "xsd:date",
        "text": "2023-06-13",
        "isNil": false
      }
    }
  "#;
  let expected = r#"
    {
      "simple": {
        "type": "xsd:date",
        "text": "2023-06-13",
        "isNil": false
      },
      "components": null,
      "list": null
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::Date(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_simple_time() {
  let input = r#"
    {
      "simple": {
        "type": "xsd:time",
        "text": "14:39:14",
        "isNil": false
      }
    }
  "#;
  let expected = r#"
    {
      "simple": {
        "type": "xsd:time",
        "text": "14:39:14",
        "isNil": false
      },
      "components": null,
      "list": null
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::Time(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_simple_date_time() {
  let input = r#"
    {
      "simple": {
        "type": "xsd:dateTime",
        "text": "2023-06-13T14:39:14",
        "isNil": false
      }
    }
  "#;
  let expected = r#"
    {
      "simple": {
        "type": "xsd:dateTime",
        "text": "2023-06-13T14:39:14",
        "isNil": false
      },
      "components": null,
      "list": null
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::DateTime(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_simple_ym_duration() {
  let input = r#"
    {
      "simple": {
        "type": "xsd:duration",
        "text": "P1Y2M",
        "isNil": false
      }
    }
  "#;
  let expected = r#"
    {
      "simple": {
        "type": "xsd:duration",
        "text": "P1Y2M",
        "isNil": false
      },
      "components": null,
      "list": null
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::YearsAndMonthsDuration(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_simple_dt_duration() {
  let input = r#"
    {
      "simple": {
        "type": "xsd:duration",
        "text": "P2DT3H2M12S",
        "isNil": false
      }
    }
  "#;
  let expected = r#"
    {
      "simple": {
        "type": "xsd:duration",
        "text": "P2DT3H2M12S",
        "isNil": false
      },
      "components": null,
      "list": null
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::DaysAndTimeDuration(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_component() {
  let input = r#"
    {
      "components": [
        {
          "name": "First name",
          "value": {
            "simple": {
              "type": "xsd:string",
              "text": "John",
              "isNil": false
            }
          },
          "isNil": false
        }
      ]
    }
  "#;
  let expected = r#"
    {
      "simple": null,
      "components": [
        {
          "name": "First name",
          "value": {
            "simple": {
              "type": "xsd:string",
              "text": "John",
              "isNil": false
            },
            "components": null,
            "list": null
          },
          "isNil": false
        }
      ],
      "list": null
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::Context(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_component_nil() {
  let input = r#"
    {
      "components": [
        {
          "name": "First name",
          "isNil": true
        }
      ]
    }
  "#;
  let expected = r#"
    {
      "simple": null,
      "components": [
        {
          "name": "First name",
          "value": {
            "simple": {
              "type": null,
              "text": null,
              "isNil": true
            },
            "components": null,
            "list": null
          },
          "isNil": false
        }
      ],
      "list": null
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::Context(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_list() {
  let input = r#"
    {
      "list": {
        "items": [
          {
            "simple": {
              "type": "xsd:string",
              "text": "John",
              "isNil": false
            }
          }
        ],
        "isNil": false
      }
    }
  "#;
  let expected = r#"
    {
      "simple": null,
      "components": null,
      "list": {
        "items": [
          {
            "simple": {
              "type": "xsd:string",
              "text": "John",
              "isNil": false
            },
            "components": null,
            "list": null
          }
        ],
        "isNil": false
      }
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::List(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_list_multiple_items() {
  let input = r#"
    {
      "list": {
        "items": [
          {
            "simple": {
              "type": "xsd:string",
              "text": "John",
              "isNil": false
            }
          },
          {
            "simple": {
              "type": "xsd:string",
              "text": "Andy",
              "isNil": false
            }
          }
        ],
        "isNil": false
      }
    }
  "#;
  let expected = r#"
    {
      "simple": null,
      "components": null,
      "list": {
        "items": [
          {
            "simple": {
              "type": "xsd:string",
              "text": "John",
              "isNil": false
            },
            "components": null,
            "list": null
          },
          {
            "simple": {
              "type": "xsd:string",
              "text": "Andy",
              "isNil": false
            },
            "components": null,
            "list": null
          }
        ],
        "isNil": false
      }
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::List(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_list_nil() {
  let input = r#"
    {
      "list": {
        "items": [],
        "isNil": true
      }
    }
  "#;
  let expected = r#"
    {
      "simple": {
        "type": null,
        "text": null,
        "isNil": true
      },
      "components": null,
      "list": null
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto).unwrap();
  assert!(matches!(value, Value::Null(_)));
  let output_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&output_dto).unwrap();
  eq(expected, output);
}

#[test]
fn test_component_no_name() {
  let input = r#"
    {
      "components": [
        {
          "value": {
            "simple": {
              "type": "xsd:string",
              "text": "John",
              "isNil": false
            }
          },
          "isNil": false
        }
      ]
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto);
  assert_eq!("<DtoError> invalid attribute: component must have a name", format!("{}", value.err().unwrap()));
}

#[test]
fn test_component_no_value() {
  let input = r#"
    {
      "components": [
        {
          "name": "First name",
          "value": null,
          "isNil": false
        }
      ]
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto);
  assert_eq!("<DtoError> invalid attribute: component must have a value", format!("{}", value.err().unwrap()));
}

#[test]
fn test_simple_string_invalid_type() {
  let input = r#"
    {
      "simple": {
        "type": "xsd:luminescence",
        "text": "What a light!",
        "isNil": false
      }
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto);
  assert_eq!("<DtoError> invalid attribute: invalid type 'xsd:luminescence'", format!("{}", value.err().unwrap()));
}

#[test]
fn test_simple_string_no_type() {
  let input = r#"
    {
      "simple": {
        "type": null,
        "text": "hello",
        "isNil": false
      }
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto);
  assert_eq!("<DtoError> missing attribute: simple value must have 'type' attribute", format!("{}", value.err().unwrap()));
}

#[test]
fn test_simple_string_no_value() {
  let input = r#"
    {
      "simple": {
        "type": "xsd:string",
        "text": null,
        "isNil": false
      }
    }
  "#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto);
  assert_eq!("<DtoError> missing attribute: simple value must have 'text' attribute", format!("{}", value.err().unwrap()));
}

#[test]
fn test_invalid_value() {
  let input = r#"{}"#;
  let input_dto = serde_json::from_str::<ValueDto>(input).unwrap();
  let value = Value::try_from(&input_dto);
  assert_eq!(
    "<DtoError> missing attribute: no 'simple', 'components' or 'list' attribute",
    format!("{}", value.err().unwrap())
  );
}

#[test]
fn test_invalid_value_type() {
  let expected = r#"
    {
      "simple": null,
      "components": null,
      "list": null
    }
  "#;
  let value = Value::FeelType(FeelType::Any);
  let value_dto = ValueDto::try_from(&value).unwrap();
  let output = serde_json::to_string_pretty(&value_dto).unwrap();
  eq(expected, output);
}
