use crate::errors::{err_empty_flags, err_invalid_flag, err_invalid_pattern};
use crate::unicode_blocks::replace_block_names;
use crate::utils::is_special_character;
use regex::Regex;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;

/// Maximum number of compiled patterns kept in one thread's cache.
///
/// Patterns come from a decision model, so their number is bounded by the size of the
/// model. The limit only protects against a model that builds patterns from input data.
/// When the count reaches the limit, this code drops the whole cache. That costs one
/// recompilation per live pattern, but keeps the cache bounded.
const MAX_CACHED_PATTERNS: usize = 256;

thread_local! {
  /// Cache of compiled regular expressions, keyed by the pattern text after all
  /// pattern fixes were applied. **One cache per thread**, and this is the point.
  ///
  /// A process-wide cache behind an `RwLock`, read-locked once per call, has the same
  /// mistake `feel/src/names.rs` documents for its own text table. Taking a read lock
  /// still *writes* the lock word, so every reader steals that cache line from every
  /// other core. Measured on a 200,000-element parallel `for`, a `matches` body ran at
  /// 1.00x the serial time. The same body with `contains` ran at 1.24x. The whole
  /// parallel gain was eaten by one contended line.
  ///
  /// A thread-local cache touches no shared state on the hit path. It also keeps each
  /// compiled expression on the thread that made it. That matters because a `Regex`
  /// shared across threads serializes again on its internal scratch pool. The cost is
  /// one compilation of each pattern per worker thread instead of per process, a
  /// bounded one-time price that the first few hundred calls repay.
  static CACHE: RefCell<HashMap<String, Regex>> = RefCell::new(HashMap::new());
}

/// Compiles `pattern` if needed, then calls `f` with the compiled regular expression.
///
/// Compiling a regular expression is expensive, and a `matches` call inside a `for`
/// expression compiles the same constant pattern once per element. Measured on a model
/// with a single `matches` call over 50,000 records, `Regex::new` was 25% of the total
/// run time. The result is the same either way, because compiling a pattern is
/// deterministic and the compiled expression is immutable.
///
/// `f` must not call back into this function. Both callers pass a plain match closure.
///
/// Returns `None` when the pattern does not compile.
fn with_compiled<T>(pattern: String, f: impl FnOnce(&Regex) -> T) -> Option<T> {
  CACHE.with(|cell| {
    let mut cache = cell.borrow_mut();
    if !cache.contains_key(&pattern) {
      let re = Regex::new(&pattern).ok()?;
      if cache.len() >= MAX_CACHED_PATTERNS {
        cache.clear();
      }
      cache.insert(pattern.clone(), re);
    }
    Some(f(&cache[&pattern]))
  })
}

pub fn is_match(input: &str, pattern: &str) -> dsntk_common::Result<bool> {
  let input = fix_input(input);
  match with_compiled(fix_pattern(pattern, false, false), |re| re.is_match(&input)) {
    Some(result) => Ok(result),
    None => Err(err_invalid_pattern(pattern)),
  }
}

pub fn is_match_with_flags(input: &str, pattern: &str, flags: &str) -> dsntk_common::Result<bool> {
  let flags = flags.trim();
  if flags.is_empty() {
    return Err(err_empty_flags());
  }
  let mut regex_flags = String::new();
  let mut whitespaces = false;
  let mut themselves = false;
  for ch in flags.chars() {
    match ch {
      'i' | 'm' | 's' => regex_flags.push(ch),
      'x' => {
        whitespaces = true;
        regex_flags.push(ch);
      }
      'q' => themselves = true,
      _ => return Err(err_invalid_flag(ch)),
    }
  }
  if !regex_flags.is_empty() {
    regex_flags = format!("(?{})", regex_flags);
  }
  let pattern = format!("{}{}", regex_flags, fix_pattern(pattern, whitespaces, themselves));
  let input = fix_input(input);
  match with_compiled(pattern.clone(), |re| re.is_match(&input)) {
    Some(result) => Ok(result),
    None => Err(err_invalid_pattern(&pattern)),
  }
}

// Nasty tricks on input.
//
// Almost no input contains a carriage return. This runs once per `matches` call, so
// this code borrows the unchanged input instead of copying it.
fn fix_input(s: &str) -> Cow<'_, str> {
  if s.contains('\r') { Cow::Owned(s.replace("\r\n", "\n").replace('\r', "\n")) } else { Cow::Borrowed(s) }
}

// Nasty tricks on pattern.
fn fix_pattern(pattern: &str, whitespaces: bool, themselves: bool) -> String {
  let mut pattern = replace_block_names(pattern, whitespaces);
  pattern = pattern.replace("-[", "--[");
  if whitespaces {
    pattern = pattern.replace("\\ ", "\\").replace("[ ]", "[\\ ]");
  }
  if themselves {
    pattern = pattern
      .chars()
      .map(|ch| if is_special_character(ch) { format!(r#"\{}"#, ch) } else { format!("{}", ch) })
      .collect();
  }
  pattern
}
