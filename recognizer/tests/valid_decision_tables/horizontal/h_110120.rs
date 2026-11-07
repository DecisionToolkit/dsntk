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
        t_input_clauses(&[(r#"Day"#, None)]),
        // expected output clauses
        t_output_clauses(&[
            (r#"Weekday"#.into(), None, None),
            (r#"Short name"#.into(), None, None),
        ]),
        // expected annotation clauses
        t_annotation_clauses(&[]),
        // expected rules
        t_rules(&[
            (&[r#"1"#], &[r#""Monday""#, r#""Mon""#], &[]),
            (&[r#"1"#], &[r#""Tuesday""#, r#""Tue""#], &[]),
            (&[r#"2"#], &[r#""Wednesday""#, r#""Wed""#], &[]),
            (&[r#"2"#], &[r#""Thursday""#, r#""Thu""#], &[]),
            (&[r#"3"#], &[r#""Friday""#, r#""Fri""#], &[]),
            (&[r#"3"#], &[r#""Saturday""#, r#""Sat""#], &[]),
            (&[r#"0"#], &[r#""Sunday""#, r#""Sun""#], &[]),
        ]),
    );
    t_eq(&expected, from_unicode(H_110120, false).unwrap());
    t_eq(&expected, from_markdown(H_110120, false).unwrap());
}
