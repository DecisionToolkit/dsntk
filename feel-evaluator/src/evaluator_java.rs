//! # Evaluator for Java external functions

use dsntk_feel::values::Value;
use dsntk_feel::{value_null, FeelNumber};
use std::str::FromStr;

/// Evaluates external Java function.
pub fn evaluate_external_java_function(class_name: &str, method_signature: &str, arguments: &[Value]) -> Value {
  let mut parts = method_signature.trim().split('(');
  let Some(method_name) = parts.next() else {
    return value_null!("no method name in method signature");
  };
  let Some(parameter_type_names) = parts.next() else {
    return value_null!("no parameter types in method signature");
  };
  let parameter_types: Vec<String> = parameter_type_names
    .trim()
    .trim_end_matches(')')
    .split(',')
    .filter_map(|s| if s.trim().is_empty() { None } else { Some(s.trim().to_string()) })
    .collect();
  if parameter_types.len() != arguments.len() {
    return value_null!(
      "the number of parameter types ({}) differs from the number of arguments ({})",
      parameter_types.len(),
      arguments.len()
    );
  }
  // Validate argument types against parameter types before evaluation.
  for (param_type, arg) in parameter_types.iter().zip(arguments.iter()) {
    if let Err(reason) = validate_argument_type(param_type, arg) {
      return value_null!("{}", reason);
    }
  }
  evaluate_builtin(class_name, method_name, &parameter_types, arguments)
}

/// Validates that a FEEL value can be converted to the expected Java parameter type.
fn validate_argument_type(param_type: &str, arg: &Value) -> Result<(), String> {
  match param_type {
    "double" | "float" | "int" | "long" | "short" | "byte" => match arg {
      Value::Number(_) => Ok(()),
      _ => Err(format!("simple DTO conversion to object failed, class: {param_type}, type: {}", xsd_type_name(arg))),
    },
    "java.lang.String" => match arg {
      Value::String(_) => Ok(()),
      _ => Err(format!("simple DTO conversion to object failed, class: {param_type}, type: {}", xsd_type_name(arg))),
    },
    "boolean" | "java.lang.Boolean" => match arg {
      Value::Boolean(_) => Ok(()),
      _ => Err(format!("simple DTO conversion to object failed, class: {param_type}, type: {}", xsd_type_name(arg))),
    },
    s if s.starts_with("[L") => Ok(()),
    "char" => Err(format!("simple DTO conversion to object failed, class: {param_type}, type: {}", xsd_type_name(arg))),
    _ => Ok(()),
  }
}

/// Returns the XSD type name for a FEEL value.
fn xsd_type_name(value: &Value) -> &'static str {
  match value {
    Value::Number(_) => "XSD_DECIMAL",
    Value::String(_) => "XSD_STRING",
    Value::Boolean(_) => "XSD_BOOLEAN",
    Value::Date(_) => "XSD_DATE",
    Value::DateTime(_) => "XSD_DATE_TIME",
    Value::Time(_) => "XSD_TIME",
    Value::YearsAndMonthsDuration(_) | Value::DaysAndTimeDuration(_) => "XSD_DURATION",
    _ => "UNKNOWN",
  }
}

/// Evaluates a built-in Java function.
fn evaluate_builtin(class_name: &str, method_name: &str, parameter_types: &[String], arguments: &[Value]) -> Value {
  match class_name {
    "java.lang.Math" => evaluate_java_lang_math(class_name, method_name, parameter_types, arguments),
    "java.lang.String" => evaluate_java_lang_string(class_name, method_name, parameter_types, arguments),
    "java.lang.Integer" => evaluate_java_lang_integer(class_name, method_name, parameter_types, arguments),
    "java.lang.Float" => evaluate_java_lang_float(class_name, method_name, parameter_types, arguments),
    "java.lang.Double" => evaluate_java_lang_double(class_name, method_name, parameter_types, arguments),
    "java.lang.Long" => evaluate_java_lang_long(class_name, method_name, parameter_types, arguments),
    _ => value_null!("java.lang.ClassNotFoundException: {}", class_name),
  }
}

/// Evaluates methods of `java.lang.Math`.
fn evaluate_java_lang_math(class_name: &str, method_name: &str, parameter_types: &[String], arguments: &[Value]) -> Value {
  match method_name {
    "cos" if parameter_types == ["double"] => {
      if let Value::Number(n) = &arguments[0] {
        let v: f64 = n.to_string().parse().unwrap_or(0.0);
        FeelNumber::from_str(&v.cos().to_string()).map_or_else(|_| value_null!("cos conversion failed"), Value::Number)
      } else {
        value_null!("expected number argument")
      }
    }
    "sin" if parameter_types == ["double"] => {
      if let Value::Number(n) = &arguments[0] {
        let v: f64 = n.to_string().parse().unwrap_or(0.0);
        FeelNumber::from_str(&v.sin().to_string()).map_or_else(|_| value_null!("sin conversion failed"), Value::Number)
      } else {
        value_null!("expected number argument")
      }
    }
    "sqrt" if parameter_types == ["double"] => {
      if let Value::Number(n) = &arguments[0] {
        let v: f64 = n.to_string().parse().unwrap_or(0.0);
        FeelNumber::from_str(&v.sqrt().to_string()).map_or_else(|_| value_null!("sqrt conversion failed"), Value::Number)
      } else {
        value_null!("expected number argument")
      }
    }
    "abs" if parameter_types == ["double"] => {
      if let Value::Number(n) = &arguments[0] {
        let v: f64 = n.to_string().parse().unwrap_or(0.0);
        FeelNumber::from_str(&v.abs().to_string()).map_or_else(|_| value_null!("abs conversion failed"), Value::Number)
      } else {
        value_null!("expected number argument")
      }
    }
    _ => value_null!("java.lang.NoSuchMethodException: {}.{}({})", class_name, method_name, parameter_types.join(", ")),
  }
}

/// Evaluates methods of `java.lang.String`.
fn evaluate_java_lang_string(class_name: &str, method_name: &str, parameter_types: &[String], arguments: &[Value]) -> Value {
  match method_name {
    "valueOf" if parameter_types == ["double"] => {
      if let Value::Number(n) = &arguments[0] {
        let v: f64 = n.to_string().parse().unwrap_or(0.0);
        // Java's Double.toString always includes a decimal point
        let s = if v.fract() == 0.0 && v.is_finite() { format!("{:.1}", v) } else { format!("{}", v) };
        Value::String(s)
      } else {
        value_null!("expected number argument")
      }
    }
    "format" if !parameter_types.is_empty() && parameter_types[0] == "java.lang.String" => {
      if let Value::String(fmt_str) = &arguments[0] {
        let mut result = fmt_str.clone();
        for arg in &arguments[1..] {
          let replacement = match arg {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Boolean(b) => b.to_string(),
            other => other.to_string(),
          };
          if let Some(pos) = result.find("%s") {
            result.replace_range(pos..pos + 2, &replacement);
          }
        }
        Value::String(result)
      } else {
        value_null!("expected string argument")
      }
    }
    _ => value_null!("java.lang.NoSuchMethodException: {}.{}({})", class_name, method_name, parameter_types.join(", ")),
  }
}

/// Evaluates methods of `java.lang.Integer`.
fn evaluate_java_lang_integer(class_name: &str, method_name: &str, parameter_types: &[String], arguments: &[Value]) -> Value {
  match method_name {
    "valueOf" if parameter_types == ["int"] => {
      if let Value::Number(n) = &arguments[0] {
        let v: f64 = n.to_string().parse().unwrap_or(0.0);
        let i = v as i64;
        Value::Number(FeelNumber::new(i, 0))
      } else {
        value_null!("expected number argument")
      }
    }
    "valueOf" if parameter_types == ["java.lang.String"] => {
      if let Value::String(s) = &arguments[0] {
        match s.parse::<i64>() {
          Ok(i) => Value::Number(FeelNumber::new(i, 0)),
          Err(_) => value_null!("java.lang.NumberFormatException: For input string: \"{}\"", s),
        }
      } else {
        value_null!("expected string argument")
      }
    }
    _ => value_null!("java.lang.NoSuchMethodException: {}.{}({})", class_name, method_name, parameter_types.join(", ")),
  }
}

/// Evaluates methods of `java.lang.Float`.
fn evaluate_java_lang_float(class_name: &str, method_name: &str, parameter_types: &[String], arguments: &[Value]) -> Value {
  match method_name {
    "valueOf" if parameter_types == ["java.lang.String"] => {
      if let Value::String(s) = &arguments[0] {
        match s.parse::<f64>() {
          Ok(v) => FeelNumber::from_str(&v.to_string()).map_or_else(|_| value_null!("conversion failed"), Value::Number),
          Err(_) => value_null!("java.lang.NumberFormatException: For input string: \"{}\"", s),
        }
      } else {
        value_null!("expected string argument")
      }
    }
    "valueOf" if parameter_types == ["float"] => {
      if let Value::Number(n) = &arguments[0] {
        Value::Number(*n)
      } else {
        value_null!("expected number argument")
      }
    }
    _ => value_null!("java.lang.NoSuchMethodException: {}.{}({})", class_name, method_name, parameter_types.join(", ")),
  }
}

/// Evaluates methods of `java.lang.Double`.
fn evaluate_java_lang_double(class_name: &str, method_name: &str, parameter_types: &[String], arguments: &[Value]) -> Value {
  match method_name {
    "valueOf" if parameter_types == ["java.lang.String"] => {
      if let Value::String(s) = &arguments[0] {
        match s.parse::<f64>() {
          Ok(v) => FeelNumber::from_str(&v.to_string()).map_or_else(|_| value_null!("conversion failed"), Value::Number),
          Err(_) => value_null!("java.lang.NumberFormatException: For input string: \"{}\"", s),
        }
      } else {
        value_null!("expected string argument")
      }
    }
    _ => value_null!("java.lang.NoSuchMethodException: {}.{}({})", class_name, method_name, parameter_types.join(", ")),
  }
}

/// Evaluates methods of `java.lang.Long`.
fn evaluate_java_lang_long(class_name: &str, method_name: &str, parameter_types: &[String], arguments: &[Value]) -> Value {
  match method_name {
    "valueOf" if parameter_types == ["long"] => {
      if let Value::Number(n) = &arguments[0] {
        let v: f64 = n.to_string().parse().unwrap_or(0.0);
        let i = v as i64;
        Value::Number(FeelNumber::new(i, 0))
      } else {
        value_null!("expected number argument")
      }
    }
    _ => value_null!("java.lang.NoSuchMethodException: {}.{}({})", class_name, method_name, parameter_types.join(", ")),
  }
}
