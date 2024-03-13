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

export function initModel() {
  const vao = create_attribute_array();
  bind_attribute_array(vao);
  push_attr(3); // pos
  // push_attr(4); // color
  push_attr(2); // tex coord

  const vbo = create_float_buffer();
  bind_float_buffer(FloatBufferType.ArrayBuffer, vbo);
  write_float_buffer(
    FloatBufferType.ArrayBuffer,
    new Float64Array([
      ...[-0.5, 0.5, -0.5], // pos
      // ...[1.0, 0.0, 0.0, 1.0], // color
      ...[0.0, 1.0], // tex coord

      ...[-0.5, -0.5, -0.5], // pos
      // ...[0.0, 1.0, 0.0, 1.0], // color
      ...[0.0, 0.0], // tex coord

      ...[0.5, -0.5, -0.5], // pos
      // ...[0.0, 0.0, 1.0, 1.0], // color
      ...[1.0, 0.0], // tex coord

      ...[0.5, 0.5, -0.5], // pos
      // ...[1.0, 1.0, 0.0, 1.0], // color
      ...[1.0, 1.0], // tex coord

      ...[-0.5, 0.5, 0.5], // pos
      // ...[1.0, 0.0, 1.0, 1.0], // color
      ...[1.0, 1.0], // tex coord

      ...[-0.5, -0.5, 0.5], // pos
      // ...[0.0, 1.0, 1.0, 1.0], // color
      ...[1.0, 0.0], // tex coord

      ...[0.5, -0.5, 0.5], // pos
      // ...[0.0, 0.0, 0.0, 1.0], // color
      ...[0.0, 0.0], // tex coord

      ...[0.5, 0.5, 0.5], // pos
      // ...[1.0, 1.0, 1.0, 1.0], // color
      ...[0.0, 1.0], // tex coord

      ...[-0.5, 0.5, 0.5], // pos
      // ...[1.0, 0.0, 1.0, 1.0], // color
      ...[0.0, 0.0], // tex coord

      ...[0.5, 0.5, 0.5], // pos
      // ...[1.0, 1.0, 1.0, 1.0], // color
      ...[1.0, 0.0], // tex coord

      ...[-0.5, -0.5, 0.5], // pos
      // ...[0.0, 1.0, 1.0, 1.0], // color
      ...[0.0, 1.0], // tex coord

      ...[0.5, -0.5, 0.5], // pos
      // ...[0.0, 0.0, 0.0, 1.0], // color
      ...[1.0, 1.0], // tex coord
    ])
  );

  const ebo = create_uint_buffer();
  bind_uint_buffer(UintBufferType.ElementArrayBuffer, ebo);
  write_uint_buffer(
    UintBufferType.ElementArrayBuffer,
    Uint32Array.from([
      ...[0, 1, 2, 0, 2, 3],
      ...[3, 2, 6, 3, 6, 7],
      ...[7, 6, 5, 7, 5, 4],
      ...[4, 5, 1, 4, 1, 0],
      ...[8, 0, 3, 8, 3, 9],
      ...[10, 11, 2, 10, 2, 1],
    ])
  );
}
