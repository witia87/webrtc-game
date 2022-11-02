import { System, TickerDataLike } from 'super-ecs';

import { PositionComponent } from '../components/position-component';
import { RandomMovementComponent } from '../components/random-movement-component';

export class RandomMovementSystem extends System {
	private readonly _stageWidth: number;
	private readonly _stageHeight: number;

	constructor(props = { width: 600, height: 400 }) {
		super();
		this._stageWidth = props.width;
		this._stageHeight = props.height;
	}

	update(tickerData: TickerDataLike): void {
		const entities = this.world.getEntities([PositionComponent.TYPE, RandomMovementComponent.TYPE]);

		if (entities.length === 0) {
			return;
		}

		const { deltaTime } = tickerData;
		entities.forEach(entity => {
			const positionComponent = entity.getComponent<PositionComponent>(PositionComponent.TYPE);
			const randomMovementComponent = entity.getComponent<RandomMovementComponent>(RandomMovementComponent.TYPE);

			if (positionComponent && randomMovementComponent) {
				const { speed, direction } = randomMovementComponent;
				positionComponent.x += speed * direction * deltaTime;
				positionComponent.y += speed * direction * deltaTime;

				const stageWidth = this._stageWidth;
				const stageHeight = this._stageHeight;

				const offset = 92;

				if (positionComponent.x < -offset) positionComponent.x = stageWidth + offset;

				if (positionComponent.y < -offset) positionComponent.y = stageHeight + offset;

				if (positionComponent.x > stageWidth + offset) positionComponent.x = -offset;

				if (positionComponent.y > stageHeight + offset) positionComponent.y = -offset;
			}
		});
	}
}
