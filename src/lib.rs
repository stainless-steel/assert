//! Assertions for testing.

extern crate num;

use std::fmt::Debug;
use num::traits::Float;

/// Assert that the distance between the absolute values of the corresponding
/// elements of two vectors is smaller than a given value.
pub fn absolute_within<T: Debug + Float>(x: &[T], y: &[T], delta: T) {
    for (&x, &y) in x.iter().zip(y.iter()) {
        if x.is_finite() && y.is_finite() {
            assert!((x.abs() - y.abs()).abs() < delta, "|{:?}| !~ |{:?}|", x, y);
        } else {
            assert!(x == y, "|{:?}| !~ |{:?}|", x, y);
        }
    }
}

/// Assert that two vectors are equal.
pub fn equal<T: Debug + PartialEq>(x: T, y: T) {
    assert_eq!(x, y);
}

/// Assert that the result is unsuccessful.
pub fn error<S, E>(result: Result<S, E>) {
    match result {
        Ok(..) => assert!(false, "got an OK, expected an error"),
        Err(..) => {},
    }
}

/// Assert that the result is successful.
pub fn success<S, E>(result: Result<S, E>) {
    match result {
        Ok(..) => {},
        Err(..) => assert!(false, "got an error, expected an OK"),
    }
}

/// Assert that the distance between the corresponding elements of two vectors
/// is smaller than a given value.
pub fn within<T: Debug + Float>(x: &[T], y: &[T], delta: T) {
    for (&x, &y) in x.iter().zip(y.iter()) {
        if x.is_finite() && y.is_finite() {
            assert!((x - y).abs() < delta, "{:?} !~ {:?}", x, y);
        } else {
            assert!(x == y, "{:?} !~ {:?}", x, y);
        }
    }
}

#[cfg(test)]
mod test {
    struct Success;
    struct Failure;

    #[test]
    fn absolute_within() {
        ::absolute_within(&[1.0, 2.0, 3.0], &[-1.0, 2.0 + 1e-10, -3.0 - 1e-10], 2e-10);
    }

    #[test]
    fn equal() {
        ::equal(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn error() {
        fn work() -> Result<Success, Failure> { Err(Failure) }
        ::error(work());
    }

    #[test]
    fn success() {
        fn work() -> Result<Success, Failure> { Ok(Success) }
        ::success(work());
    }

    #[test]
    fn within() {
        ::within(&[1.0, 2.0, 3.0], &[1.0, 2.0 + 1e-10, 3.0 - 1e-10], 2e-10);
    }
}
