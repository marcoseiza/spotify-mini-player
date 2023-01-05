export type HandleOnClick = (
  event: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement }
) => any;
