/// Trims whitespaces before and after each line, preserves empty lines.
pub fn trim_multiline(input: String) -> String {
  input.trim().lines().map(|line| line.trim().to_string()).collect::<Vec<String>>().join("\n")
}
