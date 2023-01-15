import { PlayersPositionsModel } from '../models/players-positions-model';
import { PlayerSprite } from '../views/player-sprite';
import { PlayerModel } from '../models/player-model';
import { PixiContainerUtilities } from '../utils/pixi-container-utilities';

export class PlayersPresenter {
  private readonly _container: PIXI.Container;
  private readonly _playerModel: PlayerModel;
  private readonly _playerPositionsModel: PlayersPositionsModel;

  constructor(container: PIXI.Container,
              playerModel: PlayerModel,
              playerPositionsModel: PlayersPositionsModel) {
    this._container = container;
    this._playerModel = playerModel;
    this._playerPositionsModel = playerPositionsModel;
  }

  update(delta: number): void {

    PixiContainerUtilities.detachChildren(this._container,
      (child: PlayerSprite) =>
        !this._playerPositionsModel.positions.has(child.playerId));

    this._playerPositionsModel.positions.forEach(
      (position, playerId) => {
        if (!this._container.children.some(
          (displayObject) => (displayObject as PlayerSprite).playerId === playerId)) {
          let texture = PIXI.Texture.fromFrame(
            playerId === this._playerModel.playerId ? 'p1' : 'p2');
          this._container.addChild(new PlayerSprite(texture, playerId));
        }

        this._container.children
          .find(displayObject => (displayObject as PlayerSprite).playerId === playerId)
          .position.set(position.x, position.y);
      },
    );
  }
}
