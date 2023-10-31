/// Check whether the given value is present in the array or not.
///
/// # Example
/// ```rust
/// use lo_::array::*;
/// let array = vec![1, 2, 3, 4, 5, 6, 7];
/// let element = 3;
/// let result = contains(&array, &element);
/// assert_eq!(result, true);
///
/// ```
pub fn contains<T: PartialEq>(arr: &[T], element: &T) -> bool {
    arr.iter().any(|e| e == element)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_element() {
        let arr = [1, 2, 3, 4, 5];

        // Test for an element present in the array
        let element_to_check = 3;
        assert!(contains(&arr, &element_to_check));

        // Test for an element not present in the array
        let element_not_in_array = 6;
        assert!(!contains(&arr, &element_not_in_array));
    }
}
