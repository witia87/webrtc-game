export class PlayerSprite extends PIXI.Sprite {
  private readonly _playerId: number;

  constructor(texture: PIXI.Texture, playerId: number) {
    super(texture);
    this._playerId = playerId;
  }

  get playerId(): number {
    return this._playerId;
  }
}
