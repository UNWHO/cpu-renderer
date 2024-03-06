use wasm_bindgen::prelude::*;

type BufferHandle = usize;
type BufferData = f64;
type Buffer = Vec<BufferData>;

static mut HANDLE_INDEX: BufferHandle = 0;
static mut BUFFERS: Vec<Buffer> = Vec::new();
static mut BIND_MAP: Vec<BufferHandle> = Vec::new();

#[wasm_bindgen]
pub enum BufferType {
    ArrayBuffer = 0,
    ElementArrayBuffer = 1,
}

#[wasm_bindgen]
pub fn create_buffer() -> BufferHandle {
    unsafe {
        BUFFERS[HANDLE_INDEX] = Vec::<f64>::new();

        HANDLE_INDEX += 1;
        HANDLE_INDEX
    }
}

#[wasm_bindgen]
pub fn write_buffer(handle: BufferHandle, data: &[BufferData], offset: usize) {
    unsafe {
        let buffer = &mut BUFFERS[handle];

        for i in 0..data.len() {
            buffer[i + offset] = data[i];
        }
    }
}

#[wasm_bindgen]
pub fn bind_buffer(buffer_type: BufferType, handle: BufferHandle) {
    unsafe {
        BIND_MAP[buffer_type as usize] = handle;
    }
}

#[inline]
pub fn get_buffer(buffer_type: BufferType) -> &'static Buffer {
    unsafe {
        let handle = BIND_MAP[buffer_type as usize];
        &BUFFERS[handle]
    }
}
