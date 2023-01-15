import { Vector2 } from '../comms/messages/commons';

export class Vector2Utilities {
  private static DELTA: number = 0.0001;

  public static clone(vector2: Vector2): Vector2 {
    return Vector2.fromJSON(vector2);
  }

  public static create(x: number,
                       y: number): Vector2 {
    return Vector2.fromJSON({ x: x, y: y });
  }

  public static getLength(vector2: Vector2): number {
    return Math.sqrt(vector2.x * vector2.x + vector2.y * vector2.y);
  }

  public static isZero(vector2: Vector2): boolean {
    return this.getLength(vector2) < this.DELTA;
  }

  public static getNormalised(vector2: Vector2): Vector2 {
    if (this.isZero(vector2)) {
      return this.create(0, 0);
    }

    let length = this.getLength(vector2);
    return Vector2.fromJSON({ x: vector2.x / length, y: vector2.y / length });
  }
}
