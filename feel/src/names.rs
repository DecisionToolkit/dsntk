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
//! that rank order equals lexicographic order of the name text, and a rank once assigned
//! never changes. A new name takes a rank strictly between the ranks of its two
//! lexicographic neighbours.
//!
//! The rank lives in the high bits of the handle, so [Ord] is a single unsigned compare of
//! the whole handle. Nothing outside this module observes any change in ordering.
//!
//! # Why ranks can never be renumbered
//!
//! Renumbering looks safe, because renumbering that preserves lexicographic order preserves
//! the order relation, and a `BTreeMap` only needs its keys to stay sorted. It is not safe.
//! **The rank lives inside the handle, and handles have already been copied into live
//! `BTreeMap`s** that are still being searched. Renumbering would rewrite the interner and
//! leave every stored key holding a stale rank. A lookup with a fresh handle would then
//! compare against stale ones and walk to the wrong node. Any scheme that wants to
//! renumber has to take the rank *out* of the handle first. That costs a load per
//! comparison on the hottest path in the engine, and it would need measurement before
//! anyone trusts it.
//!
//! # Running out of rank space
//!
//! Because this module assigns ranks without ever renumbering, a gap between two
//! neighbours can be used up. When that happens the new name is left **unranked** and
//! compares by text. That is slower for that name and stays correct, because both
//! comparison paths agree with lexicographic order exactly.
//!
//! The bound matters, because ascending numbered identifiers are the ordinary case in a
//! generated model, not a corner case, and a narrow stride lets a run of them exhaust a
//! gap fast. Rules named `Kb01`, `Kb02`, ... `Kb24` are one example. The current stride
//! keeps every name in all four benchmark models ranked.
//!
//! Two things keep names ranked in practice. The gap left by an append is far wider than
//! a single halving would leave. A name that continues a **family** consumes a fixed
//! [RANK_STEP] rather than half of what is left, so a run of numbered names costs one
//! step per name. See [Interner::rank_for] for how a family is recognised and why an
//! unrelated name is still halved into the middle.
//!
//! # Limitations, stated plainly
//!
//! - The table never shrinks. This module leaks name text deliberately, which is what
//!   makes `as_str` able to hand out `&'static str` and what makes a handle copyable. Name
//!   text comes from identifiers and context keys. The `context` built-in also interns
//!   every key that arrives as **runtime data**. So a model that builds a context from
//!   input strings grows the table with its input. A process that interns unboundedly
//!   many *distinct* names grows without bound.
//! - Only the first `2^20 - 1` distinct names carry a rank beside their identity. A name
//!   interned past that takes a **wide handle**: [WIDE_BIT] set, the rest of the handle
//!   is identity. A wide handle carries no rank, so it compares by text, exactly like a
//!   name whose rank gap was exhausted. Both paths agree with lexicographic order, so no
//!   answer changes. Lookups on wide names are slower, and that is all. The identity
//!   space of a wide handle is 63 bits, which memory exhausts long before the table does.
//!   The four models in this repository intern 50 to 175 names.
//! - The rank space is 43 bits, and it bounds three separate things. At least **65,536**
//!   names may be appended past the largest name so far, or prepended below the smallest. At
//!   least **4,096** names of one family fit in one appended gap. At least **15** more names
//!   may halve a gap that has become too narrow to step through.
//! - **The residual cliff is the third of those.** Names that repeatedly land between two
//!   ever-closer neighbours halve the gap each time. No fixed rank space can avoid that
//!   without renumbering, which the section above shows is unsound. Measured across the
//!   four models, the narrowest gap any name was inserted into is 2,048, which leaves 11
//!   halvings in hand.
//! - Past any of those three bounds a name is left unranked and compares by text. It is still
//!   ordered exactly lexicographically, so no output changes and no answer changes — only
//!   that name's lookups get slower. [Ord] mixes the two paths and stays a total order,
//!   because both paths agree with lexicographic order.

use dsntk_common::Jsonify;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::hash::{BuildHasherDefault, Hasher};
use std::sync::{LazyLock, OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Number of low bits of a handle that hold the identity of a **ranked** name.
///
/// Twenty bits allow 1,048,575 ranked names beside 43 bits of rank space. Names interned
/// past that take a [WIDE_BIT] handle and compare by text. The models in this repository
/// intern between 50 and 175 names. So only a model that interns runtime data — the
/// `context` built-in does — ever reaches the wide range.
const ID_BITS: u32 = 20;

/// Mask selecting the identity of a name.
const ID_MASK: u64 = (1 << ID_BITS) - 1;

/// Mask selecting the rank of a name.
const RANK_MASK: u64 = !ID_MASK;

/// Bit marking a **wide** handle, one whose identity is too large to fit beside a rank.
///
/// The first `2^20 - 1` names encode as `rank << ID_BITS | id`. Every later name encodes
/// as `WIDE_BIT | id`, carries no rank and compares by text. The three encodings cannot
/// collide. A ranked handle has this bit clear, because [RANK_MAX] stops one bit short
/// of it. An unranked narrow handle has every bit above [ID_BITS] clear. A wide handle
/// is the only one with this bit set.
const WIDE_BIT: u64 = 1 << 63;

/// Largest assignable rank. One bit short of [WIDE_BIT], so a ranked handle never sets it.
const RANK_MAX: u64 = (1 << (63 - ID_BITS)) - 1;

/// Room left between a newly appended name and its predecessor, so that later insertions
/// between the two have somewhere to go.
///
/// `RANK_MAX / 2 / RANK_STRIDE` is 65,536, which is how many names may be appended past the
/// largest name so far before the append room itself runs out.
const RANK_STRIDE: u64 = 1 << 26;

/// Room taken by a name that continues a family of names, while the gap can afford it.
///
/// `RANK_STRIDE / RANK_STEP` is 4,096, which is how many names of one family fit in a freshly
/// appended gap. `log2(2 * RANK_STEP)` is 15, which is how many halvings are left once a gap is
/// too narrow to step through.
///
/// Both bounds move in opposite directions with this constant. It was chosen by sweeping
/// it against the four models and reading off the narrowest gap any name landed in: 2^12 gave
/// 512, **2^14 gave 2,048**, 2^16 gave 2, and 2^18 and 2^20 left names unranked again. The
/// response is not monotone, because the constant changes which names become neighbours.
const RANK_STEP: u64 = 1 << 14;

/// Shortest shared prefix that counts as two names belonging to the same family.
///
/// One shared character is a coincidence between unrelated identifiers. Two is a stem.
const STEM_LEN: usize = 2;

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
  // Leaked on purpose: it is what lets a handle be copied and `as_str` return a `&'static
  // str`. One leak per distinct name, never per occurrence.
  let text: &'static str = Box::leak(text.to_owned().into_boxed_str());
  let handle = if id < ID_MASK {
    let rank = interner.rank_for(text);
    if rank != RANK_NONE {
      interner.ranked.insert(text, rank);
    }
    (rank << ID_BITS) | id
  } else {
    // The identity no longer fits beside a rank. The name takes a wide handle instead:
    // unranked, compared by text, identity in the low 63 bits. Interning stays injective
    // and total order is kept, so nothing changes except the speed of this name.
    WIDE_BIT | id
  };
  interner.texts.push(text);
  interner.handles.insert(text, handle);
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

/// Number of leading bytes two strings share.
///
/// This function only guesses which end of a gap a run of insertions is walking away
/// from. Its answer affects how much rank space is left, and never affects an ordering.
fn shared_prefix(left: &str, right: &str) -> usize {
  left.as_bytes().iter().zip(right.as_bytes()).take_while(|(l, r)| l == r).count()
}

impl Interner {
  /// Picks a rank for `text` that sits strictly between the ranks of its lexicographic
  /// neighbours, or [RANK_NONE] when no such rank is left.
  fn rank_for(&self, text: &str) -> u64 {
    let lower = self.ranked.range(..text).next_back().map(|(&neighbour, &rank)| (neighbour, rank));
    let upper = self.ranked.range(text..).next().map(|(&neighbour, &rank)| (neighbour, rank));
    let rank = match (lower, upper) {
      // The first ranked name starts in the middle, so both directions have room.
      (None, None) => RANK_MAX / 2,
      // Appended after every existing name.
      (Some((_, lower)), None) => {
        let room = RANK_MAX - lower;
        if room > RANK_STRIDE { lower + RANK_STRIDE } else { lower + room / 2 }
      }
      // Inserted before every existing name.
      (None, Some((_, upper))) => {
        if upper > RANK_STRIDE { upper - RANK_STRIDE } else { upper / 2 }
      }
      // Between two existing names. Two cases, and telling them apart is the whole point.
      //
      // A name that continues a **family** — one that shares a stem with the neighbour it
      // follows, like `Kb02` after `Kb01` — belongs to a run. A run must not halve the gap
      // per name, or it uses the gap up in `log2` names. It takes a fixed [RANK_STEP] away
      // from its family neighbour instead, so a run of `room / RANK_STEP` names fits. It
      // walks up from `lower` or down from `upper`, as the family lies.
      //
      // A name unrelated to both neighbours is halved into the middle, because that keeps
      // both halves of the gap roomy for whatever arrives later.
      //
      // A shared stem is at least two leading characters. One shared character is a
      // coincidence between unrelated identifiers, such as `Kb01` sharing `K` with
      // `Kr Adjacent Name`.
      //
      // Every branch returns a rank strictly inside the gap, so misjudging a family costs
      // later capacity and never an ordering.
      (Some((lower_text, lower)), Some((upper_text, upper))) => {
        let room = upper - lower;
        let with_lower = shared_prefix(text, lower_text);
        let with_upper = shared_prefix(text, upper_text);
        if room <= 2 * RANK_STEP {
          lower + room / 2
        } else if with_lower >= STEM_LEN && with_lower > with_upper {
          lower + RANK_STEP
        } else if with_upper >= STEM_LEN && with_upper > with_lower {
          upper - RANK_STEP
        } else {
          lower + room / 2
        }
      }
    };
    // A rank equal to either neighbour would order two different names the same way, and a
    // rank of zero is the "unranked" marker. Either way the gap is exhausted.
    if rank == RANK_NONE || Some(rank) == lower.map(|(_, rank)| rank) || Some(rank) == upper.map(|(_, rank)| rank) {
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
  /// rank gap compares by text, and so does a wide handle, whose high bits are identity
  /// rather than rank. Both paths agree with lexicographic order, so mixing them still
  /// gives a total order.
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    if (self.0 | other.0) & WIDE_BIT == 0 && (self.0 & RANK_MASK).min(other.0 & RANK_MASK) != RANK_NONE {
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
  /// Returns the [Name] for `text` only when it already exists, and never creates one.
  ///
  /// For looking a name up rather than making one. A lookup that interns leaks a name on
  /// every probe that misses, and the interner never gives one back. So a process probing
  /// unbounded text grows without bound, which is exactly the risk in `get value`.
  ///
  /// `None` is safe to read as "no context anywhere holds this key". The tuple field of
  /// [Name] is private to this module, and every constructor goes through [intern] or
  /// [intern_new]. So a [Name] cannot exist without sitting in `handles`. Text absent from
  /// `handles` therefore names no key in any [crate::FeelContext]. A caller that interned
  /// it would have got a fresh handle that is by construction in no map. Both branches
  /// return what the interning version returned, by construction rather than by test.
  pub fn existing(text: &str) -> Option<Self> {
    read_interner().handles.get(text.trim()).map(|&handle| Name(handle))
  }

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
    let id = if self.0 & WIDE_BIT != 0 { self.0 & !WIDE_BIT } else { self.0 & ID_MASK };
    text_of(id)
  }

  /// Returns `true` when this name is empty.
  pub fn is_empty(&self) -> bool {
    *self == *EMPTY
  }
}
