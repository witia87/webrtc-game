import { System, TickerDataLike } from 'super-ecs';

import { SpriteComponent } from '../components/sprite-component';
import { PositionComponent } from '../components/position-component';

export class PositionSystem extends System {
	update(tickerData: TickerDataLike): void {
		const entities = this.world.getEntities([PositionComponent.TYPE, SpriteComponent.TYPE]);
		if (entities.length === 0) {
			return;
		}

		entities.forEach(entity => {
			const positionComponent = entity.getComponent<PositionComponent>(PositionComponent.TYPE);
			const spriteComponent = entity.getComponent<SpriteComponent>(SpriteComponent.TYPE);

			if (positionComponent && spriteComponent) {
				const { sprite } = spriteComponent;
				sprite.position.set(positionComponent.x, positionComponent.y);
			}
		});
	}
}
