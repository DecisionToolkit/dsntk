use super::*;

from_examples!(DMN_3_1155);

static_context!(CTX, "{}");

fn eq(invocable: &str, expected: &str) {
  assert_decision(&MODEL_EVALUATOR, &MODEL_NAMESPACE, &MODEL_NAME, invocable, &CTX, expected);
}

#[test]
fn _0001() {
  eq("decision001", "[1, 4, 3]");
}

#[test]
fn _0002() {
  eq("decision002", "[1, 2, 4]");
}

#[test]
fn _0003() {
  eq("decision003", "null(position must not be zero)");
}

#[test]
fn _0004() {
  eq("decision004", "null(position exceeds list bounds)");
}

#[test]
fn _0005() {
  eq("decision005", "null(position exceeds list bounds)");
}

#[test]
fn _0006() {
  eq("decision006", "null([core::list replace] invalid argument type, expected list, actual type is Null)");
}

#[test]
fn _0007() {
  eq("decision007", "null(position must be number or function)");
}

#[test]
fn _0008() {
  eq("decision008", "[1, 2, null]");
}

#[test]
fn _0009() {
  eq("decision009", "[5, 5, 7, 8]");
}

#[test]
fn _0010() {
  eq("decision010", "null(position must be number or function)");
}

#[test]
fn _0011() {
  eq("decision011", "[1, 4, 3]");
}

#[test]
fn _0012() {
  eq("decision011_a", "[1, 2, 4]");
}

#[test]
fn _0013() {
  eq("decision012", "[1, 4, 3]");
}

#[test]
fn _0014() {
  eq("decision013", "[1, 4, 3]");
}

#[test]
fn _0015() {
  eq("decision014", "null(expected 3 parameters, actual number of parameters is 4)");
}

#[test]
fn _0016() {
  eq("decision015", "null(expected 3 parameters, actual number of parameters is 2)");
}

#[test]
fn _0017() {
  eq("decision016", "null(expected 3 parameters, actual number of parameters is 4)");
}

#[test]
fn _0018() {
  eq("decision017", "null(list replace: matching function must have exactly two arguments)");
}

#[test]
fn _0019() {
  eq("decision018", "null(list replace: matching function must have exactly two arguments)");
}

#[test]
fn _0020() {
  eq("decision019", "null(list replace: matching function must return boolean value)");
}

#[test]
fn _0021() {
  eq("decision020", "[5, 5, 5, 5]");
}

#[test]
fn _0022() {
  eq("decision021", "[5]");
}
