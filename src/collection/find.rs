/// Iterates over elements of collection, returning the first element predicate returns truthy for.
///
/// # Example
/// ```rust
/// use lo_::find;
/// let numbers = vec![1, 2, 3, 4, 5];
/// let is_even = |x: &i32| *x % 2 == 0;
/// assert_eq!(find(&numbers, is_even), Some(2));
///
/// ```
pub fn find<T, F>(collection: &[T], predicate: F) -> Option<T>
where
    F: Fn(&T) -> bool,
    T: Clone,
{
    for item in collection {
        if predicate(item) {
            return Some(item.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_existing_element() {
        let numbers = vec![1, 2, 3, 4, 5];
        let is_even = |x: &i32| *x % 2 == 0;
        assert_eq!(find(&numbers, is_even), Some(2));
    }

    #[test]
    fn test_find_non_existing_element() {
        let numbers = vec![1, 3, 5, 7, 9];
        let is_even = |x: &i32| *x % 2 == 0;
        assert_eq!(find(&numbers, is_even), None);
    }
}
