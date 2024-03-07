import { Util } from "./util";
import { Vector3 } from "./vector";

export class Matrix {
  elements: number[][];

  protected constructor(elements: number[][]) {
    this.elements = elements;
  }

  multiply(rhs: Matrix): Matrix {
    const arr = new Array(this.elements.length)
      .fill(null)
      .map(() => new Array(this.elements.length).fill(0));

    for (let i = 0; i < this.elements.length; i++) {
      for (let j = 0; j < this.elements.length; j++) {
        for (let k = 0; k < this.elements.length; k++) {
          arr[j][i] += this.elements[j][k] * rhs.elements[k][i];
        }
      }
    }

    return new Matrix(arr);
  }

  static identity(dimension: number): Matrix {
    const arr = new Array(dimension)
      .fill(null)
      .map(() => new Array(dimension).fill(0));

    for (let i = 0; i < dimension; i++) arr[i][i] = 1;
    return new Matrix(arr);
  }
}

export class Matrix2 extends Matrix {
  constructor(elements: [[number, number], [number, number]]) {
    super(elements);
  }

  static identity(): Matrix2 {
    return super.identity(2);
  }

  multiply(rhs: Matrix2): Matrix2 {
    return super.multiply(rhs);
  }
}

export class Matrix3 extends Matrix {
  constructor(
    elements: [
      [number, number, number],
      [number, number, number],
      [number, number, number]
    ]
  ) {
    super(elements);
  }

  static identity(): Matrix3 {
    return super.identity(3);
  }

  multiply(rhs: Matrix3): Matrix3 {
    return super.multiply(rhs);
  }
}

export class Matrix4 extends Matrix {
  constructor(
    elements: [
      [number, number, number, number],
      [number, number, number, number],
      [number, number, number, number],
      [number, number, number, number]
    ]
  ) {
    super(elements);
  }

  static identity(): Matrix4 {
    return super.identity(4);
  }

  multiply(rhs: Matrix3): Matrix3 {
    return super.multiply(rhs);
  }

  static translate([x, y, z]: [number, number, number]): Matrix4 {
    const matrix = Matrix4.identity();

    matrix.elements[0][3] = x;
    matrix.elements[1][3] = y;
    matrix.elements[2][3] = z;

    return matrix;
  }

  static scale([x, y, z]: [number, number, number]): Matrix4 {
    const matrix = Matrix4.identity();

    matrix.elements[0][0] = x;
    matrix.elements[1][1] = y;
    matrix.elements[2][2] = z;

    return matrix;
  }

  static rotateX(degree: number): Matrix {
    const matrix = Matrix4.identity();
    const rad = Util.degToRad(degree);

    const cos = Math.cos(rad);
    const sin = Math.sin(rad);

    matrix.elements[1][1] = cos;
    matrix.elements[1][2] = -sin;
    matrix.elements[2][1] = sin;
    matrix.elements[2][2] = cos;

    return matrix;
  }

  static rotateY(degree: number): Matrix {
    const matrix = Matrix4.identity();
    const rad = Util.degToRad(degree);

    const cos = Math.cos(rad);
    const sin = Math.sin(rad);

    matrix.elements[0][0] = cos;
    matrix.elements[0][2] = sin;
    matrix.elements[2][0] = -sin;
    matrix.elements[2][2] = cos;

    return matrix;
  }

  static rotateZ(degree: number): Matrix {
    const matrix = Matrix4.identity();
    const rad = Util.degToRad(degree);

    const cos = Math.cos(rad);
    const sin = Math.sin(rad);

    matrix.elements[0][0] = cos;
    matrix.elements[0][1] = -sin;
    matrix.elements[1][0] = sin;
    matrix.elements[1][1] = cos;

    return matrix;
  }

  static lookAt(eye: Vector3, center: Vector3, up: Vector3): Matrix4 {
    const v = up;
    const n = eye.sub(center).normalize().multiplyScala(-1);
    const u = v.cross(n);

    return new Matrix4([
      [u.x, u.y, u.z, 0],
      [v.x, v.y, v.z, 0],
      [n.x, n.y, n.z, 0],
      [0, 0, 0, 1],
    ]).multiply(Matrix4.translate([-eye.x, -eye.y, -eye.z]));
  }
}
