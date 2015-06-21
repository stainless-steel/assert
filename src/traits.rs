use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

/// A floating-point number.
pub trait Float: Add<Output=Self> + Div<Output=Self> + Mul<Output=Self> + Sub<Output=Self> +
                 Copy + Debug + PartialEq + PartialOrd
{
    fn abs(&self) -> Self;
    fn is_finite(&self) -> bool;
}

/// A collection of floating-point numbers.
pub trait Floats<'l> {
    type Item: Float;
    type Iterator: Iterator<Item=&'l Self::Item>;

    fn iterate(self) -> Self::Iterator;
}

macro_rules! implement(
    ($kind:ty) => (
        impl Float for $kind {
            #[inline(always)] fn abs(&self) -> Self { <$kind>::abs(*self) }
            #[inline(always)] fn is_finite(&self) -> bool { <$kind>::is_finite(*self) }
        }
    );
);

impl<'l, I, F> Floats<'l> for I where I: IntoIterator<Item=&'l F>, F: Float {
    type Item = F;
    type Iterator = I::IntoIter;

    #[inline(always)] fn iterate(self) -> Self::Iterator { self.into_iter() }
}

implement!(f32);
implement!(f64);
