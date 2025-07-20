use regex::Regex;
use std::collections::HashMap;

/// Replaces placeholders in the given text with corresponding values from a hashmap.
///
/// This function takes a text string containing placeholders in the form of `{key}` and replaces them
/// with values from a provided hashmap. It returns a new string with the placeholders substituted by
/// their respective values.
///
/// # Example
/// ```rust
/// use lo_::template;
/// use std::collections::HashMap;
/// let txt = "Hello, my name is {name} and I'm {age} years old.";
/// let mut values = HashMap::new();
/// values.insert("name", "Tom");
/// values.insert("age", "20");
/// assert_eq!(template(txt, &values), "Hello, my name is Tom and I'm 20 years old.");
///
/// ```
///
pub fn template(txt: &str, values: &HashMap<&str, &str>) -> String {
    let mut parsed_txt = String::from(txt);

    for (key, value) in values.iter() {
        let pattern = format!("\\{{{key}\\}}");
        let re = Regex::new(&pattern).unwrap();
        parsed_txt = re
            .replace_all(&parsed_txt, |_captures: &regex::Captures<'_>| {
                value.to_string()
            })
            .to_string();
    }

    parsed_txt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_output() {
        let txt = "Thank you for being with {name}.";
        let mut values = HashMap::new();
        values.insert("name", "lo_");
        assert_eq!(template(txt, &values), "Thank you for being with lo_.");
    }

    #[test]
    fn test_correct_output_for_multiple_values() {
        let txt = "Your bill {amount}TK is due in {days} days. Tap to pay early";
        let mut values = HashMap::new();
        values.insert("amount", "100");
        values.insert("days", "5");
        assert_eq!(
            template(txt, &values),
            "Your bill 100TK is due in 5 days. Tap to pay early"
        );
    }

    #[test]
    fn test_missing_value() {
        let txt = "Thank you for being with {name}.";
        let values = HashMap::new();
        assert_eq!(template(txt, &values), "Thank you for being with {name}.");
    }

    #[test]
    fn test_no_pattern_found() {
        let txt = "Thank you for being with us.";
        let values = HashMap::new();
        assert_eq!(template(txt, &values), "Thank you for being with us.");
    }
}
