/// Creates a slice of array with n elements dropped from the beginning.
///
/// # Example
/// ```rust
/// use lo_::drop;
/// let input = vec![1, 2, 3];
/// let expected = vec![3];
/// let result = drop(&input, 2);
/// assert_eq!(result, expected);
///
/// ```
pub fn drop<T>(input: &[T], n: usize) -> Vec<T>
where
    T: Clone,
{
    if n >= input.len() {
        Vec::new()
    } else {
        let (_, tail) = input.split_at(n);
        tail.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_empty() {
        let input: Vec<i32> = vec![];
        let n = 0;
        let result = drop(&input, n);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_drop_none() {
        let input = vec![1, 2, 3, 4, 5];
        let n = 0;
        let result = drop(&input, n);
        assert_eq!(result, input);
    }

    #[test]
    fn test_drop_some() {
        let input = vec![0, 1, 2, 3, 4, 5];
        let n = 2;
        let result = drop(&input, n);
        assert_eq!(result, vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_drop_all() {
        let input = vec![1, 2, 3, 4, 5];
        let n = 5;
        let result = drop(&input, n);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_drop_more_than_length() {
        let input = vec![1, 2, 3, 4, 5];
        let n = 10;
        let result = drop(&input, n);
        assert_eq!(result, vec![]);
    }
}
