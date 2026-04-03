//! Common definitions for tests.

#[macro_export]
macro_rules! num {
  ($n:expr) => {{ stringify!($n).parse::<FeelNumber>().unwrap() }};
}

#[macro_export]
macro_rules! eqs {
  ($expected:literal, $actual:expr) => {{ assert_eq!($expected, format!("{}", $actual)) }};
}
