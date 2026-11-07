use super::*;

#[test]
fn _0001() {
    let expected = (
        // expected information item name
        None,
        // expected output label
        r#"Days"#.into(),
        // expected hit policy
        HitPolicy::Collect(BuiltinAggregator::List),
        // expected aggregation
        BuiltinAggregator::List.into(),
        // expected preferred orientation
        DecisionTableOrientation::RulesAsRows,
        // expected input clauses
        t_input_clauses(&[(r#"Day"#, r#"0, 1, 2, 3, 4, 5, 6"#.into())]),
        // expected output clauses
        t_output_clauses(&[
            (
                r#"Weekday"#.into(),
                r#""Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday""#
                    .into(),
                None,
            ),
            (
                r#"Short name"#.into(),
                r#""Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun""#.into(),
                None,
            ),
        ]),
        // expected annotation clauses
        t_annotation_clauses(&[]),
        // expected rules
        t_rules(&[
            (&[r#"1"#], &[r#""Monday""#, r#""Mon""#], &[]),
            (&[r#"2"#], &[r#""Tuesday""#, r#""Tue""#], &[]),
            (&[r#"3"#], &[r#""Wednesday""#, r#""Wed""#], &[]),
            (&[r#"4"#], &[r#""Thursday""#, r#""Thu""#], &[]),
            (&[r#"5"#], &[r#""Friday""#, r#""Fri""#], &[]),
            (&[r#"6"#], &[r#""Saturday""#, r#""Sat""#], &[]),
            (&[r#"0"#], &[r#""Sunday""#, r#""Sun""#], &[]),
        ]),
    );
    t_eq(&expected, from_unicode(H_011120, false).unwrap());
    t_eq(&expected, from_markdown(H_011120, false).unwrap());
}
