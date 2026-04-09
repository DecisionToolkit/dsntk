use super::*;
use crate::closure::Closure;
use crate::context::FeelContext;
use crate::types::is_built_in_type_name;
use crate::values::Value;
use crate::{FeelScope, FunctionBody, IntervalType, value_null, value_number};
use dsntk_feel_temporal::{FeelDate, FeelDateTime, FeelDaysAndTimeDuration, FeelTime, FeelYearsAndMonthsDuration};
use std::str::FromStr;
use std::sync::Arc;

#[test]
fn test_from_string() {
  assert_eq!(FeelType::Any, FeelType::from_str("Any").unwrap());
  assert_eq!(FeelType::Null, FeelType::from_str("Null").unwrap());
  assert_eq!(FeelType::Boolean, FeelType::from_str("boolean").unwrap());
  assert_eq!(FeelType::Number, FeelType::from_str("number").unwrap());
  assert_eq!(FeelType::String, FeelType::from_str("string").unwrap());
  assert_eq!(FeelType::Date, FeelType::from_str("date").unwrap());
  assert_eq!(FeelType::Time, FeelType::from_str("time").unwrap());
  assert_eq!(FeelType::DateTime, FeelType::from_str("date and time").unwrap());
  assert_eq!(FeelType::DaysAndTimeDuration, FeelType::from_str("days and time duration").unwrap());
  assert_eq!(FeelType::YearsAndMonthsDuration, FeelType::from_str("years and months duration").unwrap());
  assert_eq!("<TypesError> invalid FEEL type name: range", FeelType::from_str("range").err().unwrap().to_string());
}

#[test]
fn test_from_name() {
  fn eq(expected: FeelType, s: &str) {
    let name = &Name::from(s);
    let actual: FeelType = name.into();
    assert_eq!(expected, actual);
  }
  eq(FeelType::Any, "Any");
  eq(FeelType::Null, "Null");
  eq(FeelType::Boolean, "boolean");
  eq(FeelType::Number, "number");
  eq(FeelType::String, "string");
  eq(FeelType::Date, "date");
  eq(FeelType::Time, "time");
  eq(FeelType::DateTime, "date and time");
  eq(FeelType::DaysAndTimeDuration, "days and time duration");
  eq(FeelType::YearsAndMonthsDuration, "years and months duration");
  eq(FeelType::Any, "range");
}

#[test]
fn test_get_value_checked() {
  let v_null = Value::Null(None);
  let v_boolean = Value::Boolean(true);
  let v_string = Value::String("hello".to_owned());
  assert_eq!("null", T_ANY.get_value_checked(&v_null).unwrap().to_string());
  assert_eq!("true", T_ANY.get_value_checked(&v_boolean).unwrap().to_string());
  assert!(T_BOOLEAN.get_value_checked(&v_string).is_err());
  assert_eq!(
    r#"<TypesError> invalid value for retrieving with type check, type = 'boolean', value = '"hello"'"#,
    format!("{}", T_BOOLEAN.get_value_checked(&v_string).err().unwrap()).as_str()
  );
}

#[test]
fn test_type_conformance() {
  // any
  assert!(T_ANY.is_conformant(T_ANY));
  assert!(!T_ANY.is_conformant(T_BOOLEAN));
  assert!(!T_ANY.is_conformant(&T_CONTEXT_A));
  assert!(!T_ANY.is_conformant(T_DATE));
  assert!(!T_ANY.is_conformant(T_DATE_TIME));
  assert!(!T_ANY.is_conformant(T_DAYS_AND_TIME_DURATION));
  // boolean
  assert!(T_BOOLEAN.is_conformant(T_BOOLEAN));
  assert!(T_BOOLEAN.is_conformant(T_ANY));
  assert!(!T_BOOLEAN.is_conformant(&T_CONTEXT_A));
  assert!(!T_BOOLEAN.is_conformant(T_DATE));
  assert!(!T_BOOLEAN.is_conformant(T_DATE_TIME));
  assert!(!T_BOOLEAN.is_conformant(T_DAYS_AND_TIME_DURATION));
  assert!(!T_BOOLEAN.is_conformant(&T_FUNCTION_A));
  assert!(!T_BOOLEAN.is_conformant(&T_LIST_B));
  assert!(!T_BOOLEAN.is_conformant(T_NULL));
  assert!(!T_BOOLEAN.is_conformant(T_NUMBER));
  assert!(!T_BOOLEAN.is_conformant(&T_RANGE_A));
  assert!(!T_BOOLEAN.is_conformant(T_STRING));
  assert!(!T_BOOLEAN.is_conformant(T_TIME));
  assert!(!T_BOOLEAN.is_conformant(T_YEARS_AND_MONTHS_DURATION));
  // context
  assert!(T_CONTEXT_A.is_conformant(&T_CONTEXT_A));
  assert!(!T_CONTEXT_A.is_conformant(&T_CONTEXT_B));
  assert!(!T_CONTEXT_A.is_conformant(&T_CONTEXT_C));
  assert!(T_CONTEXT_A_B.is_conformant(&T_CONTEXT_A_B));
  assert!(T_CONTEXT_A_B.is_conformant(&T_CONTEXT_A));
  assert!(T_CONTEXT_A_B_C.is_conformant(&T_CONTEXT_A_B_C));
  assert!(T_CONTEXT_A_B_C.is_conformant(&T_CONTEXT_A_B));
  assert!(!T_CONTEXT_A.is_conformant(&T_CONTEXT_A_B));
  assert!(!T_CONTEXT_A_B.is_conformant(&T_CONTEXT_A_B_C));
  assert!(!T_CONTEXT_A_B.is_conformant(&T_CONTEXT_A_B_C));
  assert!(T_CONTEXT_A.is_conformant(T_ANY));
  assert!(!T_CONTEXT_A.is_conformant(T_BOOLEAN));
  assert!(!T_CONTEXT_A.is_conformant(T_DATE));
  assert!(!T_CONTEXT_A.is_conformant(T_DATE_TIME));
  assert!(!T_CONTEXT_A.is_conformant(T_DAYS_AND_TIME_DURATION));
  assert!(!T_CONTEXT_A.is_conformant(&T_FUNCTION_A));
  assert!(!T_CONTEXT_A.is_conformant(&T_LIST_B));
  assert!(!T_CONTEXT_A.is_conformant(T_NULL));
  assert!(!T_CONTEXT_A.is_conformant(T_NUMBER));
  assert!(!T_CONTEXT_A.is_conformant(&T_RANGE_A));
  assert!(!T_CONTEXT_A.is_conformant(T_STRING));
  assert!(!T_CONTEXT_A.is_conformant(T_TIME));
  assert!(!T_CONTEXT_A.is_conformant(T_YEARS_AND_MONTHS_DURATION));
  // date
  assert!(T_DATE.is_conformant(T_DATE));
  assert!(T_DATE.is_conformant(T_ANY));
  assert!(!T_DATE.is_conformant(T_BOOLEAN));
  assert!(!T_DATE.is_conformant(&T_CONTEXT_A));
  assert!(!T_DATE.is_conformant(T_DATE_TIME));
  assert!(!T_DATE.is_conformant(T_DAYS_AND_TIME_DURATION));
  assert!(!T_DATE.is_conformant(&T_FUNCTION_A));
  assert!(!T_DATE.is_conformant(&T_LIST_B));
  assert!(!T_DATE.is_conformant(T_NULL));
  assert!(!T_DATE.is_conformant(T_NUMBER));
  assert!(!T_DATE.is_conformant(&T_RANGE_A));
  assert!(!T_DATE.is_conformant(T_STRING));
  assert!(!T_DATE.is_conformant(T_TIME));
  assert!(!T_DATE.is_conformant(T_YEARS_AND_MONTHS_DURATION));
  // date and time
  assert!(T_DATE_TIME.is_conformant(T_DATE_TIME));
  assert!(T_DATE_TIME.is_conformant(T_ANY));
  assert!(!T_DATE_TIME.is_conformant(T_BOOLEAN));
  assert!(!T_DATE_TIME.is_conformant(&T_CONTEXT_A));
  assert!(!T_DATE_TIME.is_conformant(T_DATE));
  assert!(!T_DATE_TIME.is_conformant(&T_FUNCTION_A));
  assert!(!T_DATE_TIME.is_conformant(&T_LIST_B));
  assert!(!T_DATE_TIME.is_conformant(T_NULL));
  assert!(!T_DATE_TIME.is_conformant(T_NUMBER));
  assert!(!T_DATE_TIME.is_conformant(&T_RANGE_A));
  assert!(!T_DATE_TIME.is_conformant(T_STRING));
  assert!(!T_DATE_TIME.is_conformant(T_TIME));
  assert!(!T_DATE_TIME.is_conformant(T_YEARS_AND_MONTHS_DURATION));
  // days and time duration
  assert!(T_DAYS_AND_TIME_DURATION.is_conformant(T_DAYS_AND_TIME_DURATION));
  assert!(T_DAYS_AND_TIME_DURATION.is_conformant(T_ANY));
  assert!(!T_DAYS_AND_TIME_DURATION.is_conformant(T_BOOLEAN));
  assert!(!T_DAYS_AND_TIME_DURATION.is_conformant(&T_CONTEXT_A));
  assert!(!T_DAYS_AND_TIME_DURATION.is_conformant(T_DATE));
  assert!(!T_DAYS_AND_TIME_DURATION.is_conformant(T_DATE_TIME));
  assert!(!T_DAYS_AND_TIME_DURATION.is_conformant(&T_FUNCTION_A));
  assert!(!T_DAYS_AND_TIME_DURATION.is_conformant(&T_LIST_B));
  assert!(!T_DAYS_AND_TIME_DURATION.is_conformant(T_NULL));
  assert!(!T_DAYS_AND_TIME_DURATION.is_conformant(T_NUMBER));
  assert!(!T_DAYS_AND_TIME_DURATION.is_conformant(&T_RANGE_A));
  assert!(!T_DAYS_AND_TIME_DURATION.is_conformant(T_STRING));
  assert!(!T_DAYS_AND_TIME_DURATION.is_conformant(T_TIME));
  assert!(!T_DAYS_AND_TIME_DURATION.is_conformant(T_YEARS_AND_MONTHS_DURATION));
  // function
  assert!(T_FUNCTION_A.is_conformant(&T_FUNCTION_A));
  assert!(T_FUNCTION_B.is_conformant(&T_FUNCTION_B));
  assert!(T_FUNCTION_C.is_conformant(&T_FUNCTION_C));
  assert!(!T_FUNCTION_A.is_conformant(&T_FUNCTION_B));
  assert!(!T_FUNCTION_A.is_conformant(&T_FUNCTION_C));
  assert!(!T_FUNCTION_B.is_conformant(&T_FUNCTION_A));
  assert!(!T_FUNCTION_B.is_conformant(&T_FUNCTION_C));
  assert!(!T_FUNCTION_C.is_conformant(&T_FUNCTION_A));
  assert!(!T_FUNCTION_C.is_conformant(&T_FUNCTION_B));
  assert!(!T_FUNCTION_C.is_conformant(&T_FUNCTION_G));
  assert!(T_FUNCTION_F.is_conformant(&T_FUNCTION_C));
  assert!(T_FUNCTION_A.is_conformant(T_ANY));
  assert!(!T_FUNCTION_A.is_conformant(T_BOOLEAN));
  assert!(!T_FUNCTION_A.is_conformant(&T_CONTEXT_A));
  assert!(!T_FUNCTION_A.is_conformant(T_DATE));
  assert!(!T_FUNCTION_A.is_conformant(T_DATE_TIME));
  assert!(!T_FUNCTION_A.is_conformant(T_DAYS_AND_TIME_DURATION));
  assert!(!T_FUNCTION_A.is_conformant(&T_LIST_B));
  assert!(!T_FUNCTION_A.is_conformant(T_NULL));
  assert!(!T_FUNCTION_A.is_conformant(&T_RANGE_A));
  assert!(!T_FUNCTION_A.is_conformant(T_STRING));
  assert!(!T_FUNCTION_A.is_conformant(T_TIME));
  assert!(!T_FUNCTION_A.is_conformant(T_YEARS_AND_MONTHS_DURATION));
  // list
  assert!(T_LIST_A.is_conformant(&T_LIST_A));
  assert!(!T_LIST_A.is_conformant(&T_LIST_B));
  assert!(T_LIST_B.is_conformant(T_ANY));
  assert!(!T_LIST_B.is_conformant(T_BOOLEAN));
  assert!(!T_LIST_B.is_conformant(&T_CONTEXT_A));
  assert!(!T_LIST_B.is_conformant(T_DATE));
  assert!(!T_LIST_B.is_conformant(T_DATE_TIME));
  assert!(!T_LIST_B.is_conformant(T_DAYS_AND_TIME_DURATION));
  assert!(!T_LIST_B.is_conformant(&T_FUNCTION_A));
  assert!(!T_LIST_B.is_conformant(T_NULL));
  assert!(!T_LIST_B.is_conformant(&T_RANGE_A));
  assert!(!T_LIST_B.is_conformant(T_STRING));
  assert!(!T_LIST_B.is_conformant(T_TIME));
  assert!(!T_LIST_B.is_conformant(T_YEARS_AND_MONTHS_DURATION));
  // null
  assert!(T_NULL.is_conformant(T_NULL));
  assert!(T_NULL.is_conformant(T_ANY));
  assert!(T_NULL.is_conformant(T_BOOLEAN));
  assert!(T_NULL.is_conformant(&T_CONTEXT_A));
  assert!(T_NULL.is_conformant(T_DATE));
  assert!(T_NULL.is_conformant(T_DATE_TIME));
  assert!(T_NULL.is_conformant(T_DAYS_AND_TIME_DURATION));
  assert!(T_NULL.is_conformant(&T_FUNCTION_A));
  assert!(T_NULL.is_conformant(&T_LIST_B));
  assert!(T_NULL.is_conformant(&T_RANGE_A));
  assert!(T_NULL.is_conformant(T_STRING));
  assert!(T_NULL.is_conformant(T_TIME));
  assert!(T_NULL.is_conformant(T_YEARS_AND_MONTHS_DURATION));
  // number
  assert!(T_NUMBER.is_conformant(T_NUMBER));
  assert!(T_NUMBER.is_conformant(T_ANY));
  assert!(!T_NUMBER.is_conformant(T_BOOLEAN));
  assert!(!T_NUMBER.is_conformant(&T_CONTEXT_A));
  assert!(!T_NUMBER.is_conformant(T_DATE));
  assert!(!T_NUMBER.is_conformant(T_DATE_TIME));
  assert!(!T_NUMBER.is_conformant(T_DAYS_AND_TIME_DURATION));
  assert!(!T_NUMBER.is_conformant(&T_FUNCTION_A));
  assert!(!T_NUMBER.is_conformant(&T_LIST_B));
  assert!(!T_NUMBER.is_conformant(T_NULL));
  assert!(!T_NUMBER.is_conformant(&T_RANGE_A));
  assert!(!T_NUMBER.is_conformant(T_STRING));
  assert!(!T_NUMBER.is_conformant(T_TIME));
  assert!(!T_NUMBER.is_conformant(T_YEARS_AND_MONTHS_DURATION));
  // range
  assert!(T_RANGE_A.is_conformant(&T_RANGE_A));
  assert!(T_RANGE_A.is_conformant(T_ANY));
  assert!(!T_RANGE_A.is_conformant(&T_RANGE_B));
  assert!(T_RANGE_B.is_conformant(T_ANY));
  assert!(T_RANGE_B.is_conformant(&T_RANGE_B));
  assert!(!T_RANGE_B.is_conformant(&T_RANGE_A));
  assert!(!T_RANGE_A.is_conformant(T_BOOLEAN));
  assert!(!T_RANGE_A.is_conformant(&T_CONTEXT_A));
  assert!(!T_RANGE_A.is_conformant(T_DATE));
  assert!(!T_RANGE_A.is_conformant(T_DATE_TIME));
  assert!(!T_RANGE_A.is_conformant(T_DAYS_AND_TIME_DURATION));
  assert!(!T_RANGE_A.is_conformant(&T_FUNCTION_A));
  assert!(!T_RANGE_A.is_conformant(&T_LIST_B));
  assert!(!T_RANGE_A.is_conformant(T_NULL));
  assert!(!T_RANGE_A.is_conformant(T_NUMBER));
  assert!(!T_RANGE_A.is_conformant(T_STRING));
  assert!(!T_RANGE_A.is_conformant(T_TIME));
  assert!(!T_RANGE_A.is_conformant(T_YEARS_AND_MONTHS_DURATION));
  // string
  assert!(T_STRING.is_conformant(T_STRING));
  assert!(T_STRING.is_conformant(T_ANY));
  assert!(!T_STRING.is_conformant(T_BOOLEAN));
  assert!(!T_STRING.is_conformant(&T_CONTEXT_A));
  assert!(!T_STRING.is_conformant(T_DATE));
  assert!(!T_STRING.is_conformant(T_DATE_TIME));
  assert!(!T_STRING.is_conformant(T_DAYS_AND_TIME_DURATION));
  assert!(!T_STRING.is_conformant(&T_FUNCTION_A));
  assert!(!T_STRING.is_conformant(&T_LIST_B));
  assert!(!T_STRING.is_conformant(T_NULL));
  assert!(!T_STRING.is_conformant(T_NUMBER));
  assert!(!T_STRING.is_conformant(&T_RANGE_A));
  assert!(!T_STRING.is_conformant(T_TIME));
  assert!(!T_STRING.is_conformant(T_YEARS_AND_MONTHS_DURATION));
  // time
  assert!(T_TIME.is_conformant(T_TIME));
  assert!(T_TIME.is_conformant(T_ANY));
  assert!(!T_TIME.is_conformant(T_BOOLEAN));
  assert!(!T_TIME.is_conformant(&T_CONTEXT_A));
  assert!(!T_TIME.is_conformant(T_DATE));
  assert!(!T_TIME.is_conformant(T_DATE_TIME));
  assert!(!T_TIME.is_conformant(T_DAYS_AND_TIME_DURATION));
  assert!(!T_TIME.is_conformant(&T_FUNCTION_A));
  assert!(!T_TIME.is_conformant(&T_LIST_B));
  assert!(!T_TIME.is_conformant(T_NULL));
  assert!(!T_TIME.is_conformant(T_NUMBER));
  assert!(!T_TIME.is_conformant(&T_RANGE_A));
  assert!(!T_TIME.is_conformant(T_STRING));
  assert!(!T_TIME.is_conformant(T_YEARS_AND_MONTHS_DURATION));
  // years and months duration
  assert!(T_YEARS_AND_MONTHS_DURATION.is_conformant(T_YEARS_AND_MONTHS_DURATION));
  assert!(T_YEARS_AND_MONTHS_DURATION.is_conformant(T_ANY));
  assert!(!T_YEARS_AND_MONTHS_DURATION.is_conformant(T_BOOLEAN));
  assert!(!T_YEARS_AND_MONTHS_DURATION.is_conformant(&T_CONTEXT_A));
  assert!(!T_YEARS_AND_MONTHS_DURATION.is_conformant(T_DATE));
  assert!(!T_YEARS_AND_MONTHS_DURATION.is_conformant(T_DATE_TIME));
  assert!(!T_YEARS_AND_MONTHS_DURATION.is_conformant(T_DAYS_AND_TIME_DURATION));
  assert!(!T_YEARS_AND_MONTHS_DURATION.is_conformant(&T_FUNCTION_A));
  assert!(!T_YEARS_AND_MONTHS_DURATION.is_conformant(&T_LIST_B));
  assert!(!T_YEARS_AND_MONTHS_DURATION.is_conformant(T_NULL));
  assert!(!T_YEARS_AND_MONTHS_DURATION.is_conformant(T_NUMBER));
  assert!(!T_YEARS_AND_MONTHS_DURATION.is_conformant(&T_RANGE_A));
  assert!(!T_YEARS_AND_MONTHS_DURATION.is_conformant(T_STRING));
  assert!(!T_YEARS_AND_MONTHS_DURATION.is_conformant(T_TIME));
}

#[test]
fn test_is_built_in_type_name() {
  assert!(is_built_in_type_name("Any"));
  assert!(is_built_in_type_name("boolean"));
  assert!(is_built_in_type_name("date"));
  assert!(is_built_in_type_name("date and time"));
  assert!(is_built_in_type_name("days and time duration"));
  assert!(is_built_in_type_name("Null"));
  assert!(is_built_in_type_name("number"));
  assert!(is_built_in_type_name("string"));
  assert!(is_built_in_type_name("time"));
  assert!(is_built_in_type_name("years and months duration"));
  assert!(!is_built_in_type_name("context"));
  assert!(!is_built_in_type_name("function"));
  assert!(!is_built_in_type_name("list"));
  assert!(!is_built_in_type_name("range"));
}

#[test]
fn test_is_simple_built_in_type() {
  assert!(T_ANY.is_simple_built_in_type());
  assert!(T_BOOLEAN.is_simple_built_in_type());
  assert!(T_DATE.is_simple_built_in_type());
  assert!(T_DATE_TIME.is_simple_built_in_type());
  assert!(T_DAYS_AND_TIME_DURATION.is_simple_built_in_type());
  assert!(T_NULL.is_simple_built_in_type());
  assert!(T_NUMBER.is_simple_built_in_type());
  assert!(T_STRING.is_simple_built_in_type());
  assert!(T_TIME.is_simple_built_in_type());
  assert!(T_YEARS_AND_MONTHS_DURATION.is_simple_built_in_type());
  assert!(!T_CONTEXT_A.is_simple_built_in_type());
  assert!(!T_FUNCTION_A.is_simple_built_in_type());
  assert!(!T_LIST_B.is_simple_built_in_type());
  assert!(!T_RANGE_A.is_simple_built_in_type());
}

#[test]
fn test_type_stringify() {
  assert_eq!("Any", T_ANY.to_string());
  assert_eq!("Null", T_NULL.to_string());
  assert_eq!("number", T_NUMBER.to_string());
  assert_eq!("boolean", T_BOOLEAN.to_string());
  assert_eq!("date", T_DATE.to_string());
  assert_eq!("time", T_TIME.to_string());
  assert_eq!("date and time", T_DATE_TIME.to_string());
  assert_eq!("days and time duration", T_DAYS_AND_TIME_DURATION.to_string());
  assert_eq!("years and months duration", T_YEARS_AND_MONTHS_DURATION.to_string());
  assert_eq!("string", T_STRING.to_string());
  assert_eq!("function<number, number>->number", T_FUNCTION_A.to_string());
  assert_eq!("function<>->Any", T_FUNCTION_D.to_string());
  assert_eq!("function<>->string", T_FUNCTION_E.to_string());
  assert_eq!("list<boolean>", T_LIST_B.to_string());
  assert_eq!("list<number>", T_LIST_A.to_string());
  assert_eq!("range<number>", T_RANGE_A.to_string());
  assert_eq!("range<date>", T_RANGE_B.to_string());
  assert_eq!("context<a: number>", T_CONTEXT_A.to_string());
  assert_eq!("context<a: number, b: boolean>", T_CONTEXT_A_B.to_string());
  assert_eq!("context<a: number, b: boolean, c: string>", T_CONTEXT_A_B_C.to_string());
}

macro_rules! gcv_ok {
  ($t:expr, $v1:expr, $v2:expr) => {
    assert_eq!($v1, $t.get_conformant_value(&$v2))
  };
  ($t:expr, $v:expr) => {
    assert_eq!($v, $t.get_conformant_value(&$v))
  };
}

macro_rules! gcv_ok_null {
  ($t:expr, $v:expr) => {
    assert_eq!(Value::Null(None), $t.get_conformant_value(&$v))
  };
}

macro_rules! gcv_err {
  ($typ:expr, $value:expr) => {
    assert_eq!(
      value_null!("type '{}' is not conformant with value '{}'", *$typ, $value),
      $typ.get_conformant_value(&$value)
    );
  };
}

#[test]
fn test_type_get_conformant_value() {
  // boolean
  let v_boolean_true = Value::Boolean(true);
  let v_boolean_false = Value::Boolean(false);
  // date
  let v_date = Value::Date(FeelDate::new(2022, 9, 26).unwrap());
  let v_date_b = Value::Date(FeelDate::new(2022, 11, 30).unwrap());
  // date time
  let v_date_time = Value::DateTime(FeelDateTime::new(FeelDate::new(2022, 9, 27).unwrap(), FeelTime::local_opt(9, 2, 0, 0).unwrap()));
  // days and time duration
  let v_days_and_time_duration = Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::from_s(100));
  // null
  let v_null = value_null!();
  // number
  let v_number_1 = value_number!(1);
  let v_number_2 = value_number!(2);
  // string
  let v_string = Value::String("alpha".to_string());
  // time
  let v_time = Value::Time(FeelTime::local_opt(9, 2, 0, 0).unwrap());
  // years and months duration
  let v_years_and_months_duration = Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::from_ym(2, 3));
  // list
  let v_list_a = Value::List(vec![v_boolean_true.clone(), v_boolean_false.clone()]);
  let v_list_b = Value::List(vec![v_number_1.clone(), v_number_2.clone()]);
  // contexts
  let mut ctx_a = FeelContext::default();
  ctx_a.set_entry(&NAME_A, v_number_1.clone());
  let v_context_a = Value::Context(ctx_a);
  let mut ctx_a_b = FeelContext::default();
  ctx_a_b.set_entry(&NAME_A, v_number_1.clone());
  ctx_a_b.set_entry(&NAME_B, v_boolean_false.clone());
  let v_context_a_b = Value::Context(ctx_a_b);
  let mut ctx_a_b_c = FeelContext::default();
  ctx_a_b_c.set_entry(&NAME_A, v_number_1.clone());
  ctx_a_b_c.set_entry(&NAME_B, v_boolean_false.clone());
  ctx_a_b_c.set_entry(&NAME_C, v_string.clone());
  let v_context_a_b_c = Value::Context(ctx_a_b_c);
  let mut ctx_d = FeelContext::default();
  ctx_d.set_entry(&NAME_D, v_number_1.clone());
  let v_context_d = Value::Context(ctx_d);
  // functions
  let v_function_a = Value::FunctionDefinition(
    vec![(NAME_A.clone(), T_NUMBER.clone()), (NAME_B.clone(), T_NUMBER.clone())],
    FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &FeelScope| value_number!(1)))),
    false,
    Closure::default(),
    FeelContext::default(),
    T_NUMBER.clone(),
  );
  let v_function_b = Value::FunctionDefinition(
    vec![(NAME_A.clone(), T_NUMBER.clone()), (NAME_B.clone(), T_NUMBER.clone())],
    FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &FeelScope| value_number!(2)))),
    false,
    Closure::default(),
    FeelContext::default(),
    T_BOOLEAN.clone(),
  );
  let v_function_c = Value::FunctionDefinition(
    vec![(NAME_A.clone(), T_NUMBER.clone())],
    FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &FeelScope| value_number!(3)))),
    false,
    Closure::default(),
    FeelContext::default(),
    T_STRING.clone(),
  );
  let v_function_d = Value::FunctionDefinition(
    vec![(NAME_D.clone(), T_NUMBER.clone())],
    FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &FeelScope| value_number!(4)))),
    false,
    Closure::default(),
    FeelContext::default(),
    T_STRING.clone(),
  );
  let v_function_e = Value::FunctionDefinition(
    vec![(NAME_A.clone(), T_STRING.clone())],
    FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &FeelScope| value_number!(5)))),
    false,
    Closure::default(),
    FeelContext::default(),
    T_STRING.clone(),
  );
  // ranges
  let v_range_a = Value::Range(Box::new(v_number_1.clone()), IntervalType::Opened, Box::new(v_number_2), IntervalType::Opened);
  let v_range_b = Value::Range(Box::new(v_date.clone()), IntervalType::Opened, Box::new(v_date_b), IntervalType::Opened);
  // any
  gcv_ok!(T_ANY, v_boolean_true);
  gcv_ok!(T_ANY, v_context_a);
  gcv_ok!(T_ANY, v_date);
  gcv_ok!(T_ANY, v_date_time);
  gcv_ok!(T_ANY, v_days_and_time_duration);
  gcv_ok!(T_ANY, v_function_a);
  gcv_ok!(T_ANY, v_list_a);
  gcv_ok!(T_ANY, v_null);
  gcv_ok!(T_ANY, v_number_1);
  gcv_ok!(T_ANY, v_range_a);
  gcv_ok!(T_ANY, v_string);
  gcv_ok!(T_ANY, v_time);
  gcv_ok!(T_ANY, v_years_and_months_duration);
  // boolean
  gcv_ok!(T_BOOLEAN, v_boolean_false);
  gcv_ok!(T_BOOLEAN, v_null);
  gcv_err!(T_BOOLEAN, v_context_a);
  gcv_err!(T_BOOLEAN, v_date);
  gcv_err!(T_BOOLEAN, v_date_time);
  gcv_err!(T_BOOLEAN, v_days_and_time_duration);
  gcv_err!(T_BOOLEAN, v_function_a);
  gcv_err!(T_BOOLEAN, v_list_a);
  gcv_err!(T_BOOLEAN, v_number_1);
  gcv_err!(T_BOOLEAN, v_range_a);
  gcv_err!(T_BOOLEAN, v_string);
  gcv_err!(T_BOOLEAN, v_time);
  gcv_err!(T_BOOLEAN, v_years_and_months_duration);
  // context
  gcv_ok!(T_CONTEXT_A, v_context_a);
  gcv_ok!(T_CONTEXT_A, v_context_a, v_context_a_b);
  gcv_ok!(T_CONTEXT_A_B, v_context_a_b);
  gcv_ok!(T_CONTEXT_A_B, v_context_a_b, v_context_a_b_c);
  gcv_ok!(T_CONTEXT_A, v_null);
  gcv_ok!(T_CONTEXT_A_B_C, v_context_a_b_c);
  gcv_err!(T_CONTEXT_A_B_C, v_context_a_b);
  gcv_err!(T_CONTEXT_A_B, v_context_a);
  gcv_err!(T_CONTEXT_A_B_C, v_context_d);
  gcv_err!(T_CONTEXT_A, v_boolean_false);
  gcv_err!(T_CONTEXT_A, v_date);
  gcv_err!(T_CONTEXT_A, v_date_time);
  gcv_err!(T_CONTEXT_A, v_days_and_time_duration);
  gcv_err!(T_CONTEXT_A, v_function_a);
  gcv_err!(T_CONTEXT_A, v_list_a);
  gcv_err!(T_CONTEXT_A, v_number_1);
  gcv_err!(T_CONTEXT_A, v_range_a);
  gcv_err!(T_CONTEXT_A, v_string);
  gcv_err!(T_CONTEXT_A, v_time);
  gcv_err!(T_CONTEXT_A, v_years_and_months_duration);
  // date
  gcv_ok!(T_DATE, v_date);
  gcv_ok!(T_DATE, v_null);
  gcv_err!(T_DATE, v_boolean_true);
  gcv_err!(T_DATE, v_context_a);
  gcv_err!(T_DATE, v_date_time);
  gcv_err!(T_DATE, v_days_and_time_duration);
  gcv_err!(T_DATE, v_function_a);
  gcv_err!(T_DATE, v_list_a);
  gcv_err!(T_DATE, v_number_1);
  gcv_err!(T_DATE, v_range_a);
  gcv_err!(T_DATE, v_string);
  gcv_err!(T_DATE, v_time);
  gcv_err!(T_DATE, v_years_and_months_duration);
  // date and time
  gcv_ok!(T_DATE_TIME, v_date_time);
  gcv_ok!(T_DATE_TIME, v_null);
  gcv_err!(T_DATE_TIME, v_boolean_true);
  gcv_err!(T_DATE_TIME, v_context_a);
  gcv_err!(T_DATE_TIME, v_date);
  gcv_err!(T_DATE_TIME, v_function_a);
  gcv_err!(T_DATE_TIME, v_list_a);
  gcv_err!(T_DATE_TIME, v_number_1);
  gcv_err!(T_DATE_TIME, v_range_a);
  gcv_err!(T_DATE_TIME, v_string);
  gcv_err!(T_DATE_TIME, v_time);
  gcv_err!(T_DATE_TIME, v_years_and_months_duration);
  // days and time duration
  gcv_ok!(T_DAYS_AND_TIME_DURATION, v_days_and_time_duration);
  gcv_ok!(T_DAYS_AND_TIME_DURATION, v_null);
  gcv_err!(T_DAYS_AND_TIME_DURATION, v_boolean_true);
  gcv_err!(T_DAYS_AND_TIME_DURATION, v_context_a);
  gcv_err!(T_DAYS_AND_TIME_DURATION, v_date);
  gcv_err!(T_DAYS_AND_TIME_DURATION, v_date_time);
  gcv_err!(T_DAYS_AND_TIME_DURATION, v_function_a);
  gcv_err!(T_DAYS_AND_TIME_DURATION, v_list_a);
  gcv_err!(T_DAYS_AND_TIME_DURATION, v_number_1);
  gcv_err!(T_DAYS_AND_TIME_DURATION, v_range_a);
  gcv_err!(T_DAYS_AND_TIME_DURATION, v_string);
  gcv_err!(T_DAYS_AND_TIME_DURATION, v_time);
  gcv_err!(T_DAYS_AND_TIME_DURATION, v_years_and_months_duration);
  // function
  gcv_ok!(T_FUNCTION_A, v_function_a);
  gcv_ok!(T_FUNCTION_B, v_function_b);
  gcv_ok!(T_FUNCTION_C, v_function_c);
  gcv_ok!(T_FUNCTION_F, v_function_c);
  gcv_ok!(T_FUNCTION_A, v_null);
  gcv_err!(T_FUNCTION_A, v_function_b);
  gcv_err!(T_FUNCTION_A, v_function_c);
  gcv_err!(T_FUNCTION_A, v_function_d);
  gcv_err!(T_FUNCTION_B, v_function_a);
  gcv_err!(T_FUNCTION_B, v_function_c);
  gcv_err!(T_FUNCTION_C, v_function_a);
  gcv_err!(T_FUNCTION_C, v_function_b);
  gcv_err!(T_FUNCTION_C, v_function_e);
  gcv_err!(T_FUNCTION_A, v_boolean_false);
  gcv_err!(T_FUNCTION_A, v_context_a);
  gcv_err!(T_FUNCTION_A, v_date);
  gcv_err!(T_FUNCTION_A, v_date_time);
  gcv_err!(T_FUNCTION_A, v_days_and_time_duration);
  gcv_err!(T_FUNCTION_A, v_list_a);
  gcv_err!(T_FUNCTION_A, v_number_1);
  gcv_err!(T_FUNCTION_A, v_range_a);
  gcv_err!(T_FUNCTION_A, v_string);
  gcv_err!(T_FUNCTION_A, v_time);
  gcv_err!(T_FUNCTION_A, v_years_and_months_duration);
  // list
  gcv_ok!(T_LIST_B, v_list_a);
  gcv_ok!(T_LIST_A, v_list_b);
  gcv_ok!(T_LIST_B, v_null);
  gcv_err!(T_LIST_B, v_list_b);
  gcv_err!(T_LIST_A, v_list_a);
  gcv_err!(T_LIST_B, v_boolean_true);
  gcv_err!(T_LIST_B, v_context_a);
  gcv_err!(T_LIST_B, v_date);
  gcv_err!(T_LIST_B, v_date_time);
  gcv_err!(T_LIST_B, v_days_and_time_duration);
  gcv_err!(T_LIST_B, v_function_a);
  gcv_err!(T_LIST_B, v_number_1);
  gcv_err!(T_LIST_B, v_range_a);
  gcv_err!(T_LIST_B, v_string);
  gcv_err!(T_LIST_B, v_time);
  gcv_err!(T_LIST_B, v_years_and_months_duration);
  // null
  gcv_ok!(T_NULL, v_null);
  gcv_ok_null!(T_NULL, v_boolean_true);
  gcv_ok_null!(T_NULL, v_context_a);
  gcv_ok_null!(T_NULL, v_date);
  gcv_ok_null!(T_NULL, v_date_time);
  gcv_ok_null!(T_NULL, v_days_and_time_duration);
  gcv_ok_null!(T_NULL, v_function_a);
  gcv_ok_null!(T_NULL, v_list_a);
  gcv_ok_null!(T_NULL, v_number_1);
  gcv_ok_null!(T_NULL, v_range_a);
  gcv_ok_null!(T_NULL, v_string);
  gcv_ok_null!(T_NULL, v_time);
  gcv_ok_null!(T_NULL, v_years_and_months_duration);
  // number
  gcv_ok!(T_NUMBER, v_number_1);
  gcv_ok!(T_NUMBER, v_null);
  gcv_err!(T_NUMBER, v_boolean_false);
  gcv_err!(T_NUMBER, v_context_a);
  gcv_err!(T_NUMBER, v_date);
  gcv_err!(T_NUMBER, v_date_time);
  gcv_err!(T_NUMBER, v_days_and_time_duration);
  gcv_err!(T_NUMBER, v_function_a);
  gcv_err!(T_NUMBER, v_list_a);
  gcv_err!(T_NUMBER, v_range_a);
  gcv_err!(T_NUMBER, v_string);
  gcv_err!(T_NUMBER, v_time);
  gcv_err!(T_NUMBER, v_years_and_months_duration);
  // range
  gcv_ok!(T_RANGE_A, v_range_a);
  gcv_ok!(T_RANGE_B, v_range_b);
  gcv_ok!(T_RANGE_A, v_null);
  gcv_err!(T_RANGE_A, v_boolean_true);
  gcv_err!(T_RANGE_A, v_context_a);
  gcv_err!(T_RANGE_A, v_date);
  gcv_err!(T_RANGE_A, v_date_time);
  gcv_err!(T_RANGE_A, v_days_and_time_duration);
  gcv_err!(T_RANGE_A, v_function_a);
  gcv_err!(T_RANGE_A, v_list_a);
  gcv_err!(T_RANGE_A, v_number_1);
  gcv_err!(T_RANGE_A, v_string);
  gcv_err!(T_RANGE_A, v_time);
  gcv_err!(T_RANGE_A, v_years_and_months_duration);
  // string
  gcv_ok!(T_STRING, v_string);
  gcv_ok!(T_STRING, v_null);
  gcv_err!(T_STRING, v_boolean_false);
  gcv_err!(T_STRING, v_context_a);
  gcv_err!(T_STRING, v_date);
  gcv_err!(T_STRING, v_date_time);
  gcv_err!(T_STRING, v_days_and_time_duration);
  gcv_err!(T_STRING, v_function_a);
  gcv_err!(T_STRING, v_list_a);
  gcv_err!(T_STRING, v_number_1);
  gcv_err!(T_STRING, v_range_a);
  gcv_err!(T_STRING, v_time);
  gcv_err!(T_STRING, v_years_and_months_duration);
  // time
  gcv_ok!(T_TIME, v_time);
  gcv_ok!(T_TIME, v_null);
  gcv_err!(T_TIME, v_boolean_true);
  gcv_err!(T_TIME, v_context_a);
  gcv_err!(T_TIME, v_date);
  gcv_err!(T_TIME, v_date_time);
  gcv_err!(T_TIME, v_days_and_time_duration);
  gcv_err!(T_TIME, v_function_a);
  gcv_err!(T_TIME, v_list_a);
  gcv_err!(T_TIME, v_number_1);
  gcv_err!(T_TIME, v_range_a);
  gcv_err!(T_TIME, v_string);
  gcv_err!(T_TIME, v_years_and_months_duration);
  // years and months duration
  gcv_ok!(T_YEARS_AND_MONTHS_DURATION, v_years_and_months_duration);
  gcv_ok!(T_YEARS_AND_MONTHS_DURATION, v_null);
  gcv_err!(T_YEARS_AND_MONTHS_DURATION, v_boolean_true);
  gcv_err!(T_YEARS_AND_MONTHS_DURATION, v_context_a);
  gcv_err!(T_YEARS_AND_MONTHS_DURATION, v_date);
  gcv_err!(T_YEARS_AND_MONTHS_DURATION, v_date_time);
  gcv_err!(T_YEARS_AND_MONTHS_DURATION, v_days_and_time_duration);
  gcv_err!(T_YEARS_AND_MONTHS_DURATION, v_function_a);
  gcv_err!(T_YEARS_AND_MONTHS_DURATION, v_list_a);
  gcv_err!(T_YEARS_AND_MONTHS_DURATION, v_number_1);
  gcv_err!(T_YEARS_AND_MONTHS_DURATION, v_range_a);
  gcv_err!(T_YEARS_AND_MONTHS_DURATION, v_string);
  gcv_err!(T_YEARS_AND_MONTHS_DURATION, v_time);
}
