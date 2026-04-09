use dsntk_feel::{FeelScope, scope};

#[test]
fn _0001() {
  let input = r#"{
    date: {
      time: {
        duration: {
          date and time: 10
        }
      }
    }
  }"#;
  let node = dsntk_feel_parser::parse_context(&scope!(), input, false).unwrap();
  let expected = r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `date`
          └─ Context
             └─ ContextEntry
                ├─ ContextEntryKey
                │  └─ `time`
                └─ Context
                   └─ ContextEntry
                      ├─ ContextEntryKey
                      │  └─ `duration`
                      └─ Context
                         └─ ContextEntry
                            ├─ ContextEntryKey
                            │  └─ `date and time`
                            └─ Numeric
                               └─ `10`
    "#;
  assert_eq!(expected, node.trace());
}
