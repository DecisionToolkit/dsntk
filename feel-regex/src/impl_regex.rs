use crate::errors::{err_empty_flags, err_invalid_flag, err_invalid_pattern};
use crate::unicode_blocks::replace_block_names;
use crate::utils::is_special_character;
use regex::Regex;

pub fn is_match(input: &str, pattern: &str) -> dsntk_common::Result<bool> {
    let Ok(re) = Regex::new(&fix_pattern(pattern, false, false)) else {
        return Err(err_invalid_pattern(pattern));
    };
    Ok(re.is_match(&fix_input(input)))
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
    let pattern = format!(
        "{}{}",
        regex_flags,
        fix_pattern(pattern, whitespaces, themselves)
    );
    let Ok(re) = Regex::new(&pattern) else {
        return Err(err_invalid_pattern(&pattern));
    };
    Ok(re.is_match(&fix_input(input)))
}

// Nasty tricks on input.
fn fix_input(s: &str) -> String {
    s.replace("\r\n", "\n").replace('\r', "\n")
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
            .map(|ch| {
                if is_special_character(ch) {
                    format!(r#"\{}"#, ch)
                } else {
                    format!("{}", ch)
                }
            })
            .collect();
    }
    pattern
}
