use super::*;

#[test]
fn _0001() {
    let expected = (
        // expected information item name
        None,
        // expected output label
        None,
        // expected hit policy
        HitPolicy::Collect(BuiltinAggregator::List),
        // expected aggregation
        BuiltinAggregator::List.into(),
        // expected preferred orientation
        DecisionTableOrientation::RulesAsRows,
        // expected input clauses
        t_input_clauses(&[]),
        // expected output clauses
        t_output_clauses(&[(None, None, None)]),
        // expected annotation clauses
        t_annotation_clauses(&["Description"]),
        // expected rules
        t_rules(&[
            (&[], &[r#""Monday""#], &["Desc 1"]),
            (&[], &[r#""Tuesday""#], &["Desc 2"]),
            (&[], &[r#""Wednesday""#], &["Desc 3"]),
            (&[], &[r#""Thursday""#], &["Desc 4"]),
            (&[], &[r#""Friday""#], &["Desc 5"]),
            (&[], &[r#""Saturday""#], &["Desc 6"]),
            (&[], &[r#""Sunday""#], &["Desc 7"]),
        ]),
    );
    t_eq(&expected, from_unicode(H_000011, false).unwrap());
    t_eq(&expected, from_markdown(H_000011, false).unwrap());
}
