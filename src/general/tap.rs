/// Runs the provided closure for side effects using a reference to the value,
/// then returns the original value unchanged, allowing method chaining without disruption.
///
/// # Examples
/// ```rust
/// use lo_::Tap;
/// let result = vec![1, 2, 3]
///     .tap(|v| println!("Current vector: {:?}", v))
///     .iter()
///     .sum::<i32>();
/// assert_eq!(result, 6);
///
pub fn tap<T, F>(val: T, f: F) -> T
where
    F: FnOnce(&T),
{
    f(&val);
    val
}

/// Trait for ergonomic `.tap()` method on any type
pub trait Tap: Sized {
    /// Run side effect on self by reference, return self unchanged
    fn tap<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self);
}

impl<T> Tap for T {
    fn tap<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self),
    {
        f(&self);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_tap_works() {
        let mut seen = 0;
        let val = tap(10, |v| seen = *v);
        assert_eq!(val, 10);
        assert_eq!(seen, 10);
    }

    #[test]
    fn method_tap_works() {
        let mut seen = 0;
        let val = 20.tap(|v| seen = *v);
        assert_eq!(val, 20);
        assert_eq!(seen, 20);
    }
}
