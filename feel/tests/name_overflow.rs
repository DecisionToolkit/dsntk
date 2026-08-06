//! The name table must hold more than 2^20 distinct names.
//!
//! The `context` built-in interns every string key, so a model that builds a membership
//! map from transaction ids interns one name per distinct id. A server process shares one
//! table across all requests, so the risk is a matter of total request count, not of any
//! single request's size.
//!
//! This is an integration test on purpose: it runs in its own process, so the flooded
//! table cannot slow the unit tests of this crate.

use dsntk_feel::Name;
use std::cmp::Ordering;
use std::collections::BTreeMap;

/// Number of names interned by this test, comfortably past the 1,048,575 rank ceiling.
const FLOOD: usize = 1_100_000;

#[test]
fn name_table_holds_more_than_a_million_names() {
  // A name interned before the flood, while identities still fit beside a rank.
  let early = Name::from("name overflow test 0000000");

  // Flood the table past the point where names stop carrying a rank.
  let names: Vec<Name> = (0..FLOOD).map(|i| Name::from(format!("name overflow test {i:07}"))).collect();

  // Interning is injective and stable across the ceiling: the same text gives the same
  // handle, and every handle still resolves to its own text.
  assert_eq!(early, names[0]);
  assert_eq!(names[0].as_str(), "name overflow test 0000000");
  let last = names.last().unwrap();
  assert_eq!(last.as_str(), format!("name overflow test {:07}", FLOOD - 1));
  assert_eq!(*last, Name::from(format!("name overflow test {:07}", FLOOD - 1)));
  assert_ne!(names[1_048_574], names[1_048_575]);

  // Ordering stays lexicographic on both sides of the ceiling and across it.
  let below = &names[1_048_570]; // interned with a rankable identity
  let above = &names[1_048_580]; // interned past the rank ceiling
  assert_eq!(below.cmp(above), Ordering::Less);
  assert_eq!(above.cmp(below), Ordering::Greater);
  assert_eq!(above.cmp(&above.clone()), Ordering::Equal);

  // A BTreeMap keyed by Name is what FeelContext is. Keys inserted from both sides of
  // the ceiling must all be found again, which fails if ordering is not total.
  let probes = [0usize, 524_287, 1_048_570, 1_048_574, 1_048_575, 1_048_580, FLOOD - 1];
  let mut map: BTreeMap<Name, usize> = BTreeMap::new();
  for &i in &probes {
    map.insert(names[i].clone(), i);
  }
  for &i in &probes {
    assert_eq!(Some(&i), map.get(&Name::from(format!("name overflow test {i:07}"))), "key {i} lost");
  }

  // Entries iterate in text order, because context key order is visible in output.
  let keys: Vec<&str> = map.keys().map(|k| k.as_str()).collect();
  let mut sorted = keys.clone();
  sorted.sort_unstable();
  assert_eq!(keys, sorted);
}
