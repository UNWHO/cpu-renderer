pub mod math;
mod utils;

use js_sys::Uint8ClampedArray;
use math::vector::Vector4;
use wasm_bindgen::prelude::*;

static mut WIDTH: usize = 1280;
static mut HEIGHT: usize = 720;
static mut buffer: Vec<u8> = Vec::new();
static mut color_index: usize = 0;

const RED: Vector4<u8> = Vector4::const_from([[255, 0, 0, 255]]);
const GREEN: Vector4<u8> = Vector4::const_from([[0, 255, 0, 255]]);
const BLUE: Vector4<u8> = Vector4::const_from([[0, 0, 255, 255]]);
const COLORS: [&Vector4<u8>; 3] = [&RED, &GREEN, &BLUE];

#[wasm_bindgen]
pub fn set_frame_size(width: usize, height: usize) {
    unsafe {
        WIDTH = width;
        HEIGHT = height;
        buffer.resize(WIDTH * HEIGHT * 4, 0);
    }
}

#[wasm_bindgen]
pub fn render_loop() -> Uint8ClampedArray {
    unsafe {
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                let color = COLORS[color_index];
                color_index = (color_index + 1) % 3;

                buffer[j * WIDTH * 4 + i * 4] = color.x();
                buffer[j * WIDTH * 4 + i * 4 + 1] = color.y();
                buffer[j * WIDTH * 4 + i * 4 + 2] = color.z();
                buffer[j * WIDTH * 4 + i * 4 + 3] = color.w();
            }
        }

        Uint8ClampedArray::view(&buffer)
    }
}
