///
///
/// # Arguments
///
/// * `num`:
/// * `x`:
///
/// returns: bool
///
/// # Examples
///
/// ```
/// assert_eq!(is_even(2), true);
/// ```
pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert_eq!(is_even(2), true);
    }

    #[test]
    fn is_false_when_odd(){
        assert_eq!(is_even(3), false);
    }
}
