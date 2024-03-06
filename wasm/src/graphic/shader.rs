use crate::alert;

use super::{
    attribute::get_attr_array,
    buffer::{
        float::{get_float_buffer, FloatBufferType},
        uint::{get_uint_buffer, UintBufferType},
    },
};

pub fn vertex_shader<O, F>(function: F) -> Vec<O>
where
    F: Fn(&Vec<&[f64]>) -> O,
{
    let vao = get_attr_array();
    let vbo = get_float_buffer(FloatBufferType::ArrayBuffer);
    let ebo = get_uint_buffer(UintBufferType::ElementArrayBuffer);

    let attrs = vao.get_attrs();
    let stride = vao.get_stride();

    ebo.iter()
        .map(|index| {
            let mut offset = 0;

            let inputs = attrs
                .iter()
                .map(|len| {
                    let start = (*index as usize) * stride + offset;
                    let end = start + len;
                    let input = &vbo[start..end];

                    offset += len;
                    input
                })
                .collect();

            function(&inputs)
        })
        .collect()

    // Vertex {
    //     pos: vertex
    //         .pos
    //         .to_homogeneous()
    //         .mul_matrix(get_uniform_matrix(uniform::UniformMatrix::MvpMatrix))
    //         .from_homogeneous(),
    //     color: vertex.color.clone(),
    // }
}
