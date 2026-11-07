//! Decision table orientation tests.

use crate::model::DecisionTableOrientation;
use dsntk_common::Result;

#[test]
fn test_try_from() {
    assert!(DecisionTableOrientation::RuleAsRow == "Rule-as-Row".try_into().unwrap());
    assert!(DecisionTableOrientation::RuleAsColumn == "Rule-as-Column".try_into().unwrap());
    assert!(DecisionTableOrientation::CrossTable == "CrossTable".try_into().unwrap());
    let dt_orientation: Result<DecisionTableOrientation> = "Rounded".try_into();
    assert!(dt_orientation.is_err());
}
