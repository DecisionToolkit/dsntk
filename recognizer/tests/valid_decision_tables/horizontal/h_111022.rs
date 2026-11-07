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
        t_annotation_clauses(&[r#"Description"#, r#"Reference"#]),
        // expected rules
        t_rules(&[
            (&[], &[r#""Monday""#, r#""Mon""#], &["Desc 1", "Ref 1"]),
            (&[], &[r#""Tuesday""#, r#""Tue""#], &["Desc 2", "Ref 2"]),
            (&[], &[r#""Wednesday""#, r#""Wed""#], &["Desc 3", "Ref 3"]),
            (&[], &[r#""Thursday""#, r#""Thu""#], &["Desc 4", "Ref 4"]),
            (&[], &[r#""Friday""#, r#""Fri""#], &["Desc 5", "Ref 5"]),
            (&[], &[r#""Saturday""#, r#""Sat""#], &["Desc 6", "Ref 6"]),
            (&[], &[r#""Sunday""#, r#""Sun""#], &["Desc 7", "Ref 7"]),
        ]),
    );
    t_eq(&expected, from_unicode(H_111022, false).unwrap());
    t_eq(&expected, from_markdown(H_111022, false).unwrap());
}
