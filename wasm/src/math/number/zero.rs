pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ($t: ty, $v: expr) => {
        impl Zero for $t {
            #[inline]
            fn zero() -> $t {
                $v
            }
        }
    };
}

impl_zero!(usize, 0usize);
impl_zero!(u8, 0u8);
impl_zero!(u16, 0u16);
impl_zero!(u32, 0u32);
impl_zero!(u64, 0u64);

impl_zero!(isize, 0isize);
impl_zero!(i8, 0i8);
impl_zero!(i16, 0i16);
impl_zero!(i32, 0i32);
impl_zero!(i64, 0i64);
impl_zero!(i128, 0i128);

impl_zero!(f32, 0.0f32);
impl_zero!(f64, 0.0f64);
