use super::*;

#[test]
fn _0001() {
    let expected = (
        // expected information item name
        None,
        // expected output label
        None,
        // expected hit policy
        HitPolicy::First,
        // expected aggregation
        None,
        // expected preferred orientation
        DecisionTableOrientation::RulesAsRows,
        // expected input clauses
        t_input_clauses(&[(r#"Day"#, None)]),
        // expected output clauses
        t_output_clauses(&[(None, None, None)]),
        // expected annotation clauses
        t_annotation_clauses(&[r#"Description"#]),
        // expected rules
        t_rules(&[
            (&[r#"1"#], &[r#""Monday""#], &["Desc 1"]),
            (&[r#"2"#], &[r#""Tuesday""#], &["Desc 2"]),
            (&[r#"3"#], &[r#""Wednesday""#], &["Desc 3"]),
            (&[r#"4"#], &[r#""Thursday""#], &["Desc 4"]),
            (&[r#"5"#], &[r#""Friday""#], &["Desc 5"]),
            (&[r#"6"#], &[r#""Saturday""#], &["Desc 6"]),
            (&[r#"0"#], &[r#""Sunday""#], &["Desc 7"]),
        ]),
    );
    t_eq(&expected, from_unicode(H_000111, false).unwrap());
    t_eq(&expected, from_markdown(H_000111, false).unwrap());
}
