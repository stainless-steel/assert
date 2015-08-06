use std::{fmt, ops, slice};

/// A floating-point number.
pub trait Float: Copy + fmt::Debug + PartialEq + PartialOrd + ops::Sub<Output=Self> {
    fn abs(&self) -> Self;
    fn is_finite(&self) -> bool;
}

/// One or more floating-point numbers.
pub trait Floats<T: Float> {
    fn floats(&self) -> &[T];
}

macro_rules! implement(
    ($kind:ty) => (
        impl Float for $kind {
            #[inline]
            fn abs(&self) -> Self {
                <$kind>::abs(*self)
            }

            #[inline]
            fn is_finite(&self) -> bool {
                <$kind>::is_finite(*self)
            }
        }

        impl Floats<$kind> for $kind {
            #[inline]
            fn floats(&self) -> &[$kind] {
                unsafe { slice::from_raw_parts(self, 1) }
            }
        }
    );
);

implement!(f32);
implement!(f64);

impl<T: Float> Floats<T> for Vec<T> {
    #[inline]
    fn floats(&self) -> &[T] {
        self
    }
}

impl<'l, T: Float> Floats<T> for &'l Vec<T> {
    #[inline]
    fn floats(&self) -> &[T] {
        self
    }
}

impl<'l, T: Float> Floats<T> for &'l [T] {
    #[inline]
    fn floats(&self) -> &[T] {
        self
    }
}

macro_rules! implement(
    () => ();
    ($number:expr, $($tail:tt)*) => (
        impl<'l, T: Float> Floats<T> for &'l [T; $number] {
            #[inline]
            fn floats(&self) -> &[T] {
                *self
            }
        }
        implement!($($tail)*);
    );
);

implement!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
           24, 25, 26, 27, 28, 29, 30, 31, 32,);
