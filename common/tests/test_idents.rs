use dsntk_common::gen_id;

#[test]
fn _0001() {
    let id = gen_id();
    for (i, ch) in id.chars().enumerate() {
        if matches!(i, 8 | 13 | 18 | 23) {
            assert_eq!(ch, '-');
        } else {
            assert!(matches!(ch, 'a'..='f' | '0'..='9'))
        }
    }
}
