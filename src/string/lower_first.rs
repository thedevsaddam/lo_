/// Converts the first character of string to lower case.
///
/// # Example
/// ```rust
/// use lo_::string::*;
///  assert_eq!(lower_first("Fred"), "fred");
///  assert_eq!(lower_first("FRED"), "fRED");
///
/// ```
///
pub fn lower_first(string: &str) -> String {
    let mut result = String::with_capacity(string.len());
    let mut is_first_char = true;

    for c in string.chars() {
        if is_first_char && c.is_alphanumeric() {
            result.push(c.to_ascii_lowercase());
            is_first_char = false;
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::lower_first;

    #[test]
    fn test_lower_first_empty_string() {
        let input = "";
        let result = lower_first(input);
        assert_eq!(result, "");
    }

    #[test]
    fn test_lower_first_single_char() {
        let input = "A";
        let result = lower_first(input);
        assert_eq!(result, "a");
    }

    #[test]
    fn test_lower_first_all_lowercase() {
        let input = "rust";
        let result = lower_first(input);
        assert_eq!(result, "rust");
    }

    #[test]
    fn test_lower_first_all_uppercase() {
        let input = "RUST";
        let result = lower_first(input);
        assert_eq!(result, "rUST");
    }

    #[test]
    fn test_lower_first_mixed_case() {
        let input = "RusT";
        let result = lower_first(input);
        assert_eq!(result, "rusT");
    }

    #[test]
    fn test_lower_first_with_space() {
        // Test with a string containing spaces
        assert_eq!(
            lower_first("  This is a test"),
            "  this is a test".to_string()
        );
    }
}
