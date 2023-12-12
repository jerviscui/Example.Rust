pub mod math_even;

/// Generally, the first line is a brief summary describing the function.
///
/// The next lines present detailed documentation.
///
/// ```
/// let result = scrape_url::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// This function divides two numbers.
///
/// # Example #1: 10 / 2 == 5
///
/// ```
/// assert_eq!(scrape_url::div(10, 2), 5);
/// ```
///
/// # Example #2: 6 / 3 = 2
///
/// ```
/// assert_eq!(scrape_url::div(6, 3), 2);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// scrape_url::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}

/// This function subtracts two numbers.
///
/// # Example #1: 9 - 2 == 7
///
/// ```
/// assert_eq!(scrape_url::sub(9, 2), 7);
/// ```
///
/// # Example #2: 6 - 9 == -3
///
/// ```
/// assert_eq!(scrape_url::sub(6, 9), -3);
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}
