/// Builds `null` value with a message informing about invalid argument type.
macro_rules! invalid_argument_type {
    ($function:literal, $expected:expr, $actual:expr) => {{
        use dsntk_feel::value_null;
        use std::file;
        use std::path::Path;
        value_null!(
            Path::new(file!()).file_stem().unwrap().to_string_lossy(),
            $function,
            "{}",
            format!(
                "invalid argument type, expected {}, actual type is {}",
                $expected, $actual
            )
        )
    }};
}

pub(crate) use invalid_argument_type;

/// Builds `null` value with a message informing about invalid parameter count.
macro_rules! invalid_number_of_parameters {
    ($expected:expr, $actual:expr) => {{
        use dsntk_feel::value_null;
        value_null!(
            "expected {} parameters, actual number of parameters is {}",
            $expected,
            $actual
        )
    }};
}

pub(crate) use invalid_number_of_parameters;
