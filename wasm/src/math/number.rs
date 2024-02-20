use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

pub trait Number:
    Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul
    + MulAssign
    + Div
    + DivAssign
    + Rem
    + RemAssign
    + PartialEq
    + PartialOrd
    + Copy
    + Debug
{
}

impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}

impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}

impl Number for f32 {}
impl Number for f64 {}
