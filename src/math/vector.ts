import { Matrix, Matrix2, Matrix3, Matrix4 } from "./matrix";

export class Vector {
  elements: number[];

  protected constructor(...elements: number[]) {
    this.elements = elements;
  }

  add(rhs: Vector): Vector {
    return new Vector(
      ...this.elements.map((element, index) => element + rhs.elements[index])
    );
  }

  sub(rhs: Vector): Vector {
    return new Vector(
      ...this.elements.map((element, index) => element - rhs.elements[index])
    );
  }

  normalize(): Vector {
    const lengthSquare = this.elements.reduce(
      (acc, element) => acc + element * element,
      0
    );
    const divisor = 1 / Math.sqrt(lengthSquare);

    return this.multiplyScala(divisor);
  }

  dot(rhs: Vector): number {
    return this.elements.reduce(
      (acc, elem, index) => acc + elem * rhs.elements[index],
      0
    );
  }

  multiplyScala(scala: number): Vector {
    return new Vector(...this.elements.map((element) => element * scala));
  }

  multiplyMatrix(matrix: Matrix): Vector {
    let temp: number[] = new Array(this.elements.length).fill(0);

    for (let i = 0; i < this.elements.length; i++) {
      let sum = 0;
      for (let j = 0; j < this.elements.length; j++) {
        sum += matrix.elements[i][j] * this.elements[j];
      }
      temp[i] = sum;
    }

    return new Vector(...temp);
  }
}

export class Vector2 extends Vector {
  constructor(x: number, y: number) {
    super(x, y);
  }

  static from(vector: Vector): Vector2 {
    return new Vector2(vector.elements[0], vector.elements[1]);
  }

  add(rhs: Vector2): Vector2 {
    return Vector2.from(super.add(rhs));
  }

  sub(rhs: Vector2): Vector2 {
    return Vector2.from(super.sub(rhs));
  }

  normalize(): Vector2 {
    return Vector2.from(super.normalize());
  }

  dot(rhs: Vector2): number {
    return super.dot(rhs);
  }

  multiplyScala(scala: number): Vector2 {
    return Vector2.from(super.multiplyScala(scala));
  }

  multiplyMatrix(matrix: Matrix2): Vector2 {
    return Vector2.from(super.multiplyMatrix(matrix));
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

  static from(vector: Vector): Vector3 {
    return new Vector3(
      vector.elements[0],
      vector.elements[1],
      vector.elements[2]
    );
  }

  add(rhs: Vector3): Vector3 {
    return Vector3.from(super.add(rhs));
  }

  sub(rhs: Vector3): Vector3 {
    return Vector3.from(super.sub(rhs));
  }

  normalize(): Vector3 {
    return Vector3.from(super.normalize());
  }

  dot(rhs: Vector3): number {
    return super.dot(rhs);
  }

  multiplyScala(scala: number): Vector3 {
    return Vector3.from(super.multiplyScala(scala));
  }

  multiplyMatrix(matrix: Matrix3): Vector3 {
    return Vector3.from(super.multiplyMatrix(matrix));
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

  static from(vector: Vector): Vector4 {
    return new Vector4(
      vector.elements[0],
      vector.elements[1],
      vector.elements[2],
      vector.elements[3]
    );
  }

  add(rhs: Vector4): Vector4 {
    return Vector4.from(super.add(rhs));
  }

  sub(rhs: Vector4): Vector4 {
    return Vector4.from(super.sub(rhs));
  }

  normalize(): Vector4 {
    return Vector4.from(super.normalize());
  }

  dot(rhs: Vector4): number {
    return super.dot(rhs);
  }

  multiplyScala(scala: number): Vector4 {
    return Vector4.from(super.multiplyScala(scala));
  }
  multiplyMatrix(matrix: Matrix4): Vector4 {
    return Vector4.from(super.multiplyMatrix(matrix));
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
