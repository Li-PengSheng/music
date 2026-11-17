import {
  queryLibrary,
  metadataTrackStatuses,
  searchMetadataCandidates,
  applyMetadataEnrichment,
  setEnrichmentIgnored,
  clearCoverCaches,
  getPlayerState,
  type LibraryTrack,
} from '../services/music';
import { library, refreshSummary } from './library.svelte';
import { player, accept } from './player.svelte';
import {
  initialWorkbench,
  createWorkbench,
  type WorkbenchState,
  type MissingFilter,
} from '../utils/enrichment-workbench';

interface EnrichmentSession extends WorkbenchState {
  filter: MissingFilter;
  syncedRevision: number;
  syncing: boolean;
  readonly tracks: LibraryTrack[];
  readonly selectedIds: ReadonlySet<number>;
}
/** Current WebView session only. Candidate payloads and paths never enter localStorage. */
export const enrichment: EnrichmentSession = $state({
  ...initialWorkbench(),
  syncedRevision: -1,
  syncing: false,
  get tracks() {
    return enrichment.entries.map((entry) => entry.track);
  },
  get selectedIds() {
    return new Set(
      enrichment.entries.filter((entry) => entry.checked).map((entry) => entry.track.id),
    );
  },
});
export const enrichmentWorkbench = createWorkbench(enrichment, {
  query: queryLibrary,
  statuses: metadataTrackStatuses,
  search: searchMetadataCandidates,
  apply: applyMetadataEnrichment,
  setIgnored: setEnrichmentIgnored,
  refresh: async (track) => {
    clearCoverCaches();
    library.coverEpoch++;
    library.revision++;
    await refreshSummary();
    if (player.snapshot?.track_id === track.path) {
      try {
        accept(await getPlayerState());
      } catch {
        /* Polling retries independently. */
      }
    }
  },
  wait: (ms, signal) =>
    new Promise<void>((resolve, reject) => {
      const cancel = () => {
        clearTimeout(timer);
        reject(new Error('cancelled'));
      };
      const timer = setTimeout(() => {
        signal.removeEventListener('abort', cancel);
        resolve();
      }, ms);
      signal.addEventListener('abort', cancel, { once: true });
      if (signal.aborted) cancel();
    }),
});
let pending: Promise<void> | null = null;
let attemptedRevision = -1;
export function synchronizeEnrichment(force = false): Promise<void> {
  if (pending) return pending;
  const revision = library.revision;
  if (enrichment.busy || (!force && attemptedRevision === revision)) return Promise.resolve();
  attemptedRevision = revision;
  enrichment.syncing = true;
  pending = enrichmentWorkbench
    .load()
    .then((loaded) => {
      if (loaded) enrichment.syncedRevision = revision;
    })
    .finally(() => {
      pending = null;
      enrichment.syncing = false;
    });
  return pending;
}
