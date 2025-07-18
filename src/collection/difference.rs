/// Creates an array of array values not included in the other given arrays.The order and references of result values are determined by the first array.
///
/// # Example
/// ```rust
/// use lo_::difference;
/// let array1 = vec![1, 2];
/// let array2 = vec![2, 3];
/// let result = difference(&array1, &array2);
/// assert_eq!(result, vec![1]);
///
/// ```
pub fn difference<T: Eq + std::hash::Hash + Clone>(array1: &[T], array2: &[T]) -> Vec<T> {
    // Convert the second array to a HashSet for efficient lookup
    let set2: std::collections::HashSet<&T> = array2.iter().collect();

    // Filter elements in the first array that are not in the second array
    array1
        .iter()
        .filter(|&item| !set2.contains(item))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference_with_common_elements() {
        let array1 = vec![1, 2, 3, 4, 5];
        let array2 = vec![4, 5, 6, 7, 8];
        let result = difference(&array1, &array2);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_difference_with_no_common_elements() {
        let array1 = vec![1, 2, 3];
        let array2 = vec![4, 5, 6];
        let result = difference(&array1, &array2);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_difference_with_empty_arrays() {
        let array1: Vec<i32> = vec![];
        let array2: Vec<i32> = vec![];
        let result = difference(&array1, &array2);
        assert_eq!(result, vec![]);
    }
}
