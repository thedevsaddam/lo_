/// Converts a string to Title Case.
///
/// This means the first letter of each word is capitalized,
/// and the rest of the letters are lowercased. Words are
/// identified using Unicode whitespace.
///
/// # Examples
///
/// ```rust
/// use lo_::title_case;
/// let s = "hello world";
/// assert_eq!(title_case(s), "Hello World");
///
/// let s = "rUsT iS aweSome";
/// assert_eq!(title_case(s), "Rust Is Awesome");
///
/// ```
///
/// # Notes
/// - Handles Unicode characters correctly.
/// - Strips and normalizes excess whitespace.
pub fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::title_case;

    #[test]
    fn test_basic() {
        assert_eq!(title_case("hello world"), "Hello World");
        assert_eq!(title_case("rust is fun"), "Rust Is Fun");
    }

    #[test]
    fn test_mixed_case() {
        assert_eq!(title_case("rUsT iS aweSome"), "Rust Is Awesome");
        assert_eq!(title_case("JAVA script"), "Java Script");
    }

    #[test]
    fn test_unicode() {
        assert_eq!(title_case("çalışkan öğrenci"), "Çalışkan Öğrenci");
        assert_eq!(title_case("éLèVe élite"), "Élève Élite");
    }

    #[test]
    fn test_empty_and_whitespace() {
        assert_eq!(title_case(""), "");
        assert_eq!(title_case("    "), "");
        assert_eq!(title_case("   hello   world  "), "Hello World");
    }

    #[test]
    fn test_with_punctuation() {
        assert_eq!(
            title_case("hello-world from rust!"),
            "Hello-world From Rust!"
        );
    }
}
