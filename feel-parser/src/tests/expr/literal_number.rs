use super::super::*;

#[test]
fn _0001() {
    let input = "2";
    let expected = r#"
       Numeric
       └─ `2`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0002() {
    let input = "2.3";
    let expected = r#"
       Numeric
       └─ `2.3`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0003() {
    let input = ".2";
    let expected = r#"
       Numeric
       └─ `0.2`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0004() {
    let input = "1";
    let expected = r#"
       Numeric
       └─ `1`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0005() {
    let input = "(1.32983740938573405329458372450983275)";
    let expected = r#"
       Numeric
       └─ `1.32983740938573405329458372450983275`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0006() {
    let input = "-14";
    let expected = r#"
       Neg
       └─ Numeric
          └─ `14`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0007() {
    let input = "1.0";
    let expected = r#"
       Numeric
       └─ `1.0`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0008() {
    let input = "1.23e1";
    let expected = r#"
       Numeric
       └─ `1.23e+1`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0009() {
    let input = "1.23e+1";
    let expected = r#"
       Numeric
       └─ `1.23e+1`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0010() {
    let input = "1.23e-1";
    let expected = r#"
       Numeric
       └─ `1.23e-1`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0011() {
    let input = "1e-10";
    let expected = r#"
       Numeric
       └─ `1e-10`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0012() {
    let input = "1E-10";
    let expected = r#"
       Numeric
       └─ `1e-10`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}

#[test]
fn _0013() {
    let input = "1.23E-1";
    let expected = r#"
       Numeric
       └─ `1.23e-1`
    "#;
    accept(&scope!(), StartExpression, input, expected, false);
}
