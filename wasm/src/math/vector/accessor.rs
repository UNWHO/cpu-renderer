use crate::math::number::Number;

use super::{Vector2, Vector3, Vector4};

impl<T: Number> Vector2<T> {
    #[inline]
    pub fn x(&self) -> T {
        self[0][0]
    }

    #[inline]
    pub fn y(&self) -> T {
        self[0][1]
    }
}

impl<T: Number> Vector3<T> {
    #[inline]
    pub fn x(&self) -> T {
        self[0][0]
    }

    #[inline]
    pub fn y(&self) -> T {
        self[0][1]
    }

    #[inline]
    pub fn z(&self) -> T {
        self[0][2]
    }
}

impl<T: Number> Vector4<T> {
    #[inline]
    pub fn x(&self) -> T {
        self[0][0]
    }

    #[inline]
    pub fn y(&self) -> T {
        self[0][1]
    }

    #[inline]
    pub fn z(&self) -> T {
        self[0][2]
    }

    #[inline]
    pub fn w(&self) -> T {
        self[0][3]
    }
}
