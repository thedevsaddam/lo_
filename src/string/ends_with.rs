/// Checks if string ends with the given target string
///
/// # Example
/// ```rust
/// use lo_::ends_with;
/// let content = "abc";
/// assert_eq!(ends_with(content, "b"), false);
/// assert_eq!(ends_with(content, "c"), true);
///
/// ```
///
pub fn ends_with(string: &str, target: &str) -> bool {
    ends_with_position(string, target, None)
}

/// Checks if string ends with the given target string with the given position
///
/// # Example
/// ```rust
/// use lo_::string::*;
/// let content = "abc";
/// assert_eq!(ends_with_position(content, "b", Some(2)), true);
///
/// ```
///
pub fn ends_with_position(string: &str, target: &str, position: Option<usize>) -> bool {
    let end_position = position.unwrap_or(string.len());
    if end_position == 0 {
        return target.is_empty();
    }

    if end_position > target.len() {
        let start_position = end_position - target.len();
        let remaining = &string[start_position..end_position];
        return remaining == target;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ends_with() {
        assert_eq!(ends_with("abc", "b"), false);
        assert_eq!(ends_with("abc", "c"), true);
    }

    #[test]
    fn test_ends_with_position() {
        assert_eq!(ends_with_position("abc", "b", Some(2)), true);
        assert_eq!(ends_with_position("abc", "bc", Some(3)), true);
    }
}
