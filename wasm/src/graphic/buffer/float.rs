use wasm_bindgen::prelude::*;

type BufferHandle = usize;
type BufferData = f64;
type Buffer = Vec<BufferData>;

static mut HANDLE_INDEX: BufferHandle = 0;
static mut BUFFERS: Vec<Buffer> = Vec::new();
static mut BIND_MAP: Vec<BufferHandle> = Vec::new();

pub fn init_float_buffer_map() {
    unsafe { BIND_MAP.resize(1, 0) }
}

#[wasm_bindgen]
pub enum FloatBufferType {
    ArrayBuffer = 0,
}

#[wasm_bindgen]
pub fn create_float_buffer() -> BufferHandle {
    unsafe {
        BUFFERS.push(Buffer::new());

        let handle = HANDLE_INDEX;
        HANDLE_INDEX += 1;

        handle
    }
}

#[wasm_bindgen]
pub fn write_float_buffer(buffer_type: FloatBufferType, data: &[BufferData]) {
    let buffer = get_float_buffer_as_mut(buffer_type);

    buffer.extend(data.iter());
}

#[wasm_bindgen]
pub fn bind_float_buffer(buffer_type: FloatBufferType, handle: BufferHandle) {
    unsafe {
        BIND_MAP[buffer_type as usize] = handle;
    }
}

#[inline]
pub fn get_float_buffer(buffer_type: FloatBufferType) -> &'static Buffer {
    unsafe {
        let handle = BIND_MAP[buffer_type as usize];
        &BUFFERS[handle]
    }
}

#[inline]
fn get_float_buffer_as_mut(buffer_type: FloatBufferType) -> &'static mut Buffer {
    unsafe {
        let handle = BIND_MAP[buffer_type as usize];
        &mut BUFFERS[handle]
    }
}
