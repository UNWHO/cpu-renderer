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
import { Vector3, Vector4 } from "./math/vector";
import { writeFPS, writeResolution } from "./ui";

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const context = canvas.getContext("2d")!;

let imageData: ImageData;

let theta = 0;
let previousTime = Date.now();

let camera = {
  eye: new Vector3(0, 0, -1),
  target: new Vector3(0, 0, 0),
  up: new Vector3(0, 1, 0),
  near: 0.01,
  far: 100,
};

function init() {
  initRenderer();

  writeResolution(canvas.width, canvas.height);
  set_frame_size(canvas.width, canvas.height);

  onWindowResize();
  window.addEventListener("resize", onWindowResize);

  requestAnimationFrame(loop);
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

  const fov = 0.00001;
  const projMatrix = Matrix4.frustrum(
    -canvas.width * fov,
    canvas.width * fov,
    -canvas.height * fov,
    canvas.height * fov,
    camera.near,
    camera.far
  );

  const mvpMatrix = projMatrix.multiply(viewMatrix).multiply(modelMatrix);
  console.log(mvpMatrix);
  console.log(
    new Vector4(0.0, 0.5, 0.0, 1.0).multiplyMatrix(
      new Matrix4([
        [0.001141509044593803, -0.000011824573846035142, 0, 0],

        [0.000009961798514125497, 0.0013549619553715059, 0, 0],

        [0, 0, -2.0002000200020005, -0.020002000200020003],

        [0, 0, -1, 0],
      ])
    )
  );

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

  requestAnimationFrame(loop);
}

init();
