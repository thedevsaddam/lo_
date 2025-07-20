/// Zips two iterators together, applying a function to each pair of elements.
///
/// Iteration stops when either iterator is exhausted.
///
/// # Example
/// ```rust
/// use lo_::zip_with;
///
/// let a = vec![1, 2, 3];
/// let b = vec![4, 5, 6];
///
/// let result: Vec<_> = zip_with(a.into_iter(), b.into_iter(), |x, y| x + y).collect();
/// assert_eq!(result, vec![5, 7, 9]);
///
/// ```
pub fn zip_with<A, B, C, I, J, F>(a: I, b: J, mut f: F) -> impl Iterator<Item = C>
where
    I: Iterator<Item = A>,
    J: Iterator<Item = B>,
    F: FnMut(A, B) -> C,
{
    a.zip(b).map(move |(x, y)| f(x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip_with_basic_sum() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];
        let result: Vec<_> = zip_with(a.into_iter(), b.into_iter(), |x, y| x + y).collect();
        assert_eq!(result, vec![5, 7, 9]);
    }

    #[test]
    fn test_zip_with_product() {
        let a = vec![2, 3];
        let b = vec![5, 10];
        let result: Vec<_> = zip_with(a.into_iter(), b.into_iter(), |x, y| x * y).collect();
        assert_eq!(result, vec![10, 30]);
    }

    #[test]
    fn test_zip_with_shorter_iterator() {
        let a = vec![1, 2];
        let b = vec![10, 20, 30]; // b is longer
        let result: Vec<_> = zip_with(a.into_iter(), b.into_iter(), |x, y| x + y).collect();
        assert_eq!(result, vec![11, 22]); // stops at the shorter one
    }

    #[test]
    fn test_zip_with_empty() {
        let a: Vec<i32> = vec![];
        let b: Vec<i32> = vec![1, 2];
        let result: Vec<_> = zip_with(a.into_iter(), b.into_iter(), |x, y| x + y).collect();
        assert!(result.is_empty());
    }
}
