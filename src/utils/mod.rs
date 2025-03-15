/// Sums sum numbers together.
/// ```
/// # use rust_template::utils::sum;
/// assert_eq!(sum(1, 2), 3);
/// ```
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

/// Async summer!
/// ```
/// # use tokio_test;
/// # use rust_template::utils::sum_async;
/// # tokio_test::block_on(async {
/// assert_eq!(sum_async(1, 2).await, 3);
/// # });
/// ```
pub async fn sum_async(a: i32, b: i32) -> i32 {
    a + b
}