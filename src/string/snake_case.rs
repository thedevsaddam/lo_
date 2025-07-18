pub fn snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    let mut prev_char: Option<char> = None;
    let mut prev_was_underscore = false;

    while let Some(ch) = chars.next() {
        if ch.is_ascii_alphanumeric() {
            let next_char = chars.peek();

            if ch.is_ascii_uppercase() {
                if let Some(prev) = prev_char {
                    let next_is_lower = next_char.map(|c| c.is_ascii_lowercase()).unwrap_or(false);
                    if prev.is_ascii_lowercase()
                        || prev.is_ascii_digit()
                        || (prev.is_ascii_uppercase() && next_is_lower)
                    {
                        if !prev_was_underscore {
                            result.push('_');
                            // prev_was_underscore = true;
                        }
                    }
                }
                result.push(ch.to_ascii_lowercase());
                prev_char = Some(ch);
                prev_was_underscore = false;
            } else {
                // lowercase letter or digit
                result.push(ch);
                prev_char = Some(ch);
                prev_was_underscore = false;
            }
        } else {
            // non-alphanumeric -> underscore, avoid duplicates
            if !prev_was_underscore && !result.is_empty() {
                result.push('_');
                prev_was_underscore = true;
            }
            prev_char = None;
        }
    }

    result.trim_matches('_').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case_basic() {
        assert_eq!(snake_case("HelloWorld"), "hello_world");
        assert_eq!(snake_case("JSONParserID"), "json_parser_id");
        assert_eq!(snake_case("camelCaseInput"), "camel_case_input");
        assert_eq!(snake_case("snake_case"), "snake_case");
    }

    #[test]
    fn test_to_snake_case_with_spaces_and_symbols() {
        assert_eq!(snake_case("  spaced input   "), "spaced_input");
        assert_eq!(snake_case("symbols!@#$%^&*()_+="), "symbols");
        assert_eq!(
            snake_case("MixOf UPPER and lower"),
            "mix_of_upper_and_lower"
        );
        assert_eq!(snake_case("__already__snake__case__"), "already_snake_case");
    }

    #[test]
    fn test_to_snake_case_with_numbers() {
        assert_eq!(
            snake_case("PascalCaseWith123Numbers"),
            "pascal_case_with123_numbers"
        );
        assert_eq!(snake_case("Hello123World456"), "hello123_world456");
    }

    #[test]
    fn test_to_snake_case_unicode() {
        assert_eq!(snake_case("Rust💖Lang"), "rust_lang");
        assert_eq!(snake_case("emoji😊test"), "emoji_test");
    }
}
