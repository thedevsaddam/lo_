/// Splits a string by the given delimiter and returns a `Vec<String>`.
///
/// # Example
/// ```rust
/// use lo_::str_split;
/// let csv = "id,name,age";
/// let result = str_split(csv, ",");
/// assert_eq!(result, vec!["id", "name", "age"]);
///
/// ```
pub fn str_split(input: &str, delimiter: &str) -> Vec<String> {
    input.split(delimiter).map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_split() {
        let input = "a,b,c";
        let result = str_split(input, ",");
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_empty_elements() {
        let input = "a,,c";
        let result = str_split(input, ",");
        assert_eq!(result, vec!["a", "", "c"]);
    }

    #[test]
    fn test_no_delimiter() {
        let input = "abc";
        let result = str_split(input, ",");
        assert_eq!(result, vec!["abc"]);
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let result = str_split(input, ",");
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn test_multi_char_delimiter() {
        let input = "key::value::meta";
        let result = str_split(input, "::");
        assert_eq!(result, vec!["key", "value", "meta"]);
    }
}
