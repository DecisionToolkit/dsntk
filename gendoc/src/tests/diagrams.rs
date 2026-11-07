use super::*;

macro_rules! test_export_model {
    ($test_name:tt, $model_name:tt) => {
        #[test]
        fn $test_name() {
            export_model!($model_name);
        }
    };
}

test_export_model!(_0001, DIAG_0001);
