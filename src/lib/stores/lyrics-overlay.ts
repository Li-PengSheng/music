import { desktopAvailable, desktopLyricsWindow, type LyricsOverlayState } from '../services/music';
import { notifyError, ui } from './ui.svelte';

export type { LyricsOverlayState };

let request = 0;

/** Explicit state for an AI command, returning Rust's result instead of toggling stale UI state. */
export async function setDesktopLyricsOpen(open: boolean) {
  if (!desktopAvailable) throw new Error('desktop_unavailable');
  const id = ++request;
  const actual = await desktopLyricsWindow(open, null);
  // Rust publishes its own state event before the command resolves. Accept that
  // matching event, while preserving a newer conflicting state from the tray/UI.
  if (id !== request && ui.lyricsOverlayOpen !== actual)
    throw new Error('lyrics_request_superseded');
  if (id === request) ui.lyricsOverlayOpen = actual;
  if (actual !== open) throw new Error('lyrics_state_mismatch');
  return actual;
}

/** Rust is the authority when a window, the tray, or another call changes the overlay. */
export function applyLyricsOverlay(state: LyricsOverlayState) {
  request += 1;
  ui.lyricsOverlayOpen = state.open;
  ui.lyricsOverlayLocked = state.locked;
}

export async function syncLyricsOverlayOpen() {
  if (!desktopAvailable) return;
  const id = ++request;
  try {
    const open = await desktopLyricsWindow();
    if (id === request) ui.lyricsOverlayOpen = open;
  } catch (error) {
    if (id === request) notifyError(error);
  }
}

export async function toggleDesktopLyrics() {
  if (!desktopAvailable) {
    notifyError('桌面歌词需要在桌面应用中使用');
    return;
  }
  const id = ++request;
  try {
    if (ui.lyricsOverlayLocked) {
      const open = await desktopLyricsWindow(null, false);
      if (id !== request) return;
      ui.lyricsOverlayLocked = false;
      ui.lyricsOverlayOpen = open;
      return;
    }
    const open = await desktopLyricsWindow(!ui.lyricsOverlayOpen, null);
    if (id === request) ui.lyricsOverlayOpen = open;
  } catch (error) {
    if (id === request) notifyError(error);
  }
}

export async function lockDesktopLyrics() {
  if (!desktopAvailable) return;
  const id = ++request;
  try {
    const open = await desktopLyricsWindow(null, true);
    if (id !== request) return;
    ui.lyricsOverlayLocked = true;
    ui.lyricsOverlayOpen = open;
  } catch (error) {
    if (id === request) notifyError(error);
  }
}

export async function hideDesktopLyrics() {
  if (!desktopAvailable) return;
  const id = ++request;
  try {
    const open = await desktopLyricsWindow(false, null);
    if (id === request) ui.lyricsOverlayOpen = open;
  } catch (error) {
    if (id === request) notifyError(error);
  }
}
