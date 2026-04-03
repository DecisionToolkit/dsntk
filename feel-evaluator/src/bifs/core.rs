//! # Core implementation of build-in functions

use crate::macros::invalid_argument_type;
use crate::{evaluate_equals, evaluate_range_literal};
use dsntk_common::DsntkError;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::{Value, Values, VALUE_FALSE, VALUE_TRUE};
use dsntk_feel::{value_null, value_number, value_string, FeelNumber, FeelScope, Name, ToFeelString};
use dsntk_feel_temporal::{DayOfWeek, DayOfYear, FeelDate, FeelDateTime, FeelDaysAndTimeDuration, FeelTime, FeelYearsAndMonthsDuration, MonthOfYear, WeekOfYear};
use regex::Regex;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::str::FromStr;
use std::sync::LazyLock;

/// Returns the absolute value of the argument.
pub fn abs(value: &Value) -> Value {
  match value {
    Value::Number(v) => Value::Number(v.abs()),
    Value::DaysAndTimeDuration(v) => Value::DaysAndTimeDuration(v.abs()),
    Value::YearsAndMonthsDuration(v) => Value::YearsAndMonthsDuration(v.abs()),
    _ => invalid_argument_type!("abs", "number", value.type_of()),
  }
}

/// Returns `true` when value1 **is after** value2.
pub fn after(value1: &Value, value2: &Value) -> Value {
  match value1 {
    Value::Number(point1) => match value2 {
      Value::Number(point2) => return Value::Boolean(point1 > point2),
      Value::Range(_, _, range_end, closed_end) => {
        if let Value::Number(point2) = range_end.borrow() {
          Value::Boolean(point1 > point2 || (point1 == point2 && !*closed_end))
        } else {
          invalid_argument_type!("after", "range of numbers", value2.type_of())
        }
      }
      _ => invalid_argument_type!("after", "number or range of numbers", value2.type_of()),
    },
    Value::Date(point1) => match value2 {
      Value::Date(point2) => return Value::Boolean(point1 > point2),
      Value::Range(_, _, range_end, closed_end) => {
        if let Value::Date(point2) = range_end.borrow() {
          Value::Boolean(point1 > point2 || (point1 == point2 && !*closed_end))
        } else {
          invalid_argument_type!("after", "range of dates", value2.type_of())
        }
      }
      _ => invalid_argument_type!("after", "date or range of dates", value2.type_of()),
    },
    Value::Time(point1) => match value2 {
      Value::Time(point2) => return Value::Boolean(point1 > point2),
      Value::Range(_, _, range_end, closed_end) => {
        if let Value::Time(point2) = range_end.borrow() {
          Value::Boolean(point1 > point2 || (point1 == point2 && !*closed_end))
        } else {
          invalid_argument_type!("after", "range of times", value2.type_of())
        }
      }
      _ => invalid_argument_type!("after", "time or range of times", value2.type_of()),
    },
    Value::DateTime(point1) => match value2 {
      Value::DateTime(point2) => return Value::Boolean(point1 > point2),
      Value::Range(_, _, range_end, closed_end) => {
        if let Value::DateTime(point2) = range_end.borrow() {
          Value::Boolean(point1 > point2 || (point1 == point2 && !*closed_end))
        } else {
          invalid_argument_type!("after", "range of date and times", value2.type_of())
        }
      }
      _ => invalid_argument_type!("after", "date and time or range of date and times", value2.type_of()),
    },
    Value::DaysAndTimeDuration(point1) => match value2 {
      Value::DaysAndTimeDuration(point2) => return Value::Boolean(point1 > point2),
      Value::Range(_, _, range_end, closed_end) => {
        if let Value::DaysAndTimeDuration(point2) = range_end.borrow() {
          Value::Boolean(point1 > point2 || (point1 == point2 && !*closed_end))
        } else {
          invalid_argument_type!("after", "range of date and time durations", value2.type_of())
        }
      }
      _ => invalid_argument_type!("after", "date and time duration or range of date and time durations", value2.type_of()),
    },
    Value::YearsAndMonthsDuration(point1) => match value2 {
      Value::YearsAndMonthsDuration(point2) => return Value::Boolean(point1 > point2),
      Value::Range(_, _, range_end, closed_end) => {
        if let Value::YearsAndMonthsDuration(point2) = range_end.borrow() {
          Value::Boolean(point1 > point2 || (point1 == point2 && !*closed_end))
        } else {
          invalid_argument_type!("after", "range of years and months durations", value2.type_of())
        }
      }
      _ => invalid_argument_type!("after", "years and months duration or range of years and months durations", value2.type_of()),
    },
    Value::Range(range1_start, closed1_start, _, _) => match range1_start.borrow() {
      Value::Number(point1) => match value2 {
        Value::Number(point2) => return Value::Boolean(point1 > point2 || (point1 == point2 && !*closed1_start)),
        Value::Range(_, _, range2_end, closed2_end) => {
          if let Value::Number(point2) = range2_end.borrow() {
            Value::Boolean(point1 > point2 || (point1 == point2 && (!*closed1_start || !*closed2_end)))
          } else {
            invalid_argument_type!("after", "range of numbers", value2.type_of())
          }
        }
        _ => invalid_argument_type!("after", "number or range of numbers", value2.type_of()),
      },
      Value::Date(point1) => match value2 {
        Value::Date(point2) => return Value::Boolean(point1 > point2 || (point1 == point2 && !*closed1_start)),
        Value::Range(_, _, range2_end, closed2_end) => {
          if let Value::Date(point2) = range2_end.borrow() {
            Value::Boolean(point1 > point2 || (point1 == point2 && (!*closed1_start || !*closed2_end)))
          } else {
            invalid_argument_type!("after", "range of dates", value2.type_of())
          }
        }
        _ => invalid_argument_type!("after", "date or range of dates", value2.type_of()),
      },
      Value::Time(point1) => match value2 {
        Value::Time(point2) => return Value::Boolean(point1 > point2 || (point1 == point2 && !*closed1_start)),
        Value::Range(_, _, range2_end, closed2_end) => {
          if let Value::Time(point2) = range2_end.borrow() {
            Value::Boolean(point1 > point2 || (point1 == point2 && (!*closed1_start || !*closed2_end)))
          } else {
            invalid_argument_type!("after", "range of times", value2.type_of())
          }
        }
        _ => invalid_argument_type!("after", "time or range of times", value2.type_of()),
      },
      Value::DateTime(point1) => match value2 {
        Value::DateTime(point2) => return Value::Boolean(point1 > point2 || (point1 == point2 && !*closed1_start)),
        Value::Range(_, _, range2_end, closed2_end) => {
          if let Value::DateTime(point2) = range2_end.borrow() {
            Value::Boolean(point1 > point2 || (point1 == point2 && (!*closed1_start || !*closed2_end)))
          } else {
            invalid_argument_type!("after", "range of date and times", value2.type_of())
          }
        }
        _ => invalid_argument_type!("after", "date and time or range of date and times", value2.type_of()),
      },
      Value::DaysAndTimeDuration(point1) => match value2 {
        Value::DaysAndTimeDuration(point2) => return Value::Boolean(point1 > point2 || (point1 == point2 && !*closed1_start)),
        Value::Range(_, _, range2_end, closed2_end) => {
          if let Value::DaysAndTimeDuration(point2) = range2_end.borrow() {
            Value::Boolean(point1 > point2 || (point1 == point2 && (!*closed1_start || !*closed2_end)))
          } else {
            invalid_argument_type!("after", "range of days and time durations", value2.type_of())
          }
        }
        _ => invalid_argument_type!("after", "days and time duration or range of days and time durations", value2.type_of()),
      },
      Value::YearsAndMonthsDuration(point1) => match value2 {
        Value::YearsAndMonthsDuration(point2) => return Value::Boolean(point1 > point2 || (point1 == point2 && !*closed1_start)),
        Value::Range(_, _, range2_end, closed2_end) => {
          if let Value::YearsAndMonthsDuration(point2) = range2_end.borrow() {
            Value::Boolean(point1 > point2 || (point1 == point2 && (!*closed1_start || !*closed2_end)))
          } else {
            invalid_argument_type!("after", "range of years and months durations", value2.type_of())
          }
        }
        _ => invalid_argument_type!("after", "years and months duration or range of years and months durations", value2.type_of()),
      },
      _ => invalid_argument_type!("after", "scalar or range of scalars", value2.type_of()),
    },
    _ => invalid_argument_type!("after", "scalar or range of scalars", value1.type_of()),
  }
}

/// Returns `false` if any item is `false`, `true` if empty or all items are true, else `null`.
pub fn all(values: &[Value]) -> Value {
  if values.is_empty() {
    return VALUE_TRUE;
  }
  for value in values {
    if let Value::Boolean(v) = value {
      if !v {
        return VALUE_FALSE;
      }
    } else {
      return value_null!();
    }
  }
  VALUE_TRUE
}

/// Returns `true` if any item is `true`, `false` if empty or all items are `false`, else `null`.
pub fn any(values: &[Value]) -> Value {
  if values.is_empty() {
    return VALUE_FALSE;
  }
  let mut has_true = false;
  let mut all_boolean = true;
  for value in values {
    match value {
      Value::Boolean(v) => {
        if *v {
          has_true = true;
        }
      }
      Value::Null(_) => return value_null!(),
      _ => all_boolean = false,
    }
  }
  match (has_true, all_boolean) {
    (false, false) => value_null!(),
    (false, true) => VALUE_FALSE,
    (true, false) => value_null!(),
    (true, true) => VALUE_TRUE,
  }
}

/// Returns new list with items appended.
pub fn append(list: &Value, values: &[Value]) -> Value {
  match list {
    Value::List(items) => {
      let mut appended = items.clone();
      for value in values {
        appended.push(value.clone());
      }
      Value::List(appended)
    }
    v @ Value::Null(_) => v.clone(),
    other => invalid_argument_type!("append", "list", other.type_of()),
  }
}

/// Returns `true` when value1 **is before** value2.
pub fn before(value1: &Value, value2: &Value) -> Value {
  match value1 {
    Value::Number(point1) => match value2 {
      Value::Number(point2) => return Value::Boolean(point1 < point2),
      Value::Range(range_start2, closed_start2, _, _) => {
        if let Value::Number(start2) = range_start2.borrow() {
          return Value::Boolean(point1 < start2 || (point1 == start2 && !*closed_start2));
        }
      }
      _ => {}
    },
    Value::Date(point1) => match value2 {
      Value::Date(point2) => return Value::Boolean(point1 < point2),
      Value::Range(range_start2, closed_start2, _, _) => {
        if let Value::Date(start2) = range_start2.borrow() {
          return Value::Boolean(point1 < start2 || (point1 == start2 && !*closed_start2));
        }
      }
      _ => {}
    },
    Value::Time(point1) => match value2 {
      Value::Time(point2) => return Value::Boolean(point1 < point2),
      Value::Range(range_start2, closed_start2, _, _) => {
        if let Value::Time(start2) = range_start2.borrow() {
          return Value::Boolean(point1 < start2 || (point1 == start2 && !*closed_start2));
        }
      }
      _ => {}
    },
    Value::DateTime(point1) => match value2 {
      Value::DateTime(point2) => return Value::Boolean(point1 < point2),
      Value::Range(range_start2, closed_start2, _, _) => {
        if let Value::DateTime(start2) = range_start2.borrow() {
          return Value::Boolean(point1 < start2 || (point1 == start2 && !*closed_start2));
        }
      }
      _ => {}
    },
    Value::DaysAndTimeDuration(point1) => match value2 {
      Value::DaysAndTimeDuration(point2) => return Value::Boolean(point1 < point2),
      Value::Range(range_start2, closed_start2, _, _) => {
        if let Value::DaysAndTimeDuration(start2) = range_start2.borrow() {
          return Value::Boolean(point1 < start2 || (point1 == start2 && !*closed_start2));
        }
      }
      _ => {}
    },
    Value::YearsAndMonthsDuration(point1) => match value2 {
      Value::YearsAndMonthsDuration(point2) => return Value::Boolean(point1 < point2),
      Value::Range(range_start2, closed_start2, _, _) => {
        if let Value::YearsAndMonthsDuration(start2) = range_start2.borrow() {
          return Value::Boolean(point1 < start2 || (point1 == start2 && !*closed_start2));
        }
      }
      _ => {}
    },
    Value::Range(_, _, range_end1, closed_end1) => match range_end1.borrow() {
      Value::Number(end1) => match value2 {
        Value::Number(point2) => return Value::Boolean(end1 < point2 || (!*closed_end1 && end1 == point2)),
        Value::Range(range_start2, closed_start2, _, _) => {
          if let Value::Number(start2) = range_start2.borrow() {
            return Value::Boolean(end1 < start2 || (end1 == start2 && (!*closed_end1 || !*closed_start2)));
          }
        }
        _ => {}
      },
      Value::Date(end1) => match value2 {
        Value::Date(point2) => return Value::Boolean(end1 < point2 || (!*closed_end1 && end1 == point2)),
        Value::Range(range_start2, closed_start2, _, _) => {
          if let Value::Date(start2) = range_start2.borrow() {
            return Value::Boolean(end1 < start2 || (end1 == start2 && (!*closed_end1 || !*closed_start2)));
          }
        }
        _ => {}
      },
      Value::Time(end1) => match value2 {
        Value::Time(point2) => return Value::Boolean(end1 < point2 || (!*closed_end1 && end1 == point2)),
        Value::Range(range_start2, closed_start2, _, _) => {
          if let Value::Time(start2) = range_start2.borrow() {
            return Value::Boolean(end1 < start2 || (end1 == start2 && (!*closed_end1 || !*closed_start2)));
          }
        }
        _ => {}
      },
      Value::DateTime(end1) => match value2 {
        Value::DateTime(point2) => return Value::Boolean(end1 < point2 || (!*closed_end1 && end1 == point2)),
        Value::Range(range_start2, closed_start2, _, _) => {
          if let Value::DateTime(start2) = range_start2.borrow() {
            return Value::Boolean(end1 < start2 || (end1 == start2 && (!*closed_end1 || !*closed_start2)));
          }
        }
        _ => {}
      },
      Value::DaysAndTimeDuration(end1) => match value2 {
        Value::DaysAndTimeDuration(point2) => return Value::Boolean(end1 < point2 || (!*closed_end1 && end1 == point2)),
        Value::Range(range_start2, closed_start2, _, _) => {
          if let Value::DaysAndTimeDuration(start2) = range_start2.borrow() {
            return Value::Boolean(end1 < start2 || (end1 == start2 && (!*closed_end1 || !*closed_start2)));
          }
        }
        _ => {}
      },
      Value::YearsAndMonthsDuration(end1) => match value2 {
        Value::YearsAndMonthsDuration(point2) => return Value::Boolean(end1 < point2 || (!*closed_end1 && end1 == point2)),
        Value::Range(range_start2, closed_start2, _, _) => {
          if let Value::YearsAndMonthsDuration(start2) = range_start2.borrow() {
            return Value::Boolean(end1 < start2 || (end1 == start2 && (!*closed_end1 || !*closed_start2)));
          }
        }
        _ => {}
      },
      _ => {}
    },
    _ => {}
  }
  invalid_argument_type!("before", "scalar or range of scalars", value1.type_of())
}

/// Returns the smallest integer >= argument.
pub fn ceiling(n: &Value, scale: &Value) -> Value {
  if let Value::Number(n) = n {
    if let Value::Number(scale) = scale {
      if let Ok(scale) = scale.try_into() {
        match n.ceiling(scale) {
          Ok(rounded) => Value::Number(rounded),
          Err(reason) => value_null!("[core::ceiling] {}", reason),
        }
      } else {
        value_null!("[core::ceiling] invalid scale: {}", scale)
      }
    } else {
      invalid_argument_type!("ceiling", "number", scale.type_of())
    }
  } else {
    invalid_argument_type!("ceiling", "number", n.type_of())
  }
}

/// Returns `true` when two point are equal or two ranges are equal.
pub fn coincides(value1: &Value, value2: &Value) -> Value {
  match value1 {
    Value::Number(point1) => {
      if let Value::Number(point2) = value2 {
        return Value::Boolean(point1 == point2);
      }
    }
    Value::Date(point1) => {
      if let Value::Date(point2) = value2 {
        return Value::Boolean(point1 == point2);
      }
    }
    Value::Time(point1) => {
      if let Value::Time(point2) = value2 {
        return Value::Boolean(point1 == point2);
      }
    }
    Value::DateTime(point1) => {
      if let Value::DateTime(point2) = value2 {
        return Value::Boolean(point1 == point2);
      }
    }
    Value::DaysAndTimeDuration(point1) => {
      if let Value::DaysAndTimeDuration(point2) = value2 {
        return Value::Boolean(point1 == point2);
      }
    }
    Value::YearsAndMonthsDuration(point1) => {
      if let Value::YearsAndMonthsDuration(point2) = value2 {
        return Value::Boolean(point1 == point2);
      }
    }
    Value::Range(range1_start, closed1_start, range1_end, closed1_end) => match (range1_start.borrow(), range1_end.borrow()) {
      (Value::Number(point1_start), Value::Number(point1_end)) => {
        if let Value::Range(range2_start, closed2_start, range2_end, closed2_end) = value2 {
          if let (Value::Number(point2_start), Value::Number(point2_end)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_start == point2_start && point1_end == point2_end && closed1_start == closed2_start && closed1_end == closed2_end);
          }
        }
      }
      (Value::Date(point1_start), Value::Date(point1_end)) => {
        if let Value::Range(range2_start, closed2_start, range2_end, closed2_end) = value2 {
          if let (Value::Date(point2_start), Value::Date(point2_end)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_start == point2_start && point1_end == point2_end && closed1_start == closed2_start && closed1_end == closed2_end);
          }
        }
      }
      (Value::Time(point1_start), Value::Time(point1_end)) => {
        if let Value::Range(range2_start, closed2_start, range2_end, closed2_end) = value2 {
          if let (Value::Time(point2_start), Value::Time(point2_end)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_start == point2_start && point1_end == point2_end && closed1_start == closed2_start && closed1_end == closed2_end);
          }
        }
      }
      (Value::DateTime(point1_start), Value::DateTime(point1_end)) => {
        if let Value::Range(range2_start, closed2_start, range2_end, closed2_end) = value2 {
          if let (Value::DateTime(point2_start), Value::DateTime(point2_end)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_start == point2_start && point1_end == point2_end && closed1_start == closed2_start && closed1_end == closed2_end);
          }
        }
      }
      (Value::DaysAndTimeDuration(point1_start), Value::DaysAndTimeDuration(point1_end)) => {
        if let Value::Range(range2_start, closed2_start, range2_end, closed2_end) = value2 {
          if let (Value::DaysAndTimeDuration(point2_start), Value::DaysAndTimeDuration(point2_end)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_start == point2_start && point1_end == point2_end && closed1_start == closed2_start && closed1_end == closed2_end);
          }
        }
      }
      (Value::YearsAndMonthsDuration(point1_start), Value::YearsAndMonthsDuration(point1_end)) => {
        if let Value::Range(range2_start, closed2_start, range2_end, closed2_end) = value2 {
          if let (Value::YearsAndMonthsDuration(point2_start), Value::YearsAndMonthsDuration(point2_end)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_start == point2_start && point1_end == point2_end && closed1_start == closed2_start && closed1_end == closed2_end);
          }
        }
      }
      _ => {}
    },
    _ => {}
  }
  invalid_argument_type!("coincides", "scalar or range of scalars", value1.type_of())
}

/// Returns new list that is a concatenation of the arguments.
pub fn concatenate(values: &[Value]) -> Value {
  let mut concatenated = vec![];
  for value in values {
    if let Value::List(items) = value {
      for item in items {
        concatenated.push(item.clone());
      }
    } else {
      return invalid_argument_type!("concatenate", "list", value.type_of());
    }
  }
  Value::List(concatenated)
}

/// Returns `true` when the input string contains the match.
pub fn contains(input_string_value: &Value, match_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(match_string) = match_string_value {
      Value::Boolean(input_string.contains(match_string))
    } else {
      invalid_argument_type!("contains", "string", match_string_value.type_of())
    }
  } else {
    invalid_argument_type!("contains", "string", input_string_value.type_of())
  }
}

pub fn context(entries: &Value) -> Value {
  match entries {
    Value::List(entries) => {
      let mut result_ctx = FeelContext::default();
      for entry in entries {
        match entry {
          Value::Context(ctx) => {
            let Some(key) = ctx.get_entry(&"key".into()) else {
              return value_null!("context: no 'key' entry in context {}", ctx);
            };
            let Some(value) = ctx.get_entry(&"value".into()) else {
              return value_null!("context: no 'value' entry in context {}", ctx);
            };
            let key: Name = match key {
              Value::String(key) => key.clone().into(),
              _ => return value_null!("context: 'key' entry is not a string, actual type is {}", key.type_of()),
            };
            if result_ctx.contains_key(&key) {
              return value_null!("context: duplicated key: {}", key);
            }
            result_ctx.set_entry(&key, value.clone());
          }
          _ => return value_null!("context: invalid entry, expected context, actual type is {}", entry.type_of()),
        }
      }
      Value::Context(result_ctx)
    }
    Value::Context(ctx) => {
      let mut result_ctx = FeelContext::default();
      let Some(key) = ctx.get_entry(&"key".into()) else {
        return value_null!("context: no 'key' entry in context {}", ctx);
      };
      let Some(value) = ctx.get_entry(&"value".into()) else {
        return value_null!("context: no 'value' entry in context {}", ctx);
      };
      let key: Name = match key {
        Value::String(key) => key.clone().into(),
        _ => return value_null!("context: 'key' entry is not a string, actual type is {}", key.type_of()),
      };
      result_ctx.set_entry(&key, value.clone());
      Value::Context(result_ctx)
    }
    null @ Value::Null(_) => null.clone(),
    other => invalid_argument_type!("context", "list or context", other.type_of()),
  }
}

pub fn context_put(context: &Value, keys: &Value, value: &Value) -> Value {
  match context {
    Value::Context(ctx) => {
      let mut result_ctx = ctx.clone();
      match keys {
        Value::String(key) => result_ctx.set_entry(&key.into(), value.clone()),
        Value::List(items) => {
          if items.is_empty() {
            return value_null!();
          }
          let mut names = vec![];
          for item in items {
            match item {
              Value::String(s) => names.push(s.into()),
              null @ Value::Null(_) => return null.clone(),
              other => return invalid_argument_type!("context put", "string", other.type_of()),
            }
          }
          if result_ctx.apply_entries(&names, value.clone()).is_err() {
            return value_null!();
          }
        }
        null @ Value::Null(_) => return null.clone(),
        other => return invalid_argument_type!("context put", "string or list<string>", other.type_of()),
      }
      Value::Context(result_ctx)
    }
    null @ Value::Null(_) => null.clone(),
    other => invalid_argument_type!("context put", "context", other.type_of()),
  }
}

pub fn context_merge(contexts: &Value) -> Value {
  match contexts {
    Value::List(items) => {
      if items.is_empty() {
        return value_null!();
      }
      let mut result_ctx = FeelContext::default();
      for item in items {
        match item {
          Value::Context(ctx) => result_ctx.zip(ctx),
          Value::Null(_) => {}
          other => return invalid_argument_type!("context merge", "context", other.type_of()),
        }
      }
      Value::Context(result_ctx)
    }
    Value::Context(ctx) => Value::Context(ctx.clone()),
    null @ Value::Null(_) => null.clone(),
    other => invalid_argument_type!("context merge", "list<context>", other.type_of()),
  }
}

/// Returns size of list, or zero if list is empty.
pub fn count(list: &Value) -> Value {
  if let Value::List(items) = list {
    Value::Number(items.len().into())
  } else {
    Value::Number(1.into())
  }
}

/// Returns date converted from string or date and time.
pub fn date_1(value: &Value) -> Value {
  match value {
    Value::String(text) => {
      if let Ok(date) = FeelDate::from_str(text) {
        Value::Date(date)
      } else {
        value_null!("[core::date] invalid date string '{}'", text)
      }
    }
    Value::Date(date) => Value::Date(date.clone()),
    Value::DateTime(date_time) => Value::Date(date_time.date()),
    _ => invalid_argument_type!("date", "string, date or date and time", value.type_of()),
  }
}

/// Returns date created from year, month and day.
pub fn date_3(year_value: &Value, month_value: &Value, day_value: &Value) -> Value {
  if let Value::Number(year) = year_value {
    if let Value::Number(month) = month_value {
      if let Value::Number(day) = day_value {
        if let Ok(date) = FeelDate::try_from((*year, *month, *day)) {
          Value::Date(date)
        } else {
          value_null!("[core::date_3] invalid date y={} m={} d={}", year, month, day)
        }
      } else {
        invalid_argument_type!("date", "number (day)", day_value.type_of())
      }
    } else {
      invalid_argument_type!("date", "number (month)", month_value.type_of())
    }
  } else {
    invalid_argument_type!("date", "number (year)", year_value.type_of())
  }
}

/// Returns date and time created from string.
pub fn date_and_time_1(value: &Value) -> Value {
  if let Value::String(text) = value {
    if let Ok(date_time) = FeelDateTime::try_from(text.as_str()) {
      return Value::DateTime(date_time);
    }
    if let Ok(date) = FeelDate::from_str(text) {
      return Value::DateTime(FeelDateTime::new(date, FeelTime::local(0, 0, 0, 0)));
    }
    value_null!("[core::date and time] invalid date or date and time '{}'", text)
  } else {
    invalid_argument_type!("date and time", "string", value.type_of())
  }
}

/// Returns date and time created from date and time.
pub fn date_and_time_2(date_value: &Value, time_value: &Value) -> Value {
  match date_value {
    Value::DateTime(date_time) => {
      if let Value::Time(time) = time_value {
        return Value::DateTime(FeelDateTime::new(date_time.date(), time.clone()));
      }
      invalid_argument_type!("date and time", "time", time_value.type_of())
    }
    Value::Date(date) => {
      if let Value::Time(time) = time_value {
        return Value::DateTime(FeelDateTime::new(date.clone(), time.clone()));
      }
      invalid_argument_type!("date and time", "time", time_value.type_of())
    }
    _ => invalid_argument_type!("date and time", "date and time or date", date_value.type_of()),
  }
}

/// Returns the day of the week according to the Gregorian calendar enumeration:
/// `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday`, `Sunday`.
pub fn day_of_week(value: &Value) -> Value {
  fn gregorian_day(opt_day_of_week: Option<DayOfWeek>) -> Value {
    if let Some(day_of_week) = opt_day_of_week {
      value_string!(day_of_week.0)
    } else {
      value_null!("[day of week] no weekday")
    }
  }
  match value {
    Value::Date(date) => gregorian_day(date.day_of_week()),
    Value::DateTime(date_time) => gregorian_day(date_time.day_of_week()),
    _ => invalid_argument_type!("day of week", "date, date and time", value.type_of()),
  }
}

/// Returns the day of the year.
pub fn day_of_year(value: &Value) -> Value {
  fn gregorian_day_of_year(opt_day_of_year: Option<DayOfYear>) -> Value {
    if let Some(day_of_year) = opt_day_of_year {
      value_number!(day_of_year)
    } else {
      value_null!("[day of year] no day of year")
    }
  }
  match value {
    Value::Date(date) => gregorian_day_of_year(date.day_of_year()),
    Value::DateTime(date_time) => gregorian_day_of_year(date_time.day_of_year()),
    _ => invalid_argument_type!("day of year", "date, date and time", value.type_of()),
  }
}

/// Returns `number` rounded to given `scale`.
pub fn decimal(number_value: &Value, scale_value: &Value) -> Value {
  if let Value::Number(number) = number_value {
    if let Value::Number(scale) = scale_value {
      let scale = &scale.trunc();
      if (-6111..6176).contains(scale) {
        Value::Number((*number).round(scale))
      } else {
        value_null!("[core::decimal] scale is out of range: {}", scale)
      }
    } else {
      value_null!("[core::decimal] scale value is not a number: {}", scale_value)
    }
  } else {
    value_null!("[core::decimal] number value is not a number: {}", number_value)
  }
}

/// Returns new list with removed duplicates.
pub fn distinct_values(value: &Value) -> Value {
  if let Value::List(items) = value {
    let mut result = vec![];
    for item in items {
      if result.iter().all(|v| !evaluate_equals(v, item)) {
        result.push(item.clone())
      }
    }
    Value::List(result)
  } else {
    invalid_argument_type!("distinct values", "list", value.type_of())
  }
}

/// Converts string value to a days and time or years and months duration.
pub fn duration(value: &Value) -> Value {
  if let Value::String(s) = value {
    if let Ok(ym_duration) = FeelYearsAndMonthsDuration::try_from(s.as_str()) {
      Value::YearsAndMonthsDuration(ym_duration)
    } else if let Ok(dt_duration) = FeelDaysAndTimeDuration::try_from(s.as_str()) {
      Value::DaysAndTimeDuration(dt_duration)
    } else {
      value_null!("duration")
    }
  } else {
    value_null!("duration")
  }
}

/// Evaluates value of the `during` function for two ranges.
macro_rules! during_rr {
  ($r1s:expr, $c1s:expr, $r1e:expr, $c1e:expr, $r2s:expr, $c2s:expr, $r2e:expr, $c2e:expr) => {
    ($r1s > $r2s || ($r1s == $r2s && (*$c1s == *$c2s || *$c2s))) && ($r1e < $r2e || ($r1e == $r2e && (*$c1e == *$c2e || *$c2e)))
  };
}

/// Returns `true` when a point is during the range or the first range is during the second.
pub fn during(value1: &Value, value2: &Value) -> Value {
  match value1 {
    Value::Number(point) => {
      if let Value::Range(range_start, closed_start, range_end, closed_end) = value2 {
        if let (Value::Number(point1), Value::Number(point2)) = (range_start.borrow(), range_end.borrow()) {
          return Value::Boolean((point > point1 || (point == point1 && *closed_start)) && (point < point2 || (point == point2 && *closed_end)));
        }
      }
    }
    Value::Date(point) => {
      if let Value::Range(range_start, closed_start, range_end, closed_end) = value2 {
        if let (Value::Date(point1), Value::Date(point2)) = (range_start.borrow(), range_end.borrow()) {
          return Value::Boolean((point > point1 || (point == point1 && *closed_start)) && (point < point2 || (point == point2 && *closed_end)));
        }
      }
    }

    Value::Time(point) => {
      if let Value::Range(range_start, closed_start, range_end, closed_end) = value2 {
        if let (Value::Time(point1), Value::Time(point2)) = (range_start.borrow(), range_end.borrow()) {
          return Value::Boolean((point > point1 || (point == point1 && *closed_start)) && (point < point2 || (point == point2 && *closed_end)));
        }
      }
    }
    Value::DateTime(point) => {
      if let Value::Range(range_start, closed_start, range_end, closed_end) = value2 {
        if let (Value::DateTime(point1), Value::DateTime(point2)) = (range_start.borrow(), range_end.borrow()) {
          return Value::Boolean((point > point1 || (point == point1 && *closed_start)) && (point < point2 || (point == point2 && *closed_end)));
        }
      }
    }
    Value::DaysAndTimeDuration(point) => {
      if let Value::Range(range_start, closed_start, range_end, closed_end) = value2 {
        if let (Value::DaysAndTimeDuration(point1), Value::DaysAndTimeDuration(point2)) = (range_start.borrow(), range_end.borrow()) {
          return Value::Boolean((point > point1 || (point == point1 && *closed_start)) && (point < point2 || (point == point2 && *closed_end)));
        }
      }
    }
    Value::YearsAndMonthsDuration(point) => {
      if let Value::Range(range_start, closed_start, range_end, closed_end) = value2 {
        if let (Value::YearsAndMonthsDuration(point1), Value::YearsAndMonthsDuration(point2)) = (range_start.borrow(), range_end.borrow()) {
          return Value::Boolean((point > point1 || (point == point1 && *closed_start)) && (point < point2 || (point == point2 && *closed_end)));
        }
      }
    }
    Value::Range(range1_start, c1s, range1_end, c1e) => match (range1_start.borrow(), range1_end.borrow()) {
      (Value::Number(r1s), Value::Number(r1e)) => {
        if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
          if let (Value::Number(r2s), Value::Number(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(during_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
        }
      }
      (Value::Date(r1s), Value::Date(r1e)) => {
        if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
          if let (Value::Date(r2s), Value::Date(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(during_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
        }
      }
      (Value::Time(r1s), Value::Time(r1e)) => {
        if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
          if let (Value::Time(r2s), Value::Time(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(during_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
        }
      }
      (Value::DateTime(r1s), Value::DateTime(r1e)) => {
        if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
          if let (Value::DateTime(r2s), Value::DateTime(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(during_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
        }
      }
      (Value::DaysAndTimeDuration(r1s), Value::DaysAndTimeDuration(r1e)) => {
        if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
          if let (Value::DaysAndTimeDuration(r2s), Value::DaysAndTimeDuration(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(during_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
        }
      }
      (Value::YearsAndMonthsDuration(r1s), Value::YearsAndMonthsDuration(r1e)) => {
        if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
          if let (Value::YearsAndMonthsDuration(r2s), Value::YearsAndMonthsDuration(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(during_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
        }
      }
      _ => {}
    },
    _ => {}
  }
  invalid_argument_type!("during", "scalar or range of scalars", value1.type_of())
}

/// Returns `true` when the input string ends with specified match string.
pub fn ends_with(input_string_value: &Value, match_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(match_string) = match_string_value {
      Value::Boolean(input_string.ends_with(match_string))
    } else {
      invalid_argument_type!("ends with", "string", match_string_value.type_of())
    }
  } else {
    invalid_argument_type!("ends with", "string", input_string_value.type_of())
  }
}

/// Returns true if number is even, false if it is odd.
pub fn even(number_value: &Value) -> Value {
  if let Value::Number(number) = number_value {
    Value::Boolean(number.even())
  } else {
    value_null!("even")
  }
}

/// Returns the Euler’s number e raised to the power of **value** given as a parameter.
pub fn exp(value: &Value) -> Value {
  if let Value::Number(num) = value {
    if let Some(n) = num.exp() {
      return Value::Number(n);
    }
  }
  value_null!("exp")
}

/// Returns `true` when point is the end of a range or ranges have the same end value.
pub fn finishes(value1: &Value, value2: &Value) -> Value {
  match value1 {
    Value::Number(point) => {
      if let Value::Range(_, _, range_end, closed_end) = value2 {
        if let Value::Number(point2) = range_end.borrow() {
          return Value::Boolean(*closed_end && point == point2);
        }
      }
    }
    Value::Date(point) => {
      if let Value::Range(_, _, range_end, closed_end) = value2 {
        if let Value::Date(point2) = range_end.borrow() {
          return Value::Boolean(*closed_end && point == point2);
        }
      }
    }
    Value::Time(point) => {
      if let Value::Range(_, _, range_end, closed_end) = value2 {
        if let Value::Time(point2) = range_end.borrow() {
          return Value::Boolean(*closed_end && point == point2);
        }
      }
    }
    Value::DateTime(point) => {
      if let Value::Range(_, _, range_end, closed_end) = value2 {
        if let Value::DateTime(point2) = range_end.borrow() {
          return Value::Boolean(*closed_end && point == point2);
        }
      }
    }
    Value::DaysAndTimeDuration(point) => {
      if let Value::Range(_, _, range_end, closed_end) = value2 {
        if let Value::DaysAndTimeDuration(point2) = range_end.borrow() {
          return Value::Boolean(*closed_end && point == point2);
        }
      }
    }
    Value::YearsAndMonthsDuration(point) => {
      if let Value::Range(_, _, range_end, closed_end) = value2 {
        if let Value::YearsAndMonthsDuration(point2) = range_end.borrow() {
          return Value::Boolean(*closed_end && point == point2);
        }
      }
    }
    Value::Range(_, _, range1_end, closed1_end) => {
      if let Value::Range(_, _, range2_end, closed2_end) = value2 {
        match range1_end.borrow() {
          Value::Number(r1_end) => {
            if let Value::Number(r2_end) = range2_end.borrow() {
              return Value::Boolean(*closed1_end == *closed2_end && r1_end == r2_end);
            }
          }
          Value::Date(r1_end) => {
            if let Value::Date(r2_end) = range2_end.borrow() {
              return Value::Boolean(*closed1_end == *closed2_end && r1_end == r2_end);
            }
          }
          Value::Time(r1_end) => {
            if let Value::Time(r2_end) = range2_end.borrow() {
              return Value::Boolean(*closed1_end == *closed2_end && r1_end == r2_end);
            }
          }
          Value::DateTime(r1_end) => {
            if let Value::DateTime(r2_end) = range2_end.borrow() {
              return Value::Boolean(*closed1_end == *closed2_end && r1_end == r2_end);
            }
          }
          Value::DaysAndTimeDuration(r1_end) => {
            if let Value::DaysAndTimeDuration(r2_end) = range2_end.borrow() {
              return Value::Boolean(*closed1_end == *closed2_end && r1_end == r2_end);
            }
          }
          Value::YearsAndMonthsDuration(r1_end) => {
            if let Value::YearsAndMonthsDuration(r2_end) = range2_end.borrow() {
              return Value::Boolean(*closed1_end == *closed2_end && r1_end == r2_end);
            }
          }
          _ => {}
        }
      } else {
        return invalid_argument_type!("finishes", "range of scalars", value2.type_of());
      }
    }
    _ => {}
  }
  invalid_argument_type!("finishes", "scalar or range of scalars", value1.type_of())
}

/// Returns `true` when two ranges or range and point have the same ending point.
pub fn finished_by(value1: &Value, value2: &Value) -> Value {
  if let Value::Range(_, _, range1_end, closed1_end) = value1 {
    match range1_end.borrow() {
      Value::Number(point1) => match value2 {
        Value::Number(point2) => Value::Boolean(*closed1_end && point1 == point2),
        Value::Range(_, _, range2_end, closed2_end) => match range2_end.borrow() {
          Value::Number(point2) => Value::Boolean(*closed1_end == *closed2_end && point1 == point2),
          _ => invalid_argument_type!("finished by", "range<number>", value2.type_of()),
        },
        _ => invalid_argument_type!("finished by", "number or range<number>", value2.type_of()),
      },
      Value::Date(point1) => match value2 {
        Value::Date(point2) => Value::Boolean(*closed1_end && point1 == point2),
        Value::Range(_, _, range2_end, closed2_end) => match range2_end.borrow() {
          Value::Date(point2) => Value::Boolean(*closed1_end == *closed2_end && point1 == point2),
          _ => invalid_argument_type!("finished by", "range<date>", value2.type_of()),
        },
        _ => invalid_argument_type!("finished by", "date or range<date>", value2.type_of()),
      },
      Value::Time(point1) => match value2 {
        Value::Time(point2) => Value::Boolean(*closed1_end && point1 == point2),
        Value::Range(_, _, range2_end, closed2_end) => match range2_end.borrow() {
          Value::Time(point2) => Value::Boolean(*closed1_end == *closed2_end && point1 == point2),
          _ => invalid_argument_type!("finished by", "range<time>", value2.type_of()),
        },
        _ => invalid_argument_type!("finished by", "time or range<time>", value2.type_of()),
      },
      Value::DateTime(point1) => match value2 {
        Value::DateTime(point2) => Value::Boolean(*closed1_end && point1 == point2),
        Value::Range(_, _, range2_end, closed2_end) => match range2_end.borrow() {
          Value::DateTime(point2) => Value::Boolean(*closed1_end == *closed2_end && point1 == point2),
          _ => invalid_argument_type!("finished by", "range<date and time>", value2.type_of()),
        },
        _ => invalid_argument_type!("finished by", "date and time or range<date and time>", value2.type_of()),
      },
      Value::DaysAndTimeDuration(point1) => match value2 {
        Value::DaysAndTimeDuration(point2) => Value::Boolean(*closed1_end && point1 == point2),
        Value::Range(_, _, range2_end, closed2_end) => match range2_end.borrow() {
          Value::DaysAndTimeDuration(point2) => Value::Boolean(*closed1_end == *closed2_end && point1 == point2),
          _ => invalid_argument_type!("finished by", "range<days and time duration>", value2.type_of()),
        },
        _ => invalid_argument_type!("finished by", "days and time duration or range<days and time duration>", value2.type_of()),
      },
      Value::YearsAndMonthsDuration(point1) => match value2 {
        Value::YearsAndMonthsDuration(point2) => Value::Boolean(*closed1_end && point1 == point2),
        Value::Range(_, _, range2_end, closed2_end) => match range2_end.borrow() {
          Value::YearsAndMonthsDuration(point2) => Value::Boolean(*closed1_end == *closed2_end && point1 == point2),
          _ => invalid_argument_type!("finished by", "range<years and months duration>", value2.type_of()),
        },
        _ => invalid_argument_type!("finished by", "years and months duration or range<years and months duration>", value2.type_of()),
      },
      _ => invalid_argument_type!("finished by", "range of scalars", value1.type_of()),
    }
  } else {
    invalid_argument_type!("finished by", "range of scalars", value1.type_of())
  }
}

/// Returns new list with flattened nested lists.
pub fn flatten(value: &Value) -> Value {
  if let Value::List(_) = value {
    let mut flattened = vec![];
    flatten_value(value, &mut flattened);
    Value::List(flattened)
  } else {
    invalid_argument_type!("flatten", "list", value.type_of())
  }
}

/// Flattens nested lists.
fn flatten_value(value: &Value, flattened: &mut Vec<Value>) {
  if let Value::List(items) = value {
    for item in items {
      if let Value::List(_) = item {
        flatten_value(item, flattened);
      } else {
        flattened.push(item.clone())
      }
    }
  }
}

/// Returns greatest **integer** <= **value** specified as a parameter.
pub fn floor(n: &Value, scale: &Value) -> Value {
  if let Value::Number(n) = n {
    if let Value::Number(scale) = scale {
      if let Ok(scale) = scale.try_into() {
        match n.floor(scale) {
          Ok(rounded) => Value::Number(rounded),
          Err(reason) => value_null!("[core::floor] {}", reason),
        }
      } else {
        value_null!("[core::floor] invalid scale: {}", scale)
      }
    } else {
      invalid_argument_type!("floor", "number", scale.type_of())
    }
  } else {
    invalid_argument_type!("floor", "number", n.type_of())
  }
}

pub fn get_entries(context: &Value) -> Value {
  if let Value::Context(ctx) = context {
    let name_key: Name = "key".into();
    let name_value: Name = "value".into();
    let mut entries = vec![];
    ctx.get_entries().iter().for_each(|(name, value)| {
      let mut key_value_pair = FeelContext::default();
      key_value_pair.set_entry(&name_key, Value::String(name.to_string()));
      key_value_pair.set_entry(&name_value, (**value).clone());
      entries.push(Value::Context(key_value_pair));
    });
    Value::List(entries)
  } else {
    invalid_argument_type!("get entries", "context", context.type_of())
  }
}

pub fn get_value(context: &Value, key: &Value) -> Value {
  if let Value::Context(ctx) = context {
    if let Value::String(entry_key) = key {
      let name = Name::from(entry_key.to_owned());
      if let Some(entry_value) = ctx.get_entry(&name) {
        entry_value.clone()
      } else {
        value_null!()
      }
    } else {
      invalid_argument_type!("get value", "string", key.type_of())
    }
  } else {
    invalid_argument_type!("get value", "context", context.type_of())
  }
}

/// Evaluates the value of the `includes` function for range and point.
macro_rules! includes_rp {
  ($rs:expr, $cs:expr, $re:expr, $ce:expr, $p:expr) => {
    ($p > $rs || (*$cs && $p >= $rs)) && ($p < $re || (*$ce && $p <= $re))
  };
}

/// Evaluates the value of the `includes` function for two ranges.
macro_rules! includes_rr {
  ($r1s:expr, $c1s:expr, $r1e:expr, $c1e:expr, $r2s:expr, $c2s:expr, $r2e:expr, $c2e:expr) => {
    ($r2s > $r1s || ((*$c1s == *$c2s || *$c1s) && $r2s == $r1s)) && ($r2e < $r1e || ((*$c1e == *$c2e || *$c1e) && $r2e == $r1e))
  };
}

pub fn includes(value1: &Value, value2: &Value) -> Value {
  if let Value::Range(range1_start, c1s, range1_end, c1e) = value1 {
    match (range1_start.borrow(), range1_end.borrow()) {
      (Value::Number(r1s), Value::Number(r1e)) => match value2 {
        Value::Range(range2_start, c2s, range2_end, c2e) => {
          if let (Value::Number(r2s), Value::Number(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(includes_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
          invalid_argument_type!("includes", "range<number>", value2.type_of())
        }
        Value::Number(point2) => Value::Boolean(includes_rp!(r1s, c1s, r1e, c1e, point2)),
        _ => invalid_argument_type!("includes", "number or range<number>", value2.type_of()),
      },
      (Value::Date(r1s), Value::Date(r1e)) => match value2 {
        Value::Range(range2_start, c2s, range2_end, c2e) => {
          if let (Value::Date(r2s), Value::Date(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(includes_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
          invalid_argument_type!("includes", "range<date>", value2.type_of())
        }
        Value::Date(point2) => Value::Boolean(includes_rp!(r1s, c1s, r1e, c1e, point2)),
        _ => invalid_argument_type!("includes", "date or range<date>", value2.type_of()),
      },
      (Value::Time(r1s), Value::Time(r1e)) => match value2 {
        Value::Range(range2_start, c2s, range2_end, c2e) => {
          if let (Value::Time(r2s), Value::Time(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(includes_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
          invalid_argument_type!("includes", "range<time>", value2.type_of())
        }
        Value::Time(point2) => Value::Boolean(includes_rp!(r1s, c1s, r1e, c1e, point2)),
        _ => invalid_argument_type!("includes", "time or range<time>", value2.type_of()),
      },
      (Value::DateTime(r1s), Value::DateTime(r1e)) => match value2 {
        Value::Range(range2_start, c2s, range2_end, c2e) => {
          if let (Value::DateTime(r2s), Value::DateTime(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(includes_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
          invalid_argument_type!("includes", "range<date and time>", value2.type_of())
        }
        Value::DateTime(point2) => Value::Boolean(includes_rp!(r1s, c1s, r1e, c1e, point2)),
        _ => invalid_argument_type!("includes", "date and time or range<date and time>", value2.type_of()),
      },
      (Value::DaysAndTimeDuration(r1s), Value::DaysAndTimeDuration(r1e)) => match value2 {
        Value::Range(range2_start, c2s, range2_end, c2e) => {
          if let (Value::DaysAndTimeDuration(r2s), Value::DaysAndTimeDuration(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(includes_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
          invalid_argument_type!("includes", "range<days and time duration>", value2.type_of())
        }
        Value::DaysAndTimeDuration(point2) => Value::Boolean(includes_rp!(r1s, c1s, r1e, c1e, point2)),
        _ => invalid_argument_type!("includes", "days and time duration or range<days and time duration>", value2.type_of()),
      },
      (Value::YearsAndMonthsDuration(r1s), Value::YearsAndMonthsDuration(r1e)) => match value2 {
        Value::Range(range2_start, c2s, range2_end, c2e) => {
          if let (Value::YearsAndMonthsDuration(r2s), Value::YearsAndMonthsDuration(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(includes_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
          invalid_argument_type!("includes", "range<years and months duration>", value2.type_of())
        }
        Value::YearsAndMonthsDuration(point2) => Value::Boolean(includes_rp!(r1s, c1s, r1e, c1e, point2)),
        _ => invalid_argument_type!("includes", "years and months duration or range<years and months duration>", value2.type_of()),
      },
      _ => invalid_argument_type!("includes", "scalar or range<scalar>", value1.type_of()),
    }
  } else {
    invalid_argument_type!("includes", "range<scalar>", value1.type_of())
  }
}

/// Return ascending list of list positions containing match.
pub fn index_of(list: &Value, element: &Value) -> Value {
  if let Value::List(items) = list {
    let mut indexes = vec![];
    for (i, item) in items.iter().enumerate() {
      if evaluate_equals(item, element) {
        indexes.push(Value::Number((i + 1).into()));
      }
    }
    Value::List(indexes)
  } else {
    invalid_argument_type!("index of", "list", list.type_of())
  }
}

/// ???
pub fn insert_before(list: &Value, position_value: &Value, new_item_value: &Value) -> Value {
  if let Value::List(mut items) = list.clone() {
    if let Value::Number(position) = position_value {
      if position.is_positive() {
        if let Ok(i) = <&FeelNumber as TryInto<usize>>::try_into(position) {
          if i <= items.len() {
            items.insert(i - 1, new_item_value.clone());
            return Value::List(items);
          }
        }
      }
      if position.is_negative() {
        if let Ok(i) = <FeelNumber as TryInto<usize>>::try_into(position.abs()) {
          if i <= items.len() {
            items.insert(items.len() - i, new_item_value.clone());
            return Value::List(items);
          }
        }
      }
    }
  }
  value_null!("index is out of range")
}

/// Returns `true` if both values are the same element in the FEEL semantic domain.
pub fn is(value1: &Value, value2: &Value) -> Value {
  match value1 {
    Value::Number(v1) => match value2 {
      Value::Number(v2) => Value::Boolean(v1 == v2),
      _ => Value::Boolean(false),
    },
    Value::String(v1) => match value2 {
      Value::String(v2) => Value::Boolean(v1 == v2),
      _ => Value::Boolean(false),
    },
    Value::Boolean(v1) => match value2 {
      Value::Boolean(v2) => Value::Boolean(v1 == v2),
      _ => Value::Boolean(false),
    },
    Value::Date(v1) => match value2 {
      Value::Date(v2) => Value::Boolean(v1 == v2),
      _ => Value::Boolean(false),
    },
    Value::Time(v1) => match value2 {
      Value::Time(v2) => Value::Boolean(v1 == v2),
      _ => Value::Boolean(false),
    },
    Value::DateTime(v1) => match value2 {
      Value::DateTime(v2) => Value::Boolean(v1.is(v2)),
      _ => Value::Boolean(false),
    },
    Value::DaysAndTimeDuration(v1) => match value2 {
      Value::DaysAndTimeDuration(v2) => Value::Boolean(v1 == v2),
      _ => Value::Boolean(false),
    },
    Value::YearsAndMonthsDuration(v1) => match value2 {
      Value::YearsAndMonthsDuration(v2) => Value::Boolean(v1 == v2),
      _ => Value::Boolean(false),
    },
    _ => invalid_argument_type!("is", "scalar", value1.type_of()),
  }
}

/// Returns `true` when the list contain the specified element.
pub fn list_contains(list: &Value, element: &Value) -> Value {
  if let Value::List(items) = list {
    for item in items {
      if evaluate_equals(item, element) {
        return VALUE_TRUE;
      }
    }
    VALUE_FALSE
  } else {
    invalid_argument_type!("list contains", "list", list.type_of())
  }
}

pub fn list_replace(list: &Value, position_or_match: &Value, new_item: &Value) -> Value {
  let mut items = match list {
    Value::List(items) => items.clone(),
    Value::Null(_) => return invalid_argument_type!("list replace", "list", list.type_of()),
    other => vec![other.clone()],
  };
  match position_or_match {
    Value::Number(position) => {
      if let Ok(index) = position.trunc().try_into() as Result<isize, _> {
        let len = items.len();
        match index.cmp(&0) {
          Ordering::Greater => {
            let index = index as usize;
            if index <= len {
              items[index - 1] = new_item.clone();
              Value::List(items)
            } else {
              value_null!("position exceeds list bounds")
            }
          }
          Ordering::Less => {
            let index = index.unsigned_abs();
            if index <= len {
              items[len - index] = new_item.clone();
              Value::List(items)
            } else {
              value_null!("position exceeds list bounds")
            }
          }
          _ => value_null!("position must not be zero"),
        }
      } else {
        value_null!("position exceeds integer range")
      }
    }
    Value::FunctionDefinition(parameters, body, false, _, closure_ctx, _) => {
      if parameters.len() != 2 {
        return value_null!("list replace: matching function must have exactly two arguments");
      }
      let mut elements = vec![];
      for item in items {
        let mut ctx = closure_ctx.clone();
        ctx.set_entry(&parameters[0].0, item.clone());
        ctx.set_entry(&parameters[1].0, new_item.clone());
        let scope: FeelScope = ctx.into();
        if let Value::Boolean(result) = body.evaluate(&scope) {
          if result {
            elements.push(new_item.clone());
          } else {
            elements.push(item)
          }
        } else {
          return value_null!("list replace: matching function must return boolean value");
        }
      }
      Value::List(elements)
    }
    _ => value_null!("position must be number or function"),
  }
}

/// Returns the natural logarithm (base `e`) of the number parameter.
pub fn log(number: &Value) -> Value {
  if let Value::Number(num) = number {
    if *num > FeelNumber::zero() {
      if let Some(num_log) = num.ln() {
        return Value::Number(num_log);
      }
    }
  }
  value_null!()
}

/// Returns lower-cased string.
pub fn lower_case(input_string_value: &Value) -> Value {
  match input_string_value {
    Value::List(items) => {
      if items.len() == 1 {
        if let Value::String(input_string) = &items[0] {
          return Value::String(input_string.to_lowercase());
        }
      }
    }
    Value::String(input_string) => return Value::String(input_string.to_lowercase()),
    _ => {}
  }
  invalid_argument_type!("lower case", "string", input_string_value.type_of())
}

/// Returns `true` when the input matches the regex pattern.
pub fn matches_2(input: &Value, pattern: &Value) -> Value {
  // The input must be a string.
  let Value::String(input) = input else {
    return invalid_argument_type!("matches", "string", input.type_of());
  };
  // The pattern must be a string.
  let Value::String(pattern) = pattern else {
    return invalid_argument_type!("matches", "string", pattern.type_of());
  };
  // Match input against the pattern.
  match dsntk_feel_regex::is_match(input, pattern) {
    Ok(value) => Value::Boolean(value),
    Err(reason) => value_null!("{}", reason),
  }
}

/// Returns `true` when the input matches the regex pattern.
pub fn matches_3(input: &Value, pattern: &Value, flags: &Value) -> Value {
  // The input must be a string.
  let Value::String(input) = input else {
    return invalid_argument_type!("matches", "string", input.type_of());
  };
  // The pattern must be a string.
  let Value::String(pattern) = pattern else {
    return invalid_argument_type!("matches", "string", pattern.type_of());
  };
  // Flags if present must be a string.
  let Value::String(flags) = flags else {
    return invalid_argument_type!("matches", "string", flags.type_of());
  };
  // Match input against the pattern with flags.
  match dsntk_feel_regex::is_match_with_flags(input, pattern, flags) {
    Ok(value) => Value::Boolean(value),
    Err(reason) => value_null!("{}", reason),
  }
}

/// Returns the maximum value in the collection of comparable values.
pub fn max(values: &[Value]) -> Value {
  if values.is_empty() {
    return value_null!();
  }
  match &values[0] {
    Value::Number(n) => {
      let mut max = *n;
      for value in values.iter().skip(1) {
        match value {
          Value::Number(v) => {
            if *v > max {
              max = *v;
            }
          }
          Value::Null(_) => {}
          other => return invalid_argument_type!("max", "number", other.type_of()),
        }
      }
      Value::Number(max)
    }
    Value::String(s) => {
      let mut max = s.clone();
      for value in values.iter().skip(1) {
        match value {
          Value::String(v) => {
            if *v > max {
              max.clone_from(v)
            }
          }
          Value::Null(_) => {}
          other => return invalid_argument_type!("max", "string", other.type_of()),
        }
      }
      Value::String(max)
    }
    other => invalid_argument_type!("max", "number, string", other.type_of()),
  }
}

/// Returns the mean of numbers.
pub fn mean(values: &[Value]) -> Value {
  if values.is_empty() {
    return value_null!();
  }
  let mut sum = FeelNumber::zero();
  for value in values {
    if let Value::Number(n) = value {
      sum += *n;
    } else {
      return invalid_argument_type!("mean", "number", value.type_of());
    }
  }
  Value::Number(sum / values.len().into())
}

/// Returns `true` when range1 `meets` range2.
pub fn meets(value1: &Value, value2: &Value) -> Value {
  if let Value::Range(range1_start, _, range1_end, closed1_end) = value1 {
    match (range1_start.borrow(), range1_end.borrow()) {
      (Value::Number(_), Value::Number(point1_end)) => {
        if let Value::Range(range2_start, closed2_start, range2_end, _) = value2 {
          if let (Value::Number(point2_start), Value::Number(_)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_end == point2_start && closed1_end == closed2_start);
          }
        }
      }
      (Value::Date(_), Value::Date(point1_end)) => {
        if let Value::Range(range2_start, closed2_start, range2_end, _) = value2 {
          if let (Value::Date(point2_start), Value::Date(_)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_end == point2_start && closed1_end == closed2_start);
          }
        }
      }
      (Value::Time(_), Value::Time(point1_end)) => {
        if let Value::Range(range2_start, closed2_start, range2_end, _) = value2 {
          if let (Value::Time(point2_start), Value::Time(_)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_end == point2_start && closed1_end == closed2_start);
          }
        }
      }
      (Value::DateTime(_), Value::DateTime(point1_end)) => {
        if let Value::Range(range2_start, closed2_start, range2_end, _) = value2 {
          if let (Value::DateTime(point2_start), Value::DateTime(_)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_end == point2_start && closed1_end == closed2_start);
          }
        }
      }
      (Value::DaysAndTimeDuration(_), Value::DaysAndTimeDuration(point1_end)) => {
        if let Value::Range(range2_start, closed2_start, range2_end, _) = value2 {
          if let (Value::DaysAndTimeDuration(point2_start), Value::DaysAndTimeDuration(_)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_end == point2_start && closed1_end == closed2_start);
          }
        }
      }
      (Value::YearsAndMonthsDuration(_), Value::YearsAndMonthsDuration(point1_end)) => {
        if let Value::Range(range2_start, closed2_start, range2_end, _) = value2 {
          if let (Value::YearsAndMonthsDuration(point2_start), Value::YearsAndMonthsDuration(_)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_end == point2_start && closed1_end == closed2_start);
          }
        }
      }
      _ => {}
    }
  }
  invalid_argument_type!("meets", "range of scalars", value1.type_of())
}

/// Returns the median of numbers.
pub fn median(values: &[Value]) -> Value {
  if values.is_empty() {
    return value_null!();
  }
  let mut list = vec![];
  for value in values {
    if let Value::Number(n) = value {
      list.push(*n);
    } else {
      return value_null!("median");
    }
  }
  list.sort_by(|x, y| x.partial_cmp(y).unwrap_or(Ordering::Equal));
  let index = values.len() / 2;
  if list.len() % 2 == 0 {
    Value::Number((list[index - 1] + list[index]) / FeelNumber::two())
  } else {
    Value::Number(list[index])
  }
}

/// Returns `true` when range2 is `met by` range1.
pub fn met_by(value1: &Value, value2: &Value) -> Value {
  if let Value::Range(range1_start, closed1_start, range1_end, _) = value1 {
    if let Value::Range(range2_start, _, range2_end, closed2_end) = value2 {
      return match (range1_start.borrow(), range1_end.borrow()) {
        (Value::Number(point1_start), Value::Number(_)) => {
          if let (Value::Number(_), Value::Number(point2_end)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_start == point2_end && closed1_start == closed2_end);
          }
          invalid_argument_type!("meets", "range<number>", value2.type_of())
        }
        (Value::Date(point1_start), Value::Date(_)) => {
          if let (Value::Date(_), Value::Date(point2_end)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_start == point2_end && closed1_start == closed2_end);
          }
          invalid_argument_type!("meets", "range<date>", value2.type_of())
        }
        (Value::Time(point1_start), Value::Time(_)) => {
          if let (Value::Time(_), Value::Time(point2_end)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_start == point2_end && closed1_start == closed2_end);
          }
          invalid_argument_type!("meets", "range<time>", value2.type_of())
        }
        (Value::DateTime(point1_start), Value::DateTime(_)) => {
          if let (Value::DateTime(_), Value::DateTime(point2_end)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_start == point2_end && closed1_start == closed2_end);
          }
          invalid_argument_type!("meets", "range<date and time>", value2.type_of())
        }
        (Value::DaysAndTimeDuration(point1_start), Value::DaysAndTimeDuration(_)) => {
          if let (Value::DaysAndTimeDuration(_), Value::DaysAndTimeDuration(point2_end)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_start == point2_end && closed1_start == closed2_end);
          }
          invalid_argument_type!("meets", "range<days and time duration>", value2.type_of())
        }
        (Value::YearsAndMonthsDuration(point1_start), Value::YearsAndMonthsDuration(_)) => {
          if let (Value::YearsAndMonthsDuration(_), Value::YearsAndMonthsDuration(point2_end)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(point1_start == point2_end && closed1_start == closed2_end);
          }
          invalid_argument_type!("meets", "range<years and months duration>", value2.type_of())
        }
        _ => invalid_argument_type!("meets", "range of scalars", value1.type_of()),
      };
    }
    return invalid_argument_type!("meets", "range of scalars", value2.type_of());
  }
  invalid_argument_type!("meets", "range of scalars", value1.type_of())
}

/// Returns the minimum value in the collection of comparable values.
pub fn min(values: &[Value]) -> Value {
  if values.is_empty() {
    return value_null!();
  }
  match &values[0] {
    Value::Number(n) => {
      let mut min = *n;
      for value in values.iter().skip(1) {
        if let Value::Number(v) = value {
          if *v < min {
            min = *v;
          }
        } else {
          return invalid_argument_type!("min", "number", value.type_of());
        }
      }
      Value::Number(min)
    }
    Value::String(s) => {
      let mut min = s.clone();
      for value in values.iter().skip(1) {
        if let Value::String(v) = value {
          if *v < min {
            min.clone_from(v)
          }
        } else {
          return invalid_argument_type!("min", "string", value.type_of());
        }
      }
      Value::String(min)
    }
    other => invalid_argument_type!("min", "number, string", other.type_of()),
  }
}

/// Returns the mode of numbers.
pub fn mode(values: &[Value]) -> Value {
  if values.is_empty() {
    return Value::List(Values::default());
  }
  // make sure all values are numbers and prepare the list of them
  let mut list = vec![];
  for value in values {
    if let Value::Number(n) = value {
      list.push(*n);
    } else {
      return invalid_argument_type!("mode", "number", value.type_of());
    }
  }
  // sort values in ascending order
  list.sort_by(|x, y| x.partial_cmp(y).unwrap_or(Ordering::Equal));
  // calculate the frequencies of the numbers
  let mut mode: Vec<(usize, FeelNumber)> = vec![];
  for x in list {
    if let Some((count, value)) = mode.pop() {
      if x == value {
        mode.push((count + 1, value));
      } else {
        mode.push((count, value));
        mode.push((1_usize, x));
      }
    } else {
      mode.push((1_usize, x));
    }
  }
  // sort frequencies in descending order, and when equal then by number in ascending order
  mode.sort_by(|x, y| match x.0.cmp(&y.0).reverse() {
    Ordering::Equal => x.1.partial_cmp(&y.1).unwrap_or(Ordering::Equal),
    other => other,
  });
  // there is minimum one element in the list, so unwrap is ok
  let max = mode.first().unwrap().0;
  // return items with maximum frequency
  Value::List(mode.iter().filter_map(|(c, v)| if *c == max { Some(Value::Number(*v)) } else { None }).collect())
}

/// Returns the remainder of the division of dividend by divisor.
pub fn modulo(dividend_value: &Value, divisor_value: &Value) -> Value {
  if let Value::Number(dividend) = *dividend_value {
    if let Value::Number(divisor) = *divisor_value {
      if divisor.abs() == FeelNumber::zero() {
        value_null!("[core::modulo] division by zero")
      } else {
        // in floor function below the scale is zero, so unwrap is safe
        Value::Number(dividend - divisor * (dividend / divisor).floor(0).unwrap())
      }
    } else {
      invalid_argument_type!("modulo", "number", divisor_value.type_of())
    }
  } else {
    invalid_argument_type!("modulo", "number", dividend_value.type_of())
  }
}

/// ???
fn gregorian_month(opt_month_of_year: Option<MonthOfYear>) -> Value {
  if let Some(month_of_year) = opt_month_of_year {
    value_string!(month_of_year.0)
  } else {
    value_null!("[month of year] no month")
  }
}

/// Returns the month of the year according to the Gregorian calendar enumeration:
/// `January`, `February`, `March`, `April`, `May`, `June`, `July`,
/// `August`, `September`, `October`, `November`, `December`.
pub fn month_of_year(value: &Value) -> Value {
  match value {
    Value::Date(date) => gregorian_month(date.month_of_year()),
    Value::DateTime(date_time) => gregorian_month(date_time.month_of_year()),
    _ => invalid_argument_type!("month of year", "date, date and time", value.type_of()),
  }
}

/// Logical negation.
pub fn not(negand: &Value) -> Value {
  if let Value::Boolean(v) = negand {
    Value::Boolean(!(*v))
  } else {
    invalid_argument_type!("not", "boolean", negand.type_of())
  }
}

/// Returns current date and time.
pub fn now() -> Value {
  Value::DateTime(FeelDateTime::now())
}

/// Converts string to a number.
/// Grouping...
pub fn number(from: &Value, grouping_separator: &Value, decimal_separator: &Value) -> Value {
  // function for converting string to Value::Number
  let convert = |value: String| match value.parse::<FeelNumber>() {
    Ok(number) => Value::Number(number),
    Err(reason) => value_null!("[core::number] {}", reason),
  };
  match from {
    Value::String(value) => {
      // prepare grouping separator from Value::String ot VALUE_NULL
      let grouping_sep = match grouping_separator {
        Value::String(s) => match s.as_str() {
          " " | "." | "," => Some((*s).clone()),
          _ => return value_null!("[core::number] grouping separator must be space, period, comma or null"),
        },
        Value::Null(_) => None,
        _ => return value_null!("[core::number] grouping separator must be space, period, comma or null"),
      };
      // prepare decimal separator from Value::String ot VALUE_NULL
      let decimal_sep = match decimal_separator {
        Value::String(s) => match s.as_str() {
          "." | "," => Some((*s).clone()),
          _ => return value_null!("[core::number] decimal separator must be period, comma or null"),
        },
        Value::Null(_) => None,
        _ => return value_null!("[core::number] decimal separator must be period, comma or null"),
      };
      // replace both separators and try to convert
      if let Some(grp_sep) = &grouping_sep {
        if let Some(dec_sep) = &decimal_sep {
          return if *grp_sep != *dec_sep {
            convert(value.replace(grp_sep, "").replace(dec_sep, "."))
          } else {
            value_null!("[core::number] decimal separator must be different from grouping separator")
          };
        }
      }
      // replace grouping separator and try to convert
      if decimal_sep.is_none() {
        if let Some(sep) = grouping_sep {
          return convert(value.replace(&sep, ""));
        }
      }
      // replace decimal separator and try to convert
      if grouping_sep.is_none() {
        if let Some(sep) = decimal_sep {
          return convert(value.replace(&sep, "."));
        }
      }
      // try to convert an input parameter without replacing
      convert(value.clone())
    }
    _ => invalid_argument_type!("number", "string", from.type_of()),
  }
}

/// Returns **true** if number is odd, **false** if it is even.
pub fn odd(value: &Value) -> Value {
  if let Value::Number(v) = value {
    Value::Boolean(v.odd())
  } else {
    invalid_argument_type!("odd", "number", value.type_of())
  }
}

/// Evaluates the value of the `overlaps` function for two ranges.
macro_rules! overlaps_rr {
  ($r1s:expr, $c1s:expr, $r1e:expr, $c1e:expr, $r2s:expr, $c2s:expr, $r2e:expr, $c2e:expr) => {
    ($r1e >= $r2s && ($r1e != $r2s || (*$c1e == *$c2s && (*$c1e || *$c2s)))) && ($r2e >= $r1s && ($r2e != $r1s || (*$c2e == *$c1s && (*$c2e || *$c1s))))
  };
}

/// Returns `true` when two ranges overlap.
pub fn overlaps(value1: &Value, value2: &Value) -> Value {
  if let Value::Range(range1_start, c1s, range1_end, c1e) = value1 {
    if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
      match (range1_start.borrow(), range1_end.borrow()) {
        (Value::Number(r1s), Value::Number(r1e)) => {
          return if let (Value::Number(r2s), Value::Number(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps", "range<number>", value2.type_of())
          }
        }
        (Value::Date(r1s), Value::Date(r1e)) => {
          return if let (Value::Date(r2s), Value::Date(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps", "range<date>", value2.type_of())
          }
        }
        (Value::Time(r1s), Value::Time(r1e)) => {
          return if let (Value::Time(r2s), Value::Time(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps", "range<time>", value2.type_of())
          }
        }
        (Value::DateTime(r1s), Value::DateTime(r1e)) => {
          return if let (Value::DateTime(r2s), Value::DateTime(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps", "range<date and time>", value2.type_of())
          }
        }
        (Value::DaysAndTimeDuration(r1s), Value::DaysAndTimeDuration(r1e)) => {
          return if let (Value::DaysAndTimeDuration(r2s), Value::DaysAndTimeDuration(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps", "range<days and time duration>", value2.type_of())
          }
        }
        (Value::YearsAndMonthsDuration(r1s), Value::YearsAndMonthsDuration(r1e)) => {
          return if let (Value::YearsAndMonthsDuration(r2s), Value::YearsAndMonthsDuration(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps", "range<years and months duration>", value2.type_of())
          }
        }
        _ => {}
      }
    }
  }
  invalid_argument_type!("overlaps", "range<scalar>", value1.type_of())
}

/// Evaluates the value of the `overlaps_after` function for two ranges.
macro_rules! overlaps_after_rr {
  ($r1s:expr, $c1s:expr, $r1e:expr, $c1e:expr, $r2s:expr, $c2s:expr, $r2e:expr, $c2e:expr) => {
    ($r2s < $r1s || ($r2s == $r1s && *$c2s && !*$c1s)) && ($r2e > $r1s || ($r2e == $r1s && *$c2e && *$c1s)) && ($r2e < $r1e || ($r2e == $r1e && (!*$c2e || *$c1e)))
  };
}

/// Returns `true` when first range overlaps the end of the second range.
pub fn overlaps_after(value1: &Value, value2: &Value) -> Value {
  if let Value::Range(range1_start, c1s, range1_end, c1e) = value1 {
    if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
      match (range1_start.borrow(), range1_end.borrow()) {
        (Value::Number(r1s), Value::Number(r1e)) => {
          return if let (Value::Number(r2s), Value::Number(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_after_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps after", "range<number>", value2.type_of())
          }
        }
        (Value::Date(r1s), Value::Date(r1e)) => {
          return if let (Value::Date(r2s), Value::Date(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_after_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps after", "range<date>", value2.type_of())
          }
        }
        (Value::Time(r1s), Value::Time(r1e)) => {
          return if let (Value::Time(r2s), Value::Time(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_after_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps after", "range<time>", value2.type_of())
          }
        }
        (Value::DateTime(r1s), Value::DateTime(r1e)) => {
          return if let (Value::DateTime(r2s), Value::DateTime(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_after_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps after", "range<date and time>", value2.type_of())
          }
        }
        (Value::DaysAndTimeDuration(r1s), Value::DaysAndTimeDuration(r1e)) => {
          return if let (Value::DaysAndTimeDuration(r2s), Value::DaysAndTimeDuration(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_after_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps after", "range<days and time duration>", value2.type_of())
          }
        }
        (Value::YearsAndMonthsDuration(r1s), Value::YearsAndMonthsDuration(r1e)) => {
          return if let (Value::YearsAndMonthsDuration(r2s), Value::YearsAndMonthsDuration(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_after_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps after", "range<years and months duration>", value2.type_of())
          }
        }
        _ => {}
      }
    }
  }
  invalid_argument_type!("overlaps", "range<scalar>", value1.type_of())
}

/// Evaluates the value of the `overlaps_before` function for two ranges.
macro_rules! overlaps_before_rr {
  ($r1s:expr, $c1s:expr, $r1e:expr, $c1e:expr, $r2s:expr, $c2s:expr, $r2e:expr, $c2e:expr) => {
    ($r1s < $r2s || ($r1s == $r2s && *$c1s && !*$c2s)) && ($r1e > $r2s || ($r1e == $r2s && *$c1e && *$c2s)) && ($r1e < $r2e || ($r1e == $r2e && (!*$c1e || *$c2e)))
  };
}

/// Returns `true` when first range overlaps the beginning of the second range.
pub fn overlaps_before(value1: &Value, value2: &Value) -> Value {
  if let Value::Range(range1_start, c1s, range1_end, c1e) = value1 {
    if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
      match (range1_start.borrow(), range1_end.borrow()) {
        (Value::Number(r1s), Value::Number(r1e)) => {
          return if let (Value::Number(r2s), Value::Number(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_before_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps before", "range<number>", value2.type_of())
          }
        }
        (Value::Date(r1s), Value::Date(r1e)) => {
          return if let (Value::Date(r2s), Value::Date(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_before_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps before", "range<date>", value2.type_of())
          }
        }
        (Value::Time(r1s), Value::Time(r1e)) => {
          return if let (Value::Time(r2s), Value::Time(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_before_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps before", "range<time>", value2.type_of())
          }
        }
        (Value::DateTime(r1s), Value::DateTime(r1e)) => {
          return if let (Value::DateTime(r2s), Value::DateTime(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_before_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps before", "range<date and time>", value2.type_of())
          }
        }
        (Value::DaysAndTimeDuration(r1s), Value::DaysAndTimeDuration(r1e)) => {
          return if let (Value::DaysAndTimeDuration(r2s), Value::DaysAndTimeDuration(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_before_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps before", "range<days and time duration>", value2.type_of())
          }
        }
        (Value::YearsAndMonthsDuration(r1s), Value::YearsAndMonthsDuration(r1e)) => {
          return if let (Value::YearsAndMonthsDuration(r2s), Value::YearsAndMonthsDuration(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            Value::Boolean(overlaps_before_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e))
          } else {
            invalid_argument_type!("overlaps before", "range<years and months duration>", value2.type_of())
          }
        }
        _ => {}
      }
    }
  }
  invalid_argument_type!("overlaps", "range<scalar>", value1.type_of())
}

/// Returns the product of numbers.
pub fn product(values: &[Value]) -> Value {
  if values.is_empty() {
    return value_null!();
  }
  let mut list = vec![];
  for value in values {
    if let Value::Number(n) = value {
      list.push(*n);
    } else {
      return invalid_argument_type!("product", "number", value.type_of());
    }
  }
  Value::Number(list.iter().fold(FeelNumber::one(), |acc, n| acc * (*n)))
}

/// Returns already parsed range or null value.
pub fn range(value: &Value) -> Value {
  match value {
    v @ Value::Null(_) => v.clone(),
    Value::String(range_literal) => {
      let scope = FeelScope::default();
      if let Ok(range) = evaluate_range_literal(&scope, range_literal) {
        if range.is_valid_range() {
          range
        } else {
          value_null!("invalid range")
        }
      } else {
        value_null!("invalid range literal")
      }
    }
    other => invalid_argument_type!("range", "string", other.type_of()),
  }
}

/// ???
pub fn remove(list: &Value, position_value: &Value) -> Value {
  if let Value::List(mut items) = list.clone() {
    if let Value::Number(position_number) = position_value {
      if position_number.is_positive() {
        if let Ok(mut index) = position_number.try_into() {
          index -= 1;
          if index < items.len() {
            items.remove(index);
            return Value::List(items);
          }
        }
      }
      if position_number.is_negative() {
        if let Ok(index) = <FeelNumber as TryInto<usize>>::try_into(position_number.abs()) {
          if index <= items.len() {
            items.remove(items.len() - index);
            return Value::List(items);
          }
        }
      }
    }
  }
  value_null!("probably index is out of range")
}

// Rust implementation is eager when parsing matching groups, so place numbers in square brackets.
static RG_REPLACE_NUM: LazyLock<Regex> = LazyLock::new(|| Regex::new("\\$([1-9][0-9]*)").unwrap());

/// ???
pub fn replace(input_string_value: &Value, pattern_string_value: &Value, replacement_string_value: &Value, flags_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(pattern_string) = pattern_string_value {
      if let Value::String(replacement_string) = replacement_string_value {
        let repl = RG_REPLACE_NUM.replace_all(replacement_string.as_str(), "$${${1}}").to_string();
        // check and use flags
        if let Value::String(flags_string) = flags_string_value {
          let mut flags = "".to_string();
          let mut flag_q = false;
          let mut clear_flag_q = false;
          for ch in flags_string.chars() {
            if ch == 'q' {
              flag_q = true;
            }
            if matches!(ch, 's' | 'm' | 'i' | 'x') {
              flags.push(ch);
              if ch != 'i' {
                clear_flag_q = true;
              }
            }
          }
          if clear_flag_q {
            flag_q = false;
          }
          let mut patt = "".to_string();
          for ch in pattern_string.chars() {
            if flag_q {
              patt.push('\\');
            }
            patt.push(ch);
          }
          if flags.is_empty() {
            if let Ok(re) = Regex::new(&patt) {
              let result = re.replace_all(input_string.as_str(), repl.as_str()).to_string();
              return Value::String(result);
            }
          } else if let Ok(re) = Regex::new(format!("(?{flags}){patt}").as_str()) {
            let result = re.replace_all(input_string.as_str(), repl.as_str()).to_string();
            return Value::String(result);
          }
        }
        // replace without any flags
        if let Ok(re) = Regex::new(pattern_string) {
          let result = re.replace_all(input_string.as_str(), repl.as_str()).to_string();
          Value::String(result)
        } else {
          value_null!("replace: invalid pattern")
        }
      } else {
        value_null!("replace: replacement must be a string")
      }
    } else {
      value_null!("replace: pattern must be a string")
    }
  } else {
    value_null!("replace: input must be a string")
  }
}

pub fn reverse(list: &Value) -> Value {
  if let Value::List(mut items) = list.clone() {
    items.reverse();
    Value::List(items)
  } else {
    invalid_argument_type!("reverse", "list", list.type_of())
  }
}

pub fn round_down(n: &Value, scale: &Value) -> Value {
  let Value::Number(n) = n else {
    return invalid_argument_type!("round down", "number", n.type_of());
  };
  let Value::Number(scale) = scale else {
    return invalid_argument_type!("round down", "number", scale.type_of());
  };
  let Ok(scale): Result<i32, DsntkError> = scale.try_into() else {
    return value_null!("[core::round_down] invalid scale: {}", scale);
  };
  match n.round_down(scale) {
    Ok(rounded) => Value::Number(rounded),
    Err(reason) => value_null!("[core::round_down] {}", reason),
  }
}

pub fn round_half_down(n: &Value, scale: &Value) -> Value {
  let Value::Number(n) = n else {
    return invalid_argument_type!("round half down", "number", n.type_of());
  };
  let Value::Number(scale) = scale else {
    return invalid_argument_type!("round half down", "number", scale.type_of());
  };
  let Ok(scale): Result<i32, DsntkError> = scale.try_into() else {
    return value_null!("[core::round_half_down] invalid scale: {}", scale);
  };
  match n.round_half_down(scale) {
    Ok(rounded) => Value::Number(rounded),
    Err(reason) => value_null!("[core::round_half_down] {}", reason),
  }
}

pub fn round_half_up(n: &Value, scale: &Value) -> Value {
  let Value::Number(n) = n else {
    return invalid_argument_type!("round half up", "number", n.type_of());
  };
  let Value::Number(scale) = scale else {
    return invalid_argument_type!("round half up", "number", scale.type_of());
  };
  let Ok(scale): Result<i32, DsntkError> = scale.try_into() else {
    return value_null!("[core::round_half_up] invalid scale: {}", scale);
  };
  match n.round_half_up(scale) {
    Ok(rounded) => Value::Number(rounded),
    Err(reason) => value_null!("[core::round_half_up] {}", reason),
  }
}

pub fn round_up(n: &Value, scale: &Value) -> Value {
  let Value::Number(n) = n else {
    return invalid_argument_type!("round up", "number", n.type_of());
  };
  let Value::Number(scale) = scale else {
    return invalid_argument_type!("round up", "number", scale.type_of());
  };
  let Ok(scale): Result<i32, DsntkError> = scale.try_into() else {
    return value_null!("[core::round_up] invalid scale: {}", scale);
  };
  match n.round_up(scale) {
    Ok(rounded) => Value::Number(rounded),
    Err(reason) => value_null!("[core::round_up] {}", reason),
  }
}

pub fn sort(list: &Value, ordering_function: &Value) -> Value {
  if let Value::List(items) = list {
    if let Value::FunctionDefinition(parameters, body, false, _, closure_ctx, _) = ordering_function {
      if parameters.len() == 2 {
        let mut elements = items.clone();
        elements.sort_by(|x, y| {
          let mut ctx = closure_ctx.clone();
          ctx.set_entry(&parameters[0].0, x.clone());
          ctx.set_entry(&parameters[1].0, y.clone());
          let scope: FeelScope = ctx.into();
          if let Value::Boolean(result) = body.evaluate(&scope) {
            if result {
              Ordering::Less
            } else {
              Ordering::Equal
            }
          } else {
            Ordering::Equal
          }
        });
        Value::List(elements)
      } else {
        value_null!("sort: ordering function should take exactly two arguments")
      }
    } else {
      value_null!("sort: expected ordering function definition as a second argument")
    }
  } else {
    value_null!("sort: expected a list of values as a first argument")
  }
}

pub fn split(input_string_value: &Value, delimiter_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(delimiter_string) = delimiter_string_value {
      if let Ok(re) = Regex::new(delimiter_string) {
        Value::List(re.split(input_string).map(|s| Value::String(s.to_string())).collect())
      } else {
        value_null!("split: invalid delimiter")
      }
    } else {
      value_null!("split: delimiter must be a string")
    }
  } else {
    value_null!("split: input must be a string")
  }
}

/// Returns the square root of the given [Value].
///
/// When the given number is negative, this function returns [Value::Null].
pub fn sqrt(value: &Value) -> Value {
  if let Value::Number(v) = value {
    if *v >= FeelNumber::zero() {
      if let Some(result) = v.sqrt() {
        Value::Number(result)
      } else {
        value_null!("sqrt: result is not a finite number")
      }
    } else {
      value_null!("sqrt: argument must be positive number or zero")
    }
  } else {
    value_null!("sqrt: argument must be a number")
  }
}

/// Evaluates the value of the `started by` function for range and point.
macro_rules! started_by_rp {
  ($rs:expr, $cs:expr, $p:expr) => {
    *$cs && $rs == $p
  };
}

/// Evaluates the value of the `started by` function for two ranges.
macro_rules! started_by_rr {
  ($r1s:expr, $c1s:expr, $r2s:expr, $c2s:expr) => {
    $c1s == $c2s && $r1s == $r2s
  };
}

/// Returns `true` when range is _started_ by point or other range.
pub fn started_by(value1: &Value, value2: &Value) -> Value {
  if let Value::Range(range1_start, c1s, _, _) = value1 {
    match range1_start.borrow() {
      Value::Number(r1s) => match value2 {
        Value::Range(range2_start, c2s, _, _) => {
          if let Value::Number(r2s) = range2_start.borrow() {
            return Value::Boolean(started_by_rr!(r1s, c1s, r2s, c2s));
          }
          invalid_argument_type!("started by", "range<number>", value2.type_of())
        }
        Value::Number(point2) => Value::Boolean(started_by_rp!(r1s, c1s, point2)),
        _ => invalid_argument_type!("started by", "number or range<number>", value2.type_of()),
      },
      Value::Date(r1s) => match value2 {
        Value::Range(range2_start, c2s, _, _) => {
          if let Value::Date(r2s) = range2_start.borrow() {
            return Value::Boolean(started_by_rr!(r1s, c1s, r2s, c2s));
          }
          invalid_argument_type!("started by", "range<date>", value2.type_of())
        }
        Value::Date(point2) => Value::Boolean(started_by_rp!(r1s, c1s, point2)),
        _ => invalid_argument_type!("started by", "date or range<date>", value2.type_of()),
      },
      Value::Time(r1s) => match value2 {
        Value::Range(range2_start, c2s, _, _) => {
          if let Value::Time(r2s) = range2_start.borrow() {
            return Value::Boolean(started_by_rr!(r1s, c1s, r2s, c2s));
          }
          invalid_argument_type!("started by", "range<time>", value2.type_of())
        }
        Value::Time(point2) => Value::Boolean(started_by_rp!(r1s, c1s, point2)),
        _ => invalid_argument_type!("started by", "time or range<time>", value2.type_of()),
      },
      Value::DateTime(r1s) => match value2 {
        Value::Range(range2_start, c2s, _, _) => {
          if let Value::DateTime(r2s) = range2_start.borrow() {
            return Value::Boolean(started_by_rr!(r1s, c1s, r2s, c2s));
          }
          invalid_argument_type!("started by", "range<date and time>", value2.type_of())
        }
        Value::DateTime(point2) => Value::Boolean(started_by_rp!(r1s, c1s, point2)),
        _ => invalid_argument_type!("started by", "date and time or range<date and time>", value2.type_of()),
      },
      Value::DaysAndTimeDuration(r1s) => match value2 {
        Value::Range(range2_start, c2s, _, _) => {
          if let Value::DaysAndTimeDuration(r2s) = range2_start.borrow() {
            return Value::Boolean(started_by_rr!(r1s, c1s, r2s, c2s));
          }
          invalid_argument_type!("started by", "range<days and time duration>", value2.type_of())
        }
        Value::DaysAndTimeDuration(point2) => Value::Boolean(started_by_rp!(r1s, c1s, point2)),
        _ => invalid_argument_type!("started by", "days and time duration or range<days and time duration>", value2.type_of()),
      },
      Value::YearsAndMonthsDuration(r1s) => match value2 {
        Value::Range(range2_start, c2s, _, _) => {
          if let Value::YearsAndMonthsDuration(r2s) = range2_start.borrow() {
            return Value::Boolean(started_by_rr!(r1s, c1s, r2s, c2s));
          }
          invalid_argument_type!("started by", "range<years and months duration>", value2.type_of())
        }
        Value::YearsAndMonthsDuration(point2) => Value::Boolean(started_by_rp!(r1s, c1s, point2)),
        _ => invalid_argument_type!("started by", "years and months duration or range<years and months duration>", value2.type_of()),
      },
      _ => invalid_argument_type!("started by", "scalar or range<scalar>", value1.type_of()),
    }
  } else {
    invalid_argument_type!("started by", "range<scalar>", value1.type_of())
  }
}

/// Evaluates the value of the `starts` function for point and range.
macro_rules! starts_pr {
  ($r1s:expr, $c1s:expr, $p:expr) => {
    $r1s == $p && *$c1s
  };
}

/// Evaluates the value of the `starts` function for two ranges.
macro_rules! starts_rr {
  ($r1s:expr, $c1s:expr, $r1e:expr, $c1e:expr, $r2s:expr, $c2s:expr, $r2e:expr, $c2e:expr) => {
    $r1s == $r2s && *$c1s == *$c2s && ($r1e < $r2e || ($r1e == $r2e && (!*$c1e || *$c2e)))
  };
}

pub fn starts(value1: &Value, value2: &Value) -> Value {
  match value1 {
    Value::Number(point) => {
      if let Value::Range(range_start, c1s, _, _) = value2 {
        if let Value::Number(r1s) = range_start.borrow() {
          return Value::Boolean(starts_pr!(r1s, c1s, point));
        }
      }
    }
    Value::Date(point) => {
      if let Value::Range(range_start, c1s, _, _) = value2 {
        if let Value::Date(r1s) = range_start.borrow() {
          return Value::Boolean(starts_pr!(r1s, c1s, point));
        }
      }
    }

    Value::Time(point) => {
      if let Value::Range(range_start, c1s, _, _) = value2 {
        if let Value::Time(r1s) = range_start.borrow() {
          return Value::Boolean(starts_pr!(r1s, c1s, point));
        }
      }
    }
    Value::DateTime(point) => {
      if let Value::Range(range_start, c1s, _, _) = value2 {
        if let Value::DateTime(r1s) = range_start.borrow() {
          return Value::Boolean(starts_pr!(r1s, c1s, point));
        }
      }
    }
    Value::DaysAndTimeDuration(point) => {
      if let Value::Range(range_start, c1s, _, _) = value2 {
        if let Value::DaysAndTimeDuration(r1s) = range_start.borrow() {
          return Value::Boolean(starts_pr!(r1s, c1s, point));
        }
      }
    }
    Value::YearsAndMonthsDuration(point) => {
      if let Value::Range(range_start, c1s, _, _) = value2 {
        if let Value::YearsAndMonthsDuration(r1s) = range_start.borrow() {
          return Value::Boolean(starts_pr!(r1s, c1s, point));
        }
      }
    }
    Value::Range(range1_start, c1s, range1_end, c1e) => match (range1_start.borrow(), range1_end.borrow()) {
      (Value::Number(r1s), Value::Number(r1e)) => {
        if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
          if let (Value::Number(r2s), Value::Number(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(starts_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
        }
      }
      (Value::Date(r1s), Value::Date(r1e)) => {
        if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
          if let (Value::Date(r2s), Value::Date(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(starts_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
        }
      }
      (Value::Time(r1s), Value::Time(r1e)) => {
        if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
          if let (Value::Time(r2s), Value::Time(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(starts_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
        }
      }
      (Value::DateTime(r1s), Value::DateTime(r1e)) => {
        if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
          if let (Value::DateTime(r2s), Value::DateTime(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(starts_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
        }
      }
      (Value::DaysAndTimeDuration(r1s), Value::DaysAndTimeDuration(r1e)) => {
        if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
          if let (Value::DaysAndTimeDuration(r2s), Value::DaysAndTimeDuration(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(starts_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
        }
      }
      (Value::YearsAndMonthsDuration(r1s), Value::YearsAndMonthsDuration(r1e)) => {
        if let Value::Range(range2_start, c2s, range2_end, c2e) = value2 {
          if let (Value::YearsAndMonthsDuration(r2s), Value::YearsAndMonthsDuration(r2e)) = (range2_start.borrow(), range2_end.borrow()) {
            return Value::Boolean(starts_rr!(r1s, c1s, r1e, c1e, r2s, c2s, r2e, c2e));
          }
        }
      }
      _ => {}
    },
    _ => {}
  }
  invalid_argument_type!("starts", "scalar or range of scalars", value1.type_of())
}

/// Returns `true` when the input string starts with specified match string.
pub fn starts_with(input_string_value: &Value, match_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(match_string) = match_string_value {
      Value::Boolean(input_string.starts_with(match_string))
    } else {
      invalid_argument_type!("starts with", "string", match_string_value.type_of())
    }
  } else {
    invalid_argument_type!("starts with", "string", input_string_value.type_of())
  }
}

/// Returns the sample standard deviation of the list of numbers.
pub fn stddev(values: &[Value]) -> Value {
  if values.len() < 2 {
    return value_null!("stddev: minimum two input arguments expected");
  }
  let mut sum = FeelNumber::zero();
  let mut numbers = vec![];
  for value in values {
    if let Value::Number(x) = *value {
      sum += x;
      numbers.push(x);
    } else {
      return value_null!("stddev: expected number, actual type is {} with value {}", value.type_of(), value);
    }
  }
  let n: FeelNumber = numbers.len().into();
  let avg = sum / n;
  let mut sum2 = FeelNumber::zero();
  for number in numbers {
    if let Some(square) = (number - avg).square() {
      sum2 += square;
    } else {
      return value_null!("stddev: intermediate result is not a finite number");
    }
  }
  if let Some(stddev) = (sum2 / (n - FeelNumber::one())).sqrt() {
    Value::Number(stddev)
  } else {
    value_null!("stddev: result is not a finite number")
  }
}

/// Converts specified value to string.
pub fn string(value: &Value) -> Value {
  match value {
    Value::Null(_) => value_null!(),
    Value::String(s) => Value::String(s.clone()),
    other => Value::String(other.to_feel_string()),
  }
}

/// Returns the number of characters in string.
pub fn string_join(list: &Value, delimiter: &Value) -> Value {
  match list {
    Value::List(items) => {
      let delimiter = match delimiter {
        Value::String(delimiter) => delimiter.clone(),
        Value::Null(_) => "".to_string(),
        _ => return value_null!("string join: invalid delimiter, expected string, actual value type is {}", delimiter.type_of()),
      };
      let mut joined_string = String::new();
      for (i, item) in items.iter().enumerate() {
        match item {
          Value::String(s) => {
            if i > 0 {
              joined_string.push_str(&delimiter);
            }
            joined_string.push_str(s);
          }
          Value::Null(_) => {}
          other => return invalid_argument_type!("string join", "string", other.type_of()),
        }
      }
      Value::String(joined_string)
    }
    Value::String(string) => Value::String(string.clone()),
    other => value_null!("string join: expected list or string, actual value type is {}", other.type_of()),
  }
}

/// Returns the number of characters in string.
pub fn string_length(input_string_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    Value::Number(input_string.chars().count().into())
  } else {
    value_null!("string length: expected string as an argument")
  }
}

/// Returns the sum of values in the collection of numbers.
pub fn sum(values: &[Value]) -> Value {
  if values.is_empty() {
    return value_null!();
  }
  if let Value::Number(n) = values[0] {
    let mut sum = n;
    for value in values.iter().skip(1) {
      if let Value::Number(v) = *value {
        sum += v;
      } else {
        return invalid_argument_type!("sum", "number", value.type_of());
      }
    }
    Value::Number(sum)
  } else {
    invalid_argument_type!("sum", "number", values[0].type_of())
  }
}

/// Returns list of all elements of list, starting with specified position, 1st position is 1, last position is -1.
pub fn sublist2(list: &Value, position_value: &Value) -> Value {
  if let Value::List(items) = list {
    if let Value::Number(position_number) = position_value {
      if position_number.is_positive() {
        if let Ok(position) = <&FeelNumber as TryInto<usize>>::try_into(position_number) {
          let index = position - 1;
          if index < items.len() {
            Value::List(items[index..].to_vec())
          } else {
            value_null!("sublist: position is out of range, len = {}, position = {}", items.len(), position)
          }
        } else {
          value_null!("sublist: invalid position value: {}", position_value)
        }
      } else if position_number.is_negative() {
        if let Ok(position) = <FeelNumber as TryInto<usize>>::try_into(position_number.abs()) {
          let index = position;
          if index <= items.len() {
            Value::List(items[items.len() - index..].to_vec())
          } else {
            value_null!("sublist: position is out of range, len = {}, position = -{}", items.len(), position)
          }
        } else {
          value_null!("sublist: invalid position value: {}", position_value)
        }
      } else {
        value_null!("sublist: position must not be zero")
      }
    } else {
      value_null!("sublist: expected number, actual position value type is {}", position_value.type_of())
    }
  } else {
    value_null!("sublist: expected list, actual value type is {}", list.type_of())
  }
}

/// Returns list of specified length of elements of list, starting with specified position, 1st position is 1, last position is -1.
pub fn sublist3(list: &Value, position_value: &Value, length_value: &Value) -> Value {
  if let Value::List(items) = list {
    if let Value::Number(length_number) = length_value {
      if let Ok(length) = <&FeelNumber as TryInto<usize>>::try_into(length_number) {
        if let Value::Number(position_number) = position_value {
          if position_number.is_positive() {
            if let Ok(position) = <&FeelNumber as TryInto<usize>>::try_into(position_number) {
              let first = position - 1;
              let last = first + length;
              if first < items.len() && last <= items.len() {
                Value::List(items[first..last].to_vec())
              } else {
                value_null!("sublist: invalid range, len = {}, start position = {}, end position = {}", items.len(), first + 1, last + 1)
              }
            } else {
              value_null!("sublist: invalid position value: {}", position_value)
            }
          } else if position_number.is_negative() {
            if let Ok(position) = <FeelNumber as TryInto<usize>>::try_into(position_number.abs()) {
              let first = items.len() - position;
              let last = first + length;
              if first < items.len() && last <= items.len() {
                Value::List(items[first..last].to_vec())
              } else {
                value_null!("sublist: invalid range, len = {}, start position = {}, end position = {}", items.len(), first + 1, last + 1)
              }
            } else {
              value_null!("sublist: invalid position value: {}", position_value)
            }
          } else {
            value_null!("sublist: position must not be zero")
          }
        } else {
          value_null!("sublist: expected number, actual position value type is {}", position_value.type_of())
        }
      } else {
        value_null!("sublist: invalid length value: {}", length_value)
      }
    } else {
      value_null!("sublist: expected number, actual length value type is {}", length_value.type_of())
    }
  } else {
    value_null!("sublist: expected list, actual value type is {}", list.type_of())
  }
}

/// Returns `length` (or all) characters from string, starting at
/// `start_position`. First position is 1, last position is -1.
pub fn substring(input_string_value: &Value, start_position_value: &Value, length_value: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::Number(start_position_number) = start_position_value {
      let Ok(position): Result<isize, DsntkError> = start_position_number.try_into() else {
        return value_null!("substring: invalid start position value: {}", start_position_number);
      };
      let input_string_len = input_string.chars().count();
      match length_value {
        Value::Number(length_number) => {
          if *length_number < FeelNumber::one() {
            return value_null!("substring: length is less than 1");
          }
          let Ok(length): Result<usize, DsntkError> = length_number.trunc().try_into() else {
            return value_null!("substring: invalid length value: {}", length_number);
          };
          match position.cmp(&0) {
            Ordering::Greater => {
              let first = (position - 1) as usize;
              let last = first + length;
              if first < input_string_len && last <= input_string_len {
                Value::String(input_string.chars().skip(first).take(length).collect())
              } else {
                value_null!(
                  "sublist: invalid range, len = {}, start position = {}, end position = {}",
                  input_string_len,
                  first + 1,
                  last + 1
                )
              }
            }
            Ordering::Less => {
              let first = (input_string_len as isize) + position;
              let last = first + length as isize;
              if first >= 0 && (first as usize) < input_string_len && (last as usize) <= input_string_len {
                Value::String(input_string.chars().skip(first as usize).take(length).collect())
              } else {
                value_null!(
                  "sublist: invalid range, len = {}, start position = {}, end position = {}",
                  input_string_len,
                  first + 1,
                  last + 1
                )
              }
            }
            Ordering::Equal => value_null!("substring: start position must not be zero"),
          }
        }
        Value::Null(_) => match position.cmp(&0) {
          Ordering::Greater => {
            let index = (position - 1) as usize;
            if index < input_string_len {
              Value::String(input_string.chars().skip(index).collect())
            } else {
              value_null!("substring: position is out of range, len = {}, position = {}", input_string_len, start_position_number)
            }
          }
          Ordering::Less => {
            let index = (input_string_len as isize) + position;
            if index >= 0 {
              Value::String(input_string.chars().skip(index as usize).collect())
            } else {
              value_null!("substring: position is out of range, len = {}, position = {}", input_string_len, start_position_number)
            }
          }
          Ordering::Equal => value_null!("substring: start position must not be zero"),
        },
        other => {
          value_null!("sublist: expected number, actual length type is {}", other.type_of())
        }
      }
    } else {
      value_null!("sublist: expected number, actual start position type is {}", start_position_value.type_of())
    }
  } else {
    value_null!("sublist: expected string, actual value type is {}", input_string_value.type_of())
  }
}

/// Returns substring of `input_string_value` after the `match_input_string` in string.
pub fn substring_after(input_string_value: &Value, match_input_string: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(match_string) = match_input_string {
      if let Some(index) = input_string.find(match_string) {
        Value::String(input_string[match_string.len() + index..].to_string())
      } else {
        Value::String("".to_string())
      }
    } else {
      value_null!("substring after: expected string, actual match type is: {}", match_input_string.type_of())
    }
  } else {
    value_null!("substring after: expected string, actual input type is: {}", input_string_value.type_of())
  }
}

/// Returns substring of `input_string_value` before the `match_input_string` in string.
pub fn substring_before(input_string_value: &Value, match_input_string: &Value) -> Value {
  if let Value::String(input_string) = input_string_value {
    if let Value::String(match_string) = match_input_string {
      if let Some(index) = input_string.find(match_string) {
        Value::String(input_string[..index].to_string())
      } else {
        Value::String("".to_string())
      }
    } else {
      value_null!("substring before: expected string, actual match type is: {}", match_input_string.type_of())
    }
  } else {
    value_null!("substring before: expected string, actual input type is: {}", input_string_value.type_of())
  }
}

pub fn time_1(value: &Value) -> Value {
  match value {
    Value::String(text) => {
      if let Ok(time) = text.parse::<FeelTime>() {
        return Value::Time(time);
      }
    }
    Value::Date(_) => return Value::Time(FeelTime::utc(0, 0, 0, 0)),
    Value::DateTime(date_time) => return Value::Time(date_time.time()),
    Value::Time(time) => return Value::Time(time.clone()),
    _ => {}
  }
  value_null!("time_1")
}

pub fn time_3(hour_value: &Value, minute_value: &Value, second_value: &Value) -> Value {
  if let Value::Number(hour) = hour_value {
    if let Value::Number(minute) = minute_value {
      if let Value::Number(second) = second_value {
        if (0..24).contains(hour) {
          if (0..60).contains(minute) {
            if (0..60).contains(second) {
              // unwraps below are safe, value ranges are checked above
              let seconds = second.trunc();
              let nanoseconds = (second.frac() * FeelNumber::billion()).trunc();
              let h = hour.try_into().unwrap();
              let m = minute.try_into().unwrap();
              let s = seconds.try_into().unwrap();
              let n = nanoseconds.try_into().unwrap();
              let feel_time = FeelTime::local_opt(h, m, s, n).unwrap();
              Value::Time(feel_time)
            } else {
              value_null!("second must be 0..59, current value is {}", second)
            }
          } else {
            value_null!("minute must be 0..59, current value is {}", minute)
          }
        } else {
          value_null!("hour must be 0..23, current value is {}", hour)
        }
      } else {
        value_null!("seconds must be a number, current type is {}", second_value.type_of())
      }
    } else {
      value_null!("minutes must be a number, current type is {}", minute_value.type_of())
    }
  } else {
    value_null!("hour must be a number, current type is {}", hour_value.type_of())
  }
}

pub fn time_4(hour_value: &Value, minute_value: &Value, second_value: &Value, offset_value: &Value) -> Value {
  if let Value::Number(hour) = hour_value {
    if let Value::Number(minute) = minute_value {
      if let Value::Number(second) = second_value {
        if (0..24).contains(hour) {
          if (0..60).contains(minute) {
            if (0..60).contains(second) {
              let seconds = second.trunc();
              // unwraps below are safe, value ranges are checked above
              let h = hour.try_into().unwrap();
              let m = minute.try_into().unwrap();
              let s = seconds.try_into().unwrap();
              let n = (second.frac() * FeelNumber::billion()).trunc().try_into().unwrap();
              match offset_value {
                Value::DaysAndTimeDuration(offset) => Value::Time(FeelTime::offset_opt(h, m, s, n, offset.as_seconds() as i32).unwrap()),
                Value::Null(_) => Value::Time(FeelTime::local_opt(h, m, s, n).unwrap()),
                _ => value_null!("expected days and time duration or null, current offset type is {}", offset_value.type_of()),
              }
            } else {
              value_null!("core", "time_4", "second must be 0..59, current value is: {}", second)
            }
          } else {
            value_null!("core", "time_4", "minute must be 0..59, current value is: {}", minute)
          }
        } else {
          value_null!("core", "time_4", "hour must be 0..23, current value is: {}", hour)
        }
      } else {
        value_null!("core", "time_4", "seconds must be a number, current type is: {}", second_value.type_of())
      }
    } else {
      value_null!("core", "time_4", "minutes must be a number, current type is: {}", minute_value.type_of())
    }
  } else {
    value_null!("core", "time_4", "hour must be a number, current type is: {}", hour_value.type_of())
  }
}

/// Returns current date.
pub fn today() -> Value {
  Value::Date(FeelDate::today())
}

/// Returns new list containing concatenated list with duplicates removed.
pub fn union(lists: &[Value]) -> Value {
  let mut result = vec![];
  for list in lists {
    if let Value::List(items) = list {
      for item in items {
        if result.iter().all(|a| !evaluate_equals(a, item)) {
          result.push(item.clone())
        }
      }
    } else {
      return invalid_argument_type!("union", "list", list.type_of());
    }
  }
  Value::List(result)
}

/// Returns upper-cased string.
pub fn upper_case(input_string_value: &Value) -> Value {
  match input_string_value {
    Value::List(items) => {
      if items.len() == 1 {
        if let Value::String(input_string) = &items[0] {
          return Value::String(input_string.to_uppercase());
        }
      }
    }
    Value::String(input_string) => return Value::String(input_string.to_uppercase()),
    _ => {}
  }
  invalid_argument_type!("upper case", "string", input_string_value.type_of())
}

/// Returns the ISO week number of the year.
pub fn week_of_year(value: &Value) -> Value {
  fn iso_week_of_year(opt_week_of_year: Option<WeekOfYear>) -> Value {
    if let Some(week_of_year) = opt_week_of_year {
      value_number!(week_of_year)
    } else {
      value_null!("[week of year] no week of year")
    }
  }
  match value {
    Value::Date(date) => iso_week_of_year(date.week_of_year()),
    Value::DateTime(date_time) => iso_week_of_year(date_time.week_of_year()),
    _ => invalid_argument_type!("week of year", "date, date and time", value.type_of()),
  }
}

/// Returns years and months duration between `from` and `to`.
pub fn years_and_months_duration(from_value: &Value, to_value: &Value) -> Value {
  if let Value::Date(from) = from_value {
    if let Value::DateTime(to) = to_value {
      return Value::YearsAndMonthsDuration(&to.date() - from);
    }
    if let Value::Date(to) = to_value {
      return Value::YearsAndMonthsDuration(to - from);
    }
    return invalid_argument_type!("years and months duration", "date, date and time", to_value.type_of());
  }
  if let Value::DateTime(from) = from_value {
    if let Value::DateTime(to) = to_value {
      return Value::YearsAndMonthsDuration(&to.date() - &from.date());
    }
    if let Value::Date(to) = to_value {
      return Value::YearsAndMonthsDuration(to - &from.date());
    }
    return invalid_argument_type!("years and months duration", "date, date and time", to_value.type_of());
  }
  invalid_argument_type!("years and months duration", "date, date and time", from_value.type_of())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gregorian_month_of_year() {
    assert_eq!("null([month of year] no month)", gregorian_month(None).to_string());
  }
}
