use super::*;

#[test]
fn _0001() {
    let expected = (
        // expected information item name
        None,
        // expected output label
        None,
        // expected hit policy
        HitPolicy::Unique,
        // expected aggregation
        None,
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
            (&[r#"2"#], &[r#""Tuesday""#, r#""Tue""#], &[]),
            (&[r#"3"#], &[r#""Wednesday""#, r#""Wed""#], &[]),
            (&[r#"4"#], &[r#""Thursday""#, r#""Thu""#], &[]),
            (&[r#"5"#], &[r#""Friday""#, r#""Fri""#], &[]),
            (&[r#"6"#], &[r#""Saturday""#, r#""Sat""#], &[]),
            (&[r#"0"#], &[r#""Sunday""#, r#""Sun""#], &[]),
        ]),
    );
    t_eq(&expected, from_unicode(H_000120, false).unwrap());
    t_eq(&expected, from_markdown(H_000120, false).unwrap());
}
