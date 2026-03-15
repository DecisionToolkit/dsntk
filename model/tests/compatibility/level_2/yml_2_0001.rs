use dsntk_examples::*;
use dsntk_model::{from_xml, from_yaml};

#[test]
fn _0001() {
  let xml = from_xml(DMN_2_0001).unwrap();
  let yaml = from_yaml(YAML_2_0001).unwrap();
  assert_eq!(xml, yaml);
}
