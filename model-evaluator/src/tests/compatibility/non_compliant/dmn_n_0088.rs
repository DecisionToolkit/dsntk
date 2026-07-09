use dsntk_examples::DMN_N_0088;

#[test]
fn _0001() {
  assert_eq!(
    "<ModelValidatorError> cyclic dependency between item definitions",
    dsntk_model::from_xml(DMN_N_0088).err().unwrap().to_string()
  );
}
