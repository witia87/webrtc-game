import { Component, ComponentProps } from 'super-ecs';

export class PositionComponent implements Component {
	public static TYPE: symbol = Symbol('PositionComponent');
	public name: symbol = PositionComponent.TYPE;
	public x: number;
	public y: number;

	constructor(props?: ComponentProps<PositionComponent>) {
		const { x = 0, y = 0 } = props || {};
		this.x = x;
		this.y = y;
	}
}
