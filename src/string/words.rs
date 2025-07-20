use regex::Regex;
use std::collections::VecDeque;

/// Splits string into an array of its words.
///
/// # Example
/// ```rust
/// use lo_::words;
/// let input = "fred, barney, & pebbles";
/// let expected = vec!["fred", "barney", "pebbles"];
/// let result = words(input);
/// assert_eq!(result, expected);
///
/// ```
///
pub fn words(string: &str) -> Vec<String> {
    words_with_pattern(string, None)
}

/// Splits string into an array of its words with the given pattern.
///
/// # Example
/// ```rust
/// use lo_::string::*;
/// use regex::Regex;
/// let input = "fred, barney, & pebbles";
/// let expected = vec!["fred", "barney", "&","pebbles"];
/// let result = words_with_pattern(input,Some(Regex::new("[^, ]+").unwrap()));
/// assert_eq!(result, expected);
///
/// ```
///
pub fn words_with_pattern(string: &str, re: Option<Regex>) -> Vec<String> {
    match re {
        Some(rgx) => rgx
            .find_iter(string)
            .map(|w| w.as_str().to_string())
            .collect(),
        None => {
            let mut result = VecDeque::new();
            let mut word = String::new();

            for char in string.chars() {
                if char.is_alphanumeric() {
                    word.push(char);
                } else if !word.is_empty() {
                    result.push_back(word);
                    word = String::new();
                }
            }

            if !word.is_empty() {
                result.push_back(word);
            }

            result.into_iter().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words() {
        // Test with a sample input
        let input = "fred, barney, & pebbles";
        let expected = vec!["fred", "barney", "pebbles"];
        let result = words(input);
        assert_eq!(result, expected);

        // Test with an input containing multiple spaces and commas
        let input2 = "  hello,   world   ,   this, is ,a, test   ";
        let expected2 = vec!["hello", "world", "this", "is", "a", "test"];
        let result2 = words(input2);
        assert_eq!(result2, expected2);

        // Test with an input containing no words
        let input3 = "   ";
        let expected3: Vec<String> = Vec::new();
        let result3 = words(input3);
        assert_eq!(result3, expected3);

        // Test with an input containing no words
        let input4 = "   ";
        let expected4: Vec<String> = Vec::new();
        let result4 = words(input4);
        assert_eq!(result4, expected4);
    }

    #[test]
    fn test_words_with_pattern() {
        let string = "fred, barney, &, pebbles";
        let ww = words_with_pattern(string, Some(Regex::new("[^, ]+").unwrap()));

        assert_eq!(ww, vec!["fred", "barney", "&", "pebbles"]);
    }
}
