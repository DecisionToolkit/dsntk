//! # Tests for generating ASCII report for DMN models

use crate::ascii_model::print_model;
use antex::ColorMode;

macro_rules! test_print_model {
  ($test_name:tt,$model_name:tt) => {
    #[test]
    #[allow(clippy::redundant_clone)]
    fn $test_name() {
      let definitions = dsntk_model::from_xml(dsntk_examples::$model_name).expect("parsing model failed");
      print_model(definitions.clone(), ColorMode::On);
      let expected = format!("{:?}", definitions);
      let actual = format!("{:?}", definitions);
      assert_eq!(expected, actual);
    }
  };
}

test_print_model!(_2_0001, DMN_2_0001);
test_print_model!(_2_0002, DMN_2_0002);
test_print_model!(_2_0003, DMN_2_0003);
test_print_model!(_2_0004, DMN_2_0004);
test_print_model!(_2_0005, DMN_2_0005);
test_print_model!(_2_0006, DMN_2_0006);
test_print_model!(_2_0007, DMN_2_0007);
test_print_model!(_2_0008, DMN_2_0008);
test_print_model!(_2_0009, DMN_2_0009);
test_print_model!(_2_0010, DMN_2_0010);
test_print_model!(_2_0100, DMN_2_0100);
test_print_model!(_2_0101, DMN_2_0101);
test_print_model!(_2_0102, DMN_2_0102);
test_print_model!(_2_0105, DMN_2_0105);
test_print_model!(_2_0106, DMN_2_0106);
test_print_model!(_2_0107, DMN_2_0107);
test_print_model!(_2_0108, DMN_2_0108);
test_print_model!(_2_0109, DMN_2_0109);
test_print_model!(_2_0110, DMN_2_0110);
test_print_model!(_2_0111, DMN_2_0111);
test_print_model!(_2_0112, DMN_2_0112);
test_print_model!(_2_0113, DMN_2_0113);
test_print_model!(_2_0114, DMN_2_0114);
test_print_model!(_2_0115, DMN_2_0115);
test_print_model!(_2_0116, DMN_2_0116);
test_print_model!(_2_0117, DMN_2_0117);
test_print_model!(_2_0118, DMN_2_0118);
test_print_model!(_2_0119, DMN_2_0119);

test_print_model!(_3_0001, DMN_3_0001);
test_print_model!(_3_0002, DMN_3_0002);
test_print_model!(_3_0003, DMN_3_0003);
test_print_model!(_3_0004, DMN_3_0004);
test_print_model!(_3_0005, DMN_3_0005);
test_print_model!(_3_0006, DMN_3_0006);
test_print_model!(_3_0007, DMN_3_0007);
test_print_model!(_3_0008, DMN_3_0008);
test_print_model!(_3_0009, DMN_3_0009);
test_print_model!(_3_0010, DMN_3_0010);
test_print_model!(_3_0011, DMN_3_0011);
test_print_model!(_3_0012, DMN_3_0012);
test_print_model!(_3_0013, DMN_3_0013);
test_print_model!(_3_0014, DMN_3_0014);
test_print_model!(_3_0016, DMN_3_0016);
test_print_model!(_3_0017, DMN_3_0017);
test_print_model!(_3_0020, DMN_3_0020);
test_print_model!(_3_0021, DMN_3_0021);
test_print_model!(_3_0030, DMN_3_0030);
test_print_model!(_3_0031, DMN_3_0031);
test_print_model!(_3_0032, DMN_3_0032);
test_print_model!(_3_0033, DMN_3_0033);
test_print_model!(_3_0034, DMN_3_0034);
test_print_model!(_3_0035, DMN_3_0035);
test_print_model!(_3_0036, DMN_3_0036);
test_print_model!(_3_0037, DMN_3_0037);
test_print_model!(_3_0038, DMN_3_0038);
test_print_model!(_3_0039, DMN_3_0039);
test_print_model!(_3_0040, DMN_3_0040);
test_print_model!(_3_0041, DMN_3_0041);
test_print_model!(_3_0050, DMN_3_0050);
test_print_model!(_3_0051, DMN_3_0051);
test_print_model!(_3_0052, DMN_3_0052);
test_print_model!(_3_0053, DMN_3_0053);
test_print_model!(_3_0054, DMN_3_0054);
test_print_model!(_3_0055, DMN_3_0055);
test_print_model!(_3_0056, DMN_3_0056);
test_print_model!(_3_0057, DMN_3_0057);
test_print_model!(_3_0058, DMN_3_0058);
test_print_model!(_3_0059, DMN_3_0059);
test_print_model!(_3_0060, DMN_3_0060);
test_print_model!(_3_0061, DMN_3_0061);
test_print_model!(_3_0062, DMN_3_0062);
test_print_model!(_3_0063, DMN_3_0063);
test_print_model!(_3_0064, DMN_3_0064);
test_print_model!(_3_0065, DMN_3_0065);
test_print_model!(_3_0066, DMN_3_0066);
test_print_model!(_3_0067, DMN_3_0067);
test_print_model!(_3_0068, DMN_3_0068);
test_print_model!(_3_0069, DMN_3_0069);
test_print_model!(_3_0070, DMN_3_0070);
test_print_model!(_3_0071, DMN_3_0071);
test_print_model!(_3_0072, DMN_3_0072);
test_print_model!(_3_0073, DMN_3_0073);
test_print_model!(_3_0074, DMN_3_0074);
test_print_model!(_3_0075, DMN_3_0075);
test_print_model!(_3_0076, DMN_3_0076);
test_print_model!(_3_0077, DMN_3_0077);
test_print_model!(_3_0078, DMN_3_0078);
test_print_model!(_3_0080, DMN_3_0080);
test_print_model!(_3_0081, DMN_3_0081);
test_print_model!(_3_0082, DMN_3_0082);
test_print_model!(_3_0083, DMN_3_0083);
test_print_model!(_3_0084, DMN_3_0084);
test_print_model!(_3_0085, DMN_3_0085);
test_print_model!(_3_0086, DMN_3_0086);
test_print_model!(_3_0086_import, DMN_3_0086_IMPORT);
test_print_model!(_3_0087, DMN_3_0087);
test_print_model!(_3_0088, DMN_3_0088);
test_print_model!(_3_0089_model_a, DMN_3_0089_MODEL_A);
test_print_model!(_3_0089_model_b1, DMN_3_0089_MODEL_B1);
test_print_model!(_3_0089_model_b2, DMN_3_0089_MODEL_B2);
test_print_model!(_3_0089_model_c, DMN_3_0089_MODEL_C);
test_print_model!(_3_0090, DMN_3_0090);
test_print_model!(_3_0091, DMN_3_0091);
test_print_model!(_3_0092, DMN_3_0092);
test_print_model!(_3_0093, DMN_3_0093);
test_print_model!(_3_0094, DMN_3_0094);
test_print_model!(_3_0095, DMN_3_0095);
test_print_model!(_3_0096, DMN_3_0096);
test_print_model!(_3_0097, DMN_3_0097);
test_print_model!(_3_0098, DMN_3_0098);
test_print_model!(_3_0099, DMN_3_0099);
test_print_model!(_3_0100, DMN_3_0100);
test_print_model!(_3_0103, DMN_3_0103);
test_print_model!(_3_1100, DMN_3_1100);
test_print_model!(_3_1101, DMN_3_1101);
test_print_model!(_3_1102, DMN_3_1102);
test_print_model!(_3_1103, DMN_3_1103);
test_print_model!(_3_1104, DMN_3_1104);
test_print_model!(_3_1105, DMN_3_1105);
test_print_model!(_3_1106, DMN_3_1106);
test_print_model!(_3_1107, DMN_3_1107);
test_print_model!(_3_1108, DMN_3_1108);
test_print_model!(_3_1109, DMN_3_1109);
test_print_model!(_3_1110, DMN_3_1110);
test_print_model!(_3_1115, DMN_3_1115);
test_print_model!(_3_1116, DMN_3_1116);
test_print_model!(_3_1117, DMN_3_1117);
test_print_model!(_3_1120, DMN_3_1120);
test_print_model!(_3_1121, DMN_3_1121);
test_print_model!(_3_1130, DMN_3_1130);

#[test]
fn test_single_model() {
  let definitions = dsntk_model::from_xml(dsntk_examples::DMN_3_1108).expect("parsing model failed");
  print_model(definitions, ColorMode::On);
}

#[test]
fn test_full_model() {
  let definitions = dsntk_model::from_xml(dsntk_examples::DMN_FULL).expect("parsing model failed");
  print_model(definitions, ColorMode::On);
}
