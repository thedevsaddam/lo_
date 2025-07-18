use std::str::FromStr;

/// Tries to parse a string into a value of type `T`.
/// Returns `Some(T)` if successful, otherwise `None`.
///
/// # Examples
///
/// ```rust
/// use lo_::safe_parse;
/// assert_eq!(safe_parse::<i32>("42"), Some(42));
/// assert_eq!(safe_parse::<f64>("not-a-number"), None);
/// assert_eq!(safe_parse::<bool>("true"), Some(true));
/// ```
pub fn safe_parse<T: FromStr>(s: &str) -> Option<T> {
    T::from_str(s).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        assert_eq!(safe_parse::<i32>("123"), Some(123));
        assert_eq!(safe_parse::<i32>("abc"), None);
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(safe_parse::<f64>("3.14"), Some(3.14));

        let result = safe_parse::<f64>("NaN");
        assert!(result.is_some() && result.unwrap().is_nan());
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(safe_parse::<bool>("true"), Some(true));
        assert_eq!(safe_parse::<bool>("nope"), None);
    }
}
