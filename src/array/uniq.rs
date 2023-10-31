#![allow(dead_code)]

/// Creates a duplicate-free version of an array, in which only the first occurrence of each element is kept. The order of result values is determined by the order they occur in the array.
///
/// # Example
/// ```rust
/// use lo_::array::*;
/// let input = vec![2, 1, 2];
/// let output = uniq(input);
/// assert_eq!(output, vec![2, 1]);
///
/// ```
pub fn uniq<T: PartialEq + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();
    for item in vec {
        if !result.contains(&item) {
            result.push(item);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniq() {
        let input = vec![2, 1, 2];
        let output = uniq(input);
        assert_eq!(output, vec![2, 1]);
    }

    #[test]
    fn test_empty_input() {
        let input: Vec<i32> = vec![];
        let output = uniq(input);
        assert_eq!(output, vec![]);
    }
}
