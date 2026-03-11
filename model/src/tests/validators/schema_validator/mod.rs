use crate::validators::validate_schema;
use dsntk_common::DsntkError;
use roxmltree::Document;

mod test_files;
mod test_full_model_v13;
mod test_full_model_v14;
mod test_full_model_v15;
mod test_input_data_v13;
mod test_input_data_v14;
mod test_root_element_v13;

fn document(input: &str) -> Document<'_> {
  Document::parse(input).unwrap()
}

fn expect_err(input: &str) -> DsntkError {
  validate_schema(&document(input)).unwrap_err()
}

fn expect_err_str(input: &str) -> String {
  expect_err(input).to_string()
}
