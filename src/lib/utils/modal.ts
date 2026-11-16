type Box = { left: number; right: number; top: number; bottom: number };
type Target = { getBoundingClientRect(): Box };
type PointerLike = { target: unknown; clientX: number; clientY: number; button?: number };

/** A modal <dialog> fills the top layer; a press on its ::backdrop targets the dialog but lies outside its box. */
export function outsideBox(box: Box, x: number, y: number) {
  return x < box.left || x > box.right || y < box.top || y > box.bottom;
}

/**
 * Backdrop and Escape dismissal for one modal dialog.
 * Dismisses only when both press and release land on the backdrop, so a text selection dragged out of an
 * input never closes the dialog, and a press that only light-dismisses an open popover is not counted.
 */
export function createDismissal(
  node: Target,
  options: { canDismiss: () => boolean; popoverOpen: () => boolean; dismiss: () => void },
) {
  let armed = false;
  const onBackdrop = (e: PointerLike) =>
    e.target === node && outsideBox(node.getBoundingClientRect(), e.clientX, e.clientY);
  return {
    pointerdown(e: PointerLike) {
      armed = (e.button ?? 0) === 0 && onBackdrop(e) && !options.popoverOpen();
    },
    click(e: PointerLike) {
      const hit = armed && onBackdrop(e);
      armed = false;
      if (hit && options.canDismiss()) options.dismiss();
    },
    /** Escape is handled here so the native close request (which Chromium may skip or force) never races app state. */
    keydown(e: {
      key: string;
      isComposing?: boolean;
      defaultPrevented?: boolean;
      preventDefault(): void;
    }) {
      if (e.key !== 'Escape' || e.isComposing || e.defaultPrevented) return;
      e.preventDefault();
      if (options.canDismiss()) options.dismiss();
    },
    /** Any other native close request: keep the dialog open while busy, otherwise let state close it. */
    cancel(e: { preventDefault(): void }) {
      e.preventDefault();
      if (options.canDismiss()) options.dismiss();
    },
  };
}
