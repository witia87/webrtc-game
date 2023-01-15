import { PlayerSetupNotificationPayload } from '../comms/messages/notifications';

export class PlayerModel {
  public playerId: number;

  public update(playerSetupNotificationPayload: PlayerSetupNotificationPayload) {
    this.playerId = playerSetupNotificationPayload.assignedPlayerId;
  }
}
