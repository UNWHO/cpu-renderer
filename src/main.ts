import {
  UniformMatrix,
  clear_buffer,
  draw_triangle,
  render,
  set_frame_size,
  set_uniform_matrix,
} from "../wasm/pkg/cpu_renderer";
import { initRenderer } from "./init";
import { Matrix4 } from "./math/matrix";
import { Resolution, resolutionConfig } from "./resolution";

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const select = document.getElementById("resolution") as HTMLSelectElement;
const fps = document.getElementById("fps") as HTMLDivElement;

const context = canvas.getContext("2d")!;

let theta = 0;

let imageData = context.createImageData(canvas.width, canvas.height);
let reqAnimFrameHandle: number;

let previousTime = Date.now();

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
  initRenderer();
  set_frame_size(canvas.width, canvas.height);

  reqAnimFrameHandle = requestAnimationFrame(loop);
}

function loop() {
  clear_buffer();

  // rotate model around z-axis
  theta += 0.5;

  const modelMatrix = Matrix4.rotateZ(theta);

  const viewMatrix = Matrix4.identity();

  const projMatrix = Matrix4.identity();

  const mvpMatrix = modelMatrix.multiply(viewMatrix).multiply(projMatrix);

  set_uniform_matrix(
    UniformMatrix.MvpMatrix,
    new Float64Array(mvpMatrix.elements.flat())
  );
  draw_triangle();

  imageData.data.set(render());
  context.putImageData(imageData, 0, 0);

  let currentTime = Date.now();
  let deltaTime = currentTime - previousTime;
  fps.innerHTML = `${1000 / deltaTime}fps`;
  previousTime = currentTime;

  reqAnimFrameHandle = requestAnimationFrame(loop);
}

init();
