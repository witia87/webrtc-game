import { Vector2 } from '../comms/messages/commons';
import { PlayerPositionUpdatePayload } from '../comms/messages/entities_updates';

export class PlayersPositionsModel {
  public positions: Map<number, Vector2> = new Map<number, Vector2>();

  public update(playerPositionUpdatePayloads: PlayerPositionUpdatePayload[]) {
    this.positions = new Map<number, Vector2>(
      playerPositionUpdatePayloads.map(payload => [payload.playerId, payload.newPosition]),
    );
  }
}
