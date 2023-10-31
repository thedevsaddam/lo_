/// Converts the first character of string to upper case and the remaining to lower case.
///
/// # Example
/// ```rust
/// use lo_::string::*;
///  assert_eq!(capitalize("FRED"), "Fred");
///
/// ```
///
pub fn capitalize(string: &str) -> String {
    let mut result = String::with_capacity(string.len());
    let mut is_first_char = true;

    for c in string.chars() {
        if is_first_char {
            result.push(c.to_ascii_uppercase());
            is_first_char = false;
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_empty_string() {
        let result = capitalize("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_capitalize_single_char() {
        let result = capitalize("a");
        assert_eq!(result, "A");
    }

    #[test]
    fn test_capitalize_lowercase() {
        let result = capitalize("hello");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_capitalize_uppercase() {
        let result = capitalize("FRED");
        assert_eq!(result, "Fred");
    }

    #[test]
    fn test_capitalize_mixed_case() {
        let result = capitalize("mIXeD");
        assert_eq!(result, "Mixed");
    }
}
