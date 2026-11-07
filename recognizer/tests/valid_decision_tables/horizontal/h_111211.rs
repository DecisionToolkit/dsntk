use super::*;

#[test]
fn _0001() {
    let expected = (
        // expected information item name
        r#"Offered order options"#.into(),
        // expected output label
        r#"Discount"#.into(),
        // expected hit policy
        HitPolicy::Unique,
        // expected aggregation
        None,
        // expected preferred orientation
        DecisionTableOrientation::RulesAsRows,
        // expected input clauses
        t_input_clauses(&[
            (r#"Customer"#, r#""Business", "Private""#.into()),
            (r#"Order"#, r#"<10, >=10"#.into()),
        ]),
        // expected output clauses
        t_output_clauses(&[(r#"Discount"#.into(), r#"0.05, 0.10, 0.15"#.into(), None)]),
        // expected annotation clauses
        t_annotation_clauses(&[r#"Description"#]),
        // expected rules
        t_rules(&[
            (&[r#""Business""#, r#"<10"#], &[r#"0.10"#], &[r#"Desc 1"#]),
            (&[r#""Business""#, r#">=10"#], &[r#"0.15"#], &[r#"Desc 2"#]),
            (&[r#""Private""#, r#"-"#], &[r"0.05"], &[r#"Desc 3"#]),
        ]),
    );
    t_eq(&expected, from_unicode(H_111211, false).unwrap());
    t_eq(&expected, from_markdown(H_111211, false).unwrap());
}
