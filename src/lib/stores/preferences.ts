import { ui, notifyError } from './ui.svelte';
import {
  desktopAvailable,
  desktopPreferences,
  setWindowTheme,
  windowTheme,
  onWindowThemeChanged,
} from '../services/music';
import {
  THEME_KEY,
  THEME_COLOR,
  PALETTE_KEY,
  parsePalette,
  type Palette,
  parseTheme,
  resolveTheme,
  type Theme,
  type ThemeChoice,
} from '../utils/theme';

const KEY = 'music-mine-reduce-motion';

const systemDark =
  typeof matchMedia === 'function' ? matchMedia('(prefers-color-scheme: dark)') : null;

/** Paint the resolved theme; `system` is the theme Windows reported for this window, when known. */
function paint(system?: Theme) {
  const theme = resolveTheme(ui.theme, system ? system === 'dark' : (systemDark?.matches ?? true));
  const root = document.documentElement;
  root.dataset.theme = theme;
  root.dataset.palette = ui.palette;
  root.style.colorScheme = theme;
  document.querySelector('meta[name="theme-color"]')?.setAttribute('content', THEME_COLOR[theme]);
}

/** Bring the native title bar in line with the page; in `system` mode Windows then reports the real theme. */
async function syncWindow(choice: ThemeChoice) {
  if (!desktopAvailable) return;
  await setWindowTheme(choice);
  if (choice === 'system' && ui.theme === 'system') paint((await windowTheme()) ?? undefined);
}

/** Local theme and motion only. The lyrics window must not call the main-window theme API. */
export function loadLocalPreferences() {
  try {
    ui.reduceMotion = localStorage.getItem(KEY) === '1';
    ui.theme = parseTheme(localStorage.getItem(THEME_KEY));
    ui.palette = parsePalette(localStorage.getItem(PALETTE_KEY));
  } catch {
    /* storage unavailable: keep defaults */
  }
  paint();
}

export function loadPreferences() {
  loadLocalPreferences();
  syncWindow(ui.theme).catch(notifyError);
}

export function setTheme(choice: ThemeChoice) {
  ui.theme = choice;
  try {
    localStorage.setItem(THEME_KEY, choice);
  } catch {
    /* keep for this session */
  }
  paint();
  syncWindow(choice).catch(notifyError);
  // Rust applies the saved theme to the window before the page loads on the next start.
  if (desktopAvailable) desktopPreferences(null, null, choice).catch(notifyError);
}

export function setPalette(value: Palette) {
  ui.palette = parsePalette(value);
  try {
    localStorage.setItem(PALETTE_KEY, ui.palette);
  } catch {
    /* session only */
  }
  // Preserve the resolved Windows mode; palette changes never issue playback or native IPC.
  document.documentElement.dataset.palette = ui.palette;
}

/** Follow Windows light/dark changes while the user has chosen `system`. Returns a cleanup function. */
export function watchSystemTheme() {
  const onMedia = () => {
    if (ui.theme === 'system') paint();
  };
  systemDark?.addEventListener('change', onMedia);
  let unlisten: (() => void) | undefined;
  let disposed = false;
  if (desktopAvailable)
    onWindowThemeChanged((t) => {
      if (ui.theme === 'system') paint(t);
    })
      .then((u) => {
        if (disposed) u();
        else unlisten = u;
      })
      .catch(notifyError);
  return () => {
    disposed = true;
    systemDark?.removeEventListener('change', onMedia);
    unlisten?.();
  };
}

export function setReduceMotion(on: boolean) {
  ui.reduceMotion = on;
  try {
    localStorage.setItem(KEY, on ? '1' : '0');
  } catch {
    /* keep for this session */
  }
}

const systemReduced =
  typeof matchMedia === 'function' ? matchMedia('(prefers-reduced-motion: reduce)') : null;

/** Transition duration honouring both the in-app setting and the Windows animation preference. */
export function motionMs(ms: number) {
  return ui.reduceMotion || systemReduced?.matches ? 0 : ms;
}
