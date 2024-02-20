use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use super::number::Number;

pub struct Matrix<T: Number, const R: usize, const C: usize>([[T; R]; C]);

impl<T: Number, const R: usize, const C: usize> Index<usize> for Matrix<T, R, C> {
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
    type Output = [T; R];
}

impl<T: Number, const R: usize, const C: usize> IndexMut<usize> for Matrix<T, R, C> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Number, const R: usize, const C: usize> Matrix<T, R, C> {
    pub const fn const_from(arr: [[T; R]; C]) -> Self {
        Self { 0: arr }
    }

    pub fn from(arr: &[[T; R]; C]) -> Self {
        Self { 0: arr.clone() }
    }

    #[inline]
    pub fn row_size(&self) -> usize {
        R
    }

    #[inline]
    pub fn col_size(&self) -> usize {
        C
    }

    pub fn add(&mut self, rhs: &Self) -> &mut Self {
        for i in 0..R {
            for j in 0..C {
                self[j][i] += rhs[j][i];
            }
        }

        self
    }

    pub fn sub(&mut self, rhs: &Self) -> &mut Self {
        for i in 0..R {
            for j in 0..C {
                self[j][i] -= rhs[j][i];
            }
        }

        self
    }

    pub fn mul(&mut self, rhs: &Matrix<T, C, R>) -> &mut Self {
        for i in 0..R {
            for j in 0..C {
                self[j][i] *= rhs[j][i];
            }
        }

        self
    }

    pub fn div(&mut self, rhs: T) -> &mut Self {
        for i in 0..R {
            for j in 0..C {
                self[j][i] /= rhs;
            }
        }

        self
    }
}

impl<T: Number, const R: usize, const C: usize> PartialEq for Matrix<T, R, C> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..R {
            for j in 0..C {
                if self[j][i] != other[j][i] {
                    return false;
                }
            }
        }

        true
    }
}

impl<T: Number, const R: usize, const C: usize> Debug for Matrix<T, R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_list();
        for i in 0..self.col_size() {
            debug.entries(self[i]);
        }
        debug.finish()
    }
}
