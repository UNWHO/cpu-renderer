import {
  clear_buffer,
  draw_triangle,
  render,
  set_frame_size,
} from "../wasm/pkg/cpu_renderer";
import { Resolution, resolutionConfig } from "./resolution";

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const select = document.getElementById("resolution") as HTMLSelectElement;

const context = canvas.getContext("2d")!;

const vertices = new Float64Array([
  -0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0,
]);
const colors = new Float64Array([
  1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0,
]);

let imageData = context.createImageData(canvas.width, canvas.height);
let reqAnimFrameHandle: number;

select.oninput = () => {
  cancelAnimationFrame(reqAnimFrameHandle);

  const { width, height } = resolutionConfig[select.value as Resolution];

  canvas.width = width;
  canvas.height = height;

  imageData = context.createImageData(width, height);

  set_frame_size(width, height);

  reqAnimFrameHandle = requestAnimationFrame(loop);
};

function init() {
  set_frame_size(canvas.width, canvas.height);

  reqAnimFrameHandle = requestAnimationFrame(loop);
}

function loop() {
  clear_buffer();
  draw_triangle(vertices, colors);

  imageData.data.set(render());
  context.putImageData(imageData, 0, 0);

  reqAnimFrameHandle = requestAnimationFrame(loop);
}

init();
