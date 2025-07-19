use std::collections::HashSet;
use std::hash::Hash;

/// Creates an array of unique values that are included in all given arrays. The order and references of result values are determined by the first array.
///
/// # Example
/// ```rust
/// use lo_::intersection;
/// let array1 = vec![1, 2, 3];
/// let array2 = vec![3, 4, 5];
/// let mut result = intersection(&array1, &array2);
/// assert_eq!(result, vec![3]);
///
/// ```
pub fn intersection<T: Eq + Clone + Hash>(array1: &[T], array2: &[T]) -> Vec<T> {
    let set1: HashSet<&T> = array1.iter().collect();
    let set2: HashSet<&T> = array2.iter().collect();

    // Find the common elements and clone them into a Vec
    let common_elements: Vec<T> = set1
        .intersection(&set2)
        .cloned()
        .cloned() // Clone the elements to convert from &&T to T
        .collect();

    common_elements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_with_common_elements() {
        let array1 = vec![1, 2, 3, 4, 5];
        let array2 = vec![4, 5, 6, 7, 8];
        let mut result = intersection(&array1, &array2);
        result.sort(); // Sort the result
        let mut expected = vec![4, 5];
        expected.sort(); // Sort the expected output
        assert_eq!(result, expected);
    }

    #[test]
    fn test_intersection_with_no_common_elements() {
        let array1 = vec![1, 2, 3];
        let array2 = vec![4, 5, 6];
        let result = intersection(&array1, &array2);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_intersection_with_empty_arrays() {
        let array1: Vec<i32> = vec![];
        let array2: Vec<i32> = vec![];
        let result = intersection(&array1, &array2);
        assert_eq!(result, vec![]);
    }
}
