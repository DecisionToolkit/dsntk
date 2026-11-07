use super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
    te_be_value(
        false,
        &scope!(),
        r#"for n in [12,8,32] return n + 1"#,
        "[13,9,33]",
    );
}

#[test]
fn _0002() {
    te_be_value(
        false,
        &scope!(),
        r#"for n in 1..10 return n - 1"#,
        "[0,1,2,3,4,5,6,7,8,9]",
    );
}

#[test]
fn _0003() {
    let scope = &te_scope(r#"{N:4}"#);
    te_be_value(
        false,
        scope,
        r#"
      for i in 1..N return
        if i = 1 then
          1
        else
          i * partial[-1]
    "#,
        "[1,2,6,24]",
    );
}

#[test]
fn _0004() {
    let scope = &te_scope(r#"{N:5}"#);
    te_be_value(
        false,
        scope,
        r#"
      for i in 0..N return
        if i = 0 then
          1
        else if i = 1 then
          1 
        else 
          partial[-1] + partial[-2]
    "#,
        "[1,1,2,3,5,8]",
    );
}

#[test]
fn _0005() {
    te_be_value(
        false,
        &scope!(),
        r#"
      for x in [1,2,3],
          y in [6,7,8]
      return x + y
    "#,
        "[7,8,9,8,9,10,9,10,11]",
    );
}

#[test]
fn _0006() {
    te_be_value(
        false,
        &scope!(),
        r#"for x in [1,2,3], y in [6,7,8], z in [-1,-2,-3] return x + y + z"#,
        "[6,5,4,7,6,5,8,7,6,7,6,5,8,7,6,9,8,7,8,7,6,9,8,7,10,9,8]",
    );
}

#[test]
fn _0007() {
    te_be_value(false, &scope!(), r#"for n in 4..2 return n"#, "[4,3,2]");
}

#[test]
fn _0008() {
    te_be_value(
        false,
        &scope!(),
        r#"for i in 1..3, j in [4,5] return {a: i, b: j}"#,
        "[{a: 1, b: 4}, {a: 1, b: 5}, {a: 2, b: 4}, {a: 2, b: 5}, {a: 3, b: 4}, {a: 3, b: 5}]",
    );
}

#[test]
fn _0009() {
    te_be_value(
        false,
        &scope!(),
        r#"for j in [4,5], i in 1..3 return {a: i, b: j}"#,
        "[{a: 1, b: 4}, {a: 2, b: 4}, {a: 3, b: 4}, {a: 1, b: 5}, {a: 2, b: 5}, {a: 3, b: 5}]",
    );
}

#[test]
fn _0010() {
    //--------------------------------------------------------------------------------------------------------------------
    // `x` variable iterates over a list. But the elements of this list are also lists (nested lists).
    // The result is a list of elements, but because each element is also a list,
    // the we get a nested list as a result.
    //--------------------------------------------------------------------------------------------------------------------
    te_be_value(
        false,
        &scope!(),
        "for x in [[1,2],[3,4]] return x",
        "[[1, 2], [3, 4]]",
    );
}

#[test]
fn _0011() {
    //--------------------------------------------------------------------------------------------------------------------
    // In the previous test we get a nested list as a result from iteration done by variable `x`.
    // So the nested loop with variable `y` also iterates over a nested list, so we get the same result
    // as in previous test.
    //--------------------------------------------------------------------------------------------------------------------
    te_be_value(
        false,
        &scope!(),
        "for x in [[1,2],[3,4]] return for y in x return y",
        "[[1, 2], [3, 4]]",
    );
}

#[test]
fn _0012() {
    //--------------------------------------------------------------------------------------------------------------------
    // This is tricky. `x` iterates over values that are lists. So in each iteration,
    // the value of `x` is a list (the nested one).
    // So the variable `y` always sees a list of numbers, so it iterates over numbers,
    // and because we care only about variable `y` in the result (which is a number in each iteration),
    // so the final result is a list of numbers.
    //--------------------------------------------------------------------------------------------------------------------
    te_be_value(
        false,
        &scope!(),
        "for x in [[1,2],[3,4]], y in x return y",
        "[1, 2, 3, 4]",
    );
}

#[test]
fn _0013() {
    //--------------------------------------------------------------------------------------------------------------------
    // Binding variable pointing to another variable can not be the before the pointed one.
    //--------------------------------------------------------------------------------------------------------------------
    te_null(
        false,
        &scope!(),
        "(for y in x, x in [[1,2],[3,4]] return y)[1]",
        "context has no value for key 'x'",
    );
    te_null(
        false,
        &scope!(),
        "(for y in x, x in [[1,2],[3,4]] return y)[2]",
        "context has no value for key 'x'",
    );
}

#[test]
fn _0014() {
    //--------------------------------------------------------------------------------------------------------------------
    // Binding variable is repeated in the output, because is not in the last iteration context.
    //--------------------------------------------------------------------------------------------------------------------
    te_be_value(
        false,
        &scope!(),
        "for x in [[1,2],[3,4]], y in x, z in [8,9,10] return y",
        "[1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4]",
    );
}

#[test]
fn _0015() {
    //--------------------------------------------------------------------------------------------------------------------
    // Iterating over empty list should return an empty list.
    //--------------------------------------------------------------------------------------------------------------------
    te_be_value(false, &scope!(), "for x in [] return x", "[]");
}

#[test]
fn _0016() {
    //--------------------------------------------------------------------------------------------------------------------
    // Iterating over empty list in first context should return an empty list.
    //--------------------------------------------------------------------------------------------------------------------
    te_be_value(false, &scope!(), "for x in [], y in [1,2] return y", "[]");
}

#[test]
fn _0017() {
    //--------------------------------------------------------------------------------------------------------------------
    // Iterating over empty list in last context should return an empty list.
    //--------------------------------------------------------------------------------------------------------------------
    te_be_value(false, &scope!(), "for x in [1,2], y in [] return x", "[]");
}

#[test]
fn _0018() {
    //--------------------------------------------------------------------------------------------------------------------
    // Iterating over empty list in any context should return an empty list.
    //--------------------------------------------------------------------------------------------------------------------
    te_be_value(
        false,
        &scope!(),
        "for x in [1,2,3], y in [], z in [4,5] return z",
        "[]",
    );
}

#[test]
fn _0019() {
    //--------------------------------------------------------------------------------------------------------------------
    // Binding variable points on another one, but the list is not nested.
    //--------------------------------------------------------------------------------------------------------------------
    te_be_value(
        false,
        &scope!(),
        "for x in [1,2,3,4], y in x return y",
        "[1, 2, 3, 4]",
    );
}

#[test]
fn _0020() {
    //--------------------------------------------------------------------------------------------------------------------
    // Binding variables chain.
    //--------------------------------------------------------------------------------------------------------------------
    te_be_value(
        false,
        &scope!(),
        "for x in [1,2,3,4], y in x, z in y return z",
        "[1, 2, 3, 4]",
    );
}

#[test]
fn _0021() {
    //--------------------------------------------------------------------------------------------------------------------
    // Binding variables chain with nested lists.
    //--------------------------------------------------------------------------------------------------------------------
    te_be_value(
        false,
        &scope!(),
        "for x in [[1,2],[3,4]], y in x, z in y return z",
        "[1, 2, 3, 4]",
    );
}

#[test]
fn _0022() {
    //--------------------------------------------------------------------------------------------------------------------
    // Binding variables chain with triple nested lists.
    //--------------------------------------------------------------------------------------------------------------------
    te_be_value(
        false,
        &scope!(),
        "for x in [[[1,2],[3,4]],[[5,6],[7,8]]], y in x, z in y return z",
        "[1, 2, 3, 4, 5, 6, 7, 8]",
    );
}
