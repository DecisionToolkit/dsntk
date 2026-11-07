use super::*;

#[test]
fn _0001() {
    let expected = (
        // expected information item name
        "Days of week".into(),
        // expected output label
        "Weekday".into(),
        // expected hit policy
        HitPolicy::Collect(BuiltinAggregator::List),
        // expected aggregation
        BuiltinAggregator::List.into(),
        // expected preferred orientation
        DecisionTableOrientation::RulesAsRows,
        // expected input clauses
        t_input_clauses(&[]),
        // expected output clauses
        t_output_clauses(&[("Weekday".into(), None, None)]),
        // expected annotation clauses
        t_annotation_clauses(&[]),
        // expected rules
        t_rules(&[
            (&[], &[r#""Monday""#], &[]),
            (&[], &[r#""Tuesday""#], &[]),
            (&[], &[r#""Wednesday""#], &[]),
            (&[], &[r#""Thursday""#], &[]),
            (&[], &[r#""Friday""#], &[]),
            (&[], &[r#""Saturday""#], &[]),
            (&[], &[r#""Sunday""#], &[]),
        ]),
    );
    t_eq(&expected, from_unicode(H_110010, false).unwrap());
    t_eq(&expected, from_markdown(H_110010, false).unwrap());
}
