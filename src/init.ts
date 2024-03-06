import {
  FloatBufferType,
  UintBufferType,
  bind_attribute_array,
  bind_float_buffer,
  bind_uint_buffer,
  create_attribute_array,
  create_float_buffer,
  create_uint_buffer,
  push_attr,
  write_float_buffer,
  write_uint_buffer,
} from "../wasm/pkg/cpu_renderer";

export function initRenderer() {
  const vao = create_attribute_array();
  bind_attribute_array(vao);
  push_attr(3); // pos
  push_attr(4); // color

  const vbo = create_float_buffer();
  bind_float_buffer(FloatBufferType.ArrayBuffer, vbo);
  write_float_buffer(
    FloatBufferType.ArrayBuffer,
    new Float64Array([
      // top
      ...[0.0, 0.5, 0.0], // pos
      ...[1.0, 0.0, 0.0, 1.0], // color

      // left
      ...[-0.5, -0.5, 0.0], // pos
      ...[0.0, 1.0, 0.0, 1.0], // color

      // right
      ...[0.5, -0.5, 0.0], // pos
      ...[0.0, 0.0, 1.0, 1.0], // color
    ])
  );

  const ebo = create_uint_buffer();
  bind_uint_buffer(UintBufferType.ElementArrayBuffer, ebo);
  write_uint_buffer(
    UintBufferType.ElementArrayBuffer,
    BigUint64Array.from([0n, 1n, 2n])
  );
}
