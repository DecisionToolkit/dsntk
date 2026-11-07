use dsntk_recognizer::{BuiltinAggregator, HitPolicy, from_unicode};

#[test]
fn _0001() {
    let dt = from_unicode(
        r#"
 ┌───╥─────────────┐
 │ U ║             │
 ╞═══╬═════════════╡
 │ 1 ║  "Monday"   │
 └───╨─────────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Unique, dt.hit_policy)
}

#[test]
fn _0002() {
    let dt = from_unicode(
        r#"
 ┌───╥─────────────┐
 │ A ║             │
 ╞═══╬═════════════╡
 │ 1 ║  "Monday"   │
 └───╨─────────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Any, dt.hit_policy)
}

#[test]
fn _0003() {
    let dt = from_unicode(
        r#"
 ┌───╥─────────────┐
 │ P ║             │
 ╞═══╬═════════════╡
 │ 1 ║  "Monday"   │
 └───╨─────────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Priority, dt.hit_policy)
}

#[test]
fn _0004() {
    let dt = from_unicode(
        r#"
 ┌───╥─────────────┐
 │ F ║             │
 ╞═══╬═════════════╡
 │ 1 ║  "Monday"   │
 └───╨─────────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::First, dt.hit_policy)
}

#[test]
fn _0005() {
    let dt = from_unicode(
        r#"
 ┌───╥─────────────┐
 │ R ║             │
 ╞═══╬═════════════╡
 │ 1 ║  "Monday"   │
 └───╨─────────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::RuleOrder, dt.hit_policy)
}

#[test]
fn _0006() {
    let dt = from_unicode(
        r#"
 ┌───╥─────────────┐
 │ O ║             │
 ╞═══╬═════════════╡
 │ 1 ║  "Monday"   │
 └───╨─────────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::OutputOrder, dt.hit_policy)
}

#[test]
fn _0007() {
    let dt = from_unicode(
        r#"
 ┌───╥─────────────┐
 │ C ║             │
 ╞═══╬═════════════╡
 │ 1 ║  "Monday"   │
 └───╨─────────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Collect(BuiltinAggregator::List), dt.hit_policy)
}

#[test]
fn _0008() {
    let dt = from_unicode(
        r#"
 ┌────╥─────────────┐
 │ C+ ║             │
 ╞════╬═════════════╡
 │ 1  ║  "Monday"   │
 └────╨─────────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Collect(BuiltinAggregator::Sum), dt.hit_policy)
}

#[test]
fn _0009() {
    let dt = from_unicode(
        r#"
 ┌────╥─────────────┐
 │ C# ║             │
 ╞════╬═════════════╡
 │ 1  ║  "Monday"   │
 └────╨─────────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Collect(BuiltinAggregator::Count), dt.hit_policy)
}

#[test]
fn _0010() {
    let dt = from_unicode(
        r#"
 ┌────╥─────────────┐
 │ C< ║             │
 ╞════╬═════════════╡
 │ 1  ║  "Monday"   │
 └────╨─────────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Collect(BuiltinAggregator::Min), dt.hit_policy)
}

#[test]
fn _0011() {
    let dt = from_unicode(
        r#"
 ┌────╥─────────────┐
 │ C> ║             │
 ╞════╬═════════════╡
 │ 1  ║  "Monday"   │
 └────╨─────────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Collect(BuiltinAggregator::Max), dt.hit_policy)
}

#[test]
fn _0012() {
    from_unicode(
        r#"
 ┌────╥─────────────┐
 │ C$ ║             │
 ╞════╬═════════════╡
 │ 1  ║  "Monday"   │
 └────╨─────────────┘
"#,
        false,
    )
    .unwrap_err();
}

#[test]
fn _0013() {
    let dt = from_unicode(
        r#"
  ┌─────────╥──────────┐
  │ Weekday ║     -    │
  ╞═════════╬══════════╡
  │         ║ "Monday" │
  ├─────────╫──────────┤
  │   U     ║    1     │
  └─────────╨──────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Unique, dt.hit_policy)
}

#[test]
fn _0014() {
    let dt = from_unicode(
        r#"
  ┌─────────╥──────────┐
  │ Weekday ║     -    │
  ╞═════════╬══════════╡
  │         ║ "Monday" │
  ├─────────╫──────────┤
  │   A     ║    1     │
  └─────────╨──────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Any, dt.hit_policy)
}

#[test]
fn _0015() {
    let dt = from_unicode(
        r#"
  ┌─────────╥──────────┐
  │ Weekday ║     -    │
  ╞═════════╬══════════╡
  │         ║ "Monday" │
  ├─────────╫──────────┤
  │   P     ║    1     │
  └─────────╨──────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Priority, dt.hit_policy)
}

#[test]
fn _0016() {
    let dt = from_unicode(
        r#"
  ┌─────────╥──────────┐
  │ Weekday ║     -    │
  ╞═════════╬══════════╡
  │         ║ "Monday" │
  ├─────────╫──────────┤
  │   F     ║    1     │
  └─────────╨──────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::First, dt.hit_policy)
}

#[test]
fn _0017() {
    let dt = from_unicode(
        r#"
  ┌─────────╥──────────┐
  │ Weekday ║     -    │
  ╞═════════╬══════════╡
  │         ║ "Monday" │
  ├─────────╫──────────┤
  │   R     ║    1     │
  └─────────╨──────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::RuleOrder, dt.hit_policy)
}

#[test]
fn _0018() {
    let dt = from_unicode(
        r#"
  ┌─────────╥──────────┐
  │ Weekday ║     -    │
  ╞═════════╬══════════╡
  │         ║ "Monday" │
  ├─────────╫──────────┤
  │   O     ║    1     │
  └─────────╨──────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::OutputOrder, dt.hit_policy)
}

#[test]
fn _0019() {
    let dt = from_unicode(
        r#"
  ┌─────────╥──────────┐
  │ Weekday ║     -    │
  ╞═════════╬══════════╡
  │         ║ "Monday" │
  ├─────────╫──────────┤
  │   C     ║    1     │
  └─────────╨──────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Collect(BuiltinAggregator::List), dt.hit_policy)
}

#[test]
fn _0020() {
    let dt = from_unicode(
        r#"
  ┌─────────╥──────────┐
  │ Weekday ║     -    │
  ╞═════════╬══════════╡
  │         ║ "Monday" │
  ├─────────╫──────────┤
  │   C+    ║    1     │
  └─────────╨──────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Collect(BuiltinAggregator::Sum), dt.hit_policy)
}

#[test]
fn _0021() {
    let dt = from_unicode(
        r#"
  ┌─────────╥──────────┐
  │ Weekday ║     -    │
  ╞═════════╬══════════╡
  │         ║ "Monday" │
  ├─────────╫──────────┤
  │   C#    ║    1     │
  └─────────╨──────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Collect(BuiltinAggregator::Count), dt.hit_policy)
}

#[test]
fn _0022() {
    let dt = from_unicode(
        r#"
  ┌─────────╥──────────┐
  │ Weekday ║     -    │
  ╞═════════╬══════════╡
  │         ║ "Monday" │
  ├─────────╫──────────┤
  │   C<    ║    1     │
  └─────────╨──────────┘
"#,
        false,
    )
    .unwrap();
    assert_eq!(HitPolicy::Collect(BuiltinAggregator::Min), dt.hit_policy)
}

#[test]
fn _0023() {
    let dt = from_unicode(
        r#"
  ┌─────────╥──────────┐
  │ Weekday ║     -    │
  ╞═════════╬══════════╡
  │         ║ "Monday" │
  ├─────────╫──────────┤
  │   C>    ║    1     │
  └─────────╨──────────┘
"#,
        true,
    )
    .unwrap();
    assert_eq!(HitPolicy::Collect(BuiltinAggregator::Max), dt.hit_policy)
}

#[test]
fn _0024() {
    from_unicode(
        r#"
  ┌─────────╥──────────┐
  │ Weekday ║     -    │
  ╞═════════╬══════════╡
  │         ║ "Monday" │
  ├─────────╫──────────┤
  │   C$    ║    1     │
  └─────────╨──────────┘
"#,
        true,
    )
    .unwrap_err();
}
