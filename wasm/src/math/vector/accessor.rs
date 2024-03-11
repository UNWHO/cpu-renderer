use crate::math::number::Number;

use super::{Vector2, Vector3, Vector4};

pub trait Accessor2d<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;
}

pub trait Accessor3d<T>: Accessor2d<T> {
    fn z(&self) -> T;
}

pub trait Accessor4d<T>: Accessor3d<T> {
    fn w(&self) -> T;
}

impl<T: Number> Accessor2d<T> for Vector2<T> {
    #[inline]
    fn x(&self) -> T {
        self[0][0]
    }

    #[inline]
    fn y(&self) -> T {
        self[0][1]
    }
}

impl<T: Number> Accessor2d<T> for Vector3<T> {
    #[inline]
    fn x(&self) -> T {
        self[0][0]
    }

    #[inline]
    fn y(&self) -> T {
        self[0][1]
    }
}

impl<T: Number> Accessor2d<T> for Vector4<T> {
    #[inline]
    fn x(&self) -> T {
        self[0][0]
    }

    #[inline]
    fn y(&self) -> T {
        self[0][1]
    }
}

impl<T: Number> Accessor3d<T> for Vector3<T> {
    #[inline]
    fn z(&self) -> T {
        self[0][2]
    }
}

impl<T: Number> Accessor3d<T> for Vector4<T> {
    #[inline]
    fn z(&self) -> T {
        self[0][2]
    }
}

impl<T: Number> Accessor4d<T> for Vector4<T> {
    #[inline]
    fn w(&self) -> T {
        self[0][3]
    }
}
