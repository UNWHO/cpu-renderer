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
    pub fn from(arr: &[[T; R]; C]) -> Self {
        Self { 0: arr.clone() }
    }

    pub fn zero() -> Self {
        Self {
            0: [[T::zero(); R]; C],
        }
    }

    // #[inline]
    // pub fn row_size(&self) -> usize {
    //     R
    // }

    #[inline]
    pub fn col_size(&self) -> usize {
        C
    }

    // pub fn add(&mut self, rhs: &Self) -> &mut Self {
    //     for i in 0..R {
    //         for j in 0..C {
    //             self[j][i] += rhs[j][i];
    //         }
    //     }

    //     self
    // }

    // pub fn sub(&mut self, rhs: &Self) -> &mut Self {
    //     for i in 0..R {
    //         for j in 0..C {
    //             self[j][i] -= rhs[j][i];
    //         }
    //     }

    //     self
    // }

    // pub fn mul_scala(&mut self, rhs: T) -> &mut Self {
    //     for i in 0..R {
    //         for j in 0..C {
    //             self[j][i] *= rhs;
    //         }
    //     }

    //     self
    // }

    // pub fn mul<const R2: usize>(&self, rhs: &Matrix<T, R2, R>) -> Matrix<T, C, R2> {
    //     let mut result = Matrix::<T, C, R2>::zero();

    //     for i in 0..R2 {
    //         for j in 0..C {
    //             for k in 0..R {
    //                 result[j][i] += self[j][k] * rhs[k][i];
    //             }
    //         }
    //     }

    //     result
    // }
}

impl<T: Number, const S: usize> Matrix<T, S, S> {
    // pub fn identity() -> Self {
    //     let mut zero = Self::zero();

    //     for i in 0..S {
    //         zero[i][i] = T::one();
    //     }

    //     zero
    // }
}

impl<T: Number, const R: usize, const C: usize> Clone for Matrix<T, R, C> {
    fn clone(&self) -> Self {
        Self { 0: self.0.clone() }
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
