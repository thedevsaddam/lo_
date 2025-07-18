use crate::string::*;
use std::collections::HashMap;
use std::str::FromStr;
use std::{borrow::ToOwned, string::String};

/// The `Transform` trait provides common string manipulation methods for
/// string-like types, enabling easy and consistent transformations such as
/// case conversions, truncation, parsing, and templating.
///
/// It abstracts over owned and borrowed strings (`ToOwned`), allowing
/// seamless usage across different string types.
///
/// # Key methods include:
/// - Case conversions: `to_camel_case`, `to_snake_case`, `to_slug`, `to_capitalize`
/// - Text modifications: `to_lower_first`, `to_upper_first`, `to_truncate_middle`
/// - Utilities: `to_words`, `to_template`
///
/// Implementing this trait centralizes string utilities, reducing boilerplate
/// and improving code clarity.
///
/// ```rust
/// use lo_::Transform;
///
/// let s = "hello_world";
/// assert_eq!(s.to_camel_case(), "helloWorld");
/// ```
pub trait Transform: ToOwned {
    fn to_lower_first(&self) -> String;
    fn to_upper_first(&self) -> String;
    fn to_camel_case(&self) -> String;
    fn to_snake_case(&self) -> String;
    fn to_slug(&self) -> String;
    fn to_capitalize(&self) -> String;
    fn to_words(&self) -> Vec<String>;
    fn to_truncate_middle(&self, max_len: usize) -> String;
    fn to_template(&self, values: &HashMap<&str, &str>) -> String;
    fn to_safe_parse<T: FromStr>(&self) -> Option<T>;
}

impl Transform for String {
    fn to_lower_first(&self) -> String {
        lower_first(self)
    }

    fn to_upper_first(&self) -> String {
        upper_first(self)
    }

    fn to_camel_case(&self) -> String {
        camel_case(self)
    }

    fn to_snake_case(&self) -> String {
        snake_case(self)
    }

    fn to_slug(&self) -> String {
        slugify(self)
    }

    fn to_capitalize(&self) -> String {
        capitalize(self)
    }

    fn to_words(&self) -> Vec<String> {
        words(self)
    }

    fn to_truncate_middle(&self, max_len: usize) -> String {
        truncate_middle(self, max_len)
    }

    fn to_template(&self, values: &HashMap<&str, &str>) -> String {
        template(self, values)
    }

    fn to_safe_parse<T: FromStr>(&self) -> Option<T> {
        safe_parse(self)
    }
}

impl Transform for str {
    fn to_lower_first(&self) -> String {
        lower_first(self)
    }

    fn to_upper_first(&self) -> String {
        upper_first(self)
    }

    fn to_camel_case(&self) -> String {
        camel_case(self)
    }

    fn to_snake_case(&self) -> String {
        snake_case(self)
    }

    fn to_slug(&self) -> String {
        slugify(self)
    }

    fn to_capitalize(&self) -> String {
        capitalize(self)
    }

    fn to_words(&self) -> Vec<String> {
        words(self)
    }

    fn to_truncate_middle(&self, max_len: usize) -> String {
        truncate_middle(self, max_len)
    }

    fn to_template(&self, values: &HashMap<&str, &str>) -> String {
        template(self, values)
    }

    fn to_safe_parse<T: FromStr>(&self) -> Option<T> {
        safe_parse(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_lower_first_from_str() {
        let input: &str = "Hello, World!";
        let expected = "hello, World!";
        assert_eq!(input.to_lower_first(), expected);
    }

    #[test]
    fn test_to_lower_first_from_string() {
        let input = String::from("Hello, World!");
        let expected = "hello, World!";
        assert_eq!(input.to_lower_first(), expected);
    }

    #[test]
    fn test_to_slug_from_str() {
        let input: &str = "Hello, World!";
        let expected = "hello-world";
        assert_eq!(input.to_slug(), expected);
    }

    #[test]
    fn test_to_slug_from_string() {
        let input = String::from("Hello, World!");
        let expected = "hello-world";
        assert_eq!(input.to_slug(), expected);
    }
}
