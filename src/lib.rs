//! Assertions for testing.

#![cfg_attr(test, feature(core))]

/// Assert that the distance between the absolute values of the corresponding
/// elements of two vectors is smaller than `delta!()`.
#[macro_export]
macro_rules! assert_abs_close(
    ($x:expr, $y:expr) => ({
        use std::num::Float;
        let delta = delta!();
        for (&x, &y) in $x.iter().zip($y.iter()) {
            if x.is_finite() && y.is_finite() {
                assert!((x.abs() - y.abs()).abs() < delta, "|{}| !~ |{}|", x, y);
            } else {
                assert_eq!(x, y);
            }
        }
    });
);

/// Assert that the distance between the corresponding elements of two vectors
/// is smaller than `delta!()`.
#[macro_export]
macro_rules! assert_close(
    ($x:expr, $y:expr) => ({
        use std::num::Float;
        let delta = delta!();
        for (&x, &y) in $x.iter().zip($y.iter()) {
            if x.is_finite() && y.is_finite() {
                assert!((x - y).abs() < delta, "{} !~ {}", x, y);
            } else {
                assert_eq!(x, y);
            }
        }
    });
);

/// Assert that two vectors are equal.
#[macro_export]
macro_rules! assert_equal(
    ($x:expr, $y:expr) => ({
        for (&x, &y) in $x.iter().zip($y.iter()) {
            assert_eq!(x, y);
        }
    });
);

/// Assert that the result is unsuccessful.
#[macro_export]
macro_rules! assert_err(
    ($result:expr) => {
        match $result {
            Ok(..) => assert!(false, "got Ok(..), expected Err(..)"),
            Err(..) => {},
        }
    };
);

/// Assert that the result is successful.
#[macro_export]
macro_rules! assert_ok(
    ($result:expr) => {
        match $result {
            Ok(..) => {},
            Err(..) => assert!(false, "got Err(..), expected Ok(..)"),
        }
    };
);

/// Return the square root of the machine epsilon.
#[macro_export]
macro_rules! delta(
    () => (::std::f64::EPSILON.sqrt());
);

#[cfg(test)]
mod test {
    struct Success;
    struct Failure;

    #[test]
    fn assert_abs_close() {
        assert_abs_close!([1f64, 2.0, 3.0], [-1f64, 2.0 + 1e-10, -3.0 - 1e-10]);
    }

    #[test]
    fn assert_close() {
        assert_close!([1f64, 2.0, 3.0], [1f64, 2.0 + 1e-10, 3.0 - 1e-10]);
    }

    #[test]
    fn assert_equal() {
        assert_equal!([1f64, 2.0, 3.0], [1f64, 2.0, 3.0]);
    }

    #[test]
    fn assert_err() {
        fn work() -> Result<Success, Failure> { Err(Failure) }
        assert_err!(work());
    }

    #[test]
    fn assert_ok() {
        fn work() -> Result<Success, Failure> { Ok(Success) }
        assert_ok!(work());
    }
}
