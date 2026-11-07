use super::*;

#[test]
fn _0001() {
    let expected = (
        // expected information item name
        r#"Days of week"#.into(),
        // expected output label
        r#"Days"#.into(),
        // expected hit policy
        HitPolicy::Collect(BuiltinAggregator::List),
        // expected aggregation
        BuiltinAggregator::List.into(),
        // expected preferred orientation
        DecisionTableOrientation::RulesAsRows,
        // expected input clauses
        t_input_clauses(&[]),
        // expected output clauses
        t_output_clauses(&[
            (r#"Weekday"#.into(), None, None),
            (r#"Short name"#.into(), None, None),
        ]),
        // expected annotation clauses
        t_annotation_clauses(&[]),
        // expected rules
        t_rules(&[
            (&[], &[r#""Monday""#, r#""Mon""#], &[]),
            (&[], &[r#""Tuesday""#, r#""Tue""#], &[]),
            (&[], &[r#""Wednesday""#, r#""Wed""#], &[]),
            (&[], &[r#""Thursday""#, r#""Thu""#], &[]),
            (&[], &[r#""Friday""#, r#""Fri""#], &[]),
            (&[], &[r#""Saturday""#, r#""Sat""#], &[]),
            (&[], &[r#""Sunday""#, r#""Sun""#], &[]),
        ]),
    );
    t_eq(&expected, from_unicode(H_110020, false).unwrap());
    t_eq(&expected, from_markdown(H_110020, false).unwrap());
}
