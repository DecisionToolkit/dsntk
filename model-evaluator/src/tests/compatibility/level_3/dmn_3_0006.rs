use super::*;

from_examples!(DMN_3_0006);

const INPUT_DATA: &str = r#"
  {
    DeptTable:  [
      {
        manager: "Smith",
        name: "Sales",
        number: 10
      },
      {
        manager: "Jones",
        name: "Finance",
        number: 20
      },
      {
        manager: "King",
        name: "Engineering",
        number: 30
      }
    ],
    EmployeeTable:  [
      {
        deptNum: 10,
        id: "7792",
        name: "Clark"
      },
      {
        deptNum: 10,
        id: "7934",
        name: "Miller"
      },
      {
        deptNum: 20,
        id: "7976",
        name: "Adams"
      },
      {
        deptNum: 20,
        id: "7902",
        name: "Ford"
      },
      {
        deptNum: 30,
        id: "7900",
        name: "James"
      }
    ],
    LastName: "Clark"
  }
"#;

#[test]
fn _0001() {
    let ctx = context(INPUT_DATA);
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        "Join",
        &ctx,
        r#""Smith""#,
    );
}
