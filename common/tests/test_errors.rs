use dsntk_common::DsntkError;

#[test]
fn test_new() {
  assert_eq!("<TestError> unexpected", format!("{}", DsntkError::new("TestError", "unexpected")));
}

#[test]
fn test_debug() {
  assert_eq!(r#"DsntkError("<TestError> unexpected")"#, format!("{:?}", DsntkError::new("TestError", "unexpected")));
}

#[test]
fn test_equal() {
  let err1 = DsntkError::new("TestError", "unexpected");
  let err2 = DsntkError::new("TestError", "unexpected");
  assert!((err1 == err2));
}

#[test]
fn test_not_equal() {
  let err1 = DsntkError::new("TestError", "expected");
  let err2 = DsntkError::new("TestError", "unexpected");
  assert!((err1 != err2));
}
