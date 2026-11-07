/// Markdown emphases.
pub const EMPHASES: [&str; 5] = ["**", "__", "*", "_", "`"];

/// Returns the default output value retrieved from the list of allowed values.
pub fn get_allowed_and_default_output_values(
    input: &Option<String>,
) -> (Option<String>, Option<String>) {
    let Some(text) = input else {
        return (None, None);
    };
    for part in text.split(",").map(|text| text.trim()) {
        for emphasis in EMPHASES {
            if part.starts_with(emphasis) && part.ends_with(emphasis) {
                let trimmed_part = part.trim_start_matches(emphasis).trim_end_matches(emphasis);
                let allowed_values = text.replace(part, trimmed_part);
                let default_value = trimmed_part.to_string();
                return (Some(allowed_values), Some(default_value));
            }
        }
    }
    (input.clone(), None)
}
