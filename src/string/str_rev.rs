/// Reverses the input string.
///
/// This function returns a new `String` where the characters
/// of the input string `s` are reversed in order.
///
/// # Examples
///
/// ```rust
/// use lo_::str_rev;
/// let original = "hello";
/// let reversed = str_rev(original);
/// assert_eq!(reversed, "olleh");
///
/// let unicode = "你好";
/// let reversed_unicode = str_rev(unicode);
/// assert_eq!(reversed_unicode, "好你");
///
/// ```
///
/// # Note
///
/// This function reverses by Unicode scalar values, so it handles
/// multi-byte UTF-8 characters correctly.
pub fn str_rev(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii() {
        assert_eq!(str_rev("hello"), "olleh");
        assert_eq!(str_rev(""), "");
        assert_eq!(str_rev("a"), "a");
    }

    #[test]
    fn test_unicode() {
        assert_eq!(str_rev("你好"), "好你");
        assert_eq!(str_rev("👋🌍"), "🌍👋");
    }

    #[test]
    fn test_mixed() {
        assert_eq!(str_rev("héllo"), "olléh");
        assert_eq!(str_rev("rust🦀lang"), "gnal🦀tsur");
    }
}
