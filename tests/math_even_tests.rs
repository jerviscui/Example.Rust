use scrape_url::math_even;

#[test]
fn is_true() {
    assert_eq!(math_even::is_even(2), true);
}

#[test]
fn is_false() {
    assert_eq!(math_even::is_even(3), false);
}
