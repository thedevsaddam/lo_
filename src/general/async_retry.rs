#[cfg(feature = "async_retry")]
use std::time::Duration;
#[cfg(feature = "async_retry")]
use tokio::time::sleep;

/// Retry an async operation with exponential backoff.
///
/// Requires the `async_retry` feature.
///
/// # Examples
/// ```rust
/// # use std::time::Duration;
/// # async fn do_work() -> Result<u8, &'static str> { Err("fail") }
/// # #[tokio::main]
/// # async fn main() {
/// use lo_::async_retry;
///
/// let result = async_retry(3, Duration::from_millis(100), || do_work()).await;
/// # }
/// ```
///
/// # Features
/// This function is only available when the `async_retry` feature is enabled.
#[cfg(feature = "async_retry")]
pub async fn async_retry<T, E, Fut, F>(times: usize, delay: Duration, mut op: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    let mut attempts = 0;
    let mut current_delay = delay;

    loop {
        match op().await {
            Ok(res) => return Ok(res),
            Err(e) if attempts + 1 >= times => return Err(e),
            Err(_) => {
                attempts += 1;
                sleep(current_delay).await;
                current_delay *= 2; // Exponential backoff
            }
        }
    }
}
