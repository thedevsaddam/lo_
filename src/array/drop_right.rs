/// Creates a slice of array with n elements dropped from the end.
///
/// # Example
/// ```rust
/// use lo_::drop_right;
/// let input = vec![1, 2, 3];
/// let expected = vec![1];
/// let result = drop_right(&input, 2);
/// assert_eq!(result, expected);
///
/// ```
pub fn drop_right<T: Clone>(slice: &[T], n: usize) -> Vec<T> {
    let len = slice.len();
    if n >= len {
        Vec::new()
    } else {
        slice[0..len - n].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_right_empty() {
        let input: Vec<i32> = vec![];
        let n = 0;
        let result = drop_right(&input, n);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_drop_right_none() {
        let input = vec![1, 2, 3, 4, 5];
        let n = 0;
        let result = drop_right(&input, n);
        assert_eq!(result, input);
    }

    #[test]
    fn test_drop_right_some() {
        let input = vec![0, 1, 2, 3, 4, 5];
        let n = 2;
        let result = drop_right(&input, n);
        assert_eq!(result, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_drop_right_all() {
        let input = vec![1, 2, 3, 4, 5];
        let n = 5;
        let result = drop_right(&input, n);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_drop_right_more_than_length() {
        let input = vec![1, 2, 3, 4, 5];
        let n = 10;
        let result = drop_right(&input, n);
        assert_eq!(result, vec![]);
    }
}
