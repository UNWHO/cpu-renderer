import {
  TextureIndex,
  bind_texture,
  create_texture,
  write_texture,
} from "../../wasm/pkg/cpu_renderer";
import image from "./box.bmp";

export async function initTexture() {
  const image = await readImage();
  const handle = create_texture();
  bind_texture(TextureIndex.T0, handle);
  write_texture(
    TextureIndex.T0,
    image.width,
    image.height,
    new Float64Array(image.data)
  );
}

async function readImage() {
  const file = await fetch(image);
  const arrayBuffer = await file.arrayBuffer();
  const dataView = new DataView(arrayBuffer, 0);

  const pixelDataOffset = dataView.getInt32(10, true);

  const width = dataView.getInt32(18, true);
  const height = dataView.getInt32(22, true);

  const remain = (width * 3) % 4;
  const padding = remain === 0 ? 0 : 4 - remain;

  const pixelArray: number[] = [];

  for (let y = width - 1; y >= 0; y--) {
    for (let x = 0; x < width; x++) {
      const offset = y * (width * 3 + padding) + x * 3 + pixelDataOffset;

      const blue = dataView.getUint8(offset);
      const green = dataView.getUint8(offset + 1);
      const red = dataView.getUint8(offset + 2);

      pixelArray.push(red, green, blue);
    }
  }

  return {
    width,
    height,
    data: pixelArray.map((el) => el / 255),
  };
}
