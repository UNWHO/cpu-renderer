mod graphic;
mod math;
mod utils;

use std::convert::TryInto;

use graphic::{
    buffer::{
        float::init_float_buffer_map,
        uint::{get_uint_buffer, init_uint_buffer_map, UintBufferType},
    },
    rasterize::{self, rasterize},
    shader::vertex_shader,
    uniform::{get_uniform_matrix, init_uniform_matrix_map, UniformMatrix},
};
use js_sys::Uint8ClampedArray;

use math::vector::{accessor::Accessor2d, Vector3};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

static mut WIDTH: usize = 1280;
static mut HEIGHT: usize = 720;
static mut FRAME_BUFFER: Vec<u8> = Vec::new();

#[wasm_bindgen]
pub fn set_frame_size(width: usize, height: usize) {
    unsafe {
        WIDTH = width;
        HEIGHT = height;
        FRAME_BUFFER.resize(WIDTH * HEIGHT * 4, 0);
    }
}

#[wasm_bindgen]
pub fn render() -> Uint8ClampedArray {
    unsafe { Uint8ClampedArray::view(&FRAME_BUFFER) }
}

#[wasm_bindgen]
pub fn draw() {
    unsafe {
        let ebo = get_uint_buffer(UintBufferType::ElementArrayBuffer);

        let mvp_matrix = get_uniform_matrix(UniformMatrix::MvpMatrix);

        for i in (0..ebo.len()).step_by(3) {
            // alert(&i.to_string());
            let indices: &[usize; 3] = &ebo[i..i + 3].try_into().unwrap();

            let vertices: [rasterize::Input<4>; 3] = vertex_shader(indices, |inputs| {
                let pos = Vector3::new(inputs[0][0], inputs[0][1], inputs[0][2]);

                let pos = pos
                    .to_homogeneous()
                    .mul_matrix(mvp_matrix)
                    .from_homogeneous();

                rasterize::Input {
                    pos,
                    varying: [inputs[1][0], inputs[1][1], inputs[1][2], inputs[1][3]],
                }
            });

            let fragments = rasterize(&vertices.try_into().unwrap(), WIDTH, HEIGHT);
            fragments.iter().for_each(|fragment| {
                let index = (fragment.pos.y() * WIDTH + fragment.pos.x()) * 4;

                FRAME_BUFFER[index] = (fragment.varying[0] * 0.0) as u8;
                FRAME_BUFFER[index + 1] = (fragment.varying[1] * 255.0) as u8;
                FRAME_BUFFER[index + 2] = (fragment.varying[2] * 255.0) as u8;
                FRAME_BUFFER[index + 3] = (fragment.varying[3] * 255.0) as u8;
            });
        }
    }
}

#[wasm_bindgen]
pub fn clear_buffer() {
    unsafe {
        FRAME_BUFFER.fill(255);
    }
}

#[wasm_bindgen(start)]
pub fn init() {
    set_panic_hook();

    init_float_buffer_map();
    init_uint_buffer_map();

    init_uniform_matrix_map();
}
