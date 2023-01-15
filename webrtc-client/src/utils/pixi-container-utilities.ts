export class PixiContainerUtilities {

  public static detachChildren<T extends PIXI.Container>(
    container: PIXI.Container,
    shouldBeDetachedPredicate: (value: T) => boolean) {

    for (let i = container.children.length - 1; i >= 0; i--) {
      if (shouldBeDetachedPredicate(container.children[i] as T)) {
        container.removeChildAt(i);
      }
    }
  }
}
