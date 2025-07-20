/// Wraps a string to a given number of characters using a string break character.
///
/// # Arguments
/// - `input`: The string to wrap.
/// - `width`: The maximum line width.
/// - `break_str`: The break string (default is `\n`).
/// - `cut_long_words`: Whether to force-break long words (default `false`).
///
/// # Example
/// ```rust
/// use lo_::wordwrap;
///
/// let input = "Rust is blazing fast and memory-efficient.";
/// let wrapped = wordwrap(input, 10, "\n", false);
/// assert_eq!(wrapped, "Rust is\nblazing\nfast and\nmemory-efficient.");
///
/// ```
pub fn wordwrap(s: &str, width: usize, break_str: &str, cut: bool) -> String {
    if width == 0 {
        return s.to_string();
    }

    let mut b_str: &str = "\n";
    if break_str.trim() != "" {
        b_str = break_str;
    }

    let mut result = String::new();
    for line in s.lines() {
        let mut current = String::new();

        for word in line.split_whitespace() {
            if !cut && word.len() > width {
                // Don't cut long words, just push the whole word
                if !current.is_empty() {
                    result.push_str(current.trim_end());
                    result.push_str(b_str);
                    current.clear();
                }
                result.push_str(word);
                result.push_str(b_str);
            } else if cut && word.len() > width {
                // Cut long words
                let mut start = 0;
                let chars: Vec<char> = word.chars().collect();
                while start < chars.len() {
                    let end = usize::min(start + width, chars.len());
                    let slice: String = chars[start..end].iter().collect();

                    if !current.is_empty() {
                        result.push_str(current.trim_end());
                        result.push_str(b_str);
                        current.clear();
                    }
                    result.push_str(&slice);
                    result.push_str(b_str);
                    start = end;
                }
            } else {
                // Regular word fits
                if current.len() + word.len() + 1 > width {
                    result.push_str(current.trim_end());
                    result.push_str(b_str);
                    current.clear();
                }
                current.push_str(word);
                current.push(' ');
            }
        }

        if !current.is_empty() {
            result.push_str(current.trim_end());
            result.push_str(b_str);
        }
    }

    // Remove trailing break if exists
    if result.ends_with(b_str) {
        result.truncate(result.len() - b_str.len());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordwrap_cut() {
        let input = "longwordtoolong here";
        let expected = "longwordt\noolong\nhere";
        assert_eq!(wordwrap(input, 9, "\n", true), expected);
    }

    #[test]
    fn test_wordwrap_no_cut() {
        let input = "Rust is blazing fast and memory-efficient.";
        let expected = "Rust is\nblazing\nfast and\nmemory-efficient.";
        assert_eq!(wordwrap(input, 10, "\n", false), expected);
    }

    #[test]
    fn test_wordwrap_default_break() {
        let input = "Rust is blazing fast and memory-efficient.";
        let expected = "Rust is\nblazing\nfast and\nmemory-efficient.";
        assert_eq!(wordwrap(input, 10, "", false), expected);
    }
}
