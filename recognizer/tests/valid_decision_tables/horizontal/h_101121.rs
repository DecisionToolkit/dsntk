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
        t_annotation_clauses(&[r#"Description"#]),
        // expected rules
        t_rules(&[
            (&[r#"1"#], &[r#""Monday""#, r#""Mon""#], &["Desc 1"]),
            (&[r#"2"#], &[r#""Tuesday""#, r#""Tue""#], &["Desc 2"]),
            (&[r#"3"#], &[r#""Wednesday""#, r#""Wed""#], &["Desc 3"]),
            (&[r#"4"#], &[r#""Thursday""#, r#""Thu""#], &["Desc 4"]),
            (&[r#"5"#], &[r#""Friday""#, r#""Fri""#], &["Desc 5"]),
            (&[r#"6"#], &[r#""Saturday""#, r#""Sat""#], &["Desc 6"]),
            (&[r#"0"#], &[r#""Sunday""#, r#""Sun""#], &["Desc 7"]),
        ]),
    );
    t_eq(&expected, from_unicode(H_101121, false).unwrap());
    t_eq(&expected, from_markdown(H_101121, false).unwrap());
}
