use unicode_general_category::get_general_category;
use unicode_general_category::GeneralCategory;
use unicode_normalization::UnicodeNormalization;

/// Converts a string to [kebab-case](https://developer.mozilla.org/en-US/docs/Glossary/Kebab_case).
///
/// ### Behavior:
/// - Converts all characters to lowercase.
/// - Normalizes Unicode characters to ASCII (e.g., "résumé" → "resume").
/// - Replaces spaces and underscores with hyphens.
/// - Removes punctuation and special characters (e.g., emojis, symbols).
/// - Trims extra hyphens and avoids duplicates.
///
/// ### Examples
/// ```rust
/// use lo_::kebab_case;
///
/// assert_eq!(kebab_case("Hello World!"), "hello-world");
/// assert_eq!(kebab_case("My Résumé"), "my-resume");
/// assert_eq!(kebab_case("  Foo___Bar!!"), "foo-bar");
///
/// ```
///
pub fn kebab_case(s: &str) -> String {
    let cleaned = s
        .nfkd() // Normalize: café → c + a + f + e + ́
        .filter(|c| {
            // Remove combining marks and non-word symbols
            matches!(
                get_general_category(*c),
                GeneralCategory::UppercaseLetter
                    | GeneralCategory::LowercaseLetter
                    | GeneralCategory::DecimalNumber
                    | GeneralCategory::SpaceSeparator
                    | GeneralCategory::DashPunctuation
            ) || *c == '_'
        })
        .collect::<String>();

    cleaned
        .to_lowercase()
        .replace('_', " ")
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_basic_strings() {
        assert_eq!(kebab_case("Hello World"), "hello-world");
        assert_eq!(kebab_case("foo_bar Baz"), "foo-bar-baz");
        assert_eq!(kebab_case("SimpleTEST_String"), "simpletest-string");
    }

    #[test]
    fn handles_unicode_and_accents() {
        assert_eq!(kebab_case("Mëtàl Résumé 🎸!"), "metal-resume");
        assert_eq!(kebab_case("Café Déjà Vu"), "cafe-deja-vu");
    }

    #[test]
    fn strips_symbols_and_emojis() {
        assert_eq!(kebab_case("Rust 💖 is 🔥"), "rust-is");
        assert_eq!(kebab_case("@hello#world$%^&"), "helloworld");
    }

    #[test]
    fn handles_numbers() {
        assert_eq!(kebab_case("Version 2.0.1-alpha"), "version-201-alpha");
        assert_eq!(kebab_case("user_123_test"), "user-123-test");
    }

    #[test]
    fn trims_and_sanitizes_whitespace() {
        assert_eq!(kebab_case("   A   B   C   "), "a-b-c");
        assert_eq!(kebab_case("   kebab-case   test "), "kebab-case-test");
    }

    #[test]
    fn filters_non_ascii_completely() {
        assert_eq!(kebab_case("中文测试"), "");
    }

    #[test]
    fn empty_string_returns_empty() {
        assert_eq!(kebab_case(""), "");
        assert_eq!(kebab_case("    "), "");
        assert_eq!(kebab_case("!!!"), "");
    }
}
