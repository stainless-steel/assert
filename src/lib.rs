//! Assertions for testing.

#![feature(macro_rules)]

/// Assert that the distance between the absolute values of the corresponding
/// elements of two vectors is smaller than `delta!()`.
#[macro_export]
macro_rules! assert_abs_close(
    ($x:expr, $y:expr) => ({
        use std::num::abs;
        let delta = delta!();
        for (&x, &y) in $x.iter().zip($y.iter()) {
            if (x as f64).is_finite() && (y as f64).is_finite() {
                assert!(abs(abs(x) - abs(y)) < delta, "|{}| !~ |{}|", x, y);
            } else {
                assert_eq!(x, y);
            }
        }
    });
)

/// Assert that the distance between the corresponding elements of two vectors
/// is smaller than `delta!()`.
#[macro_export]
macro_rules! assert_close(
    ($x:expr, $y:expr) => ({
        use std::num::abs;
        let delta = delta!();
        for (&x, &y) in $x.iter().zip($y.iter()) {
            if (x as f64).is_finite() && (y as f64).is_finite() {
                assert!(abs(x - y) < delta, "{} !~ {}", x, y);
            } else {
                assert_eq!(x, y);
            }
        }
    });
)

/// Assert that two vectors are equal.
#[macro_export]
macro_rules! assert_equal(
    ($x:expr, $y:expr) => ({
        for (&x, &y) in $x.iter().zip($y.iter()) {
            assert_eq!(x, y);
        }
    });
)

/// Assert that the result is unsuccessful.
#[macro_export]
macro_rules! assert_err(
    ($result:expr) => {
        match $result {
            Ok(value) => assert!(false, "Ok({})", value),
            Err(error) => error,
        }
    };
)

/// Assert that the result is successful.
#[macro_export]
macro_rules! assert_ok(
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(error) => assert!(false, "Err({})", error),
        }
    };
)

/// Return the square root of the machine epsilon.
#[macro_export]
macro_rules! delta(
    () => (::std::f64::EPSILON.sqrt());
)

#[cfg(test)]
mod test {
    #[test]
    fn assert_equal() {
        assert_equal!([1f64, 2.0, 3.0], [1f64, 2.0, 3.0]);
    }

    #[test]
    fn assert_close() {
        assert_close!([1f64, 2.0, 3.0], [1f64, 2.0 + 1e-10, 3.0 - 1e-10]);
    }

    #[test]
    fn assert_abs_close() {
        assert_abs_close!([1f64, 2.0, 3.0], [-1f64, 2.0 + 1e-10, -3.0 - 1e-10]);
    }

    #[test]
    fn assert_err() {
        fn work() -> Result<(), ()> { Err(()) }
        assert_err!(work());
    }

    #[test]
    fn assert_ok() {
        fn work() -> Result<(), ()> { Ok(()) }
        assert_ok!(work());
    }
}
