import 'pixi.js';

import { PlayersPresenter } from './presenters/players-presenter';
import { WebrtcServer } from './comms/webrtc-server';
import {
  EntitiesUpdatesNotificationPayload,
  NotificationType,
  PlayerSetupNotificationPayload,
} from './comms/messages/notifications';
import { filter, map, mergeMap } from 'rxjs';
import { PlayersPositionsModel } from './models/players-positions-model';
import { KeyboardView } from './views/keyboard-view';
import { PlayerMovementController } from './controllers/player-movement-controller';
import { PlayerActionsEncoder } from './comms/player-actions-encoder';
import { PlayerModel } from './models/player-model';
import { EntitiesUpdatesDecoder } from './comms/entities-updates-decoder';

const app = new PIXI.Application({
  width: 600,
  height: 400,
  backgroundColor: 0x1099bb,
  resolution: window.devicePixelRatio || 1,
  sharedTicker: true,
});

document.body.appendChild(app.view);
const container = new PIXI.Container();
app.stage.addChild(container);

PIXI.loader
  .add('p1', './assets/p1_front.png')
  .add('p2', './assets/p2_front.png')
  .load(() => init());

function init(): void {
  const webrtcServer = new WebrtcServer();

  const playerModel = new PlayerModel();
  const playerPositionsModel = new PlayersPositionsModel();

  const entitiesUpdatesDecoder = new EntitiesUpdatesDecoder(playerPositionsModel);

  webrtcServer.notifications$
    .pipe(
      filter(notification => notification.notificationType === NotificationType.PlayerSetupNotification),
      map(notification => PlayerSetupNotificationPayload.decode(notification.notificationPayload)),
    )
    .subscribe(playerSetupPayload => {
      playerModel.update(playerSetupPayload);

      webrtcServer.notifications$
        .pipe(
          filter(notification => notification.notificationType === NotificationType.EntitiesUpdatesNotification),
          map(notification => EntitiesUpdatesNotificationPayload.decode(notification.notificationPayload)),
          mergeMap(entitiesUpdatesNotification => entitiesUpdatesDecoder.decodeAndUpdateModels(entitiesUpdatesNotification)),
        )
        .subscribe(payloadBytes => playerPositionsModel.update(payloadBytes));
    });

  webrtcServer.start();

  const keyboardView = new KeyboardView();

  const playerMovementController = new PlayerMovementController(keyboardView);

  webrtcServer.notifications$
    .subscribe(() => {
      let direction = playerMovementController.collectDirection();
      if (direction) {
        let playerActionCommand = PlayerActionsEncoder.encode(direction);
        webrtcServer.sendCommand(playerActionCommand);
      }
    });

  let playersPresenter = new PlayersPresenter(container, playerModel, playerPositionsModel);

  app.ticker.add(delta => playersPresenter.update(delta));
}
