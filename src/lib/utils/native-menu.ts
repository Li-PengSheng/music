// WebView2 shows a browser menu (Back, Refresh, Print, Inspect…) on every right click.
// Text fields keep it for copy/paste; in development Shift+right click keeps Inspect.
const EDITABLE =
  'textarea,[contenteditable]:not([contenteditable="false"]),input:not([type=range]):not([type=checkbox]):not([type=radio]):not([type=button]):not([type=submit]):not([type=color]):not([type=file])';

export function allowsNativeMenu(
  target: { closest(selector: string): unknown } | null,
  dev: boolean,
  shiftKey: boolean,
) {
  if (dev && shiftKey) return true;
  return !!target?.closest(EDITABLE);
}

type Key = Pick<KeyboardEvent, 'key' | 'ctrlKey' | 'shiftKey' | 'altKey' | 'metaKey'>;
// Browser reload/print/find-in-page accelerators make no sense in a desktop player.
export function isBrowserAccelerator(e: Key) {
  const key = e.key.length === 1 ? e.key.toLowerCase() : e.key;
  if (key === 'F5' || key === 'BrowserRefresh' || key === 'BrowserBack' || key === 'BrowserForward')
    return true;
  if (
    e.ctrlKey &&
    !e.altKey &&
    (key === 'r' || key === 'p' || key === 's' || key === 'u' || key === 'g')
  )
    return true;
  return false;
}

export function installNativeMenuGuard(dev: boolean) {
  const menu = (e: MouseEvent) => {
    const target = e.target instanceof Element ? e.target : null;
    if (!allowsNativeMenu(target, dev, e.shiftKey)) e.preventDefault();
  };
  const keys = (e: KeyboardEvent) => {
    if (!dev && isBrowserAccelerator(e)) e.preventDefault();
  };
  window.addEventListener('contextmenu', menu);
  window.addEventListener('keydown', keys, true);
  return () => {
    window.removeEventListener('contextmenu', menu);
    window.removeEventListener('keydown', keys, true);
  };
}
