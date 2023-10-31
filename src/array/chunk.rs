#![allow(dead_code)]

/// Creates an array of elements split into groups the length of size. If array can't be split evenly, the final chunk will be the remaining elements.
///
/// # Example
/// ```rust
/// use lo_::array::*;
/// let array = vec![1, 2, 3, 4, 5, 6, 7];
/// let size = 3;
/// let chunks = chunk(array, size);
/// assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
///
/// ```

pub fn chunk<T: Clone>(array: Vec<T>, size: usize) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut chunk = Vec::new();

    for item in array {
        if chunk.len() == size {
            result.push(chunk.clone());
            chunk.clear();
        }
        chunk.push(item);
    }

    if !chunk.is_empty() {
        result.push(chunk);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_with_size_greater_than_length() {
        let input = vec![1, 2, 3, 4, 5];
        let size = 6;
        let result = chunk(input, size);
        assert_eq!(result, vec![vec![1, 2, 3, 4, 5]]);
    }

    #[test]
    fn test_chunk_with_size_equal_to_length() {
        let input = vec![1, 2, 3, 4, 5];
        let size = 5;
        let result = chunk(input, size);
        assert_eq!(result, vec![vec![1, 2, 3, 4, 5]]);
    }

    #[test]
    fn test_chunk_with_size_less_than_length() {
        let input = vec![1, 2, 3, 4, 5];
        let size = 2;
        let result = chunk(input, size);
        assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5]]);
    }

    #[test]
    fn test_chunk_with_empty_array() {
        let input: Vec<i32> = vec![];
        let size = 3;
        let result = chunk(input, size);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }
}
