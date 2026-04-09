//! # Definitions of built-in functions

use crate::FeelType;
use crate::errors::*;
use dsntk_common::DsntkError;
use std::str::FromStr;

const BIF_ABS: &str = "abs";
const BIF_AFTER: &str = "after";
const BIF_ANY: &str = "any";
const BIF_ALL: &str = "all";
const BIF_APPEND: &str = "append";
const BIF_BEFORE: &str = "before";
const BIF_CEILING: &str = "ceiling";
const BIF_COINCIDES: &str = "coincides";
const BIF_CONCATENATE: &str = "concatenate";
const BIF_CONTAINS: &str = "contains";
const BIF_CONTEXT: &str = "context";
const BIF_CONTEXT_MERGE: &str = "context merge";
const BIF_CONTEXT_PUT: &str = "context put";
const BIF_COUNT: &str = "count";
const BIF_DATE: &str = "date";
const BIF_DATE_AND_TIME: &str = "date and time";
const BIF_DAY_OF_WEEK: &str = "day of week";
const BIF_DAY_OF_YEAR: &str = "day of year";
const BIF_DECIMAL: &str = "decimal";
const BIF_DISTINCT_VALUES: &str = "distinct values";
const BIF_DURATION: &str = "duration";
const BIF_DURING: &str = "during";
const BIF_ENDS_WITH: &str = "ends with";
const BIF_EVEN: &str = "even";
const BIF_EXP: &str = "exp";
const BIF_FINISHED_BY: &str = "finished by";
const BIF_FINISHES: &str = "finishes";
const BIF_FLATTEN: &str = "flatten";
const BIF_FLOOR: &str = "floor";
const BIF_GET_ENTRIES: &str = "get entries";
const BIF_GET_VALUE: &str = "get value";
const BIF_INCLUDES: &str = "includes";
const BIF_INDEX_OF: &str = "index of";
const BIF_INSERT_BEFORE: &str = "insert before";
const BIF_IS: &str = "is";
const BIF_LIST_CONTAINS: &str = "list contains";
const BIF_LIST_REPLACE: &str = "list replace";
const BIF_LOG: &str = "log";
const BIF_LOWER_CASE: &str = "lower case";
const BIF_MATCHES: &str = "matches";
const BIF_MAX: &str = "max";
const BIF_MEAN: &str = "mean";
const BIF_MEDIAN: &str = "median";
const BIF_MEETS: &str = "meets";
const BIF_MET_BY: &str = "met by";
const BIF_MIN: &str = "min";
const BIF_MODE: &str = "mode";
const BIF_MODULO: &str = "modulo";
const BIF_MONTH_OF_YEAR: &str = "month of year";
const BIF_NOT: &str = "not";
const BIF_NOW: &str = "now";
const BIF_NUMBER: &str = "number";
const BIF_ODD: &str = "odd";
const BIF_OVERLAPS: &str = "overlaps";
const BIF_OVERLAPS_AFTER: &str = "overlaps after";
const BIF_OVERLAPS_BEFORE: &str = "overlaps before";
const BIF_PRODUCT: &str = "product";
const BIF_RANGE: &str = "range";
const BIF_REMOVE: &str = "remove";
const BIF_REPLACE: &str = "replace";
const BIF_REVERSE: &str = "reverse";
const BIF_ROUND_DOWN: &str = "round down";
const BIF_ROUND_HALF_DOWN: &str = "round half down";
const BIF_ROUND_HALF_UP: &str = "round half up";
const BIF_ROUND_UP: &str = "round up";
const BIF_SORT: &str = "sort";
const BIF_SPLIT: &str = "split";
const BIF_SQRT: &str = "sqrt";
const BIF_STARTED_BY: &str = "started by";
const BIF_STARTS: &str = "starts";
const BIF_STARTS_WITH: &str = "starts with";
const BIF_STDDEV: &str = "stddev";
const BIF_STRING: &str = "string";
const BIF_STRING_JOIN: &str = "string join";
const BIF_STRING_LENGTH: &str = "string length";
const BIF_SUBLIST: &str = "sublist";
const BIF_SUBSTRING: &str = "substring";
const BIF_SUBSTRING_AFTER: &str = "substring after";
const BIF_SUBSTRING_BEFORE: &str = "substring before";
const BIF_SUM: &str = "sum";
const BIF_TIME: &str = "time";
const BIF_TODAY: &str = "today";
const BIF_UNION: &str = "union";
const BIF_UPPER_CASE: &str = "upper case";
const BIF_WEEK_OF_YEAR: &str = "week of year";
const BIF_YEARS_AND_MONTHS_DURATION: &str = "years and months duration";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bif {
  Abs,
  After,
  All,
  Any,
  Append,
  Before,
  Ceiling,
  Coincides,
  Concatenate,
  Contains,
  Context,
  ContextMerge,
  ContextPut,
  Count,
  Date,
  DateAndTime,
  DayOfWeek,
  DayOfYear,
  Decimal,
  DistinctValues,
  Duration,
  During,
  EndsWith,
  Even,
  Exp,
  FinishedBy,
  Finishes,
  Flatten,
  Floor,
  GetEntries,
  GetValue,
  Includes,
  IndexOf,
  InsertBefore,
  Is,
  ListContains,
  ListReplace,
  Log,
  LoweCase,
  Matches,
  Max,
  Mean,
  Median,
  Meets,
  MetBy,
  Min,
  Mode,
  Modulo,
  MonthOfYear,
  Not,
  Now,
  Number,
  Odd,
  Overlaps,
  OverlapsAfter,
  OverlapsBefore,
  Product,
  Range,
  Remove,
  Replace,
  Reverse,
  RoundDown,
  RoundHalfDown,
  RoundHalfUp,
  RoundUp,
  Sort,
  Split,
  Sqrt,
  StartedBy,
  Starts,
  StartsWith,
  Stddev,
  String,
  StringJoin,
  StringLength,
  Sublist,
  Substring,
  SubstringAfter,
  SubstringBefore,
  Sum,
  Time,
  Today,
  Union,
  UpperCase,
  WeekOfYear,
  YearsAndMonthsDuration,
}

impl FromStr for Bif {
  type Err = DsntkError;
  /// Converts a string into corresponding built-in function enumeration.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      BIF_ABS => Ok(Self::Abs),
      BIF_AFTER => Ok(Self::After),
      BIF_ALL => Ok(Self::All),
      BIF_ANY => Ok(Self::Any),
      BIF_APPEND => Ok(Self::Append),
      BIF_BEFORE => Ok(Self::Before),
      BIF_CEILING => Ok(Self::Ceiling),
      BIF_COINCIDES => Ok(Self::Coincides),
      BIF_CONCATENATE => Ok(Self::Concatenate),
      BIF_CONTAINS => Ok(Self::Contains),
      BIF_CONTEXT => Ok(Self::Context),
      BIF_CONTEXT_MERGE => Ok(Self::ContextMerge),
      BIF_CONTEXT_PUT => Ok(Self::ContextPut),
      BIF_COUNT => Ok(Self::Count),
      BIF_DATE => Ok(Self::Date),
      BIF_DATE_AND_TIME => Ok(Self::DateAndTime),
      BIF_DAY_OF_WEEK => Ok(Self::DayOfWeek),
      BIF_DAY_OF_YEAR => Ok(Self::DayOfYear),
      BIF_DECIMAL => Ok(Self::Decimal),
      BIF_DISTINCT_VALUES => Ok(Self::DistinctValues),
      BIF_DURATION => Ok(Self::Duration),
      BIF_DURING => Ok(Self::During),
      BIF_ENDS_WITH => Ok(Self::EndsWith),
      BIF_EVEN => Ok(Self::Even),
      BIF_EXP => Ok(Self::Exp),
      BIF_FINISHED_BY => Ok(Self::FinishedBy),
      BIF_FINISHES => Ok(Self::Finishes),
      BIF_FLATTEN => Ok(Self::Flatten),
      BIF_FLOOR => Ok(Self::Floor),
      BIF_GET_ENTRIES => Ok(Self::GetEntries),
      BIF_GET_VALUE => Ok(Self::GetValue),
      BIF_INCLUDES => Ok(Self::Includes),
      BIF_INDEX_OF => Ok(Self::IndexOf),
      BIF_INSERT_BEFORE => Ok(Self::InsertBefore),
      BIF_IS => Ok(Self::Is),
      BIF_LIST_CONTAINS => Ok(Self::ListContains),
      BIF_LIST_REPLACE => Ok(Self::ListReplace),
      BIF_LOG => Ok(Self::Log),
      BIF_LOWER_CASE => Ok(Self::LoweCase),
      BIF_MATCHES => Ok(Self::Matches),
      BIF_MAX => Ok(Self::Max),
      BIF_MEAN => Ok(Self::Mean),
      BIF_MEDIAN => Ok(Self::Median),
      BIF_MEETS => Ok(Self::Meets),
      BIF_MET_BY => Ok(Self::MetBy),
      BIF_MIN => Ok(Self::Min),
      BIF_MODE => Ok(Self::Mode),
      BIF_MODULO => Ok(Self::Modulo),
      BIF_MONTH_OF_YEAR => Ok(Self::MonthOfYear),
      BIF_NOT => Ok(Self::Not),
      BIF_NOW => Ok(Self::Now),
      BIF_NUMBER => Ok(Self::Number),
      BIF_ODD => Ok(Self::Odd),
      BIF_OVERLAPS => Ok(Self::Overlaps),
      BIF_OVERLAPS_AFTER => Ok(Self::OverlapsAfter),
      BIF_OVERLAPS_BEFORE => Ok(Self::OverlapsBefore),
      BIF_PRODUCT => Ok(Self::Product),
      BIF_RANGE => Ok(Self::Range),
      BIF_REMOVE => Ok(Self::Remove),
      BIF_REPLACE => Ok(Self::Replace),
      BIF_REVERSE => Ok(Self::Reverse),
      BIF_ROUND_DOWN => Ok(Self::RoundDown),
      BIF_ROUND_HALF_DOWN => Ok(Self::RoundHalfDown),
      BIF_ROUND_HALF_UP => Ok(Self::RoundHalfUp),
      BIF_ROUND_UP => Ok(Self::RoundUp),
      BIF_SORT => Ok(Self::Sort),
      BIF_SPLIT => Ok(Self::Split),
      BIF_SQRT => Ok(Self::Sqrt),
      BIF_STARTED_BY => Ok(Self::StartedBy),
      BIF_STARTS => Ok(Self::Starts),
      BIF_STARTS_WITH => Ok(Self::StartsWith),
      BIF_STDDEV => Ok(Self::Stddev),
      BIF_STRING => Ok(Self::String),
      BIF_STRING_JOIN => Ok(Self::StringJoin),
      BIF_STRING_LENGTH => Ok(Self::StringLength),
      BIF_SUBLIST => Ok(Self::Sublist),
      BIF_SUBSTRING => Ok(Self::Substring),
      BIF_SUBSTRING_AFTER => Ok(Self::SubstringAfter),
      BIF_SUBSTRING_BEFORE => Ok(Self::SubstringBefore),
      BIF_SUM => Ok(Self::Sum),
      BIF_TIME => Ok(Self::Time),
      BIF_TODAY => Ok(Self::Today),
      BIF_UNION => Ok(Self::Union),
      BIF_UPPER_CASE => Ok(Self::UpperCase),
      BIF_WEEK_OF_YEAR => Ok(Self::WeekOfYear),
      BIF_YEARS_AND_MONTHS_DURATION => Ok(Self::YearsAndMonthsDuration),
      unknown => Err(err_unknown_function_name(unknown)),
    }
  }
}

impl Bif {
  /// Returns a [FeelType] returned from built-in function.
  pub fn feel_type(&self) -> FeelType {
    match self {
      Bif::Abs => FeelType::Function(vec![FeelType::Number], Box::new(FeelType::Number)),
      Bif::Sqrt => FeelType::Function(vec![FeelType::Number], Box::new(FeelType::Number)),
      _ => {
        // TODO Implement the rest of bif types
        FeelType::Any
      }
    }
  }
}

/// Returns `true` when the specified name is a built-in function name.
pub fn is_built_in_function_name(name: impl AsRef<str>) -> bool {
  matches!(
    name.as_ref(),
    BIF_ABS
      | BIF_AFTER
      | BIF_ALL
      | BIF_ANY
      | BIF_APPEND
      | BIF_BEFORE
      | BIF_CEILING
      | BIF_COINCIDES
      | BIF_CONCATENATE
      | BIF_CONTAINS
      | BIF_CONTEXT
      | BIF_CONTEXT_MERGE
      | BIF_CONTEXT_PUT
      | BIF_COUNT
      | BIF_DATE
      | BIF_DATE_AND_TIME
      | BIF_DAY_OF_WEEK
      | BIF_DAY_OF_YEAR
      | BIF_DECIMAL
      | BIF_DISTINCT_VALUES
      | BIF_DURATION
      | BIF_DURING
      | BIF_ENDS_WITH
      | BIF_EVEN
      | BIF_EXP
      | BIF_FINISHED_BY
      | BIF_FINISHES
      | BIF_FLATTEN
      | BIF_FLOOR
      | BIF_GET_ENTRIES
      | BIF_GET_VALUE
      | BIF_INCLUDES
      | BIF_INDEX_OF
      | BIF_INSERT_BEFORE
      | BIF_IS
      | BIF_LIST_CONTAINS
      | BIF_LIST_REPLACE
      | BIF_LOG
      | BIF_LOWER_CASE
      | BIF_MATCHES
      | BIF_MAX
      | BIF_MEAN
      | BIF_MEDIAN
      | BIF_MEETS
      | BIF_MET_BY
      | BIF_MIN
      | BIF_MODE
      | BIF_MODULO
      | BIF_MONTH_OF_YEAR
      | BIF_NOT
      | BIF_NOW
      | BIF_NUMBER
      | BIF_ODD
      | BIF_OVERLAPS
      | BIF_OVERLAPS_AFTER
      | BIF_OVERLAPS_BEFORE
      | BIF_PRODUCT
      | BIF_RANGE
      | BIF_REMOVE
      | BIF_REPLACE
      | BIF_REVERSE
      | BIF_ROUND_DOWN
      | BIF_ROUND_HALF_DOWN
      | BIF_ROUND_HALF_UP
      | BIF_ROUND_UP
      | BIF_SORT
      | BIF_SPLIT
      | BIF_SQRT
      | BIF_STARTED_BY
      | BIF_STARTS
      | BIF_STARTS_WITH
      | BIF_STDDEV
      | BIF_STRING
      | BIF_STRING_JOIN
      | BIF_STRING_LENGTH
      | BIF_SUBLIST
      | BIF_SUBSTRING
      | BIF_SUBSTRING_AFTER
      | BIF_SUBSTRING_BEFORE
      | BIF_SUM
      | BIF_TIME
      | BIF_TODAY
      | BIF_UNION
      | BIF_UPPER_CASE
      | BIF_WEEK_OF_YEAR
      | BIF_YEARS_AND_MONTHS_DURATION
  )
}

/// Returns `true` when the specified name is one of the following built-in functions:
/// - `date`,
/// - `time`,
/// - `date and time`,
/// - `duration`.
pub fn is_built_in_date_time_function_name(name: impl AsRef<str>) -> bool {
  matches!(name.as_ref(), BIF_DATE | BIF_TIME | BIF_DATE_AND_TIME | BIF_DURATION)
}
