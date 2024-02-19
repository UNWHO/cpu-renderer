mod utils;

use js_sys::Uint8ClampedArray;
use wasm_bindgen::prelude::*;

static mut WIDTH: usize = 1280;
static mut HEIGHT: usize = 720;
static mut buffer: Vec<u8> = Vec::new();
static mut color_index: usize = 0;

const RED: [u8; 4] = [255, 0, 0, 255];
const GREEN: [u8; 4] = [0, 255, 0, 255];
const BLUE: [u8; 4] = [0, 0, 255, 255];
const COLORS: [[u8; 4]; 3] = [RED, GREEN, BLUE];

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

                buffer[j * WIDTH * 4 + i * 4] = color[0];
                buffer[j * WIDTH * 4 + i * 4 + 1] = color[1];
                buffer[j * WIDTH * 4 + i * 4 + 2] = color[2];
                buffer[j * WIDTH * 4 + i * 4 + 3] = color[3];
            }
        }

        Uint8ClampedArray::view(&buffer)
    }
}
