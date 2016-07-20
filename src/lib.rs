//! Assertions for testing.

mod traits;
pub use traits::{Float, Floats};

/// Assert that the absolute difference between two quantities is small.
///
/// In case of vectors, the assertion is elementwise.
pub fn close<F, F1, F2>(x: F1, y: F2, delta: F) where F: Float, F1: Floats<F>, F2: Floats<F> {
    let (x, y) = (x.floats(), y.floats());
    assert_eq!(x.len(), y.len());
    for (&x, &y) in x.iter().zip(y) {
        if x.is_finite() && y.is_finite() {
            assert!((x - y).abs() <= delta, "{:?} !~ {:?}", x, y);
        } else {
            assert!(x == y, "{:?} !~ {:?}", x, y);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn close() {
        ::close(1.0, 1.0 + 1e-10, 2e-10);
        ::close(&[1.0], &[1.0 + 1e-10], 2e-10);
        ::close(vec![1.0], &[1.0 + 1e-10], 2e-10);
        ::close(&vec![1.0], &[1.0 + 1e-10], 2e-10);
    }

    #[should_panic]
    #[test]
    fn close_empty() {
        ::close(vec![], vec![1.0], 1.0);
    }

    #[test]
    fn close_zero() {
        ::close(vec![1.0], vec![1.0], 0.0);
    }
}
