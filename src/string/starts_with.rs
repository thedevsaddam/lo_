/// Checks if string starts with the given target string
///
/// # Example
/// ```rust
/// use lo_::string::*;
/// let content = "abcdef";
///  assert_eq!(starts_with(content, "abc"), true);
///
/// ```
pub fn starts_with(string: &str, target: &str) -> bool {
    return starts_with_position(string, target, None);
}

/// Checks if string starts with the given target string at some given position
///
/// # Example
/// ```rust
/// use lo_::string::*;
/// let content = "abcdef";
///  assert_eq!(starts_with_position(content, "abc", Some(0)), true);
///
/// ```
pub fn starts_with_position(string: &str, target: &str, position: Option<usize>) -> bool {
    let start_position = position.unwrap_or(0);
    if start_position > string.len() {
        false
    } else if start_position == string.len() {
        target.is_empty()
    } else {
        let remaining = &string[start_position..];
        remaining.starts_with(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_with() {
        // Test case 1: The string starts with the target at position 0
        assert_eq!(starts_with("abcdef", "abc"), true);
    }

    #[test]
    fn test_starts_with_position() {
        // Test case 1: The string starts with the target at position 0
        assert_eq!(starts_with_position("abcdef", "abc", Some(0)), true);

        // Test case 2: The string does not start with the target at position 2
        assert_eq!(starts_with_position("abcdef", "def", Some(2)), false);

        // Test case 3: The target is an empty string, so it should always return true
        assert_eq!(starts_with_position("abcdef", "", Some(2)), true);

        // Test case 4: The position is not provided, and it starts with the target at position 0
        assert_eq!(starts_with_position("abcdef", "abc", None), true);

        // Test case 5: The position is not provided, and it does not start with the target at position 2
        assert_eq!(starts_with_position("abcdef", "cde", None), false);

        // Test case 6: The position is greater than or equal to the string length, so it should return false
        assert_eq!(starts_with_position("abcdef", "abc", Some(10)), false);

        // Test case 7: The position is equal to the string length, and the target is an empty string, so it should return true
        assert_eq!(starts_with_position("abcdef", "", Some(6)), true);
    }
}
