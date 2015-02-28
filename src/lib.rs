//! Assertions for testing.

/// Assert that the distance between the absolute values of the corresponding
/// elements of two vectors is smaller than the square root of the machine
/// epsilon.
#[macro_export]
#[deprecated(reason = "use assert_abs_within!")]
macro_rules! assert_abs_close(
    ($x:expr, $y:expr) => (
        assert_abs_within!($x, $y, ::std::f64::EPSILON.sqrt())
    );
);

/// Assert that the distance between the absolute values of the corresponding
/// elements of two vectors is smaller than a given value.
#[macro_export]
macro_rules! assert_abs_within(
    ($x:expr, $y:expr, $delta:expr) => ({
        use std::num::Float;
        let delta = $delta;
        for (&x, &y) in $x.iter().zip($y.iter()) {
            if x.is_finite() && y.is_finite() {
                assert!((x.abs() - y.abs()).abs() < delta, "|{}| !~ |{}|", x, y);
            } else {
                assert!(x == y, "|{}| !~ |{}|", x, y);
            }
        }
    });
);

/// Assert that the distance between the corresponding elements of two vectors
/// is smaller than the square root of the machine epsilon.
#[macro_export]
#[deprecated(reason = "use assert_within!")]
macro_rules! assert_close(
    ($x:expr, $y:expr) => (
        assert_within!($x, $y, ::std::f64::EPSILON.sqrt())
    );
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
            Ok(..) => assert!(false, "got an OK, expected an error"),
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
            Err(..) => assert!(false, "got an error, expected an OK"),
        }
    };
);

/// Assert that the distance between the corresponding elements of two vectors
/// is smaller than a given value.
#[macro_export]
macro_rules! assert_within(
    ($x:expr, $y:expr, $delta:expr) => ({
        use std::num::Float;
        let delta = $delta;
        for (&x, &y) in $x.iter().zip($y.iter()) {
            if x.is_finite() && y.is_finite() {
                assert!((x - y).abs() < delta, "{} !~ {}", x, y);
            } else {
                assert!(x == y, "{} !~ {}", x, y);
            }
        }
    });
);

#[cfg(test)]
mod test {
    struct Success;
    struct Failure;

    #[test]
    fn assert_abs_close() {
        assert_abs_close!([1.0, 2.0, 3.0], [-1.0, 2.0 + 1e-10, -3.0 - 1e-10]);
    }

    #[test]
    fn assert_abs_within() {
        assert_abs_within!([1.0, 2.0, 3.0], [-1.0, 2.0 + 1e-10, -3.0 - 1e-10], 2e-10);
    }

    #[test]
    fn assert_close() {
        assert_close!([1.0, 2.0, 3.0], [1.0, 2.0 + 1e-10, 3.0 - 1e-10]);
    }

    #[test]
    fn assert_equal() {
        assert_equal!([1.0, 2.0, 3.0], [1.0, 2.0, 3.0]);
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

    #[test]
    fn assert_within() {
        assert_within!([1.0, 2.0, 3.0], [1.0, 2.0 + 1e-10, 3.0 - 1e-10], 2e-10);
    }
}
