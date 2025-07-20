/// Defines the padding type: Left, Right, or Center.
/// Alignment options for padding strings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

/// Pad a string to a certain length using another string as padding.
///
/// # Arguments
/// - `input` – The original string to pad.
/// - `length` – The total length after padding.
/// - `pad_str` – The string to pad with.
/// - `pad_type` – Alignment enum: Left, Right, or Center.
///
/// # Returns
/// A new string padded to the desired length.
///
/// # Example
/// ```rust
/// use lo_::str_pad;
/// use lo_::Alignment::{Left, Right};
/// assert_eq!(str_pad("42", 5, "0", Left), "00042");
/// assert_eq!(str_pad("42", 5, " ", Right), "42   ");
///
/// ```
pub fn str_pad(input: &str, length: usize, pad_str: &str, pad_type: Alignment) -> String {
    let input_len = input.chars().count();
    if length <= input_len || pad_str.is_empty() {
        return input.to_string();
    }

    let pad_len = length - input_len;
    let repeat_times = pad_len.div_ceil(pad_str.chars().count());
    let full_pad: String = pad_str.repeat(repeat_times).chars().take(pad_len).collect();

    match pad_type {
        Alignment::Left => format!("{full_pad}{input}"),
        Alignment::Right => format!("{input}{full_pad}"),
        Alignment::Center => {
            let left_len = pad_len / 2;
            let right_len = pad_len - left_len;
            let left_pad: String = full_pad.chars().take(left_len).collect();
            let right_pad: String = full_pad.chars().skip(left_len).take(right_len).collect();
            format!("{left_pad}{input}{right_pad}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_left() {
        assert_eq!(str_pad("42", 5, "0", Alignment::Left), "00042");
    }

    #[test]
    fn test_pad_right() {
        assert_eq!(str_pad("foo", 7, "-", Alignment::Right), "foo----");
    }

    #[test]
    fn test_pad_both() {
        assert_eq!(str_pad("hi", 6, "*", Alignment::Center), "**hi**");
    }

    #[test]
    fn test_shorter_than_input() {
        assert_eq!(str_pad("hello", 3, "_", Alignment::Left), "hello");
    }

    #[test]
    fn test_empty_pad_str() {
        assert_eq!(str_pad("test", 10, "", Alignment::Right), "test");
    }
}
