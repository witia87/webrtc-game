import { Component, ComponentProps } from 'super-ecs';

export class RandomMovementComponent implements Component {
	public static TYPE: symbol = Symbol('RandomMovementComponent');
	public name: symbol = RandomMovementComponent.TYPE;
	public speed: number;
	public direction: number;

	constructor(props?: ComponentProps<RandomMovementComponent>) {
		const { speed = 2, direction = 1 } = props || {};
		this.speed = speed;
		this.direction = direction;
	}
}
