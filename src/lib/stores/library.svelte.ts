import {
  desktopAvailable,
  librarySummary,
  scanProgress,
  clearCoverCaches,
  type LibrarySummary,
  type ScanProgress,
} from '../services/music';
import { notifyError } from './ui.svelte';

export const library = $state({
  summary: { tracks: 0, albums: [], artists: [], folders: [], playlists: [] } as LibrarySummary,
  loaded: false,
  /** Bumped after any change to tracks or playlists so open lists (including the built-in “本地”) reload. */
  revision: 0,
  progress: null as ScanProgress | null,
  /** Bumped only when a scan finishes, so visible covers reload without a theme or progress refresh. */
  coverEpoch: 0,
});

export async function refreshSummary() {
  if (!desktopAvailable) return;
  try {
    library.summary = await librarySummary();
    library.loaded = true;
    library.revision++;
  } catch (e) {
    notifyError(e);
  }
}

/** Polls scan progress and refreshes the library when a scan finishes. */
export function watchScans() {
  if (!desktopAvailable) return () => {};
  let disposed = false;
  let timer: ReturnType<typeof setTimeout>;
  async function poll() {
    try {
      const next = await scanProgress();
      if (disposed) return;
      const was = library.progress?.running;
      library.progress = next;
      if (was && !next.running) {
        clearCoverCaches();
        library.coverEpoch++;
        await refreshSummary();
      }
    } catch (e) {
      if (!disposed) notifyError(e);
    }
    if (!disposed) timer = setTimeout(poll, library.progress?.running ? 500 : 2000);
  }
  void poll();
  return () => {
    disposed = true;
    clearTimeout(timer);
  };
}
