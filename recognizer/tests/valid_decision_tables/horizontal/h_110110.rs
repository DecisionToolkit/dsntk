use super::*;

#[test]
fn _0001() {
    let expected = (
        // expected information item name
        r#"Days of week"#.into(),
        // expected output label
        r#"Weekday"#.into(),
        // expected hit policy
        HitPolicy::Unique,
        // expected aggregation
        None,
        // expected preferred orientation
        DecisionTableOrientation::RulesAsRows,
        // expected input clauses
        t_input_clauses(&[(r#"Day"#, None)]),
        // expected output clauses
        t_output_clauses(&[(r#"Weekday"#.into(), None, None)]),
        // expected annotation clauses
        t_annotation_clauses(&[]),
        // expected rules
        t_rules(&[
            (&[r#"1"#], &[r#""Monday""#], &[]),
            (&[r#"2"#], &[r#""Tuesday""#], &[]),
            (&[r#"3"#], &[r#""Wednesday""#], &[]),
            (&[r#"4"#], &[r#""Thursday""#], &[]),
            (&[r#"5"#], &[r#""Friday""#], &[]),
            (&[r#"6"#], &[r#""Saturday""#], &[]),
            (&[r#"0"#], &[r#""Sunday""#], &[]),
        ]),
    );
    t_eq(&expected, from_unicode(H_110110, false).unwrap());
    t_eq(&expected, from_markdown(H_110110, false).unwrap());
}
