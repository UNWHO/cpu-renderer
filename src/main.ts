import {
  UniformMatrix,
  clear_buffer,
  draw_triangle,
  render,
  set_frame_size,
  set_uniform_matrix,
} from "../wasm/pkg/cpu_renderer";
import { initRenderer } from "./init";
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
  const rad = (theta * Math.PI) / 180;

  const modelMatrix = [
    [Math.cos(rad), -Math.sin(rad), 0, 0],
    [Math.sin(rad), Math.cos(rad), 0, 0],
    [0, 0, 1, 0],
    [0, 0, 0, 1],
  ];

  const viewMatrix = [
    [1, 0, 0, 0],
    [0, 1, 0, 0],
    [0, 0, 1, 0],
    [0, 0, 0, 1],
  ];

  const projMatrix = [
    [1, 0, 0, 0],
    [0, 1, 0, 0],
    [0, 0, 1, 0],
    [0, 0, 0, 1],
  ];

  const mvpMatrix = multiplyMatrix(
    multiplyMatrix(modelMatrix, viewMatrix),
    projMatrix
  );

  set_uniform_matrix(
    UniformMatrix.MvpMatrix,
    new Float64Array(mvpMatrix.flat())
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

function multiplyMatrix(a: number[][], b: number[][]): number[][] {
  const result = new Array(4).fill(null).map(() => new Array(4).fill(0));

  for (let i = 0; i < 4; i++) {
    for (let j = 0; j < 4; j++) {
      for (let k = 0; k < 4; k++) {
        result[j][i] += a[j][k] * b[k][i];
      }
    }
  }

  return result;
}
