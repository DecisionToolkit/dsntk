//! Hit policy tests.

use crate::model::{BuiltinAggregator, HitPolicy};
use dsntk_common::Result;

#[test]
fn test_display() {
    assert_eq!("U", format!("{}", HitPolicy::Unique));
    assert_eq!("A", format!("{}", HitPolicy::Any));
    assert_eq!("P", format!("{}", HitPolicy::Priority));
    assert_eq!("F", format!("{}", HitPolicy::First));
    assert_eq!(
        "C",
        format!("{}", HitPolicy::Collect(BuiltinAggregator::List))
    );
    assert_eq!(
        "C#",
        format!("{}", HitPolicy::Collect(BuiltinAggregator::Count))
    );
    assert_eq!(
        "C+",
        format!("{}", HitPolicy::Collect(BuiltinAggregator::Sum))
    );
    assert_eq!(
        "C<",
        format!("{}", HitPolicy::Collect(BuiltinAggregator::Min))
    );
    assert_eq!(
        "C>",
        format!("{}", HitPolicy::Collect(BuiltinAggregator::Max))
    );
    assert_eq!("O", format!("{}", HitPolicy::OutputOrder));
    assert_eq!("R", format!("{}", HitPolicy::RuleOrder));
}

#[test]
fn test_debug() {
    assert_eq!("Unique", format!("{:?}", HitPolicy::Unique));
    assert_eq!("Any", format!("{:?}", HitPolicy::Any));
    assert_eq!("Priority", format!("{:?}", HitPolicy::Priority));
    assert_eq!("First", format!("{:?}", HitPolicy::First));
    assert_eq!(
        "Collect(List)",
        format!("{:?}", HitPolicy::Collect(BuiltinAggregator::List))
    );
    assert_eq!(
        "Collect(Count)",
        format!("{:?}", HitPolicy::Collect(BuiltinAggregator::Count))
    );
    assert_eq!(
        "Collect(Sum)",
        format!("{:?}", HitPolicy::Collect(BuiltinAggregator::Sum))
    );
    assert_eq!(
        "Collect(Min)",
        format!("{:?}", HitPolicy::Collect(BuiltinAggregator::Min))
    );
    assert_eq!(
        "Collect(Max)",
        format!("{:?}", HitPolicy::Collect(BuiltinAggregator::Max))
    );
    assert_eq!("OutputOrder", format!("{:?}", HitPolicy::OutputOrder));
    assert_eq!("RuleOrder", format!("{:?}", HitPolicy::RuleOrder));
}

#[test]
fn test_equality() {
    assert!((HitPolicy::Unique == HitPolicy::Unique));
    assert!((HitPolicy::Any == HitPolicy::Any));
    assert!((HitPolicy::Priority == HitPolicy::Priority));
    assert!((HitPolicy::First == HitPolicy::First));
    assert!(
        (HitPolicy::Collect(BuiltinAggregator::List)
            == HitPolicy::Collect(BuiltinAggregator::List))
    );
    assert!(
        (HitPolicy::Collect(BuiltinAggregator::Count)
            == HitPolicy::Collect(BuiltinAggregator::Count))
    );
    assert!(
        (HitPolicy::Collect(BuiltinAggregator::Sum) == HitPolicy::Collect(BuiltinAggregator::Sum))
    );
    assert!(
        (HitPolicy::Collect(BuiltinAggregator::Min) == HitPolicy::Collect(BuiltinAggregator::Min))
    );
    assert!(
        (HitPolicy::Collect(BuiltinAggregator::Max) == HitPolicy::Collect(BuiltinAggregator::Max))
    );
    assert!((HitPolicy::OutputOrder == HitPolicy::OutputOrder));
    assert!((HitPolicy::RuleOrder == HitPolicy::RuleOrder));
    assert!((HitPolicy::Unique != HitPolicy::First));
    assert!((HitPolicy::Any != HitPolicy::OutputOrder));
    assert!((HitPolicy::Priority != HitPolicy::Collect(BuiltinAggregator::Count)));
}

#[test]
fn test_try_from() {
    assert!(HitPolicy::Unique == "U".try_into().unwrap());
    assert!(HitPolicy::Any == "A".try_into().unwrap());
    assert!(HitPolicy::Priority == "P".try_into().unwrap());
    assert!(HitPolicy::First == "F".try_into().unwrap());
    assert!(HitPolicy::Collect(BuiltinAggregator::List) == "C".try_into().unwrap());
    assert!(HitPolicy::Collect(BuiltinAggregator::Count) == "C#".try_into().unwrap());
    assert!(HitPolicy::Collect(BuiltinAggregator::Sum) == "C+".try_into().unwrap());
    assert!(HitPolicy::Collect(BuiltinAggregator::Min) == "C<".try_into().unwrap());
    assert!(HitPolicy::Collect(BuiltinAggregator::Max) == "C>".try_into().unwrap());
    assert!(HitPolicy::OutputOrder == "O".try_into().unwrap());
    assert!(HitPolicy::RuleOrder == "R".try_into().unwrap());
    let hit_policy: Result<HitPolicy> = "K".try_into();
    assert!(hit_policy.is_err());
}
