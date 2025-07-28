/// Converts a string to [SCREAMING_SNAKE_CASE](https://en.wiktionary.org/wiki/screaming_snake_case).
///
/// # Examples
/// ```rust
/// use lo_::screaming_snake_case;
///
/// assert_eq!(screaming_snake_case("hello world"), "HELLO_WORLD");
/// assert_eq!(screaming_snake_case("   foo bar   "), "FOO_BAR");
/// ```
///
pub fn screaming_snake_case(s: &str) -> String {
    s.split_whitespace()
        .map(|w| w.to_uppercase())
        .collect::<Vec<_>>()
        .join("_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        assert_eq!(screaming_snake_case("hello world"), "HELLO_WORLD");
    }

    #[test]
    fn test_trailing_leading_spaces() {
        assert_eq!(screaming_snake_case("   foo bar   "), "FOO_BAR");
    }

    #[test]
    fn test_multiple_spaces() {
        assert_eq!(screaming_snake_case("foo   bar"), "FOO_BAR");
    }

    #[test]
    fn test_already_uppercase() {
        assert_eq!(screaming_snake_case("HELLO WORLD"), "HELLO_WORLD");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(screaming_snake_case("hello"), "HELLO");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(screaming_snake_case(""), "");
    }

    #[test]
    fn test_numerics_and_symbols() {
        assert_eq!(screaming_snake_case("version 2.0"), "VERSION_2.0");
    }
}
