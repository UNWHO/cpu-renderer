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
import { Vector3 } from "./math/vector";
import { writeFPS, writeResolution } from "./ui";

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const context = canvas.getContext("2d")!;

let imageData: ImageData;
let reqAnimFrameHandle: number;

let theta = 0;
let previousTime = Date.now();

let camera = {
  eye: new Vector3(0, 0, -1),
  target: new Vector3(0, 0, 0),
  up: new Vector3(0, 1, 0),
};

function init() {
  initRenderer();

  writeResolution(canvas.width, canvas.height);
  set_frame_size(canvas.width, canvas.height);

  onWindowResize();
  window.addEventListener("resize", onWindowResize);

  reqAnimFrameHandle = requestAnimationFrame(loop);
}

function onWindowResize() {
  const width = document.body.clientWidth;
  const height = document.body.clientHeight;

  canvas.width = width;
  canvas.height = height;

  set_frame_size(width, height);
  imageData = context.createImageData(canvas.width, canvas.height);

  writeResolution(canvas.width, canvas.height);
}

function loop() {
  clear_buffer();

  // rotate model around z-axis
  theta += 0.5;

  const modelMatrix = Matrix4.rotateZ(theta);

  const viewMatrix = Matrix4.lookAt(camera.eye, camera.target, camera.up);

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
  writeFPS(1000 / deltaTime);
  previousTime = currentTime;

  reqAnimFrameHandle = requestAnimationFrame(loop);
}

init();
