use crate::bifs::core;
use dsntk_feel::bif::Bif;
use dsntk_feel::values::Value;
use dsntk_feel::{FeelNumber, Name, value_null};
use std::sync::LazyLock;

static NAME_CONTEXT: LazyLock<Name> = LazyLock::new(|| Name::from("context"));
static NAME_CONTEXTS: LazyLock<Name> = LazyLock::new(|| Name::from("contexts"));
static NAME_DATE: LazyLock<Name> = LazyLock::new(|| Name::from("date"));
static NAME_DAY: LazyLock<Name> = LazyLock::new(|| Name::from("day"));
static NAME_DECIMAL_SEPARATOR: LazyLock<Name> = LazyLock::new(|| Name::new(&["decimal", "separator"]));
static NAME_DELIMITER: LazyLock<Name> = LazyLock::new(|| Name::from("delimiter"));
static NAME_DIVIDEND: LazyLock<Name> = LazyLock::new(|| Name::from("dividend"));
static NAME_DIVISOR: LazyLock<Name> = LazyLock::new(|| Name::from("divisor"));
static NAME_ENTRIES: LazyLock<Name> = LazyLock::new(|| Name::from("entries"));
static NAME_FLAGS: LazyLock<Name> = LazyLock::new(|| Name::from("flags"));
static NAME_FROM: LazyLock<Name> = LazyLock::new(|| Name::from("from"));
static NAME_GROUPING_SEPARATOR: LazyLock<Name> = LazyLock::new(|| Name::new(&["grouping", "separator"]));
static NAME_HOUR: LazyLock<Name> = LazyLock::new(|| Name::from("hour"));
static NAME_INPUT: LazyLock<Name> = LazyLock::new(|| Name::from("input"));
static NAME_KEY: LazyLock<Name> = LazyLock::new(|| Name::from("key"));
static NAME_KEYS: LazyLock<Name> = LazyLock::new(|| Name::from("keys"));
static NAME_LENGTH: LazyLock<Name> = LazyLock::new(|| Name::from("length"));
static NAME_LIST: LazyLock<Name> = LazyLock::new(|| Name::from("list"));
static NAME_MATCH: LazyLock<Name> = LazyLock::new(|| Name::from("match"));
static NAME_MONTH: LazyLock<Name> = LazyLock::new(|| Name::from("month"));
static NAME_MINUTE: LazyLock<Name> = LazyLock::new(|| Name::from("minute"));
static NAME_N: LazyLock<Name> = LazyLock::new(|| Name::from("n"));
static NAME_M: LazyLock<Name> = LazyLock::new(|| Name::from("m"));
static NAME_NEGAND: LazyLock<Name> = LazyLock::new(|| Name::from("negand"));
static NAME_NEW_ITEM: LazyLock<Name> = LazyLock::new(|| Name::from("newItem"));
static NAME_NUMBER: LazyLock<Name> = LazyLock::new(|| Name::from("number"));
static NAME_OFFSET: LazyLock<Name> = LazyLock::new(|| Name::from("offset"));
static NAME_PATTERN: LazyLock<Name> = LazyLock::new(|| Name::from("pattern"));
static NAME_POINT: LazyLock<Name> = LazyLock::new(|| Name::from("point"));
static NAME_POINT_1: LazyLock<Name> = LazyLock::new(|| Name::from("point1"));
static NAME_POINT_2: LazyLock<Name> = LazyLock::new(|| Name::from("point2"));
static NAME_POSITION: LazyLock<Name> = LazyLock::new(|| Name::from("position"));
static NAME_PRECEDES: LazyLock<Name> = LazyLock::new(|| Name::from("precedes"));
static NAME_RANGE: LazyLock<Name> = LazyLock::new(|| Name::from("range"));
static NAME_RANGE_1: LazyLock<Name> = LazyLock::new(|| Name::from("range1"));
static NAME_RANGE_2: LazyLock<Name> = LazyLock::new(|| Name::from("range2"));
static NAME_REPLACEMENT: LazyLock<Name> = LazyLock::new(|| Name::from("replacement"));
static NAME_SCALE: LazyLock<Name> = LazyLock::new(|| Name::from("scale"));
static NAME_SECOND: LazyLock<Name> = LazyLock::new(|| Name::from("second"));
static NAME_START_POSITION: LazyLock<Name> = LazyLock::new(|| Name::new(&["start", "position"]));
static NAME_STRING: LazyLock<Name> = LazyLock::new(|| Name::from("string"));
static NAME_TIME: LazyLock<Name> = LazyLock::new(|| Name::from("time"));
static NAME_TO: LazyLock<Name> = LazyLock::new(|| Name::from("to"));
static NAME_VALUE: LazyLock<Name> = LazyLock::new(|| Name::from("value"));
static NAME_VALUE_1: LazyLock<Name> = LazyLock::new(|| Name::from("value1"));
static NAME_VALUE_2: LazyLock<Name> = LazyLock::new(|| Name::from("value2"));
static NAME_YEAR: LazyLock<Name> = LazyLock::new(|| Name::from("year"));

type NamedParameters = Value;

macro_rules! invalid_number_of_parameters {
  ($expected:expr, $actual:expr) => {{ value_null!("expected {} parameters, actual number of parameters is {}", $expected, $actual) }};
}

macro_rules! parameter_not_found {
  ($p1:expr, $p2:expr) => {{ value_null!(r"parameter '{}' or '{}' not found", *$p1, *$p2) }};
  ($p:expr) => {{ value_null!(r"parameter '{}' not found", *$p) }};
}

pub fn evaluate_bif(bif: Bif, parameters: &NamedParameters) -> Value {
  match bif {
    Bif::Abs => bif_abs(parameters),
    Bif::After => bif_after(parameters),
    Bif::All => bif_all(parameters),
    Bif::Any => bif_any(parameters),
    Bif::Append => bif_append(parameters),
    Bif::Before => bif_before(parameters),
    Bif::Ceiling => bif_ceiling(parameters),
    Bif::Coincides => bif_coincides(parameters),
    Bif::Concatenate => bif_concatenate(parameters),
    Bif::Contains => bif_contains(parameters),
    Bif::Context => bif_context(parameters),
    Bif::ContextMerge => bif_context_merge(parameters),
    Bif::ContextPut => bif_context_put(parameters),
    Bif::Count => bif_count(parameters),
    Bif::Date => bif_date(parameters),
    Bif::DateAndTime => bif_date_and_time(parameters),
    Bif::DayOfWeek => bif_day_of_week(parameters),
    Bif::DayOfYear => bif_day_of_year(parameters),
    Bif::Decimal => bif_decimal(parameters),
    Bif::DistinctValues => bif_distinct_values(parameters),
    Bif::Duration => bif_duration(parameters),
    Bif::During => bif_during(parameters),
    Bif::EndsWith => bif_ends_with(parameters),
    Bif::Even => bif_even(parameters),
    Bif::Exp => bif_exp(parameters),
    Bif::FinishedBy => bif_finished_by(parameters),
    Bif::Finishes => bif_finishes(parameters),
    Bif::Flatten => bif_flatten(parameters),
    Bif::Floor => bif_floor(parameters),
    Bif::GetEntries => bif_get_entries(parameters),
    Bif::GetValue => bif_get_value(parameters),
    Bif::Includes => bif_includes(parameters),
    Bif::IndexOf => bif_index_of(parameters),
    Bif::InsertBefore => bif_insert_before(parameters),
    Bif::Is => bif_is(parameters),
    Bif::ListContains => bif_list_contains(parameters),
    Bif::ListReplace => bif_list_replace(parameters),
    Bif::Log => bif_log(parameters),
    Bif::LoweCase => bif_lower_case(parameters),
    Bif::Matches => bif_matches(parameters),
    Bif::Max => bif_max(parameters),
    Bif::Mean => bif_mean(parameters),
    Bif::Meets => bif_meets(parameters),
    Bif::Median => bif_median(parameters),
    Bif::MetBy => bif_met_by(parameters),
    Bif::Min => bif_min(parameters),
    Bif::Mode => bif_mode(parameters),
    Bif::Modulo => bif_modulo(parameters),
    Bif::MonthOfYear => bif_month_of_year(parameters),
    Bif::Not => bif_not(parameters),
    Bif::Now => bif_now(parameters),
    Bif::Number => bif_number(parameters),
    Bif::Odd => bif_odd(parameters),
    Bif::Overlaps => bif_overlaps(parameters),
    Bif::OverlapsAfter => bif_overlaps_after(parameters),
    Bif::OverlapsBefore => bif_overlaps_before(parameters),
    Bif::Product => bif_product(parameters),
    Bif::Range => bif_range(parameters),
    Bif::Remove => bif_remove(parameters),
    Bif::Replace => bif_replace(parameters),
    Bif::Reverse => bif_reverse(parameters),
    Bif::RoundDown => bif_round_down(parameters),
    Bif::RoundHalfDown => bif_round_half_down(parameters),
    Bif::RoundHalfUp => bif_round_half_up(parameters),
    Bif::RoundUp => bif_round_up(parameters),
    Bif::Sort => bif_sort(parameters),
    Bif::Split => bif_split(parameters),
    Bif::Sqrt => bif_sqrt(parameters),
    Bif::StartedBy => bif_started_by(parameters),
    Bif::Starts => bif_starts(parameters),
    Bif::StartsWith => bif_starts_with(parameters),
    Bif::Stddev => bif_stddev(parameters),
    Bif::String => bif_string(parameters),
    Bif::StringJoin => bif_string_join(parameters),
    Bif::StringLength => bif_string_length(parameters),
    Bif::Sublist => bif_sublist(parameters),
    Bif::Substring => bif_substring(parameters),
    Bif::SubstringAfter => bif_substring_after(parameters),
    Bif::SubstringBefore => bif_substring_before(parameters),
    Bif::Sum => bif_sum(parameters),
    Bif::Time => bif_time(parameters),
    Bif::Today => bif_today(parameters),
    Bif::Union => bif_union(parameters),
    Bif::UpperCase => bif_upper_case(parameters),
    Bif::WeekOfYear => bif_week_of_year(parameters),
    Bif::YearsAndMonthsDuration => bif_years_and_months_duration(parameters),
  }
}

fn bif_abs(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_N) {
    core::abs(value)
  } else {
    parameter_not_found!(NAME_N)
  }
}

fn bif_after(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT_1).zip(get_param(parameters, &NAME_POINT_2)) {
    core::after(value1, value2)
  } else if let Some(((value1, pos1), (value2, pos2))) = get_param(parameters, &NAME_POINT).zip(get_param(parameters, &NAME_RANGE)) {
    if pos1 > pos2 { core::after(value2, value1) } else { core::after(value1, value2) }
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::after(value1, value2)
  } else {
    value_null!("[named::after] invalid named parameters")
  }
}

fn bif_all(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::all(list)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_any(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::any(list)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

/// `append` built-in function, currently no version with named parameters, always returns `null`.
fn bif_append(_parameters: &NamedParameters) -> Value {
  value_null!("[named::append] this function has no version with named parameters")
}

fn bif_before(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT_1).zip(get_param(parameters, &NAME_POINT_2)) {
    core::before(value1, value2)
  } else if let Some(((value1, pos1), (value2, pos2))) = get_param(parameters, &NAME_POINT).zip(get_param(parameters, &NAME_RANGE)) {
    if pos1 > pos2 { core::before(value2, value1) } else { core::before(value1, value2) }
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::before(value1, value2)
  } else {
    value_null!("[named::before] invalid named parameters")
  }
}

fn bif_ceiling(parameters: &NamedParameters) -> Value {
  match get_param_count(parameters) {
    1 => {
      if let Some((value, _)) = get_param(parameters, &NAME_N) {
        core::ceiling(value, &Value::Number(FeelNumber::zero()))
      } else {
        parameter_not_found!(NAME_N)
      }
    }
    2 => {
      if let Some((value, _)) = get_param(parameters, &NAME_N) {
        if let Some((scale, _)) = get_param(parameters, &NAME_SCALE) {
          core::ceiling(value, scale)
        } else {
          parameter_not_found!(NAME_SCALE)
        }
      } else {
        parameter_not_found!(NAME_N)
      }
    }
    n => invalid_number_of_parameters!("1,2", n),
  }
}

/// `coincides` built-in function, named parameters.
fn bif_coincides(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT_1).zip(get_param(parameters, &NAME_POINT_2)) {
    core::coincides(value1, value2)
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::coincides(value1, value2)
  } else {
    value_null!("[named::coincides] invalid named parameters")
  }
}

/// `concatenate` built-in function, currently no version with named parameters, always returns `null`.
fn bif_concatenate(_parameters: &NamedParameters) -> Value {
  value_null!("[named::concatenate] this function has no version with named parameters")
}

fn bif_contains(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    if let Some((match_string_value, _)) = get_param(parameters, &NAME_MATCH) {
      core::contains(input_string_value, match_string_value)
    } else {
      parameter_not_found!(NAME_MATCH)
    }
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_context(parameters: &NamedParameters) -> Value {
  if let Some((entries_value, _)) = get_param(parameters, &NAME_ENTRIES) {
    core::context(entries_value)
  } else {
    parameter_not_found!(NAME_ENTRIES)
  }
}

fn bif_context_merge(parameters: &NamedParameters) -> Value {
  if let Some((contexts_value, _)) = get_param(parameters, &NAME_CONTEXTS) {
    core::context_merge(contexts_value)
  } else {
    parameter_not_found!(NAME_CONTEXTS)
  }
}

fn bif_context_put(parameters: &NamedParameters) -> Value {
  if let Some((v_context, _)) = get_param(parameters, &NAME_CONTEXT) {
    if let Some((v_value, _)) = get_param(parameters, &NAME_VALUE) {
      if let Some((v_keys, _)) = get_param(parameters, &NAME_KEYS) {
        if v_keys.is_list() {
          core::context_put(v_context, v_keys, v_value)
        } else {
          value_null!()
        }
      } else if let Some((v_key, _)) = get_param(parameters, &NAME_KEY) {
        if v_key.is_string() {
          core::context_put(v_context, v_key, v_value)
        } else {
          value_null!()
        }
      } else {
        parameter_not_found!(NAME_KEY, NAME_KEYS)
      }
    } else {
      parameter_not_found!(NAME_VALUE)
    }
  } else {
    parameter_not_found!(NAME_CONTEXT)
  }
}

fn bif_count(parameters: &NamedParameters) -> Value {
  if let Some((list, _)) = get_param(parameters, &NAME_LIST) {
    core::count(list)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_date(parameters: &NamedParameters) -> Value {
  if let Some((from, _)) = get_param(parameters, &NAME_FROM) {
    return core::date_1(from);
  }
  if let Some((year, _)) = get_param(parameters, &NAME_YEAR)
    && let Some((month, _)) = get_param(parameters, &NAME_MONTH)
    && let Some((day, _)) = get_param(parameters, &NAME_DAY)
  {
    return core::date_3(year, month, day);
  }
  value_null!("invalid parameters in named::bif_date")
}

fn bif_date_and_time(parameters: &NamedParameters) -> Value {
  if let Some((from_value, _)) = get_param(parameters, &NAME_FROM) {
    return core::date_and_time_1(from_value);
  }
  if let Some((date_value, _)) = get_param(parameters, &NAME_DATE)
    && let Some((time_value, _)) = get_param(parameters, &NAME_TIME)
  {
    return core::date_and_time_2(date_value, time_value);
  }
  value_null!("invalid parameters in named::bif_date_and_time")
}

fn bif_day_of_week(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_DATE) {
    core::day_of_week(value)
  } else {
    parameter_not_found!(NAME_DATE)
  }
}

fn bif_day_of_year(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_DATE) {
    core::day_of_year(value)
  } else {
    parameter_not_found!(NAME_DATE)
  }
}

fn bif_decimal(parameters: &NamedParameters) -> Value {
  if let Some((number, _)) = get_param(parameters, &NAME_N) {
    if let Some((scale, _)) = get_param(parameters, &NAME_SCALE) {
      core::decimal(number, scale)
    } else {
      parameter_not_found!(NAME_SCALE)
    }
  } else {
    parameter_not_found!(NAME_N)
  }
}

fn bif_distinct_values(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_LIST) {
    core::distinct_values(value)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_duration(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_FROM) {
    core::duration(value)
  } else {
    parameter_not_found!(NAME_FROM)
  }
}

fn bif_during(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT).zip(get_param(parameters, &NAME_RANGE)) {
    core::during(value1, value2)
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::during(value1, value2)
  } else {
    value_null!("[named::during] invalid named parameters")
  }
}

fn bif_ends_with(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    if let Some((match_string_value, _)) = get_param(parameters, &NAME_MATCH) {
      core::ends_with(input_string_value, match_string_value)
    } else {
      parameter_not_found!(NAME_MATCH)
    }
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_even(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_NUMBER) {
    core::even(value)
  } else {
    parameter_not_found!(NAME_NUMBER)
  }
}

fn bif_exp(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_NUMBER) {
    core::exp(value)
  } else {
    parameter_not_found!(NAME_NUMBER)
  }
}

fn bif_finished_by(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::finished_by(value1, value2)
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE).zip(get_param(parameters, &NAME_POINT)) {
    core::finished_by(value1, value2)
  } else {
    value_null!("[named::finished by] invalid named parameters")
  }
}

fn bif_finishes(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT).zip(get_param(parameters, &NAME_RANGE)) {
    core::finishes(value1, value2)
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::finishes(value1, value2)
  } else {
    value_null!("[named::finishes] invalid named parameters")
  }
}

fn bif_flatten(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_LIST) {
    core::flatten(value)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_floor(parameters: &NamedParameters) -> Value {
  match get_param_count(parameters) {
    1 => {
      if let Some((value, _)) = get_param(parameters, &NAME_N) {
        core::floor(value, &Value::Number(FeelNumber::zero()))
      } else {
        parameter_not_found!(NAME_N)
      }
    }
    2 => {
      if let Some((value, _)) = get_param(parameters, &NAME_N) {
        if let Some((scale, _)) = get_param(parameters, &NAME_SCALE) {
          core::floor(value, scale)
        } else {
          parameter_not_found!(NAME_SCALE)
        }
      } else {
        parameter_not_found!(NAME_N)
      }
    }
    n => invalid_number_of_parameters!("1,2", n),
  }
}

fn bif_get_entries(parameters: &NamedParameters) -> Value {
  if let Some((context, _)) = get_param(parameters, &NAME_M) {
    core::get_entries(context)
  } else {
    parameter_not_found!(NAME_M)
  }
}

fn bif_get_value(parameters: &NamedParameters) -> Value {
  if let Some((context, _)) = get_param(parameters, &NAME_M) {
    if let Some((key, _)) = get_param(parameters, &NAME_KEY) {
      core::get_value(context, key)
    } else {
      parameter_not_found!(NAME_KEY)
    }
  } else {
    parameter_not_found!(NAME_M)
  }
}

fn bif_includes(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE).zip(get_param(parameters, &NAME_POINT)) {
    core::includes(value1, value2)
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::includes(value1, value2)
  } else {
    value_null!("[named::includes] invalid named parameters")
  }
}

fn bif_index_of(parameters: &NamedParameters) -> Value {
  if let Some((list_value, _)) = get_param(parameters, &NAME_LIST) {
    if let Some((match_value, _)) = get_param(parameters, &NAME_MATCH) {
      core::index_of(list_value, match_value)
    } else {
      parameter_not_found!(NAME_MATCH)
    }
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_insert_before(parameters: &NamedParameters) -> Value {
  if let Some((list_value, _)) = get_param(parameters, &NAME_LIST) {
    if let Some((position_value, _)) = get_param(parameters, &NAME_POSITION) {
      if let Some((new_item_value, _)) = get_param(parameters, &NAME_NEW_ITEM) {
        core::insert_before(list_value, position_value, new_item_value)
      } else {
        parameter_not_found!(NAME_NEW_ITEM)
      }
    } else {
      parameter_not_found!(NAME_POSITION)
    }
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_is(parameters: &NamedParameters) -> Value {
  match get_param_count(parameters) {
    1 => Value::Boolean(false),
    2 => {
      if let Some((value1, _)) = get_param(parameters, &NAME_VALUE_1) {
        if let Some((value2, _)) = get_param(parameters, &NAME_VALUE_2) {
          core::is(value1, value2)
        } else {
          parameter_not_found!(NAME_VALUE_2)
        }
      } else {
        parameter_not_found!(NAME_VALUE_1)
      }
    }
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_list_contains(parameters: &NamedParameters) -> Value {
  if let Some((list_value, _)) = get_param(parameters, &NAME_LIST) {
    if let Some((match_value, _)) = get_param(parameters, &NAME_MATCH) {
      core::list_contains(list_value, match_value)
    } else {
      parameter_not_found!(NAME_MATCH)
    }
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_list_replace(parameters: &NamedParameters) -> Value {
  match get_param_count(parameters) {
    3 => {
      let Some((list_value, _)) = get_param(parameters, &NAME_LIST) else {
        return parameter_not_found!(NAME_LIST);
      };
      let Some((new_item_value, _)) = get_param(parameters, &NAME_NEW_ITEM) else {
        return parameter_not_found!(NAME_NEW_ITEM);
      };
      if let Some((position_value, _)) = get_param(parameters, &NAME_POSITION) {
        return core::list_replace(list_value, position_value, new_item_value);
      }
      if let Some((match_value, _)) = get_param(parameters, &NAME_MATCH) {
        return core::list_replace(list_value, match_value, new_item_value);
      }
      parameter_not_found!(NAME_POSITION, NAME_MATCH)
    }
    n => invalid_number_of_parameters!("3", n),
  }
}

fn bif_log(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_NUMBER) {
    core::log(value)
  } else {
    parameter_not_found!(NAME_NUMBER)
  }
}

fn bif_lower_case(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    core::lower_case(input_string_value)
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_matches(parameters: &NamedParameters) -> Value {
  match get_param_count(parameters) {
    2 => {
      if let Some((input_string_value, _)) = get_param(parameters, &NAME_INPUT) {
        if let Some((pattern_string_value, _)) = get_param(parameters, &NAME_PATTERN) {
          core::matches_2(input_string_value, pattern_string_value)
        } else {
          parameter_not_found!(NAME_PATTERN)
        }
      } else {
        parameter_not_found!(NAME_INPUT)
      }
    }
    3 => {
      if let Some((v_input, _)) = get_param(parameters, &NAME_INPUT) {
        if let Some((v_pattern, _)) = get_param(parameters, &NAME_PATTERN) {
          if let Some((v_flags, _)) = get_param(parameters, &NAME_FLAGS) {
            if v_flags.is_null() {
              core::matches_2(v_input, v_pattern)
            } else {
              core::matches_3(v_input, v_pattern, v_flags)
            }
          } else {
            parameter_not_found!(NAME_FLAGS)
          }
        } else {
          parameter_not_found!(NAME_PATTERN)
        }
      } else {
        parameter_not_found!(NAME_INPUT)
      }
    }
    n => invalid_number_of_parameters!("2,3", n),
  }
}

fn bif_max(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::max(list)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_mean(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::mean(list)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_meets(parameters: &NamedParameters) -> Value {
  if let Some((value1, _)) = get_param(parameters, &NAME_RANGE_1) {
    if let Some((value2, _)) = get_param(parameters, &NAME_RANGE_2) {
      core::meets(value1, value2)
    } else {
      parameter_not_found!(NAME_RANGE_2)
    }
  } else {
    parameter_not_found!(NAME_RANGE_1)
  }
}

fn bif_median(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::median(list)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_met_by(parameters: &NamedParameters) -> Value {
  if let Some((value1, _)) = get_param(parameters, &NAME_RANGE_1) {
    if let Some((value2, _)) = get_param(parameters, &NAME_RANGE_2) {
      core::met_by(value1, value2)
    } else {
      parameter_not_found!(NAME_RANGE_2)
    }
  } else {
    parameter_not_found!(NAME_RANGE_1)
  }
}

fn bif_min(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::min(list)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_modulo(parameters: &NamedParameters) -> Value {
  if let Some((dividend, _)) = get_param(parameters, &NAME_DIVIDEND) {
    if let Some((divisor, _)) = get_param(parameters, &NAME_DIVISOR) {
      core::modulo(dividend, divisor)
    } else {
      parameter_not_found!(NAME_DIVISOR)
    }
  } else {
    parameter_not_found!(NAME_DIVIDEND)
  }
}

fn bif_month_of_year(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_DATE) {
    core::month_of_year(value)
  } else {
    parameter_not_found!(NAME_DATE)
  }
}

fn bif_mode(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::mode(list)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_not(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_NEGAND) {
    core::not(value)
  } else {
    parameter_not_found!(NAME_NEGAND)
  }
}

fn bif_now(_: &NamedParameters) -> Value {
  value_null!("[named::now] this function has no implementation with named parameters")
}

fn bif_number(parameters: &NamedParameters) -> Value {
  if let Some((from, _)) = get_param(parameters, &NAME_FROM) {
    if let Some((grouping_separator, _)) = get_param(parameters, &NAME_GROUPING_SEPARATOR) {
      if let Some((decimal_separator, _)) = get_param(parameters, &NAME_DECIMAL_SEPARATOR) {
        core::number(from, grouping_separator, decimal_separator)
      } else {
        parameter_not_found!(NAME_DECIMAL_SEPARATOR)
      }
    } else {
      parameter_not_found!(NAME_GROUPING_SEPARATOR)
    }
  } else {
    parameter_not_found!(NAME_FROM)
  }
}

fn bif_odd(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_NUMBER) {
    core::odd(value)
  } else {
    parameter_not_found!(NAME_NUMBER)
  }
}

fn bif_overlaps(parameters: &NamedParameters) -> Value {
  if let Some((value1, _)) = get_param(parameters, &NAME_RANGE_1) {
    if let Some((value2, _)) = get_param(parameters, &NAME_RANGE_2) {
      core::overlaps(value1, value2)
    } else {
      parameter_not_found!(NAME_RANGE_2)
    }
  } else {
    parameter_not_found!(NAME_RANGE_1)
  }
}

fn bif_overlaps_after(parameters: &NamedParameters) -> Value {
  if let Some((value1, _)) = get_param(parameters, &NAME_RANGE_1) {
    if let Some((value2, _)) = get_param(parameters, &NAME_RANGE_2) {
      core::overlaps_after(value1, value2)
    } else {
      parameter_not_found!(NAME_RANGE_2)
    }
  } else {
    parameter_not_found!(NAME_RANGE_1)
  }
}

fn bif_overlaps_before(parameters: &NamedParameters) -> Value {
  if let Some((value1, _)) = get_param(parameters, &NAME_RANGE_1) {
    if let Some((value2, _)) = get_param(parameters, &NAME_RANGE_2) {
      core::overlaps_before(value1, value2)
    } else {
      parameter_not_found!(NAME_RANGE_2)
    }
  } else {
    parameter_not_found!(NAME_RANGE_1)
  }
}

fn bif_product(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::product(list)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_range(parameters: &NamedParameters) -> Value {
  match get_param_count(parameters) {
    1 => {
      if let Some((value, _)) = get_param(parameters, &NAME_FROM) {
        core::range(value)
      } else {
        parameter_not_found!(NAME_FROM)
      }
    }
    n => invalid_number_of_parameters!("1", n),
  }
}

fn bif_remove(parameters: &NamedParameters) -> Value {
  if let Some((list, _)) = get_param(parameters, &NAME_LIST) {
    if let Some((position, _)) = get_param(parameters, &NAME_POSITION) {
      core::remove(list, position)
    } else {
      parameter_not_found!(NAME_POSITION)
    }
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_replace(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_INPUT) {
    if let Some((pattern_string_value, _)) = get_param(parameters, &NAME_PATTERN) {
      if let Some((replacement_string_value, _)) = get_param(parameters, &NAME_REPLACEMENT) {
        if let Some((flags_string_value, _)) = get_param(parameters, &NAME_FLAGS) {
          core::replace(input_string_value, pattern_string_value, replacement_string_value, flags_string_value)
        } else {
          core::replace(input_string_value, pattern_string_value, replacement_string_value, &value_null!())
        }
      } else {
        parameter_not_found!(NAME_REPLACEMENT)
      }
    } else {
      parameter_not_found!(NAME_PATTERN)
    }
  } else {
    parameter_not_found!(NAME_INPUT)
  }
}

fn bif_reverse(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_LIST) {
    core::reverse(value)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

/// Returns `n` with given `scale` and rounding mode **round down**.
fn bif_round_down(parameters: &NamedParameters) -> Value {
  match get_param_count(parameters) {
    2 => {
      if let Some((n, _)) = get_param(parameters, &NAME_N) {
        if let Some((scale, _)) = get_param(parameters, &NAME_SCALE) {
          core::round_down(n, scale)
        } else {
          parameter_not_found!(NAME_SCALE)
        }
      } else {
        parameter_not_found!(NAME_N)
      }
    }
    n => invalid_number_of_parameters!(2, n),
  }
}

/// Returns `n` with given `scale` and rounding mode **round half down**.
fn bif_round_half_down(parameters: &NamedParameters) -> Value {
  match get_param_count(parameters) {
    2 => {
      if let Some((n, _)) = get_param(parameters, &NAME_N) {
        if let Some((scale, _)) = get_param(parameters, &NAME_SCALE) {
          core::round_half_down(n, scale)
        } else {
          parameter_not_found!(NAME_SCALE)
        }
      } else {
        parameter_not_found!(NAME_N)
      }
    }
    n => invalid_number_of_parameters!(2, n),
  }
}

/// Returns `n` with given `scale` and rounding mode **round half up**.
fn bif_round_half_up(parameters: &NamedParameters) -> Value {
  match get_param_count(parameters) {
    2 => {
      if let Some((n, _)) = get_param(parameters, &NAME_N) {
        if let Some((scale, _)) = get_param(parameters, &NAME_SCALE) {
          core::round_half_up(n, scale)
        } else {
          parameter_not_found!(NAME_SCALE)
        }
      } else {
        parameter_not_found!(NAME_N)
      }
    }
    n => invalid_number_of_parameters!(2, n),
  }
}

/// Returns `n` with given `scale` and rounding mode **round up**.
fn bif_round_up(parameters: &NamedParameters) -> Value {
  match get_param_count(parameters) {
    2 => {
      if let Some((n, _)) = get_param(parameters, &NAME_N) {
        if let Some((scale, _)) = get_param(parameters, &NAME_SCALE) {
          core::round_up(n, scale)
        } else {
          parameter_not_found!(NAME_SCALE)
        }
      } else {
        parameter_not_found!(NAME_N)
      }
    }
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_sort(parameters: &NamedParameters) -> Value {
  if let Some((list, _)) = get_param(parameters, &NAME_LIST) {
    if let Some((ordering_function, _)) = get_param(parameters, &NAME_PRECEDES) {
      core::sort(list, ordering_function)
    } else {
      parameter_not_found!(NAME_PRECEDES)
    }
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_split(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    if let Some((delimiter_string_value, _)) = get_param(parameters, &NAME_DELIMITER) {
      core::split(input_string_value, delimiter_string_value)
    } else {
      parameter_not_found!(NAME_DELIMITER)
    }
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_sqrt(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_NUMBER) {
    core::sqrt(value)
  } else {
    parameter_not_found!(NAME_NUMBER)
  }
}

fn bif_started_by(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE).zip(get_param(parameters, &NAME_POINT)) {
    core::started_by(value1, value2)
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::started_by(value1, value2)
  } else {
    value_null!("[named::started by] invalid named parameters")
  }
}

fn bif_starts(parameters: &NamedParameters) -> Value {
  if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_POINT).zip(get_param(parameters, &NAME_RANGE)) {
    core::starts(value1, value2)
  } else if let Some(((value1, _), (value2, _))) = get_param(parameters, &NAME_RANGE_1).zip(get_param(parameters, &NAME_RANGE_2)) {
    core::starts(value1, value2)
  } else {
    value_null!("[named::starts] invalid named parameters")
  }
}

fn bif_starts_with(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    if let Some((match_string_value, _)) = get_param(parameters, &NAME_MATCH) {
      core::starts_with(input_string_value, match_string_value)
    } else {
      parameter_not_found!(NAME_MATCH)
    }
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_stddev(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::stddev(list)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_string(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_FROM) {
    core::string(value)
  } else {
    parameter_not_found!(NAME_FROM)
  }
}

fn bif_string_join(parameters: &NamedParameters) -> Value {
  match get_param_count(parameters) {
    1 => {
      if let Some((list_value, _)) = get_param(parameters, &NAME_LIST) {
        core::string_join(list_value, &value_null!())
      } else {
        parameter_not_found!(NAME_LIST)
      }
    }
    2 => {
      if let Some((list_value, _)) = get_param(parameters, &NAME_LIST) {
        if let Some((delimiter_value, _)) = get_param(parameters, &NAME_DELIMITER) {
          core::string_join(list_value, delimiter_value)
        } else {
          parameter_not_found!(NAME_DELIMITER)
        }
      } else {
        parameter_not_found!(NAME_LIST)
      }
    }
    n => invalid_number_of_parameters!("1,2", n),
  }
}

fn bif_string_length(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    core::string_length(input_string_value)
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_sublist(parameters: &NamedParameters) -> Value {
  match get_param_count(parameters) {
    2 => {
      if let Some((list_value, _)) = get_param(parameters, &NAME_LIST) {
        if let Some((start_position_value, _)) = get_param(parameters, &NAME_START_POSITION) {
          core::sublist2(list_value, start_position_value)
        } else {
          parameter_not_found!(NAME_START_POSITION)
        }
      } else {
        parameter_not_found!(NAME_LIST)
      }
    }
    3 => {
      if let Some((list_value, _)) = get_param(parameters, &NAME_LIST) {
        if let Some((start_position_value, _)) = get_param(parameters, &NAME_START_POSITION) {
          if let Some((length_value, _)) = get_param(parameters, &NAME_LENGTH) {
            core::sublist3(list_value, start_position_value, length_value)
          } else {
            parameter_not_found!(NAME_LENGTH)
          }
        } else {
          parameter_not_found!(NAME_START_POSITION)
        }
      } else {
        parameter_not_found!(NAME_LIST)
      }
    }
    n => invalid_number_of_parameters!("2,3", n),
  }
}

fn bif_substring(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    if let Some((start_position_value, _)) = get_param(parameters, &NAME_START_POSITION) {
      if let Some((length_value, _)) = get_param(parameters, &NAME_LENGTH) {
        core::substring(input_string_value, start_position_value, length_value)
      } else {
        core::substring(input_string_value, start_position_value, &value_null!())
      }
    } else {
      parameter_not_found!(NAME_START_POSITION)
    }
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_substring_after(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    if let Some((input_string_match, _)) = get_param(parameters, &NAME_MATCH) {
      core::substring_after(input_string_value, input_string_match)
    } else {
      parameter_not_found!(NAME_MATCH)
    }
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_substring_before(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    if let Some((input_string_match, _)) = get_param(parameters, &NAME_MATCH) {
      core::substring_before(input_string_value, input_string_match)
    } else {
      parameter_not_found!(NAME_MATCH)
    }
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_sum(parameters: &NamedParameters) -> Value {
  if let Some((Value::List(list), _)) = get_param(parameters, &NAME_LIST) {
    core::sum(list)
  } else {
    parameter_not_found!(NAME_LIST)
  }
}

fn bif_time(parameters: &NamedParameters) -> Value {
  if let Some((from_value, _)) = get_param(parameters, &NAME_FROM) {
    return core::time_1(from_value);
  }
  if let Some((hour_value, _)) = get_param(parameters, &NAME_HOUR)
    && let Some((minute_value, _)) = get_param(parameters, &NAME_MINUTE)
    && let Some((second_value, _)) = get_param(parameters, &NAME_SECOND)
  {
    return if let Some((offset_value, _)) = get_param(parameters, &NAME_OFFSET) {
      core::time_4(hour_value, minute_value, second_value, offset_value)
    } else {
      core::time_3(hour_value, minute_value, second_value)
    };
  }
  value_null!("invalid parameters in bif time")
}

fn bif_today(_: &NamedParameters) -> Value {
  value_null!("[named::today] this function has no implementation with named parameters")
}

fn bif_union(_: &NamedParameters) -> Value {
  value_null!("[named::union] this function has no implementation with named parameters")
}

fn bif_upper_case(parameters: &NamedParameters) -> Value {
  if let Some((input_string_value, _)) = get_param(parameters, &NAME_STRING) {
    core::upper_case(input_string_value)
  } else {
    parameter_not_found!(NAME_STRING)
  }
}

fn bif_week_of_year(parameters: &NamedParameters) -> Value {
  if let Some((value, _)) = get_param(parameters, &NAME_DATE) {
    core::week_of_year(value)
  } else {
    parameter_not_found!(NAME_DATE)
  }
}

fn bif_years_and_months_duration(parameters: &NamedParameters) -> Value {
  if let Some((from_value, _)) = get_param(parameters, &NAME_FROM) {
    if let Some((to_value, _)) = get_param(parameters, &NAME_TO) {
      core::years_and_months_duration(from_value, to_value)
    } else {
      parameter_not_found!(NAME_TO)
    }
  } else {
    parameter_not_found!(NAME_FROM)
  }
}

/// Returns reference to the value and position of the parameter with specified name.
/// The position of the named parameter is counted from 1.
/// Additionally, the total number of parameters is returned.
fn get_param<'a>(parameters: &'a NamedParameters, name: &'a Name) -> Option<(&'a Value, &'a usize)> {
  if let Value::NamedParameters(map) = parameters
    && let Some((value, position)) = map.get(name)
  {
    return Some((value, position));
  }
  None
}

/// Returns the number of named parameters.
fn get_param_count(parameters: &NamedParameters) -> usize {
  if let Value::NamedParameters(map) = parameters { map.len() } else { 0 }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_param_count() {
    assert_eq!(0, get_param_count(&Value::Boolean(false)))
  }
}
