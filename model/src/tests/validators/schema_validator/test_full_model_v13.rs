use super::test_files::*;
use super::*;

#[test]
fn _9999() {
    validate_schema(&document(v13::_9999)).unwrap();
}
