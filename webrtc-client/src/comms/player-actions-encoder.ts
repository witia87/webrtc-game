import { Command, CommandType, PlayerAction } from './messages/commands';
import { MovePlayerActionPayload, PlayerActionType } from './messages/player_actions';
import { Vector2 } from './messages/commons';

export class PlayerActionsEncoder {

  public static encode(collectedMovementDirection: Vector2 | undefined): Command {

    const movePlayerActionPayload = MovePlayerActionPayload.fromJSON({
      direction: collectedMovementDirection,
    });

    let playerActionBytes = PlayerAction.encode({
      playerActionType: PlayerActionType.Move,
      playerActionPayload: MovePlayerActionPayload.encode(movePlayerActionPayload).finish(),
    }).finish();

    return {
      commandType: CommandType.PlayerActionCommand,
      commandPayload: playerActionBytes,
    };
  }
}
