use wasm_bindgen::prelude::*;

type Handle = usize;

static mut HANDLE_INDEX: Handle = 0;
static mut ATTRIBUTES: Vec<AttributeArray> = Vec::new();
static mut BIND_TARGET: Handle = 0;

#[wasm_bindgen]
pub fn create_attribute_array() -> Handle {
    unsafe {
        ATTRIBUTES.push(AttributeArray::new());

        let handle = HANDLE_INDEX;
        HANDLE_INDEX += 1;

        handle
    }
}

#[wasm_bindgen]
pub fn bind_attribute_array(handle: Handle) {
    unsafe { BIND_TARGET = handle }
}

#[inline]
pub fn get_attr_array() -> &'static AttributeArray {
    unsafe { &ATTRIBUTES[BIND_TARGET] }
}

#[inline]
fn get_attr_array_as_mut() -> &'static mut AttributeArray {
    unsafe { &mut ATTRIBUTES[BIND_TARGET] }
}

#[wasm_bindgen]
pub fn push_attr(length: usize) {
    let attr_array = get_attr_array_as_mut();
    attr_array.attr.push(length)
}

pub struct AttributeArray {
    attr: Vec<usize>,
}

impl AttributeArray {
    pub fn new() -> Self {
        Self { attr: Vec::new() }
    }

    pub fn get_attrs(&self) -> &Vec<usize> {
        &self.attr
    }

    pub fn get_stride(&self) -> usize {
        self.attr.iter().fold(0, |acc, len| acc + len)
    }
}
