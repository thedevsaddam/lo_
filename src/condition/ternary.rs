/// Ternary Operator
/// Returns one of two values based on a condition.
///
/// # Arguments
///
/// * `condition` - A boolean condition determining which value to return.
/// * `if_output` - The value to return if the condition is true.
/// * `else_output` - The value to return if the condition is false.
///
/// # Example
/// ```rust
/// use lo_::ternary;
/// assert_eq!(ternary(true, "a", "b"), "a");
/// assert_eq!(ternary(false, "a", "b"), "b");
///
/// ```

pub fn ternary<T>(condition: bool, if_output: T, else_output: T) -> T {
    if condition {
        if_output
    } else {
        else_output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary() {
        assert_eq!(ternary(true, 16, 18), 16);
        assert_eq!(ternary(false, 16, 18), 18);
    }
}
