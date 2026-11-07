/// Returns `true` when the specified character is a special one in patterns.
pub fn is_special_character(ch: char) -> bool {
    matches!(
        ch,
        '.' | '+' | '*' | '^' | '$' | '?' | '[' | ']' | '(' | ')' | '|' | '{' | '}' | '\\'
    )
}
