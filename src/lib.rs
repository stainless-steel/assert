//! Assertions for testing.

mod traits;
pub use traits::{Float, Floats};

/// Assert that the absolute difference between two quantities is small.
///
/// In case of vectors, the assertion is elementwise.
pub fn close<F, F1, F2>(x: F1, y: F2, delta: F)
where
    F: Float,
    F1: Floats<F>,
    F2: Floats<F>,
{
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

/// Assert that two sequences have the same length, and equal elements. This
/// macro has similar iteration mechanics to [zip], except that sequences of
/// different lengths will not equate to each other.
#[macro_export]
macro_rules! assert_seq_eq (
    ($left:expr, $right:expr) => (
        _assert_seq_eq!($left, $right, |&a| a, |&a| a);
    );

    ($left:expr, $right:expr, $left_deref:expr, $right_deref:expr) => (
        _assert_seq_eq!($left, $right, $left_deref, $right_deref);
    );
);

#[macro_export]
#[doc(hidden)]
macro_rules! _assert_seq_eq (
    ($left:expr, $right:expr, $left_deref:expr, $right_deref:expr) => (
        let mut iter_left = $left.into_iter();
        let mut iter_right = $right.into_iter();
        let mut index = 0usize;

        loop {
            let (item_left, item_right) = (iter_left.next(), iter_right.next());

            // Ensure that both iterators are still yielding items, or that
            // they have both stopped
            assert_eq!(item_left.is_none(), item_right.is_none(),
                "Sequences differ at index {}; {} sequence stops early",
                index, if item_left.is_none() { "left" } else { "right" });

            if item_left.is_none() {
                break;
            }

            let item_left = $left_deref(item_left.as_ref().unwrap());
            let item_right = $right_deref(item_right.as_ref().unwrap());
            assert_eq!(item_left, item_right,
                "Sequences differ at index {}. Failed on: `{:?} == {:?}`",
                index, item_left, item_right);
            index += 1;
        }
    );
);


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

    #[test]
    fn sequence_equal() {
        let left = &[1, 2, 3];
        let right = &[11, 12, 13];
        assert_seq_eq!(left.iter().cloned(), right.iter().map(|&x| x - 10));
    }

    #[test]
    #[should_panic]
    fn sequence_equal_not_equal() {
        let left = &[1, 2, 3];
        let right = &[4, 5, 6];
        assert_seq_eq!(left.iter(), right.iter());
    }

    #[test]
    #[should_panic]
    fn sequence_equal_left_short() {
        let shorter = &[1, 2, 3];
        let longer = &[1, 2, 3, 4];
        assert_seq_eq!(shorter.iter(), longer.iter());
    }

    #[test]
    #[should_panic]
    fn sequence_equal_right_short() {
        let shorter = &[1, 2, 3];
        let longer = &[1, 2, 3, 4];
        assert_seq_eq!(longer.iter(), shorter.iter());
    }

    #[test]
    fn sequence_equal_custom_deref() {
        let left = &[1, 2, 3]; // iterates as references
        let right = &[11, 12, 13]; // iterates as values
        assert_seq_eq!(left.iter(), right.iter().map(|&x| x - 10), |&x| x, |x| x);
    }
}
