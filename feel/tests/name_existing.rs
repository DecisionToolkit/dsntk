//! Tests for [Name::existing], the lookup that must never intern.
//!
//! A lookup that interns adds a name on every probe that misses, and the interner never
//! releases one. So a process probing unbounded text would grow without bound. `get value`
//! is exactly that kind of probe. These tests pin the contract the fix relies on.

use dsntk_feel::Name;

#[test]
fn absent_text_is_none() {
  assert!(Name::existing("a name no test ever interns 8f3a2b").is_none());
}

#[test]
fn probing_does_not_create() {
  // The point of the fix: asking twice must not make the name exist.
  assert!(Name::existing("probe only 41d9c7").is_none());
  assert!(Name::existing("probe only 41d9c7").is_none());
}

#[test]
fn interned_text_is_found_and_equal() {
  let made = Name::from("interned by this test 5c1e");
  let found = Name::existing("interned by this test 5c1e");
  assert_eq!(Some(made), found, "existing must return the handle intern would");
}

#[test]
fn text_is_trimmed_like_intern() {
  let made = Name::from("  padded name 77b2  ");
  assert_eq!(Some(made.clone()), Name::existing("padded name 77b2"));
  assert_eq!(Some(made), Name::existing("   padded name 77b2 "));
}
