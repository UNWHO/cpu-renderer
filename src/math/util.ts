export namespace Util {
  export function degToRad(degree: number) {
    return (degree * Math.PI) / 180;
  }

  export function radToDeg(radian: number) {
    return (radian * 180) / Math.PI;
  }
}
