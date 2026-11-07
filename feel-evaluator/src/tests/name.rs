use super::*;

#[test]
fn test_name() {
    let scope = &te_scope(
        r#"{
        thing:2.0,
        one two three four:true,
        one and two:null,
        one or two:"found",
        before: {
          after: 1.11,
          or: {
            after: 1.12
          },
          and: {
            after: 1.13
          },
          between: {
            after: 1.14
          },
          next to between: {
            worm: 1.18
          }
        },
        income/loss:1.15,
        per/month/income/loss:1.17,
        a-b:1.19,
        to-be-or-not-to-be:1.20,
        that's:1.21,
        ok that's:1.22,
        bed+breakfast:1.23,
        night+and+day:1.24,
        fr**n*s:1.26,
        wh*t*v*r:1.27}"#,
    );
    te_number(false, scope, "thing", 20, 1);
    te_bool(false, scope, "one two three four", true);
    te_null(false, scope, "one and two", "");
    te_string(false, scope, "one or two", "found");
    te_number(false, scope, "before.after", 111, 2);
    te_number(false, scope, "before . next to between . worm", 118, 2);
    te_number(false, scope, " before . after ", 111, 2);
    te_number(false, scope, "before.and.after", 113, 2);
    te_number(false, scope, "before. and. after", 113, 2);
    te_number(false, scope, "income/loss", 115, 2);
    te_number(false, scope, "income  /   loss", 115, 2);
    te_number(false, scope, "per/month/income/loss", 117, 2);
    te_number(false, scope, " per/ month / income/loss", 117, 2);
    te_number(false, scope, "a-b", 119, 2);
    te_number(false, scope, "to-be-or-not-to-be", 120, 2);
    te_number(false, scope, "that's", 121, 2);
    te_number(false, scope, "ok that's", 122, 2);
    te_number(false, scope, "bed+breakfast", 123, 2);
    te_number(false, scope, "night+and+day", 124, 2);
    te_number(false, scope, "night + and + day", 124, 2);
    te_number(false, scope, "fr**n*s", 126, 2);
    te_number(false, scope, "wh*t*v*r", 127, 2);
}
