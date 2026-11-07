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
        t_input_clauses(&[(r#"Applicant age"#, None), (r#"Medical history"#, None)]),
        // expected output clauses
        t_output_clauses(&[
            (r#"Applicant risk rating"#.into(), None, None),
            (r#"Special discount"#.into(), None, None),
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
    t_eq(&expected, from_unicode(V_110222, false).unwrap());
    t_eq(&expected, from_markdown(V_110222, false).unwrap());
}
