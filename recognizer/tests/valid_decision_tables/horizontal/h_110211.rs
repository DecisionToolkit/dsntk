use super::*;

#[test]
fn _0001() {
    let expected = (
        // expected information item name
        "Offered order options".into(),
        // expected output label
        "Discount".into(),
        // expected hit policy
        HitPolicy::Unique,
        // expected aggregation
        None,
        // expected preferred orientation
        DecisionTableOrientation::RulesAsRows,
        // expected input clauses
        t_input_clauses(&[("Customer", None), ("Order", None)]),
        // expected output clauses
        t_output_clauses(&[("Discount".into(), None, None)]),
        // expected annotation clauses
        t_annotation_clauses(&["Description"]),
        // expected rules
        t_rules(&[
            (&[r#""Business""#, "<10"], &["0.10"], &["Desc 1"]),
            (&[r#""Business""#, ">=10"], &["0.15"], &["Desc 2"]),
            (&[r#""Private""#, "-"], &["0.05"], &["Desc 3"]),
        ]),
    );
    t_eq(&expected, from_unicode(H_110211, false).unwrap());
    t_eq(&expected, from_markdown(H_110211, false).unwrap());
}
