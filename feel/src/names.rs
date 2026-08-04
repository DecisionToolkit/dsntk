//! `FEEL` name implementation.
//!
//! A [Name] is an interned handle, not a string. This module stores every distinct name
//! text once in a process-wide table, so every [Name] that spells that text is the same
//! 8-byte value. Three properties of the hot paths motivate this:
//!
//! 1. [crate::context::FeelContext] keys a [std::collections::BTreeMap] by [Name]. A
//!    lookup runs a binary search, so it compares keys `log2(width)` times. With a string
//!    key, each comparison is a `memcmp` over the key text. Real key sets share long
//!    prefixes (`orderDate`, `orderDetail`, `order_id`), so the compare runs deep before
//!    it decides. Comparing handles decides in one integer compare.
//! 2. Cloning a name was a heap allocation. Every context entry insert cloned its key, so
//!    a run over half a million records allocated the same key text millions of times.
//!    Cloning a handle copies 8 bytes.
//! 3. The key array of a `BTreeMap` node shrinks from 24 bytes per key to 8.
//!
//! # Ordering is still lexicographic
//!
//! This is the part that needed care. [FeelContext](crate::context::FeelContext) exposes
//! `Deref<Target = BTreeMap<Name, Value>>` and `iter()`. `jsonify` walks the entries in
//! key order, so the order of a context's entries is *visible in program output*. If [Ord]
//! became "order in which names were interned", output key order would change.
//!
//! So a handle carries a **rank** as well as an identity. This module assigns ranks so
//! that rank order equals lexicographic order of the name text. A rank, once assigned,
//! never changes, which is what keeps already-built `BTreeMap`s valid. A new name takes
//! a rank between the ranks of its two lexicographic neighbours. Appending past the
//! current largest name, the common case, leaves a full stride of room for later
//! insertions.
//!
//! The rank lives in the high bits of the handle, so [Ord] is a single unsigned compare of
//! the whole handle. Nothing outside this module observes any change in ordering.
//!
//! # Limitations, stated plainly
//!
//! - The table never shrinks. This module leaks name text deliberately, which is what
//!   makes `as_str` able to hand out `&'static str` and what makes a handle copyable. Name
//!   text comes from identifiers and context keys, so the set is bounded by the shape of
//!   the input, not by its size. A process that parses unboundedly many *distinct* names
//!   grows without bound.
//! - The rank space is 40 bits. If a run of insertions exhausts the gap between two
//!   neighbours, the new name is left **unranked** and compares by text instead. That is
//!   slower for that name and stays correct, because both comparison paths agree with
//!   lexicographic order.
//! - At most `2^24 - 1` distinct names. Past that the handle could not stay injective, so
//!   interning panics rather than return a handle that compares equal to another name.

use dsntk_common::Jsonify;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::hash::{BuildHasherDefault, Hasher};
use std::sync::{LazyLock, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Number of low bits of a handle that hold the identity of a name.
const ID_BITS: u32 = 24;

/// Mask selecting the identity of a name.
const ID_MASK: u64 = (1 << ID_BITS) - 1;

/// Mask selecting the rank of a name.
const RANK_MASK: u64 = !ID_MASK;

/// Largest assignable rank.
const RANK_MAX: u64 = (1 << (64 - ID_BITS)) - 1;

/// Room left between a newly appended name and its predecessor, so that later insertions
/// between the two have somewhere to go.
const RANK_STRIDE: u64 = 1 << 16;

/// Rank value meaning "this name has no rank", so it compares by text.
const RANK_NONE: u64 = 0;

/// `FEEL` name.
///
/// Cheap to clone, cheap to compare and ordered exactly as its text is ordered. See the
/// module documentation for how that is arranged.
#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Name(u64);

impl Default for Name {
  /// The default name is the empty name.
  ///
  /// Written out rather than derived: a derived default would be handle zero, which is the
  /// handle of whichever name was interned first, not the handle of the empty name.
  fn default() -> Self {
    EMPTY.clone()
  }
}

/// The process-wide name table.
struct Interner {
  /// Maps name text to the handle interned for it.
  handles: HashMap<&'static str, u64, BuildHasherDefault<NameHasher>>,
  /// Maps the text of every **ranked** name to its rank, in lexicographic order.
  ///
  /// Only ranked names appear here, so the neighbours of a new name are always ranked.
  /// The rank assigned between them is therefore always valid. Unranked names take no
  /// part in rank comparison, so leaving them out changes no answer.
  ranked: BTreeMap<&'static str, u64>,
  /// Maps the identity of a name to its text.
  texts: Vec<&'static str>,
}

static INTERNER: RwLock<Interner> = RwLock::new(Interner {
  handles: HashMap::with_hasher(BuildHasherDefault::new()),
  ranked: BTreeMap::new(),
  texts: Vec::new(),
});

/// The empty name, so [Name::is_empty] needs no table lookup.
static EMPTY: LazyLock<Name> = LazyLock::new(|| intern(""));

fn read_interner() -> RwLockReadGuard<'static, Interner> {
  INTERNER.read().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn write_interner() -> RwLockWriteGuard<'static, Interner> {
  INTERNER.write().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Returns the handle for `text`, interning it when it is new.
fn intern(text: &str) -> Name {
  if let Some(&handle) = read_interner().handles.get(text) {
    return Name(handle);
  }
  intern_new(text)
}

/// Adds a name that the read above did not find.
///
/// Kept out of [intern] so that the hot path stays small enough to inline.
#[cold]
fn intern_new(text: &str) -> Name {
  let mut interner = write_interner();
  // Another thread may have interned this text between the read above and this write.
  if let Some(&handle) = interner.handles.get(text) {
    return Name(handle);
  }
  let id = interner.texts.len() as u64;
  assert!(id < ID_MASK, "the FEEL name table is full at {ID_MASK} distinct names");
  // Leaked on purpose: it is what lets a handle be copied and `as_str` return a `&'static
  // str`. One leak per distinct name, never per occurrence.
  let text: &'static str = Box::leak(text.to_owned().into_boxed_str());
  let rank = interner.rank_for(text);
  let handle = (rank << ID_BITS) | id;
  interner.texts.push(text);
  interner.handles.insert(text, handle);
  if rank != RANK_NONE {
    interner.ranked.insert(text, rank);
  }
  // Published before the write lock is released, so a thread that later holds this handle
  // always finds the slot filled.
  if let Some(slot) = TEXT_SLOTS.get(id as usize) {
    let _ = slot.set(text);
  }
  Name(handle)
}

/// Number of names whose text is readable without taking the lock.
const RESOLVE_SLOTS: usize = 1 << 16;

/// Lock-free text lookup, one slot per name identity, filled as names are interned.
///
/// [Name::as_str] is on the evaluation hot path and runs on every worker thread at once.
/// Reading the text through the [RwLock] instead measured **26 times slower than not
/// interning at all**, at n=500,000 on twelve threads. The same binary held its gain on
/// one thread. Taking a read lock still *writes* the lock word, so every reader stole
/// that cache line from every other core. A [OnceLock] is written once and then only
/// loaded, so the readers share the line rather than fight over it.
///
/// Zero-initialized, so the pages of slots no name reaches are never faulted in.
static TEXT_SLOTS: [OnceLock<&'static str>; RESOLVE_SLOTS] = [const { OnceLock::new() }; RESOLVE_SLOTS];

/// Returns the text of the name with the given identity.
#[inline]
fn text_of(id: u64) -> &'static str {
  if let Some(slot) = TEXT_SLOTS.get(id as usize)
    && let Some(&text) = slot.get()
  {
    return text;
  }
  // Past the end of the slot array. Rare enough to pay for the lock.
  read_interner().texts[id as usize]
}

impl Interner {
  /// Picks a rank for `text` that sits strictly between the ranks of its lexicographic
  /// neighbours, or [RANK_NONE] when no such rank is left.
  fn rank_for(&self, text: &str) -> u64 {
    let lower = self.ranked.range(..text).next_back().map(|(_, &rank)| rank);
    let upper = self.ranked.range(text..).next().map(|(_, &rank)| rank);
    let rank = match (lower, upper) {
      // The first ranked name starts in the middle, so both directions have room.
      (None, None) => RANK_MAX / 2,
      // Appended after every existing name.
      (Some(lower), None) => {
        let room = RANK_MAX - lower;
        if room > RANK_STRIDE { lower + RANK_STRIDE } else { lower + room / 2 }
      }
      // Inserted before every existing name.
      (None, Some(upper)) => {
        if upper > RANK_STRIDE { upper - RANK_STRIDE } else { upper / 2 }
      }
      // Between two existing names.
      (Some(lower), Some(upper)) => lower + (upper - lower) / 2,
    };
    // A rank equal to either neighbour would order two different names the same way, and a
    // rank of zero is the "unranked" marker. Either way the gap is exhausted.
    if rank == RANK_NONE || Some(rank) == lower || Some(rank) == upper {
      RANK_NONE
    } else {
      rank
    }
  }
}

/// FNV-1a over the name text, finished with a bit mixer.
///
/// The default hasher spends more time on a short key than the whole lookup needs. The
/// mixer matters because the map takes the low bits of the hash for the bucket index and
/// FNV alone leaves those weak.
#[derive(Default)]
struct NameHasher(u64);

impl Hasher for NameHasher {
  fn write(&mut self, bytes: &[u8]) {
    let mut hash = if self.0 == 0 { 0xcbf2_9ce4_8422_2325 } else { self.0 };
    for &byte in bytes {
      hash ^= byte as u64;
      hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    self.0 = hash;
  }

  fn finish(&self) -> u64 {
    let mut hash = self.0;
    hash = (hash ^ (hash >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    hash = (hash ^ (hash >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    hash ^ (hash >> 31)
  }
}

impl Ord for Name {
  /// Orders two names exactly as their texts are ordered.
  ///
  /// Ranked names compare in one unsigned compare, because the rank is the high part of the
  /// handle and rank order is lexicographic order. A name left unranked by an exhausted
  /// rank gap compares by text. Both paths agree with lexicographic order, so mixing them
  /// still gives a total order.
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    if (self.0 & RANK_MASK).min(other.0 & RANK_MASK) != RANK_NONE {
      return self.0.cmp(&other.0);
    }
    if self.0 == other.0 {
      return Ordering::Equal;
    }
    self.as_str().cmp(other.as_str())
  }
}

impl PartialOrd for Name {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl From<Vec<String>> for Name {
  /// Converts a vector of strings into [Name].
  fn from(value: Vec<String>) -> Self {
    Self::new(&value.iter().map(|string| string.as_str()).collect::<Vec<&str>>())
  }
}

impl From<Vec<&str>> for Name {
  /// Converts a vector of string references into [Name].
  fn from(value: Vec<&str>) -> Self {
    Self::new(&value)
  }
}

impl From<String> for Name {
  /// Converts a [String] into [Name].
  fn from(value: String) -> Self {
    intern(value.trim())
  }
}

impl From<&String> for Name {
  /// Converts a reference to [String] into [Name].
  fn from(value: &String) -> Self {
    intern(value.trim())
  }
}

impl From<&str> for Name {
  /// Converts a reference to [str] into [Name].
  fn from(value: &str) -> Self {
    intern(value.trim())
  }
}

impl From<Name> for String {
  /// Converts [Name] to its [String] representation.
  fn from(value: Name) -> Self {
    value.as_str().to_string()
  }
}

impl From<&Name> for String {
  /// Converts a reference to [Name] to its [String] representation.
  fn from(value: &Name) -> Self {
    value.as_str().to_string()
  }
}

impl fmt::Debug for Name {
  /// Implements [Debug](fmt::Debug) trait for [Name].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Name({:?})", self.as_str())
  }
}

impl fmt::Display for Name {
  /// Implements [Display](fmt::Display) trait for [Name].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.pad(self.as_str())
  }
}

impl Jsonify for Name {
  /// Converts [Name] to its `JSON` representation.
  fn jsonify(&self) -> String {
    self.as_str().to_string()
  }
}

impl Name {
  /// Creates a [Name] from string parts.
  pub fn new(parts: &[&str]) -> Self {
    // A name of a single part is the common case, a context key or a variable reference of
    // one token. Interning the part directly saves building a temporary string for every
    // occurrence of every key in the input.
    if let [part] = parts {
      return intern(part.trim());
    }
    let mut result = String::new();
    let mut current;
    let mut prev = false;
    for (index, part) in parts.iter().map(|s| s.trim()).enumerate() {
      current = matches!(part, "." | "/" | "-" | "'" | "+" | "*");
      if index > 0 && !prev && !current && !part.is_empty() {
        result.push(' ');
      }
      result.push_str(part);
      prev = current;
    }
    intern(&result)
  }

  /// Extracts a string slice containing the entire [Name].
  pub fn as_str(&self) -> &str {
    text_of(self.0 & ID_MASK)
  }

  /// Returns `true` when this name is empty.
  pub fn is_empty(&self) -> bool {
    *self == *EMPTY
  }
}
