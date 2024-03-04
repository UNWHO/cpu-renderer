pub mod one;
pub mod zero;

use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

use self::{one::One, zero::Zero};

pub trait Number<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + AddAssign<Rhs>
    + Sub<Rhs, Output = Output>
    + SubAssign<Rhs>
    + Mul<Rhs, Output = Output>
    + MulAssign<Rhs>
    + Div<Rhs, Output = Output>
    + DivAssign<Rhs>
    + Rem<Rhs, Output = Output>
    + RemAssign<Rhs>
    + PartialEq
    + PartialOrd
    + Copy
    + Debug
    + Zero
    + One
{
}

impl<T, Rhs, Output> Number<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + AddAssign<Rhs>
        + Sub<Rhs, Output = Output>
        + SubAssign<Rhs>
        + Mul<Rhs, Output = Output>
        + MulAssign<Rhs>
        + Div<Rhs, Output = Output>
        + DivAssign<Rhs>
        + Rem<Rhs, Output = Output>
        + RemAssign<Rhs>
        + PartialEq
        + PartialOrd
        + Copy
        + Debug
        + Zero
        + One
{
}
