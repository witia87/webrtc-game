import { EntitiesUpdatesNotificationPayload } from './messages/notifications';
import { EntitiesUpdateType, PlayerPositionUpdatePayload } from './messages/entities_updates';
import { PlayersPositionsModel } from '../models/players-positions-model';


export class EntitiesUpdatesDecoder {
  private readonly playerPositionsModel: PlayersPositionsModel;

  constructor(playerPositionsModel: PlayersPositionsModel) {
    this.playerPositionsModel = playerPositionsModel;
  }

  public decodeAndUpdateModels(entitiesUpdatesPayloadBytes: EntitiesUpdatesNotificationPayload) {
    return entitiesUpdatesPayloadBytes.entitiesUpdates.map(entitiesUpdate => {
        switch (entitiesUpdate.entitiesUpdateType) {
          case EntitiesUpdateType.PlayerPositionUpdate: {
            let payloads = entitiesUpdate.entitiesUpdatePayloadsBytes
              .map(entityUpdatePayloadBytes => PlayerPositionUpdatePayload.decode(entityUpdatePayloadBytes));
            this.playerPositionsModel.update(payloads);
          }
        }
      },
    );
  }
}
