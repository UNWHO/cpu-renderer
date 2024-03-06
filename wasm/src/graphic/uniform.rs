use crate::math::matrix::Matrix;
use wasm_bindgen::prelude::*;

static mut matrices: Vec<Matrix<f64, 4, 4>> = Vec::new();

#[wasm_bindgen]
pub enum UniformMatrix {
    MvpMatrix,
}

#[wasm_bindgen(start)]
fn resize_uniform_vectors() {
    unsafe { matrices.resize(1, Matrix::zero()) }
}

#[wasm_bindgen]
pub fn set_uniform_matrix(matrix_type: UniformMatrix, matrix: &[f64]) {
    unsafe {
        matrices[matrix_type as usize] = Matrix::from(&[
            [matrix[0], matrix[1], matrix[2], matrix[3]],
            [matrix[4], matrix[5], matrix[6], matrix[7]],
            [matrix[8], matrix[9], matrix[10], matrix[11]],
            [matrix[12], matrix[13], matrix[14], matrix[15]],
        ]);
    }
}

#[inline]
pub fn get_uniform_matrix(matrix_type: UniformMatrix) -> &'static Matrix<f64, 4, 4> {
    unsafe { &matrices[matrix_type as usize] }
}
