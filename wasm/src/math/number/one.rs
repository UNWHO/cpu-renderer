pub trait One {
    fn one() -> Self;
}

macro_rules! impl_one {
    ($t: ty, $v: expr) => {
        impl One for $t {
            #[inline]
            fn one() -> $t {
                $v
            }
        }
    };
}

impl_one!(usize, 1usize);
impl_one!(u8, 1u8);
impl_one!(u16, 1u16);
impl_one!(u32, 1u32);
impl_one!(u64, 1u64);

impl_one!(isize, 1isize);
impl_one!(i8, 1i8);
impl_one!(i16, 1i16);
impl_one!(i32, 1i32);
impl_one!(i64, 1i64);
impl_one!(i128, 1i128);

impl_one!(f32, 1.0f32);
impl_one!(f64, 1.0f64);
