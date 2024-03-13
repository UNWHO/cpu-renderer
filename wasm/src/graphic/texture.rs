use std::convert::TryInto;

use wasm_bindgen::prelude::*;

type Handle = usize;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f64>,
}

impl Texture {
    pub fn get_color(&self, u: f64, v: f64) -> [f64; 3] {
        let u = ((u * self.width as f64) as usize)
            .max(0)
            .min(self.width - 1);
        let v = ((v * self.width as f64) as usize)
            .max(0)
            .min(self.height - 1);

        let index = (v * self.width + u) * 3;

        self.data[index..index + 3].try_into().unwrap()
    }
}

const MAX_TEXTURE: usize = 16;

static mut HANDLE_INDEX: Handle = 0;
static mut TEXTURES: Vec<Texture> = Vec::new();
static mut BIND_MAP: [Handle; MAX_TEXTURE] = [0; MAX_TEXTURE];

#[wasm_bindgen]
pub enum TextureIndex {
    T0,
}

#[wasm_bindgen]
pub fn create_texture() -> usize {
    unsafe {
        TEXTURES.push(Texture {
            width: 0,
            height: 0,
            data: Vec::new(),
        });

        let handle = HANDLE_INDEX;
        HANDLE_INDEX += 1;

        handle
    }
}

#[wasm_bindgen]
pub fn write_texture(index: TextureIndex, width: usize, height: usize, data: &[f64]) {
    let texture = get_texture_as_mut(index);

    texture.width = width;
    texture.height = height;
    texture.data = data.to_vec();
}

#[wasm_bindgen]
pub fn bind_texture(index: TextureIndex, handle: Handle) {
    unsafe {
        BIND_MAP[index as usize] = handle;
    }
}

#[inline]
pub fn get_texture(index: TextureIndex) -> &'static Texture {
    unsafe {
        let handle = BIND_MAP[index as usize];
        &TEXTURES[handle]
    }
}

#[inline]
pub fn get_texture_as_mut(index: TextureIndex) -> &'static mut Texture {
    unsafe {
        let handle = BIND_MAP[index as usize];
        &mut TEXTURES[handle]
    }
}
