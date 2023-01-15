import { Vector2 } from '../comms/messages/commons';
import { KeyboardView } from '../views/keyboard-view';
import { Vector2Utilities } from '../utils/vector2-utilities';

export class PlayerMovementController {
  private keyboardView: KeyboardView;

  public collectDirection(): Vector2 | undefined {
    let y: number = this.keyboardView.isKeyPressed('w') ? -1 : 0;
    y += this.keyboardView.isKeyPressed('s') ? 1 : 0;

    let x: number = this.keyboardView.isKeyPressed('a') ? -1 : 0;
    x += this.keyboardView.isKeyPressed('d') ? 1 : 0;

    let direction = Vector2Utilities.getNormalised(Vector2Utilities.create(x, y));

    return Vector2Utilities.isZero(direction)
      ? undefined
      : direction;
  }

  constructor(keyboardView: KeyboardView) {
    this.keyboardView = keyboardView;
  }
}
