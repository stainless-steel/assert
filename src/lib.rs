//! Assertions for testing.

#![feature(macro_rules)]

#[macro_export]
macro_rules! epsilon(
    () => (::std::f64::EPSILON.sqrt());
)

#[macro_export]
macro_rules! assert_equal(
    ($left:expr , $right:expr) => ({
        for (&x, &y) in $left.iter().zip($right.iter()) {
            assert_eq!(x, y);
        }
    });
)

#[macro_export]
macro_rules! assert_close(
    ($left:expr, $right:expr) => ({
        use std::num::abs;
        let epsilon = epsilon!();
        for (&x, &y) in $left.iter().zip($right.iter()) {
            if (x as f64).is_finite() && (y as f64).is_finite() {
                assert!(abs(x - y) < epsilon, "{} !~ {}", x, y);
            } else {
                assert_eq!(x, y);
            }
        }
    });
)

#[macro_export]
macro_rules! assert_abs_close(
    ($left:expr, $right:expr) => ({
        use std::num::abs;
        let epsilon = epsilon!();
        for (&x, &y) in $left.iter().zip($right.iter()) {
            if (x as f64).is_finite() && (y as f64).is_finite() {
                assert!(abs(abs(x) - abs(y)) < epsilon, "|{}| !~ |{}|", x, y);
            } else {
                assert_eq!(x, y);
            }
        }
    });
)
