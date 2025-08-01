/// Converts string to [camelCase](https://en.wikipedia.org/wiki/Camel_case).
///
/// # Example
/// ```rust
/// use lo_::camel_case;
/// assert_eq!(camel_case("Foo Bar"), "fooBar");
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
            if !result.is_empty() {
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c.to_ascii_lowercase());
            }
            capitalize_next = false;
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
