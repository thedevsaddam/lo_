/// Ternary Operator with Functions
///
/// Executes one of two functions based on a condition and returns the result.
///
/// # Arguments
///
/// * `condition` - A boolean condition determining which function to execute.
/// * `if_func` - A function that will be executed if the condition is true.
/// * `else_func` - A function that will be executed if the condition is false.
///
/// # Returns
///
/// The result of the executed function.
///
/// # Examples
///
/// ```
/// use lo_::ternary_f;
/// fn add() -> i32 {
///     5 + 3
/// }
///
/// fn subtract() -> i32 {
///     5 - 3
/// }
///
/// let result_add = ternary_f(true, add, subtract);
/// assert_eq!(result_add, 8);
///
/// let result_sub = ternary_f(false, add, subtract);
/// assert_eq!(result_sub, 2);
/// ```
pub fn ternary_f<T, F1, F2>(condition: bool, if_func: F1, else_func: F2) -> T
where
    F1: FnOnce() -> T,
    F2: FnOnce() -> T,
{
    if condition {
        if_func()
    } else {
        else_func()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_condition() {
        let result = ternary_f(true, || 5, || 10);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_false_condition() {
        let result = ternary_f(false, || 5, || 10);
        assert_eq!(result, 10);
    }
}
