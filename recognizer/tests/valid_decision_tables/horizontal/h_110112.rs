use super::*;

#[test]
fn _0001() {
    let expected = (
        // expected information item name
        r#"Days of week"#.into(),
        // expected output label
        r#"Weekday"#.into(),
        // expected hit policy
        HitPolicy::First,
        // expected aggregation
        None,
        // expected preferred orientation
        DecisionTableOrientation::RulesAsRows,
        // expected input clauses
        t_input_clauses(&[(r#"Day"#, None)]),
        // expected output clauses
        t_output_clauses(&[(r#"Weekday"#.into(), None, None)]),
        // expected annotation clauses
        t_annotation_clauses(&[r#"Description"#, r#"Reference"#]),
        // expected rules
        t_rules(&[
            (&[r#"1"#], &[r#""Monday""#], &["Desc 1", "Ref 1"]),
            (&[r#"1"#], &[r#""Tuesday""#], &["Desc 2", "Ref 2"]),
            (&[r#"2"#], &[r#""Wednesday""#], &["Desc 3", "Ref 3"]),
            (&[r#"2"#], &[r#""Thursday""#], &["Desc 4", "Ref 4"]),
            (&[r#"3"#], &[r#""Friday""#], &["Desc 5", "Ref 5"]),
            (&[r#"3"#], &[r#""Saturday""#], &["Desc 6", "Ref 6"]),
            (&[r#"0"#], &[r#""Sunday""#], &["Desc 7", "Ref 7"]),
        ]),
    );
    t_eq(&expected, from_unicode(H_110112, false).unwrap());
    t_eq(&expected, from_markdown(H_110112, false).unwrap());
}
