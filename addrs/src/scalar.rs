use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_traits::{One, Zero};

pub trait Scalar:
    Clone
    + Zero
    + One
    + From<f64>
    + Neg<Output = Self>
    + Add<Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + Sub<Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> SubAssign<&'a Self>
    + Mul<Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> MulAssign<&'a Self>
    + Div<Output = Self>
    + for<'a> Div<&'a Self, Output = Self>
    + for<'a> DivAssign<&'a Self>
{
}

impl<T> Scalar for T where
    T: Clone
        + Zero
        + One
        + From<f64>
        + Neg<Output = Self>
        + Add<Output = Self>
        + for<'a> Add<&'a Self, Output = Self>
        + for<'a> AddAssign<&'a Self>
        + Sub<Output = Self>
        + for<'a> Sub<&'a Self, Output = Self>
        + for<'a> SubAssign<&'a Self>
        + Mul<Output = Self>
        + for<'a> Mul<&'a Self, Output = Self>
        + for<'a> MulAssign<&'a Self>
        + Div<Output = Self>
        + for<'a> Div<&'a Self, Output = Self>
        + for<'a> DivAssign<&'a Self>
{
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}
impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
}
impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
}

pub trait Exp {
    fn exp(self) -> Self;
}
impl Exp for f64 {
    fn exp(self) -> Self {
        f64::exp(self)
    }
}
impl Exp for f32 {
    fn exp(self) -> Self {
        f32::exp(self)
    }
}

pub trait Log {
    fn log(self) -> Self;
}
impl Log for f64 {
    fn log(self) -> Self {
        f64::ln(self)
    }
}
impl Log for f32 {
    fn log(self) -> Self {
        f32::ln(self)
    }
}
