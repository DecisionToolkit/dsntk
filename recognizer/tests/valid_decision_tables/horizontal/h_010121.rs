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
        t_input_clauses(&[(r#"Day"#, None)]),
        // expected output clauses
        t_output_clauses(&[
            (r#"Weekday"#.into(), None, None),
            (r#"Short name"#.into(), None, None),
        ]),
        // expected annotation clauses
        t_annotation_clauses(&[r#"Description"#]),
        // expected rules
        t_rules(&[
            (&[r#"1"#], &[r#""Monday""#, r#""Mon""#], &["Desc 1"]),
            (&[r#"1"#], &[r#""Tuesday""#, r#""Tue""#], &["Desc 2"]),
            (&[r#"2"#], &[r#""Wednesday""#, r#""Wed""#], &["Desc 3"]),
            (&[r#"2"#], &[r#""Thursday""#, r#""Thu""#], &["Desc 4"]),
            (&[r#"3"#], &[r#""Friday""#, r#""Fri""#], &["Desc 5"]),
            (&[r#"3"#], &[r#""Saturday""#, r#""Sat""#], &["Desc 6"]),
            (&[r#"0"#], &[r#""Sunday""#, r#""Sun""#], &["Desc 7"]),
        ]),
    );
    t_eq(&expected, from_unicode(H_010121, false).unwrap());
    t_eq(&expected, from_markdown(H_010121, false).unwrap());
}
