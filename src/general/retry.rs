use std::{thread::sleep, time::Duration};

/// Retry a fallible operation up to `times`, sleeping with exponential backoff between attempts.
///
/// The delay doubles after each failed attempt, starting from `base_delay`.
/// If all attempts fail, returns the last error.
///
/// # Arguments
///
/// * `times` - Maximum number of attempts to try the operation.
/// * `base_delay` - Initial delay duration before retrying, doubled on each failure.
/// * `op` - The fallible operation to retry, a closure returning `Result<T, E>`.
///
/// # Example
/// ```rust
/// use std::time::Duration;
/// use lo_::retry;
///
/// let mut count = 0;
/// let result = retry(3, Duration::from_millis(10), || {
///     count += 1;
///     if count < 3 {
///         Err("fail")
///     } else {
///         Ok("success")
///     }
/// });
/// assert_eq!(result, Ok("success"));
///
/// ```
pub fn retry<T, E, F>(times: usize, delay: Duration, mut op: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut attempts = 0;
    let mut current_delay = delay;

    loop {
        match op() {
            Ok(res) => return Ok(res),
            Err(e) if attempts + 1 >= times => return Err(e),
            Err(_) => {
                attempts += 1;
                sleep(current_delay);
                current_delay *= 2; // Exponential backoff: double the delay
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_retries_and_succeeds() {
        let mut tries = 0;
        let res = retry(3, Duration::from_millis(1), || {
            tries += 1;
            if tries == 3 {
                Ok("done")
            } else {
                Err("fail")
            }
        });

        assert_eq!(res, Ok("done"));
    }

    #[test]
    fn it_fails_after_retries() {
        let res: Result<(), &str> = retry(2, Duration::from_millis(1), || Err("nope"));
        assert_eq!(res, Err("nope"));
    }
}

/// Extension trait to add `.retry(...)` to a `Result`
pub trait Retry<T, E> {
    fn retry<F>(self, times: usize, delay: Duration, op: F) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>;
}

impl<T, E> Retry<T, E> for Result<T, E> {
    fn retry<F>(self, times: usize, delay: Duration, op: F) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
    {
        retry(times, delay, op)
    }
}

#[test]
fn retry_should_eventually_succeed() {
    use super::*;
    // use crate::general::Retry;
    use std::time::Duration;

    let mut counter = 0;
    let result = Ok(()).retry(5, Duration::from_millis(1), || {
        counter += 1;
        if counter < 3 {
            Err("fail")
        } else {
            Ok(())
        }
    });

    assert_eq!(result, Ok(()));
}

#[test]
fn test_retry_ext_failure() {
    let mut counter = 0;

    let result = Err::<(), &str>("initial error").retry(3, Duration::from_millis(1), || {
        counter += 1;
        Err("still failing")
    });

    assert_eq!(result, Err("still failing"));
    assert_eq!(counter, 3); // Ensure it retried 3 times
}
