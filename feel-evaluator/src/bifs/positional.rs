use crate::bifs::core;
use crate::macros::invalid_number_of_parameters;
use dsntk_feel::bif::Bif;
use dsntk_feel::values::Value;
use dsntk_feel::{FeelNumber, value_null};

pub fn evaluate_bif(bif: Bif, parameters: &[Value]) -> Value {
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
    Bif::Meets => bif_meats(parameters),
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

fn bif_abs(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::abs(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_after(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::after(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_all(parameters: &[Value]) -> Value {
  match parameters.len() {
    0 => invalid_number_of_parameters!("1+", 0),
    1 => match &parameters[0] {
      Value::List(values) => core::all(values),
      _ => core::all(parameters),
    },
    _ => core::all(parameters),
  }
}

fn bif_any(parameters: &[Value]) -> Value {
  match parameters.len() {
    0 => invalid_number_of_parameters!("1+", 0),
    1 => match &parameters[0] {
      Value::List(values) => core::any(values),
      _ => core::any(parameters),
    },
    _ => core::any(parameters),
  }
}

fn bif_append(parameters: &[Value]) -> Value {
  if parameters.len() > 1 {
    core::append(&parameters[0], &parameters[1..])
  } else {
    invalid_number_of_parameters!("2+", parameters.len())
  }
}

fn bif_before(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::before(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_ceiling(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::ceiling(&parameters[0], &Value::Number(FeelNumber::zero())),
    2 => core::ceiling(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!("1,2", n),
  }
}

fn bif_concatenate(parameters: &[Value]) -> Value {
  match parameters.len() {
    0 => invalid_number_of_parameters!("1+", 0),
    _ => super::core::concatenate(parameters),
  }
}

/// `coincides` built-in function, positional parameters.
fn bif_coincides(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::coincides(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_contains(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::contains(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_context(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::context(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_context_merge(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::context_merge(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_context_put(parameters: &[Value]) -> Value {
  match parameters.len() {
    3 => core::context_put(&parameters[0], &parameters[1], &parameters[2]),
    n => invalid_number_of_parameters!(3, n),
  }
}

fn bif_count(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::count(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_date(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::date_1(&parameters[0]),
    3 => core::date_3(&parameters[0], &parameters[1], &parameters[2]),
    n => invalid_number_of_parameters!("1,3", n),
  }
}

fn bif_date_and_time(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::date_and_time_1(&parameters[0]),
    2 => core::date_and_time_2(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!("1,2", n),
  }
}

fn bif_day_of_week(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::day_of_week(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_day_of_year(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::day_of_year(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_decimal(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::decimal(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_distinct_values(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::distinct_values(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_duration(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::duration(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_during(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::during(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_ends_with(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::ends_with(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_even(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::even(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_exp(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::exp(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_finished_by(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::finished_by(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_finishes(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::finishes(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_flatten(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::flatten(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_floor(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::floor(&parameters[0], &Value::Number(FeelNumber::zero())),
    2 => core::floor(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!("1,2", n),
  }
}

fn bif_get_entries(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::get_entries(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_get_value(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::get_value(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_includes(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::includes(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_index_of(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::index_of(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_insert_before(parameters: &[Value]) -> Value {
  match parameters.len() {
    3 => core::insert_before(&parameters[0], &parameters[1], &parameters[2]),
    n => invalid_number_of_parameters!(3, n),
  }
}

fn bif_is(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => Value::Boolean(false),
    2 => core::is(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_list_contains(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::list_contains(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_list_replace(parameters: &[Value]) -> Value {
  match parameters.len() {
    3 => core::list_replace(&parameters[0], &parameters[1], &parameters[2]),
    n => invalid_number_of_parameters!(3, n),
  }
}

fn bif_log(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::log(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_lower_case(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::lower_case(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_matches(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::matches_2(&parameters[0], &parameters[1]),
    3 => {
      if parameters[2].is_null() {
        core::matches_2(&parameters[0], &parameters[1])
      } else {
        core::matches_3(&parameters[0], &parameters[1], &parameters[2])
      }
    }
    n => invalid_number_of_parameters!("2,3", n),
  }
}

fn bif_max(parameters: &[Value]) -> Value {
  match parameters.len() {
    0 => invalid_number_of_parameters!("1+", 0),
    1 => match &parameters[0] {
      Value::List(values) => core::max(values),
      _ => core::max(parameters),
    },
    _ => core::max(parameters),
  }
}

fn bif_mean(parameters: &[Value]) -> Value {
  match parameters.len() {
    0 => invalid_number_of_parameters!("1+", 0),
    1 => match &parameters[0] {
      Value::List(values) => core::mean(values),
      _ => core::mean(parameters),
    },
    _ => core::mean(parameters),
  }
}

fn bif_meats(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::meets(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_median(parameters: &[Value]) -> Value {
  match parameters.len() {
    0 => invalid_number_of_parameters!("1+", 0),
    1 => match &parameters[0] {
      Value::List(values) => core::median(values),
      _ => core::median(parameters),
    },
    _ => core::median(parameters),
  }
}

fn bif_met_by(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::met_by(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_min(parameters: &[Value]) -> Value {
  match parameters.len() {
    0 => invalid_number_of_parameters!("1+", 0),
    1 => match &parameters[0] {
      Value::List(values) => core::min(values),
      _ => core::min(parameters),
    },
    _ => core::min(parameters),
  }
}

fn bif_mode(parameters: &[Value]) -> Value {
  match parameters.len() {
    0 => invalid_number_of_parameters!("1+", 0),
    1 => match &parameters[0] {
      Value::List(values) => core::mode(values),
      _ => core::mode(parameters),
    },
    _ => core::mode(parameters),
  }
}

fn bif_modulo(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::modulo(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_month_of_year(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::month_of_year(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_not(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::not(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_now(parameters: &[Value]) -> Value {
  match parameters.len() {
    0 => core::now(),
    n => invalid_number_of_parameters!(0, n),
  }
}

fn bif_number(parameters: &[Value]) -> Value {
  match parameters.len() {
    3 => core::number(&parameters[0], &parameters[1], &parameters[2]),
    n => invalid_number_of_parameters!(3, n),
  }
}

fn bif_odd(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::odd(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_overlaps(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::overlaps(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_overlaps_after(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::overlaps_after(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_overlaps_before(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::overlaps_before(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_product(parameters: &[Value]) -> Value {
  match parameters.len() {
    0 => invalid_number_of_parameters!("1+", 0),
    1 => match &parameters[0] {
      Value::List(values) => core::product(values),
      _ => core::product(parameters),
    },
    _ => core::product(parameters),
  }
}

fn bif_range(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::range(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_remove(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::remove(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_replace(parameters: &[Value]) -> Value {
  match parameters.len() {
    3 => core::replace(&parameters[0], &parameters[1], &parameters[2], &value_null!()),
    4 => core::replace(&parameters[0], &parameters[1], &parameters[2], &parameters[3]),
    n => invalid_number_of_parameters!("3,4", n),
  }
}

fn bif_reverse(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::reverse(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

/// Returns `n` with given `scale` and rounding mode **round down**.
fn bif_round_down(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::round_down(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

/// Returns `n` with given `scale` and rounding mode **round half down**.
fn bif_round_half_down(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::round_half_down(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

/// Returns `n` with given `scale` and rounding mode **round half up**.
fn bif_round_half_up(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::round_half_up(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

/// Returns `n` with given `scale` and rounding mode **round up**.
fn bif_round_up(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::round_up(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_sort(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::sort(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_split(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::split(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_sqrt(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::sqrt(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_started_by(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::started_by(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_starts(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::starts(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_starts_with(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::starts_with(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_stddev(parameters: &[Value]) -> Value {
  match parameters.len() {
    0 => invalid_number_of_parameters!("1+", 0),
    1 => match &parameters[0] {
      Value::List(values) => core::stddev(values),
      _ => value_null!("[positional::stddev] invalid argument type, expected list, actual type is {}", parameters[0].type_of()),
    },
    _ => core::stddev(parameters),
  }
}

fn bif_string(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::string(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_string_join(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::string_join(&parameters[0], &value_null!()),
    2 => core::string_join(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!("1,2", n),
  }
}

fn bif_string_length(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::string_length(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_sublist(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::sublist2(&parameters[0], &parameters[1]),
    3 => core::sublist3(&parameters[0], &parameters[1], &parameters[2]),
    n => invalid_number_of_parameters!("2,3", n),
  }
}

fn bif_substring(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::substring(&parameters[0], &parameters[1], &value_null!()),
    3 => core::substring(&parameters[0], &parameters[1], &parameters[2]),
    n => invalid_number_of_parameters!("2,3", n),
  }
}

fn bif_substring_after(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::substring_after(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_substring_before(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::substring_before(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}

fn bif_sum(parameters: &[Value]) -> Value {
  match parameters.len() {
    0 => invalid_number_of_parameters!("1+", 0),
    1 => match &parameters[0] {
      Value::List(values) => core::sum(values),
      _ => core::sum(parameters),
    },
    _ => core::sum(parameters),
  }
}

fn bif_time(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::time_1(&parameters[0]),
    3 => core::time_3(&parameters[0], &parameters[1], &parameters[2]),
    4 => core::time_4(&parameters[0], &parameters[1], &parameters[2], &parameters[3]),
    n => invalid_number_of_parameters!("1,3,4", n),
  }
}

fn bif_today(parameters: &[Value]) -> Value {
  match parameters.len() {
    0 => core::today(),
    n => invalid_number_of_parameters!(0, n),
  }
}

fn bif_union(parameters: &[Value]) -> Value {
  match parameters.len() {
    0 => invalid_number_of_parameters!("1+", 0),
    _ => core::union(parameters),
  }
}

fn bif_upper_case(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::upper_case(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_week_of_year(parameters: &[Value]) -> Value {
  match parameters.len() {
    1 => core::week_of_year(&parameters[0]),
    n => invalid_number_of_parameters!(1, n),
  }
}

fn bif_years_and_months_duration(parameters: &[Value]) -> Value {
  match parameters.len() {
    2 => core::years_and_months_duration(&parameters[0], &parameters[1]),
    n => invalid_number_of_parameters!(2, n),
  }
}
