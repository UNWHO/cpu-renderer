mod graphic;
mod math;
mod utils;

use core::f64;
use std::convert::TryInto;

use graphic::{fragment::Fragment, rasterize, vertex::Vertex, vertex_shader};
use js_sys::Uint8ClampedArray;

use math::matrix::Matrix;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn log(s: &str);
}

static mut WIDTH: usize = 1280;
static mut HEIGHT: usize = 720;
static mut frame_buffer: Vec<u8> = Vec::new();
static mut mvp_matrix: Matrix<f64, 4, 4> = Matrix::<f64, 4, 4>::const_from([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
]);

#[wasm_bindgen]
pub fn set_frame_size(width: usize, height: usize) {
    unsafe {
        WIDTH = width;
        HEIGHT = height;
        frame_buffer.resize(WIDTH * HEIGHT * 4, 0);
    }
}

#[wasm_bindgen]
pub fn render() -> Uint8ClampedArray {
    unsafe {
        // let vertices = vertex_shader(
        //     &mut triangle,
        //     &model_matrix
        //         .mul(&view_matrix)
        //         .mul(&view_matrix)
        //         .mul(&projection_matrix),
        // );
        // render(WIDTH, HEIGHT, &mut buffer);

        Uint8ClampedArray::view(&frame_buffer)
    }
}

#[wasm_bindgen]
pub fn draw_triangle(pos: &[f64], color: &[f64]) {
    let mut triangle: Vec<Vertex> = vec![];
    for i in 0..3 {
        triangle.push(Vertex::from_array(
            &pos[i * 3..i * 3 + 3]
                .try_into()
                .expect("Invalid vertex position"),
            &color[i * 4..i * 4 + 4]
                .try_into()
                .expect("Invalid vertex color"),
        ))
    }

    unsafe {
        let vertices: Vec<Vertex> = triangle
            .iter()
            .map(|vertex| vertex_shader(vertex, &mvp_matrix))
            .collect();

        let mut fragments: Vec<Fragment> = rasterize(&vertices, WIDTH, HEIGHT);
        fragments.iter().for_each(|fragment| {
            let index = (fragment.pos.y() * WIDTH + fragment.pos.x()) * 4;

            frame_buffer[index] = (fragment.color.x() * 255.0) as u8;
            frame_buffer[index + 1] = (fragment.color.y() * 255.0) as u8;
            frame_buffer[index + 2] = (fragment.color.z() * 255.0) as u8;
            frame_buffer[index + 3] = (fragment.color.w() * 255.0) as u8;
        });
    }
}

#[wasm_bindgen]
pub fn clear_buffer() {
    unsafe {
        frame_buffer.fill(255);
    }
}

#[wasm_bindgen]
pub fn set_mvp_matrix(matrix: &[f64]) {
    unsafe {
        for i in 0..4 {
            for j in 0..4 {
                mvp_matrix[j][i] = matrix[j * 4 + i];
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn init() {
    set_panic_hook()
}
