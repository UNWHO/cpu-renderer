use super::{
    attribute::get_attr_array,
    buffer::float::{get_float_buffer, FloatBufferType},
};

pub fn vertex_shader<O, F>(indices: &[usize; 3], function: F) -> [O; 3]
where
    F: Fn(&Vec<&[f64]>) -> O,
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
