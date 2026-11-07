use super::*;

#[test]
fn _0001() {
    let expected = (
        // expected information item name
        r#"Days of week"#.into(),
        // expected output label
        None,
        // expected hit policy
        HitPolicy::Collect(BuiltinAggregator::List),
        // expected aggregation
        BuiltinAggregator::List.into(),
        // expected preferred orientation
        DecisionTableOrientation::RulesAsRows,
        // expected input clauses
        t_input_clauses(&[]),
        // expected output clauses
        t_output_clauses(&[(
            None,
            r#""Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday""#
                .into(),
            None,
        )]),
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
    t_eq(&expected, from_unicode(H_101010, false).unwrap());
    t_eq(&expected, from_markdown(H_101010, false).unwrap());
}
