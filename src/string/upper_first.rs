/// Converts the first character of string to lower case.
///
/// # Example
/// ```rust
/// use lo_::string::*;
///  assert_eq!(upper_first("fred"), "Fred");
///  assert_eq!(upper_first("FRED"), "FRED");
///
/// ```
///
pub fn upper_first(string: &str) -> String {
    let mut result = String::with_capacity(string.len());
    let mut is_first_char = true;

    for c in string.chars() {
        if is_first_char && c.is_alphanumeric() {
            result.push(c.to_ascii_uppercase());
            is_first_char = false;
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upper_first() {
        // Test with an empty string
        assert_eq!(upper_first(""), "".to_string());

        // Test with a single character string
        assert_eq!(upper_first("a"), "A".to_string());

        // Test with a string containing multiple characters
        assert_eq!(upper_first("hello"), "Hello".to_string());

        // Test with a string containing spaces
        assert_eq!(
            upper_first("  this is a test"),
            "  This is a test".to_string()
        );

        // Test with a string containing numbers and special characters
        assert_eq!(upper_first("123-abc"), "123-abc".to_string());
    }
}
