/**
 * App surfaces stacked above the page, top first. The queue drawer is drawn above Now Playing,
 * so one Escape closes only the queue and leaves the lyrics page open.
 * Dialogs and popover menus live in the browser top layer and handle their own Escape.
 */
export type EscapeLayer = 'queue' | 'immersive' | 'toast';
export function escapeLayer(open: {
  queue: boolean;
  immersive: boolean;
  toast: boolean;
}): EscapeLayer | null {
  if (open.queue) return 'queue';
  if (open.immersive) return 'immersive';
  if (open.toast) return 'toast';
  return null;
}
