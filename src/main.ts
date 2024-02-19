import { render_loop, set_frame_size } from "../wasm/pkg/cpu_renderer";
import { Resolution, resolutionConfig } from "./resolution";

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const select = document.getElementById("resolution") as HTMLSelectElement;

const context = canvas.getContext("2d")!;

let imageData = context.createImageData(canvas.width, canvas.height);
let reqAnimFrameHandle: number;

select.oninput = () => {
  cancelAnimationFrame(reqAnimFrameHandle);

  const { width, height } = resolutionConfig[select.value as Resolution];

  canvas.width = width;
  canvas.height = height;

  imageData = context.createImageData(width, height);

  set_frame_size(width, height);

  reqAnimFrameHandle = requestAnimationFrame(render);
};

function init() {
  set_frame_size(canvas.width, canvas.height);
  reqAnimFrameHandle = requestAnimationFrame(render);
}

function render() {
  imageData.data.set(render_loop());
  context.putImageData(imageData, 0, 0);

  reqAnimFrameHandle = requestAnimationFrame(render);
}

init();
