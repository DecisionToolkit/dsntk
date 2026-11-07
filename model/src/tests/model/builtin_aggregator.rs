//! BuiltinAggregator tests.

use crate::model::BuiltinAggregator;

#[test]
fn test_display() {
    assert_eq!("C", format!("{}", BuiltinAggregator::List));
    assert_eq!("C#", format!("{}", BuiltinAggregator::Count));
    assert_eq!("C+", format!("{}", BuiltinAggregator::Sum));
    assert_eq!("C<", format!("{}", BuiltinAggregator::Min));
    assert_eq!("C>", format!("{}", BuiltinAggregator::Max));
}

#[test]
fn test_debug() {
    assert_eq!("List", format!("{:?}", BuiltinAggregator::List));
    assert_eq!("Count", format!("{:?}", BuiltinAggregator::Count));
    assert_eq!("Sum", format!("{:?}", BuiltinAggregator::Sum));
    assert_eq!("Min", format!("{:?}", BuiltinAggregator::Min));
    assert_eq!("Max", format!("{:?}", BuiltinAggregator::Max));
}

#[test]
fn test_equality() {
    assert!((BuiltinAggregator::List == BuiltinAggregator::List));
    assert!((BuiltinAggregator::Count == BuiltinAggregator::Count));
    assert!((BuiltinAggregator::Sum == BuiltinAggregator::Sum));
    assert!((BuiltinAggregator::Min == BuiltinAggregator::Min));
    assert!((BuiltinAggregator::Max == BuiltinAggregator::Max));
    assert!((BuiltinAggregator::Max != BuiltinAggregator::Min));
    assert!((BuiltinAggregator::Sum != BuiltinAggregator::Count));
    assert!((BuiltinAggregator::List != BuiltinAggregator::Sum));
}
