/// Check whether the given value is present in the array or not.
///
/// # Example
/// ```rust
/// use lo_::without;
/// let array = vec![1, 2, 3, 4];
/// let remove = vec![1, 3, 4];
/// let result = without(&array, &remove);
/// assert_eq!(result, vec![2]);
///
/// ```
pub fn without<T: PartialEq + Clone>(arr: &[T], values: &[T]) -> Vec<T> {
    arr.iter()
        .cloned()
        .filter(|x| !values.contains(&x))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_without() {
        let input = vec![2, 1, 2, 3];
        let values_to_remove = vec![1, 2];
        let result = without(&input, &values_to_remove);

        assert_eq!(result, vec![3]);
    }
}
