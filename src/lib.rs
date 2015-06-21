//! Assertions for testing.

mod traits;
pub use traits::{Float, Floats};

/// Assert that the distance between the corresponding elements of two vectors
/// is smaller than a given value.
pub fn close<'l, F, F1, F2>(x: F1, y: F2, delta: F)
    where F1: Floats<'l, Item=F>, F2: Floats<'l, Item=F>, F: 'l + Float
{
    for (&x, &y) in x.iterate().zip(y.iterate()) {
        if x.is_finite() && y.is_finite() {
            assert!((x - y).abs() < delta, "{:?} !~ {:?}", x, y);
        } else {
            assert!(x == y, "{:?} !~ {:?}", x, y);
        }
    }
}

/// Assert that the distance between the absolute values of the corresponding
/// elements of two vectors is smaller than a given value.
pub fn close_abs<'l, F, F1, F2>(x: F1, y: F2, delta: F)
    where F1: Floats<'l, Item=F>, F2: Floats<'l, Item=F>, F: 'l + Float
{
    for (&x, &y) in x.iterate().zip(y.iterate()) {
        if x.is_finite() && y.is_finite() {
            assert!((x.abs() - y.abs()).abs() < delta, "|{:?}| !~ |{:?}|", x, y);
        } else {
            assert!(x == y, "|{:?}| !~ |{:?}|", x, y);
        }
    }
}

/// Assert that the result is a failure.
pub fn error<S, E>(result: Result<S, E>) {
    match result {
        Ok(..) => assert!(false, "got an OK, expected an error"),
        Err(..) => {},
    }
}

/// Assert that the result is a success.
pub fn success<S, E>(result: Result<S, E>) {
    match result {
        Ok(..) => {},
        Err(..) => assert!(false, "got an error, expected an OK"),
    }
}

#[cfg(test)]
mod test {
    struct Success;
    struct Failure;

    #[test]
    fn close() {
        ::close(&[1.0, 2.0, 3.0], &[1.0, 2.0 + 1e-10, 3.0 - 1e-10], 2e-10);
    }

    #[test]
    fn close_abs() {
        ::close_abs(&[1.0, 2.0, 3.0], &[-1.0, 2.0 + 1e-10, -3.0 - 1e-10], 2e-10);
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
}
