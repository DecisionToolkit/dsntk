use dsntk_feel::FeelType;
use std::ops::Deref;

/// Type reference enumeration for simple FEEL types.
#[derive(Debug, PartialEq)]
pub enum TypeRef {
    Any(FeelType),
    String(FeelType),
    Number(FeelType),
    Boolean(FeelType),
    Date(FeelType),
    Time(FeelType),
    DateTime(FeelType),
    DayTimeDuration(FeelType),
    YearMonthDuration(FeelType),
}

impl Deref for TypeRef {
    type Target = FeelType;
    /// Unwraps inner [FeelType].
    fn deref(&self) -> &Self::Target {
        match self {
            TypeRef::Any(feel_type) => feel_type,
            TypeRef::String(feel_type) => feel_type,
            TypeRef::Number(feel_type) => feel_type,
            TypeRef::Boolean(feel_type) => feel_type,
            TypeRef::Date(feel_type) => feel_type,
            TypeRef::Time(feel_type) => feel_type,
            TypeRef::DateTime(feel_type) => feel_type,
            TypeRef::DayTimeDuration(feel_type) => feel_type,
            TypeRef::YearMonthDuration(feel_type) => feel_type,
        }
    }
}

/// Converts simple FEEL type reference into [FeelType] wrapped with [TypeRef].
pub fn type_ref_to_feel_type(type_ref: &str) -> Option<TypeRef> {
    match type_ref.trim() {
        "Any" => Some(TypeRef::Any(FeelType::Any)),
        "string" => Some(TypeRef::String(FeelType::String)),
        "number" => Some(TypeRef::Number(FeelType::Number)),
        "boolean" => Some(TypeRef::Boolean(FeelType::Boolean)),
        "date" => Some(TypeRef::Date(FeelType::Date)),
        "time" => Some(TypeRef::Time(FeelType::Time)),
        "dateTime" => Some(TypeRef::DateTime(FeelType::DateTime)),
        "dayTimeDuration" => Some(TypeRef::DayTimeDuration(FeelType::DaysAndTimeDuration)),
        "yearMonthDuration" => Some(TypeRef::YearMonthDuration(FeelType::YearsAndMonthsDuration)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_ref_to_feel_type() {
        assert_eq!(
            Some(TypeRef::Any(FeelType::Any)),
            type_ref_to_feel_type("Any")
        );
        assert_eq!(
            Some(TypeRef::String(FeelType::String)),
            type_ref_to_feel_type("string")
        );
        assert_eq!(
            Some(TypeRef::Number(FeelType::Number)),
            type_ref_to_feel_type("number")
        );
        assert_eq!(
            Some(TypeRef::Boolean(FeelType::Boolean)),
            type_ref_to_feel_type("boolean")
        );
        assert_eq!(
            Some(TypeRef::Date(FeelType::Date)),
            type_ref_to_feel_type("date")
        );
        assert_eq!(
            Some(TypeRef::Time(FeelType::Time)),
            type_ref_to_feel_type("time")
        );
        assert_eq!(
            Some(TypeRef::DateTime(FeelType::DateTime)),
            type_ref_to_feel_type("dateTime")
        );
        assert_eq!(
            Some(TypeRef::DayTimeDuration(FeelType::DaysAndTimeDuration)),
            type_ref_to_feel_type("dayTimeDuration")
        );
        assert_eq!(
            Some(TypeRef::YearMonthDuration(FeelType::YearsAndMonthsDuration)),
            type_ref_to_feel_type("yearMonthDuration")
        );
        assert_eq!(None, type_ref_to_feel_type("date and time"));
        assert_eq!(None, type_ref_to_feel_type("days and time duration"));
        assert_eq!(None, type_ref_to_feel_type("years and months duration"));
        assert_eq!(None, type_ref_to_feel_type("text"));
    }

    #[test]
    fn test_type_ref_equality() {
        assert!((type_ref_to_feel_type("Any") == type_ref_to_feel_type("Any")));
        assert!((type_ref_to_feel_type("string") == type_ref_to_feel_type("string")));
        assert!(!(type_ref_to_feel_type("string") == type_ref_to_feel_type("number")));
        assert!(!(type_ref_to_feel_type("string") == type_ref_to_feel_type("boolean")));
        assert!(!(type_ref_to_feel_type("string") == type_ref_to_feel_type("date")));
        assert!(!(type_ref_to_feel_type("string") == type_ref_to_feel_type("time")));
        assert!(!(type_ref_to_feel_type("string") == type_ref_to_feel_type("dayTimeDuration")));
        assert!(!(type_ref_to_feel_type("string") == type_ref_to_feel_type("yearMonthDuration")));
    }

    #[test]
    fn test_type_ref_debug() {
        assert_eq!(
            "Some(Any(Any))",
            format!("{:?}", type_ref_to_feel_type("Any"))
        );
        assert_eq!(
            "Some(String(String))",
            format!("{:?}", type_ref_to_feel_type("string"))
        );
        assert_eq!(
            "Some(Number(Number))",
            format!("{:?}", type_ref_to_feel_type("number"))
        );
        assert_eq!(
            "Some(Boolean(Boolean))",
            format!("{:?}", type_ref_to_feel_type("boolean"))
        );
        assert_eq!(
            "Some(Date(Date))",
            format!("{:?}", type_ref_to_feel_type("date"))
        );
        assert_eq!(
            "Some(Time(Time))",
            format!("{:?}", type_ref_to_feel_type("time"))
        );
        assert_eq!(
            "Some(DateTime(DateTime))",
            format!("{:?}", type_ref_to_feel_type("dateTime"))
        );
        assert_eq!(
            "Some(DayTimeDuration(DaysAndTimeDuration))",
            format!("{:?}", type_ref_to_feel_type("dayTimeDuration"))
        );
        assert_eq!(
            "Some(YearMonthDuration(YearsAndMonthsDuration))",
            format!("{:?}", type_ref_to_feel_type("yearMonthDuration"))
        );
    }
}
