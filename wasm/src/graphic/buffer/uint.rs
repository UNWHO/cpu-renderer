use wasm_bindgen::prelude::*;

type BufferHandle = usize;
type BufferData = usize;
type Buffer = Vec<BufferData>;

static mut HANDLE_INDEX: BufferHandle = 0;
static mut BUFFERS: Vec<Buffer> = Vec::new();
static mut BIND_MAP: Vec<BufferHandle> = Vec::new();

pub fn init_uint_buffer_map() {
    unsafe { BIND_MAP.resize(1, 0) }
}

#[wasm_bindgen]
pub enum UintBufferType {
    ElementArrayBuffer,
}

#[wasm_bindgen]
pub fn create_uint_buffer() -> BufferHandle {
    unsafe {
        BUFFERS.push(Buffer::new());

        let handle = HANDLE_INDEX;
        HANDLE_INDEX += 1;

        handle
    }
}

#[wasm_bindgen]
pub fn write_uint_buffer(buffer_type: UintBufferType, data: &[BufferData]) {
    let buffer = get_uint_buffer_as_mut(buffer_type);

    buffer.extend(data.iter());
}

#[wasm_bindgen]
pub fn bind_uint_buffer(buffer_type: UintBufferType, handle: BufferHandle) {
    unsafe {
        BIND_MAP[buffer_type as usize] = handle;
    }
}

#[inline]
pub fn get_uint_buffer(buffer_type: UintBufferType) -> &'static Buffer {
    unsafe {
        let handle = BIND_MAP[buffer_type as usize];
        &BUFFERS[handle]
    }
}

#[inline]
pub fn get_uint_buffer_as_mut(buffer_type: UintBufferType) -> &'static mut Buffer {
    unsafe {
        let handle = BIND_MAP[buffer_type as usize];
        &mut BUFFERS[handle]
    }
}
