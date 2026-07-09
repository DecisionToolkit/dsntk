use crate::from_xml;
use crate::model::DmnElement;
use dsntk_examples::DMN_FULL;

#[test]
fn _0001() {
  let definitions = from_xml(DMN_FULL).unwrap();
  assert_eq!("_id_definitions", definitions.id());
  //------------------------------------------------------------------------------------------------
  // ITEM DEFINITIONS
  //------------------------------------------------------------------------------------------------
  assert_eq!(0, definitions.item_definitions().len());
  //------------------------------------------------------------------------------------------------
  // EXTENSION ELEMENTS IN DEFINITIONS
  //------------------------------------------------------------------------------------------------
  assert_eq!(0, definitions.extension_elements().len());
  //------------------------------------------------------------------------------------------------
  // DRG ELEMENTS
  //------------------------------------------------------------------------------------------------
  assert_eq!(1, definitions.drg_elements().len());
  assert_eq!(1, definitions.decisions().len());
  assert_eq!(0, definitions.input_data().len());
  //------------------------------------------------------------------------------------------------
  // DMNDI
  //------------------------------------------------------------------------------------------------
  assert!(definitions.dmndi().is_none());
}

/// Covers all cloning and debugging functions of `Definitions`.
#[test]
#[allow(clippy::redundant_clone)]
fn _0002() {
  let definitions = from_xml(DMN_FULL).unwrap();
  let cloned_definitions = definitions.clone();
  assert_eq!("_id_definitions", cloned_definitions.id());
  let expected = format!("{definitions:?}");
  assert_eq!(expected, format!("{cloned_definitions:?}"));
}
