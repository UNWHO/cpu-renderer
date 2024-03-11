use crate::{math::vector::accessor::Accessor2d, FRAME_BUFFER, WIDTH};

use super::{
    attribute::get_attr_array,
    buffer::float::{get_float_buffer, FloatBufferType},
    rasterize::{Fragment, Vertex},
};

pub fn vertex_shader<const T: usize, F>(indices: &[usize; 3], function: F) -> [Vertex<T>; 3]
where
    F: Fn(&Vec<&[f64]>) -> Vertex<T>,
{
    let vao = get_attr_array();
    let vbo = get_float_buffer(FloatBufferType::ArrayBuffer);

    let attrs = vao.get_attrs();
    let stride = vao.get_stride();

    let operation = |index: usize| {
        let mut offset = 0;

        let inputs = attrs
            .iter()
            .map(|len| {
                let start = (index as usize) * stride + offset;
                let end = start + len;
                let input = &vbo[start..end];

                offset += len;
                input
            })
            .collect();

        function(&inputs)
    };

    [
        operation(indices[0]),
        operation(indices[1]),
        operation(indices[2]),
    ]
}

pub fn fragment_shader<const T: usize, F>(fragments: &Vec<Fragment<T>>, function: F)
where
    F: Fn(&[f64; T]) -> [f64; 4],
{
    fragments.iter().for_each(|fragment| {
        let color = function(&fragment.varying);

        unsafe {
            let index = (fragment.pos.y() * WIDTH + fragment.pos.x()) * 4;

            FRAME_BUFFER[index] = (color[0] * 255.0) as u8;
            FRAME_BUFFER[index + 1] = (color[1] * 255.0) as u8;
            FRAME_BUFFER[index + 2] = (color[2] * 255.0) as u8;
            FRAME_BUFFER[index + 3] = (color[3] * 255.0) as u8;
        }
    })
}
