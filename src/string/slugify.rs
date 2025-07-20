/// Converts a string to a URL-friendly slug format.
///
/// # Examples
///
/// ```rust
/// use lo_::slugify;
/// assert_eq!(slugify("Hello World!"), "hello-world");
/// assert_eq!(slugify("Rust is awesome 🚀"), "rust-is-awesome");
/// assert_eq!(slugify(" Clean   URL__String "), "clean-url-string");
///
/// ```
pub fn slugify(input: &str) -> String {
    use regex::Regex;

    // Replace non-alphanumeric characters with dashes
    let re_non_alnum = Regex::new(r"[^\p{L}\p{N}]+").unwrap(); // Unicode aware
    let re_trim_dashes = Regex::new(r"^-+|-+$").unwrap();

    let slug = input.to_lowercase().trim().to_string();
    let slug = re_non_alnum.replace_all(&slug, "-");
    let slug = re_trim_dashes.replace_all(&slug, "");

    slug.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_slug() {
        assert_eq!(slugify("Hello World!"), "hello-world");
    }

    #[test]
    fn test_unicode_input() {
        assert_eq!(slugify("Café del Mar"), "café-del-mar");
    }

    #[test]
    fn test_multiple_spaces_and_symbols() {
        assert_eq!(slugify(" Clean   URL__String "), "clean-url-string");
    }

    #[test]
    fn test_emojis_and_specials() {
        assert_eq!(slugify("Rust is awesome 🚀!"), "rust-is-awesome");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn test_only_symbols() {
        assert_eq!(slugify("$$$%%%!!!"), "");
    }
}
