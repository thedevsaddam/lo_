use crate::string::*;
use std::borrow::ToOwned;
use std::collections::HashMap;
use std::str::FromStr;

/// Provides case-related string transformations.
///
/// # Examples
///
/// ```rust
/// use lo_::CaseTransform;
///
/// let s = "hello_world";
/// assert_eq!(s.to_camel_case(), "helloWorld");
/// assert_eq!(s.to_snake_case(), "hello_world");
/// assert_eq!(s.to_title_case(), "Hello_world");
/// assert_eq!(s.to_lower_first(), "hello_world");
/// assert_eq!(s.to_upper_first(), "Hello_world");
/// assert_eq!(s.to_slug(), "hello-world");
/// assert_eq!(s.to_capitalize(), "Hello_world");
/// ```
pub trait CaseTransform: ToOwned + AsRef<str> {
    fn to_lower_first(&self) -> String {
        lower_first(self.as_ref())
    }
    fn to_upper_first(&self) -> String {
        upper_first(self.as_ref())
    }
    fn to_camel_case(&self) -> String {
        camel_case(self.as_ref())
    }
    fn to_snake_case(&self) -> String {
        snake_case(self.as_ref())
    }
    fn to_title_case(&self) -> String {
        title_case(self.as_ref())
    }
    fn to_slug(&self) -> String {
        slugify(self.as_ref())
    }
    fn to_capitalize(&self) -> String {
        capitalize(self.as_ref())
    }
}

/// Provides word-based and wrapping utilities.
///
/// # Examples
///
/// ```rust
/// use lo_::WordTransform;
/// use lo_::Alignment;
///
/// let s = "hello world, this is Rust";
///
/// let words = s.to_words();
/// assert_eq!(words, vec!["hello", "world", "this", "is", "Rust"]);
///
/// ```
pub trait WordTransform: ToOwned + AsRef<str> {
    fn to_words(&self) -> Vec<String> {
        words(self.as_ref())
    }
    fn wrap(&self, width: usize, break_str: &str, cut: bool) -> String {
        wordwrap(self.as_ref(), width, break_str, cut)
    }
    fn wordwrap(&self, width: usize, break_str: &str, cut: bool) -> String {
        wordwrap(self.as_ref(), width, break_str, cut)
    }
}

/// Provides miscellaneous string utilities like reversing, splitting,
/// truncation, templating, parsing, and padding.
///
/// # Examples
///
/// ```rust
/// use lo_::UtilityTransform;
/// use std::collections::HashMap;
/// use lo_::Alignment;
///
/// let s = "hello world";
///
/// assert_eq!(s.str_rev(), "dlrow olleh");
///
/// let parts = s.str_split(" ");
/// assert_eq!(parts, vec!["hello", "world"]);
///
/// assert_eq!(s.to_truncate_middle(5), "he…ld");
///
/// let mut values = HashMap::new();
/// values.insert("name", "Alice");
/// let templated = "Hello {name}".to_template(&values);
/// assert_eq!(templated, "Hello Alice");
///
/// let num: Option<i32> = "123".to_safe_parse();
/// assert_eq!(num, Some(123));
///
/// let padded = s.pad(15, "-", Alignment::Right);
/// assert_eq!(padded.len(), 15);
/// ```
pub trait UtilityTransform: ToOwned + AsRef<str> {
    fn str_rev(&self) -> String {
        str_rev(self.as_ref())
    }
    fn str_split(&self, delimiter: &str) -> Vec<String> {
        str_split(self.as_ref(), delimiter)
    }
    fn to_truncate_middle(&self, max_len: usize) -> String {
        truncate_middle(self.as_ref(), max_len)
    }
    fn to_template(&self, values: &HashMap<&str, &str>) -> String {
        template(self.as_ref(), values)
    }
    fn to_safe_parse<T: FromStr>(&self) -> Option<T> {
        safe_parse(self.as_ref())
    }
    fn pad(&self, length: usize, pad_str: &str, pad_type: Alignment) -> String {
        str_pad(self.as_ref(), length, pad_str, pad_type)
    }
}

// Implement for `str` and `String`
impl CaseTransform for str {}
impl WordTransform for str {}
impl UtilityTransform for str {}

impl CaseTransform for String {}
impl WordTransform for String {}
impl UtilityTransform for String {}

/// The `Transform` trait provides common string manipulation methods for
/// string-like types, enabling easy and consistent transformations such as
/// case conversions, truncation, parsing, and templating.
///
pub trait Transform: CaseTransform + WordTransform + UtilityTransform {}

impl<T> Transform for T where T: CaseTransform + WordTransform + UtilityTransform {}

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

    #[test]
    fn test_to_pad_from_string() {
        assert_eq!("42".pad(5, "0", Alignment::Left), "00042");
    }
}
