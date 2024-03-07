import { Matrix, Matrix2, Matrix3, Matrix4 } from "./matrix";

export abstract class Vector {
  elements: number[];

  constructor(...elements: number[]) {
    this.elements = elements;
  }

  dot(rhs: Vector): number {
    return this.elements.reduce(
      (acc, elem, index) => acc + elem * rhs.elements[index],
      0
    );
  }

  multiplyMatrix(matrix: Matrix) {
    let temp = new Array(this.elements.length).fill(0);

    for (let i = 0; i < this.elements.length; i++) {
      let sum = 0;
      for (let j = 0; j < this.elements.length; j++) {
        sum += matrix.elements[i][j] * this.elements[j];
      }
      temp[i] = sum;
    }

    this.elements = temp;
  }
}

export class Vector2 extends Vector {
  constructor(x: number, y: number) {
    super(x, y);
  }

  dot(rhs: Vector2): number {
    return super.dot(rhs);
  }

  multiplyMatrix(matrix: Matrix2) {
    super.multiplyMatrix(matrix);
  }

  get x(): number {
    return this.elements[0];
  }

  set x(value: number) {
    this.elements[0] = value;
  }

  get y(): number {
    return this.elements[1];
  }

  set y(value: number) {
    this.elements[1] = value;
  }
}

export class Vector3 extends Vector {
  constructor(x: number, y: number, z: number) {
    super(x, y, z);
  }

  dot(rhs: Vector3): number {
    return super.dot(rhs);
  }

  multiplyMatrix(matrix: Matrix3) {
    super.multiplyMatrix(matrix);
  }

  cross(rhs: Vector3): Vector3 {
    return new Vector3(
      this.y * rhs.z - this.z * rhs.y,
      this.z * rhs.x - this.x * rhs.z,
      this.x * rhs.y - this.y * rhs.x
    );
  }

  get x(): number {
    return this.elements[0];
  }

  set x(value: number) {
    this.elements[0] = value;
  }

  get y(): number {
    return this.elements[1];
  }

  set y(value: number) {
    this.elements[1] = value;
  }

  get z(): number {
    return this.elements[2];
  }

  set z(value: number) {
    this.elements[2] = value;
  }
}

export class Vector4 extends Vector {
  constructor(x: number, y: number, z: number, w: number) {
    super(x, y, z, w);
  }

  dot(rhs: Vector4): number {
    return super.dot(rhs);
  }

  multiplyMatrix(matrix: Matrix4) {
    super.multiplyMatrix(matrix);
  }

  get x(): number {
    return this.elements[0];
  }

  set x(value: number) {
    this.elements[0] = value;
  }

  get y(): number {
    return this.elements[1];
  }

  set y(value: number) {
    this.elements[1] = value;
  }

  get z(): number {
    return this.elements[2];
  }

  set z(value: number) {
    this.elements[2] = value;
  }

  get w(): number {
    return this.elements[3];
  }

  set w(value: number) {
    this.elements[3] = value;
  }
}
