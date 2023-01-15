export class KeyboardView {
  private pressedKeys: Map<string, boolean>;

  public isKeyPressed(key: string): boolean {
    return this.pressedKeys.has(key) && this.pressedKeys.get(key);
  }

  private keyDown = (event: KeyboardEvent) => {
    this.pressedKeys.set(event.key, true);
  };

  private keyUp = (event: KeyboardEvent) => {
    this.pressedKeys.set(event.key, false);
  };

  constructor() {
    this.pressedKeys = new Map();

    window.addEventListener('keydown', this.keyDown);
    window.addEventListener('keyup', this.keyUp);
  }
}
