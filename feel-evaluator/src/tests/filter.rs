use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    let scope = &te_scope(r#"{A:[1,3,5]}"#);
    te_null(
        false,
        scope,
        "A[0]",
        "index in filter is out of range [1..3], actual index is 0",
    );
    te_null(
        false,
        scope,
        "A[4]",
        "index in filter is out of range [1..3], actual index is 4",
    );
    te_null(
        false,
        scope,
        "A[-4]",
        "index in filter is out of range [-3..-1], actual index is -4",
    );
}

#[test]
fn _0002() {
    let scope = &te_scope(r#"{A:[1,3,5]}"#);
    te_number(false, scope, "A[1]", 1, 0);
    te_number(false, scope, "A[-3]", 1, 0);
}

#[test]
fn _0003() {
    let scope = &te_scope(r#"{A:[]}"#);
    te_null(false, scope, "A[1]", "");
}

#[test]
fn _0004() {
    let scope = &te_scope(r#"{A:[]}"#);
    te_null(false, scope, "A[2]", "");
}

#[test]
fn _0005() {
    let scope = &te_scope(r#"{A:[]}"#);
    te_null(false, scope, "A[-1]", "");
}

#[test]
fn _0006() {
    let scope = &te_scope(r#"{A:[]}"#);
    te_null(false, scope, "A[-2]", "");
}

#[test]
fn _0007() {
    let scope = &te_scope(r#"{A:[1,3,5]}"#);
    te_number(false, scope, "A[2]", 3, 0);
    te_number(false, scope, "A[-2]", 3, 0);
}

#[test]
fn _0008() {
    let scope = &te_scope(r#"{A:[1,3,5]}"#);
    te_number(false, scope, "A[3]", 5, 0);
    te_number(false, scope, "A[-1]", 5, 0);
}

#[test]
fn _0009() {
    let scope = &te_scope(r#"{B:["A","B","C","D","E"]}"#);
    te_null(
        false,
        scope,
        "B[0]",
        "index in filter is out of range [1..5], actual index is 0",
    );
    te_null(
        false,
        scope,
        "B[6]",
        "index in filter is out of range [1..5], actual index is 6",
    );
    te_null(
        false,
        scope,
        "B[-6]",
        "index in filter is out of range [-5..-1], actual index is -6",
    );
}

#[test]
fn _0010() {
    let scope = &te_scope(r#"{B:["A","B","C","D","E"]}"#);
    te_string(false, scope, "B[1]", "A");
    te_string(false, scope, "B[-5]", "A");
}

#[test]
fn _0011() {
    let scope = &te_scope(r#"{B:["A","B","C","D","E"]}"#);
    te_string(false, scope, "B[2]", "B");
    te_string(false, scope, "B[-4]", "B");
}

#[test]
fn _0012() {
    let scope = &te_scope(r#"{B:["A","B","C","D","E"]}"#);
    te_string(false, scope, "B[3]", "C");
    te_string(false, scope, "B[-3]", "C");
}

#[test]
fn _0013() {
    let scope = &te_scope(r#"{B:["A","B","C","D","E"]}"#);
    te_string(false, scope, "B[4]", "D");
    te_string(false, scope, "B[-2]", "D");
}

#[test]
fn _0014() {
    let scope = &te_scope(r#"{B:["A","B","C","D","E"]}"#);
    te_string(false, scope, "B[5]", "E");
    te_string(false, scope, "B[-1]", "E");
}

#[test]
fn _0015() {
    let scope = &te_scope(r#"{A:[1,3,5],B:["A","B","C","D","E"]}"#);
    te_string(false, scope, "B[A[1]]", "A");
}

#[test]
fn _0016() {
    let scope = &te_scope(r#"{A:[1,3,5],B:["A","B","C","D","E"]}"#);
    te_string(false, scope, "B[A[2]]", "C");
}

#[test]
fn _0017() {
    let scope = &te_scope(r#"{A:[1,3,5],B:["A","B","C","D","E"]}"#);
    te_string(false, scope, "B[A[3]]", "E");
}

#[test]
fn _0018() {
    let scope = &te_scope(r#"{l:[{x:1,y:2},{x:null,y:3}]}"#);
    te_be_value(false, scope, "l[x<2]", "[{x:1,y:2}]");
}

#[test]
fn _0019() {
    te_be_value(
        false,
        &scope!(),
        "[{x:1,y:2},{x:null,y:3}][x<2]",
        "[{x:1,y:2}]",
    );
}

#[test]
fn _0020() {
    te_be_value(
        false,
        &scope!(),
        "[{x:1,y:2},{x:null,y:3}][y>2]",
        "[{x:null,y:3}]",
    );
}

#[test]
fn _0021() {
    te_be_value(
        false,
        &scope!(),
        "[1,2,3,4,5,6,7,8,9,10][item>=5]",
        "[5,6,7,8,9,10]",
    );
}

#[test]
fn _0022() {
    let scope = &te_scope(r#"{l:[{x:1,y:2},{x:2,y:4},{x:3,y:6}]}"#);
    te_be_value(false, scope, "l[2]", "{x:2,y:4}");
}

#[test]
fn _0023() {
    let scope = &te_scope(r#"{l:[{x:1,y:2},{x:2,y:4},{x:3,y:6}]}"#);
    te_be_value(false, scope, "l[x=2]", "[{x:2,y:4}]");
}

#[test]
fn _0024() {
    let scope = &te_scope(r#"{l:[{x:1,y:2},{x:2,y:4},{x:3,y:6}]}"#);
    te_be_value(false, scope, "l[x=2].y", "[4]");
}

#[test]
fn _0025() {
    te_number(false, &scope!(), "[1,2,3,4,5,6][1]", 1, 0);
}

#[test]
fn _0026() {
    te_be_value(false, &scope!(), "[1,2,3,4,5,6][item=4]", r#"[4]"#);
}

#[test]
fn _0027() {
    te_be_value(false, &scope!(), "[1,2,3,4,5,6][item>=4]", "[4,5,6]");
}

#[test]
fn _0028() {
    te_number(false, &scope!(), "[1,2,3,4,5,6][1][1]", 1, 0);
}

#[test]
fn _0029() {
    te_number(false, &scope!(), "[1,2,3,4,5,6][2]", 2, 0);
}

#[test]
fn _0030() {
    te_number(false, &scope!(), "[1,2,3,4,5,6][3]", 3, 0);
}

#[test]
fn _0031() {
    te_number(false, &scope!(), "[1,2,3,4,5,6][4]", 4, 0);
}

#[test]
fn _0032() {
    te_number(false, &scope!(), "[1,2,3,4,5,6][5]", 5, 0);
}

#[test]
fn _0033() {
    te_number(false, &scope!(), "[1,2,3,4,5,6][6]", 6, 0);
}

#[test]
fn _0034() {
    te_number(false, &scope!(), "[1,2,3,4,5,6][-1]", 6, 0);
}

#[test]
fn _0035() {
    te_number(false, &scope!(), "[1,2,3,4,5,6][-2]", 5, 0);
}

#[test]
fn _0036() {
    te_number(false, &scope!(), "[1,2,3,4,5,6][-3]", 4, 0);
}

#[test]
fn _0037() {
    te_number(false, &scope!(), "[1,2,3,4,5,6][-4]", 3, 0);
}

#[test]
fn _0038() {
    te_number(false, &scope!(), "[1,2,3,4,5,6][-5]", 2, 0);
}

#[test]
fn _0039() {
    te_number(false, &scope!(), "[1,2,3,4,5,6][-6]", 1, 0);
}

#[test]
fn _0040() {
    te_null(
        false,
        &scope!(),
        "[1,2,3,4,5,6][0]",
        "index in filter is out of range [1..6], actual index is 0",
    );
}

#[test]
fn _0041() {
    te_null(
        false,
        &scope!(),
        "[1,2,3,4,5,6][7]",
        "index in filter is out of range [1..6], actual index is 7",
    );
}

#[test]
fn _0042() {
    te_null(
        false,
        &scope!(),
        "[1,2,3,4,5,6][-7]",
        "index in filter is out of range [-6..-1], actual index is -7",
    );
}

#[test]
fn _0043() {
    te_be_value(false, &scope!(), r#" true[true] "#, r#"[true]"#);
}

#[test]
fn _0044() {
    te_be_value(false, &scope!(), r#" true[false] "#, r#"[]"#);
}

#[test]
fn _0045() {
    te_date(false, &scope!(), r#" @"2023-02-06"[1] "#, 2023, 2, 6);
}

#[test]
fn _0046() {
    te_date_time_local(
        false,
        &scope!(),
        r#" @"2023-02-06T10:11:12"[1] "#,
        (2023, 2, 6),
        (10, 11, 12, 0),
    );
}

#[test]
fn _0047() {
    te_time(
        false,
        &scope!(),
        r#" @"10:11:12Z"[-1] "#,
        FeelTime::utc(10, 11, 12, 0),
    );
}

#[test]
fn _0048() {
    te_days_and_time_duration_x(false, &scope!(), r#" @"P1DT2H3M15S"[1] "#, "P1DT2H3M15S");
}

#[test]
fn _0049() {
    te_years_and_months_duration_x(false, &scope!(), r#" @"P5Y4M"[1] "#, "P5Y4M");
}

#[test]
fn _0050() {
    be_be_value(
        false,
        &scope!(),
        r#"[{a: 1}, {a: 2}, {a: 3}]"#,
        r#"[{a:1},{a:2},{a:3}]"#,
    );
}

#[test]
fn _0051() {
    te_be_value(
        false,
        &scope!(),
        r#"[{a: 1}, {a: 2}, {a: 3}][1]"#,
        r#"{a:1}"#,
    );
}

#[test]
fn _0052() {
    te_be_value(
        false,
        &scope!(),
        r#"[{item: 1}, {item: 2}, {item: 3}][item >= 2]"#,
        r#"[{item:2},{item:3}]"#,
    );
}

#[test]
fn _0053() {
    te_be_value(
        false,
        &scope!(),
        r#"[{a: 1}, {a: 2}, {a: 3}][item.a=2]"#,
        r#"[{a:2}]"#,
    );
}

#[test]
fn _0054() {
    te_be_value(
        false,
        &scope!(),
        r#"[{a: 1}, {a: 2}, {a: 3}][item.a >= 2]"#,
        r#"[{a:2},{a:3}]"#,
    );
}

#[test]
fn _0055() {
    let scope = &te_scope(
        r#"{
    DeptTable:    [ {manager:"Smith",name:"Sales",number:10},
                    {manager:"Jones",name:"Finance",number:20},
                    {manager:"King",name:"Engineering",number:30} ],
    EmployeeTable:[ {deptNum:10,id:"7792",name:"Clark"},
                    {deptNum:10,id:"7934",name:"Miller"},
                    {deptNum:20,id:"7976",name:"Adams"},
                    {deptNum:20,id:"7902",name:"Ford"},
                    {deptNum:30,id:"7900",name:"James"} ],
    LastName:"Clark"}"#,
    );
    te_string(
        false,
        scope,
        "DeptTable[number=EmployeeTable[name=LastName].deptNum[1]].manager[1]",
        "Smith",
    );
}

#[test]
fn _0056() {
    let scope = &te_scope(
        r#"
    {
       EmployeeTable:[ 
         {deptNum:10,id:"7792",name:"Clark"},
         {deptNum:10,id:"7934",name:"Miller"},
         {deptNum:20,id:"7976",name:"Adams"},
         {deptNum:20,id:"7902",name:"Ford"},
         {deptNum:30,id:"7900",name:"James"} 
       ],
       LastName:"Clark"
    }
    "#,
    );
    te_number(
        false,
        scope,
        "EmployeeTable[name=LastName].deptNum[1]",
        10,
        0,
    );
}

#[test]
fn _0057() {
    let scope = &te_scope(
        r#"
    {
       EmployeeTable:[ 
         {deptNum:10,id:"7792",name:"Clark"},
         {deptNum:10,id:"7934",name:"Miller"},
         {deptNum:20,id:"7976",name:"Adams"},
         {deptNum:20,id:"7902",name:"Ford"},
         {deptNum:30,id:"7900",name:"James"} 
       ],
       LastName:"Clark"
    }
    "#,
    );
    te_number(
        false,
        scope,
        "EmployeeTable[name=LastName].deptNum[1]",
        10,
        0,
    );
}

#[test]
fn _0058() {
    let scope = &te_scope(
        r#"{
    DeptTable:    [ {manager:"Smith",name:"Sales",number:10},
                    {manager:"Jones",name:"Finance",number:20},
                    {manager:"King",name:"Engineering",number:30} ],
    EmployeeTable:[ {deptNum:10,id:"7792",name:"Clark"},
                    {deptNum:10,id:"7934",name:"Miller"},
                    {deptNum:20,id:"7976",name:"Adams"},
                    {deptNum:20,id:"7902",name:"Ford"},
                    {deptNum:30,id:"7900",name:"James"} ],
    LastName:"Clark"}"#,
    );
    te_string(false, scope, "DeptTable[number=10].manager[1]", "Smith");
}

#[test]
fn _0059() {
    let scope = &te_scope(
        r#"{
    DeptTable:    [ {manager:"Smith",name:"Sales",number:10},
                    {manager:"Jones",name:"Finance",number:20},
                    {manager:"King",name:"Engineering",number:30} ],
    EmployeeTable:[ {deptNum:10,id:"7792",name:"Clark"},
                    {deptNum:10,id:"7934",name:"Miller"},
                    {deptNum:20,id:"7976",name:"Adams"},
                    {deptNum:20,id:"7902",name:"Ford"},
                    {deptNum:30,id:"7900",name:"James"} ],
    LastName:"Clark"}"#,
    );
    te_be_value(
        false,
        scope,
        "DeptTable[number=10]",
        r#"[{manager:"Smith",name:"Sales",number:10}]"#,
    );
}

#[test]
fn _0060() {
    let scope = &te_scope(
        r#"{
     check_eq: function(x,y) x = y,
     check_neq: function(x,y) x != y,
     num: 1 
   }"#,
    );
    te_be_value(false, scope, "[1,2,3,4,5,6][check_eq(item,4)]", r#"[4]"#);
    te_be_value(false, scope, "[1,2,3,4,5,6][check_eq(item,5)]", r#"[5]"#);
    te_be_value(false, scope, "[1,2,3,4,5,6][check_eq(item,num)]", r#"[1]"#);
    te_be_value(
        false,
        scope,
        "[1,2,3,4,5,6][check_neq(item,num)]",
        "[2,3,4,5,6]",
    );
}

#[test]
fn _0061() {
    let scope = &te_scope(
        r#"{
      "Flight List": [
        { "Flight Number": "UA456",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T19:00:00", "Arrival": @"2017-01-01T20:00:00", "Capacity": 2, "Status": "scheduled" },
        { "Flight Number": "UA123",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T18:00:00", "Arrival": @"2017-01-01T19:00:00", "Capacity": 5, "Status": "cancelled" },
        { "Flight Number": "UA789",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T21:00:00", "Arrival": @"2017-01-01T23:00:00", "Capacity": 2, "Status": "scheduled" },
        { "Flight Number": "UA1001", "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T23:00:00", "Arrival": @"2017-01-02T05:00:00", "Capacity": 0, "Status": "scheduled" },
        { "Flight Number": "UA1111", "From": "SFO", "To": "LAX", "Departure": @"2017-01-01T23:00:00", "Arrival": @"2017-01-02T05:00:00", "Capacity": 2, "Status": "scheduled" }]
    }"#,
    );
    te_be_value(
        false,
        scope,
        "Flight List[1]",
        r#"{"Arrival": @"2017-01-01T20:00:00", "Capacity": 2, "Departure": @"2017-01-01T19:00:00", "Flight Number": "UA456", "From": "SFO", "Status": "scheduled", "To": "SNA"}"#,
    );
}

#[test]
fn _0062() {
    let scope = &te_scope(
        r#"{
      "Flight List": [
        { "Flight Number": "UA456",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T19:00:00", "Arrival": @"2017-01-01T20:00:00", "Capacity": 2, "Status": "scheduled" },
        { "Flight Number": "UA123",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T18:00:00", "Arrival": @"2017-01-01T19:00:00", "Capacity": 5, "Status": "cancelled" },
        { "Flight Number": "UA789",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T21:00:00", "Arrival": @"2017-01-01T23:00:00", "Capacity": 2, "Status": "scheduled" },
        { "Flight Number": "UA1001", "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T23:00:00", "Arrival": @"2017-01-02T05:00:00", "Capacity": 0, "Status": "scheduled" },
        { "Flight Number": "UA1111", "From": "SFO", "To": "LAX", "Departure": @"2017-01-01T23:00:00", "Arrival": @"2017-01-02T05:00:00", "Capacity": 2, "Status": "scheduled" }]
    }"#,
    );
    te_be_value(
        false,
        scope,
        r#"Flight List[Status = "cancelled"]"#,
        r#"[{"Arrival": @"2017-01-01T19:00:00", "Capacity": 5, "Departure": @"2017-01-01T18:00:00", "Flight Number": "UA123", "From": "SFO", "Status": "cancelled", "To": "SNA"}]"#,
    );
    te_be_value(
        false,
        scope,
        r#"Flight List[Status = "cancelled"][1]"#,
        r#"{"Arrival": @"2017-01-01T19:00:00", "Capacity": 5, "Departure": @"2017-01-01T18:00:00", "Flight Number": "UA123", "From": "SFO", "Status": "cancelled", "To": "SNA"}"#,
    );
}

#[test]
fn _0063() {
    let scope = &te_scope(
        r#"{
      "Flight List": [
        { "Flight Number": "UA456",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T19:00:00", "Arrival": @"2017-01-01T20:00:00", "Capacity": 2, "Status": "scheduled" },
        { "Flight Number": "UA123",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T18:00:00", "Arrival": @"2017-01-01T19:00:00", "Capacity": 5, "Status": "cancelled" },
        { "Flight Number": "UA789",  "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T21:00:00", "Arrival": @"2017-01-01T23:00:00", "Capacity": 2, "Status": "scheduled" },
        { "Flight Number": "UA1001", "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T23:00:00", "Arrival": @"2017-01-02T05:00:00", "Capacity": 0, "Status": "cancelled" },
        { "Flight Number": "UA1111", "From": "SFO", "To": "LAX", "Departure": @"2017-01-01T23:00:00", "Arrival": @"2017-01-02T05:00:00", "Capacity": 2, "Status": "scheduled" }]
    }"#,
    );
    te_be_value(
        false,
        scope,
        r#"Flight List[Status = "cancelled"].Flight Number"#,
        r#"["UA123", "UA1001"]"#,
    );
}

#[test]
fn _0064() {
    let scope = &te_scope(
        r#"{
      "Flight List": [
        { "Flight Number": { n: "UA456",  tag: "A" }, "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T19:00:00", "Arrival": @"2017-01-01T20:00:00", "Capacity": 2, "Status": "scheduled" },
        { "Flight Number": { n: "UA123",  tag: "B" }, "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T18:00:00", "Arrival": @"2017-01-01T19:00:00", "Capacity": 5, "Status": "cancelled" },
        { "Flight Number": { n: "UA789",  tag: "C" }, "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T21:00:00", "Arrival": @"2017-01-01T23:00:00", "Capacity": 2, "Status": "scheduled" },
        { "Flight Number": { n: "UA1001", tag: "D" }, "From": "SFO", "To": "SNA", "Departure": @"2017-01-01T23:00:00", "Arrival": @"2017-01-02T05:00:00", "Capacity": 0, "Status": "cancelled" },
        { "Flight Number": { n: "UA1111", tag: "E" }, "From": "SFO", "To": "LAX", "Departure": @"2017-01-01T23:00:00", "Arrival": @"2017-01-02T05:00:00", "Capacity": 2, "Status": "scheduled" }]
    }"#,
    );
    te_be_value(
        false,
        scope,
        r#"Flight List[Status = "cancelled"].Flight Number.tag"#,
        r#"["B", "D"]"#,
    );
}

#[test]
fn _0065() {
    let scope = &te_scope(r#"{ A: 10 }"#);
    te_number(false, scope, "A[1]", 10, 0);
}

#[test]
fn _0066() {
    let scope = &te_scope(r#"{ A: 10 }"#);
    te_number(false, scope, "A[-1]", 10, 0);
}

#[test]
fn _0067() {
    let scope = &te_scope(r#"{ A: [1,2,3] }"#);
    te_null(
        false,
        scope,
        "A[99999999999999999999]",
        r#"index is out of range 1..2⁶⁴: 99999999999999999999"#,
    );
}

#[test]
fn _0068() {
    let scope = &te_scope(r#"{ A: [1,2,3] }"#);
    te_null(
        false,
        scope,
        "A[-99999999999999999999]",
        r#"index is out of range 1..2⁶⁴: -99999999999999999999"#,
    );
}

#[test]
fn _0069() {
    let scope = &te_scope(r#"{ A: [1,2,3] }"#);
    te_null(
        false,
        scope,
        "A[2.5]",
        r#"index in filter must be an integer value, actual value is 2.5"#,
    );
}

#[test]
fn _0070() {
    te_null(
        false,
        &scope!(),
        r#" @"2023-02-07"[2] "#,
        "for singletons, only filter index with value 1 or -1 is accepted",
    );
}

#[test]
fn _0071() {
    te_null(
        false,
        &scope!(),
        r#" @"2023-02-07"["a"] "#,
        "only number or boolean indexes are allowed in filters",
    );
}

#[test]
fn _0072() {
    te_null(
        false,
        &scope!(),
        r#" (function() 1)[1] "#,
        "unexpected value type in filter: function<>->Any",
    );
}

#[test]
fn _0073() {
    te_be_value(false, &scope!(), r#"[1,2,3,4,5][item >= 3]"#, "[3, 4, 5]");
}

#[test]
fn _0074() {
    te_be_value(false, &scope!(), r#"[1,2,3,4,5][item > 5]"#, "[]");
}

#[test]
fn _0075() {
    te_be_value(
        false,
        &scope!(),
        r#""not a list"[true]"#,
        r#"["not a list"]"#,
    );
}

#[test]
fn _0076() {
    te_null(
        false,
        &scope!(),
        r#"[1,2,3,4,5]["not a boolean"]"#,
        "only number or boolean indexes are allowed in filters",
    );
}
