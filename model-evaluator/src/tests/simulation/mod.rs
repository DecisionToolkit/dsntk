mod v13 {
    use crate::tests::*;
    const SIMULATION_V13: &str = include_str!("simulation_v13.dmn");
    model_evaluator!(SIMULATION_V13);
    const MODEL_NAMESPACE: &str = "https://camunda.org/schema/1.0/dmn";
    const MODEL_NAME: &str = "Dinner Decisions";

    #[test]
    fn _0001() {
        let ctx = context(
            r#"{ Season: "Spring", "Number of Guests": 2, "Guests with children?": true }"#,
        );
        let invocable = "Dish";
        let expected = r#""Dry Aged Gourmet Steak""#;
        assert_decision(
            &MODEL_EVALUATOR,
            MODEL_NAMESPACE,
            MODEL_NAME,
            invocable,
            &ctx,
            expected,
        );
    }

    #[test]
    fn _0002() {
        let ctx = context(
            r#"{ Season: "Spring", "Number of Guests": 2, "Guests with children?": true }"#,
        );
        let invocable = "Beverages";
        let expected = r#"["Pinot Noir", "Apple Juice"]"#;
        assert_decision(
            &MODEL_EVALUATOR,
            MODEL_NAMESPACE,
            MODEL_NAME,
            invocable,
            &ctx,
            expected,
        );
    }
}

mod v14 {
    use crate::tests::*;
    const SIMULATION_V14: &str = include_str!("simulation_v14.dmn");
    model_evaluator!(SIMULATION_V14);
    const MODEL_NAMESPACE: &str = "https://camunda.org/schema/1.0/dmn";
    const MODEL_NAME: &str = "Dinner Decisions";

    #[test]
    fn _0001() {
        let ctx = context(
            r#"{ Season: "Spring", "Number of Guests": 2, "Guests with children?": true }"#,
        );
        let invocable = "Dish";
        let expected = r#""Dry Aged Gourmet Steak""#;
        assert_decision(
            &MODEL_EVALUATOR,
            MODEL_NAMESPACE,
            MODEL_NAME,
            invocable,
            &ctx,
            expected,
        );
    }

    #[test]
    fn _0002() {
        let ctx = context(
            r#"{ Season: "Spring", "Number of Guests": 2, "Guests with children?": true }"#,
        );
        let invocable = "Beverages";
        let expected = r#"["Pinot Noir", "Apple Juice"]"#;
        assert_decision(
            &MODEL_EVALUATOR,
            MODEL_NAMESPACE,
            MODEL_NAME,
            invocable,
            &ctx,
            expected,
        );
    }
}

mod v15 {
    use crate::tests::*;
    const SIMULATION_V15: &str = include_str!("simulation_v15.dmn");
    model_evaluator!(SIMULATION_V15);
    const MODEL_NAMESPACE: &str = "https://camunda.org/schema/1.0/dmn";
    const MODEL_NAME: &str = "Dinner Decisions";

    #[test]
    fn _0001() {
        let ctx = context(
            r#"{ Season: "Spring", "Number of Guests": 2, "Guests with children?": true }"#,
        );
        let invocable = "Dish";
        let expected = r#""Dry Aged Gourmet Steak""#;
        assert_decision(
            &MODEL_EVALUATOR,
            MODEL_NAMESPACE,
            MODEL_NAME,
            invocable,
            &ctx,
            expected,
        );
    }

    #[test]
    fn _0002() {
        let ctx = context(
            r#"{ Season: "Spring", "Number of Guests": 2, "Guests with children?": true }"#,
        );
        let invocable = "Beverages";
        let expected = r#"["Pinot Noir", "Apple Juice"]"#;
        assert_decision(
            &MODEL_EVALUATOR,
            MODEL_NAMESPACE,
            MODEL_NAME,
            invocable,
            &ctx,
            expected,
        );
    }
}
