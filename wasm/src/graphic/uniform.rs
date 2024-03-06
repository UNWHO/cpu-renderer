use crate::math::matrix::Matrix;
use wasm_bindgen::prelude::*;

static mut MATRICES: Vec<Matrix<f64, 4, 4>> = Vec::new();

pub fn init_uniform_matrix_map() {
    unsafe { MATRICES.resize(1, Matrix::zero()) }
}

#[wasm_bindgen]
pub enum UniformMatrix {
    MvpMatrix,
}

#[wasm_bindgen]
pub fn set_uniform_matrix(matrix_type: UniformMatrix, matrix: &[f64]) {
    unsafe {
        MATRICES[matrix_type as usize] = Matrix::from(&[
            [matrix[0], matrix[1], matrix[2], matrix[3]],
            [matrix[4], matrix[5], matrix[6], matrix[7]],
            [matrix[8], matrix[9], matrix[10], matrix[11]],
            [matrix[12], matrix[13], matrix[14], matrix[15]],
        ]);
    }
}

#[inline]
pub fn get_uniform_matrix(matrix_type: UniformMatrix) -> &'static Matrix<f64, 4, 4> {
    unsafe { &MATRICES[matrix_type as usize] }
}
