/// Converts string to camel case (https://en.wikipedia.org/wiki/Camel_case)
///
/// # Example
/// ```rust
/// use lo_::string::*;
///  assert_eq!(camel_case("Foo Bar"), "fooBar");
///
/// ```
///
pub fn camel_case(string: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in string.chars() {
        if !c.is_alphanumeric() {
            capitalize_next = true;
            continue;
        }

        if capitalize_next {
            if result.len() > 0 {
                result.push_str(&c.to_ascii_uppercase().to_string());
            } else {
                result.push_str(&c.to_ascii_lowercase().to_string());
            }
            capitalize_next = false;
        } else {
            result.push_str(&c.to_ascii_lowercase().to_string());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case_spaces() {
        assert_eq!(camel_case("Foo Bar"), "fooBar");
    }

    #[test]
    fn test_camel_case_dashes() {
        assert_eq!(camel_case("--foo-bar--"), "fooBar");
    }

    #[test]
    fn test_camel_case_underscores() {
        assert_eq!(camel_case("__FOO_BAR__"), "fooBar");
    }

    #[test]
    fn test_camel_case_mixed() {
        assert_eq!(camel_case("hello_world-example"), "helloWorldExample");
    }

    #[test]
    fn test_camel_case_empty_string() {
        assert_eq!(camel_case(""), "");
    }

    #[test]
    fn test_camel_case_single_word() {
        assert_eq!(camel_case("word"), "word");
    }
}
