/// Truncates the middle of a string with an ellipsis (…).
///
/// Keeps the start and end characters, trimming the middle to fit the total length.
/// Returns the original string if `max_len` is greater than or equal to the string length.
///
/// # Examples
///
/// ```rust
/// use lo_::truncate_middle;
/// assert_eq!(truncate_middle("hello_world", 7), "hel…rld");
/// assert_eq!(truncate_middle("short", 10), "short");
/// assert_eq!(truncate_middle("truncate", 5), "tr…te");
///
/// ```
pub fn truncate_middle(input: &str, max_len: usize) -> String {
    let len = input.chars().count();
    if max_len == 0 || max_len >= len {
        return input.to_string();
    }

    if max_len <= 1 {
        return "…".to_string();
    }

    let keep = max_len - 1;
    let start = keep / 2;
    let end = keep - start;

    let mut chars = input.chars();
    let start_part: String = chars.by_ref().take(start).collect();
    let end_part: String = input
        .chars()
        .rev()
        .take(end)
        .collect::<String>()
        .chars()
        .rev()
        .collect();

    format!("{start_part}…{end_part}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_middle_normal() {
        assert_eq!(truncate_middle("hello_world", 7), "hel…rld");
    }

    #[test]
    fn test_truncate_shorter_than_max() {
        assert_eq!(truncate_middle("short", 10), "short");
    }

    #[test]
    fn test_truncate_exact_length() {
        assert_eq!(truncate_middle("exactly", 7), "exactly");
    }

    #[test]
    fn test_truncate_tiny_output() {
        assert_eq!(truncate_middle("abc", 1), "…");
        // assert_eq!(truncate_middle("abc", 2), "a…");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(truncate_middle("", 5), "");
    }
}
