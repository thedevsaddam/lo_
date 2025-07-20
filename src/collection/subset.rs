/// Returns a subslice starting from the given offset with up to `length` elements.
/// Similar to `slice[start..start+length]`, but safe:
/// - Accepts negative `start` (interpreted as offset from the end)
/// - Never panics on overflow
///
/// # Examples
///
/// ```rust
/// use lo_::subset;
///
/// let input = [0, 1, 2, 3, 4];
///
/// assert_eq!(subset(&input, 2, 3), &[2, 3, 4]);
/// assert_eq!(subset(&input, -4, 3), &[1, 2, 3]);
/// assert_eq!(subset(&input, -2, usize::MAX), &[3, 4]);
/// assert_eq!(subset(&input, 10, 5), &[]);
/// assert_eq!(subset::<i32>(&[], 0, 3), &[]);
///
/// ```
pub fn subset<T>(slice: &[T], offset: isize, length: usize) -> &[T] {
    let len = slice.len();

    if len == 0 || length == 0 {
        return &[];
    }

    let start = if offset < 0 {
        len.saturating_sub(offset.unsigned_abs().min(len))
    } else {
        offset as usize
    };

    if start >= len {
        return &[];
    }

    let end = len.min(start.saturating_add(length));
    &slice[start..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_start() {
        let input = [0, 1, 2, 3, 4];
        assert_eq!(subset(&input, 2, 3), &[2, 3, 4]);
        assert_eq!(subset(&input, 4, 10), &[4]);
        assert_eq!(subset(&input, 5, 1), &[]);
    }

    #[test]
    fn test_negative_start() {
        let input = [0, 1, 2, 3, 4];
        assert_eq!(subset(&input, -4, 3), &[1, 2, 3]);
        assert_eq!(subset(&input, -2, usize::MAX), &[3, 4]);
        assert_eq!(subset(&input, -10, 2), &[0, 1]);
    }

    #[test]
    fn test_zero_length_or_empty_input() {
        let input = [1, 2, 3];
        assert_eq!(subset(&input, 1, 0), &[]);
        assert_eq!(subset::<i32>(&[], 0, 3), &[]);
    }
}
