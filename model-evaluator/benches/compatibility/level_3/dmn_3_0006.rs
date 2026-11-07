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

#[bench]
fn _0001(b: &mut Bencher) {
    let ctx = context(INPUT_DATA);
    let invocable_name = "Join";
    assert_decision(
        &MODEL_EVALUATOR,
        &MODEL_NAMESPACE,
        &MODEL_NAME,
        invocable_name,
        &ctx,
        r#""Smith""#,
    );
    iter!(
        b,
        MODEL_EVALUATOR.evaluate_invocable(&MODEL_NAMESPACE, &MODEL_NAME, invocable_name, &ctx)
    );
}
