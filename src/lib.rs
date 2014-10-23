//! Assertions for testing.

#![feature(macro_rules)]

#[macro_export]
macro_rules! assert_close(
    ($left:expr, $right:expr) => ({
        use std::num::abs;
        for (&x, &y) in $left.iter().zip($right.iter()) {
            assert!(abs(x - y) < 1e-8);
        }
    });
)

#[macro_export]
macro_rules! assert_abs_close(
    ($left:expr, $right:expr) => ({
        use std::num::abs;
        for (&x, &y) in $left.iter().zip($right.iter()) {
            assert!(abs(abs(x) - abs(y)) < 1e-8);
        }
    });
)
