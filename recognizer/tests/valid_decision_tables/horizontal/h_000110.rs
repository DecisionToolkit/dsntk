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
        t_input_clauses(&[("Day", None)]),
        // expected output clauses
        t_output_clauses(&[(None, None, None)]),
        // expected annotation clauses
        t_annotation_clauses(&[]),
        // expected rules
        t_rules(&[
            (&["1"], &[r#""Monday""#], &[]),
            (&["2"], &[r#""Tuesday""#], &[]),
            (&["3"], &[r#""Wednesday""#], &[]),
            (&["4"], &[r#""Thursday""#], &[]),
            (&["5"], &[r#""Friday""#], &[]),
            (&["6"], &[r#""Saturday""#], &[]),
            (&["0"], &[r#""Sunday""#], &[]),
        ]),
    );
    t_eq(&expected, from_unicode(H_000110, false).unwrap());
    t_eq(&expected, from_markdown(H_000110, false).unwrap());
}
