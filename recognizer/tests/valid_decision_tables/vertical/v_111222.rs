use super::*;

#[test]
fn _0001() {
    let expected = (
        // expected information item name
        r#"Offered selling options"#.into(),
        // expected output label
        r#"Sell options"#.into(),
        // expected hit policy
        HitPolicy::Unique,
        // expected aggregation
        None,
        // expected preferred orientation
        DecisionTableOrientation::RulesAsColumns,
        // expected input clauses
        t_input_clauses(&[
            (r#"Applicant age"#, r#"<25, [25..60], >60"#.into()),
            (r#"Medical history"#, r#""good", "bad""#.into()),
        ]),
        // expected output clauses
        t_output_clauses(&[
            (
                r#"Applicant risk rating"#.into(),
                r#""Low", "Medium", "High""#.into(),
                r#""High""#.into(),
            ),
            (r#"Special discount"#.into(), r#"0, 5, 10"#.into(), None),
        ]),
        // expected annotation clauses
        t_annotation_clauses(&[r#"Additional acceptance"#, r#"Reference"#]),
        // expected rules
        t_rules(&[
            (
                &[r#"<25"#, r#""good""#],
                &[r#""Low""#, r#"10"#],
                &[r#"No"#, r#"Ref 1"#],
            ),
            (
                &[r#"<25"#, r#""bad""#],
                &[r#""Medium""#, r#"5"#],
                &[r#"No"#, r#"Ref 2"#],
            ),
            (
                &[r#"[25..60]"#, r#"-"#],
                &[r#""Medium""#, r#"5"#],
                &[r#"No"#, r#"Ref 3"#],
            ),
            (
                &[r#">60"#, r#""good""#],
                &[r#""Medium""#, r#"5"#],
                &[r#"No"#, r#"Ref 4"#],
            ),
            (
                &[r#">60"#, r#""bad""#],
                &[r#""High""#, r#"0"#],
                &[r#"Yes"#, r#"Ref 5"#],
            ),
        ]),
    );
    t_eq(&expected, from_unicode(V_111222, false).unwrap());
    t_eq(&expected, from_markdown(V_111222, false).unwrap());
}
