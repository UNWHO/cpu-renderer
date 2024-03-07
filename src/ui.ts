const fpsElement = document.getElementById("fps") as HTMLDivElement;
const resoultionElement = document.getElementById(
  "resolution"
) as HTMLDivElement;

export function writeFPS(fps: number) {
  fpsElement.innerHTML = `${fps}fps`;
}

export function writeResolution(width: number, height: number) {
  resoultionElement.innerHTML = `${width} * ${height}`;
}
