use super::*;

#[test]
fn _0001() {
    let scope = &te_scope(
        r#"{DeptTable:[{manager:"Smith",name:"Sales",number:10},{manager:"Jones",name:"Finance",number:20},{manager:"King",name:"Engineering",number:30}],EmployeeTable:[{deptNum:10,id:"7792",name:"Clark"},{deptNum:10,id:"7934",name:"Miller"},{deptNum:20,id:"7976",name:"Adams"},{deptNum:20,id:"7902",name:"Ford"},{deptNum:30,id:"7900",name:"James"}],LastName:"Clark"}"#,
    );
    te_be_value(
        false,
        scope,
        r#"EmployeeTable[name=LastName]"#,
        r#"[{deptNum:10,id:"7792",name:"Clark"}]"#,
    );
    te_be_value(
        false,
        scope,
        r#"EmployeeTable[name=LastName].deptNum"#,
        r#"[10]"#,
    );
    te_value(
        false,
        scope,
        r#"EmployeeTable[name=LastName].deptNum[1]"#,
        r#"10"#,
    );
    te_be_value(
        false,
        scope,
        r#"DeptTable[number=10]"#,
        r#"[{manager:"Smith",name:"Sales",number:10}]"#,
    );
    te_value(
        false,
        scope,
        r#"DeptTable[number=10].manager[1]"#,
        r#""Smith""#,
    );
    te_value(
        false,
        scope,
        r#"DeptTable[number=EmployeeTable[name=LastName].deptNum[1]].manager[1]"#,
        r#""Smith""#,
    );
}
