import type {
  LibraryTrack,
  MetadataStatus,
  EnrichmentCandidates,
  ApplyEnrichmentRequest,
  LibraryQuery,
  LibraryPage,
  TagCandidate,
} from '../services/music';

export type EntryStatus = 'idle' | 'searching' | 'matched' | 'empty' | 'applied' | 'error';
export type MissingFilter =
  'recent' | 'recent-info' | 'incomplete' | 'all' | 'info' | 'cover' | 'lyrics';
export interface EnrichmentEntry {
  track: LibraryTrack;
  missing: MetadataStatus;
  checked: boolean;
  status: EntryStatus;
  titleQuery: string;
  artistQuery: string;
  result: EnrichmentCandidates | null;
  tagIndex: string;
  lyricIndex: string;
  updateTags: boolean;
  saveCover: boolean;
  saveLyrics: boolean;
  writeAudio: boolean;
  overwriteCover: boolean;
  overwriteLyrics: boolean;
  error: string;
  draft: TagCandidate;
  draftDirty: boolean;
  queryKey: string | null;
}
export function initialWorkbench() {
  return {
    entries: [] as EnrichmentEntry[],
    activeId: null as number | null,
    filter: 'recent' as MissingFilter,
    now: Math.floor(Date.now() / 1000),
    loading: false,
    loaded: false,
    error: '',
    busy: null as 'search' | 'apply' | 'ignore' | null,
    cancelled: false,
    progress: { done: 0, total: 0, label: '' },
  };
}
export type WorkbenchState = ReturnType<typeof initialWorkbench>;
interface WorkbenchApi {
  query: (query: LibraryQuery) => Promise<LibraryPage>;
  statuses: (ids: number[]) => Promise<MetadataStatus[]>;
  search: (
    id: number,
    title: string | null,
    artist: string | null,
    forceRefresh: boolean,
  ) => Promise<EnrichmentCandidates>;
  apply: (payload: ApplyEnrichmentRequest) => Promise<LibraryTrack>;
  setIgnored: (id: number, ignored: boolean) => Promise<void>;
  refresh: (track: LibraryTrack) => Promise<void>;
  wait: (ms: number, signal: AbortSignal) => Promise<void>;
}
function entry(track: LibraryTrack, missing?: MetadataStatus): EnrichmentEntry {
  return {
    track,
    missing: missing ?? {
      track_id: track.id,
      enrichment_ignored: false,
      missing_artist: !track.artist || track.artist === '未知艺术家',
      missing_album: !track.album || track.album === '未知专辑',
      missing_cover: null,
      missing_lyrics: null,
    },
    checked: false,
    status: 'idle',
    titleQuery: track.title,
    artistQuery: track.artist === '未知艺术家' ? '' : track.artist,
    result: null,
    tagIndex: '0',
    lyricIndex: '0',
    updateTags: true,
    saveCover: false,
    saveLyrics: false,
    writeAudio: false,
    overwriteCover: false,
    overwriteLyrics: false,
    error: '',
    draft: {
      source: 'Manual',
      title: track.title,
      artist: track.artist === '未知艺术家' ? '' : track.artist,
      album: track.album === '未知专辑' ? '' : track.album,
      album_artist: track.album_artist,
      year: track.year,
      track_number: track.track_number,
      release_mbid: null,
      cover_preview_url: null,
      cover_preview: null,
      cover_url: null,
      genre: null,
    },
    draftDirty: false,
    queryKey: null,
  };
}
export function isNewTrack(track: LibraryTrack, now = Math.floor(Date.now() / 1000)) {
  return track.added_at > 0 && track.added_at <= now && track.added_at >= now - 7 * 86400;
}
function incomplete(e: EnrichmentEntry) {
  return (
    e.missing.missing_artist ||
    e.missing.missing_album ||
    e.missing.missing_cover === true ||
    e.missing.missing_lyrics === true
  );
}
export function effectiveFilter(
  state: WorkbenchState,
  filter: MissingFilter,
  now = state.now,
): MissingFilter {
  return filter === 'recent' &&
    !state.entries.some((e) => !e.missing.enrichment_ignored && isNewTrack(e.track, now))
    ? 'incomplete'
    : filter;
}
export function filteredEntries(
  state: WorkbenchState,
  filter: MissingFilter,
  query: string,
  now = state.now,
) {
  const term = query.trim().toLocaleLowerCase();
  return state.entries.filter(
    (e) =>
      (!term ||
        [e.track.title, e.track.artist, e.track.album].some((s) =>
          s.toLocaleLowerCase().includes(term),
        )) &&
      (filter === 'all' ||
        (!e.missing.enrichment_ignored &&
          ((filter === 'recent' && isNewTrack(e.track, now)) ||
            (filter === 'recent-info' && isNewTrack(e.track, now) && incomplete(e)) ||
            (filter === 'incomplete' && incomplete(e)) ||
            (filter === 'info' && (e.missing.missing_artist || e.missing.missing_album)) ||
            (filter === 'cover' && e.missing.missing_cover === true) ||
            (filter === 'lyrics' && e.missing.missing_lyrics === true)))),
  );
}
export function applyPayload(e: EnrichmentEntry): ApplyEnrichmentRequest {
  const tag = e.result?.tags[Number(e.tagIndex)] ?? null;
  const target = tag || e.draftDirty ? e.draft : null;
  const lyric = e.result?.lyrics[Number(e.lyricIndex)] ?? null;
  return {
    track_id: e.track.id,
    tags: target
      ? { ...target, album_artist: target.album_artist || target.artist, cover_preview: null }
      : null,
    update_library_tags: !!target && e.updateTags,
    synced_lyrics: lyric && e.saveLyrics ? lyric.synced_lyrics : null,
    save_lyrics: !!lyric && e.saveLyrics,
    save_cover: !!(tag?.release_mbid || tag?.cover_url) && e.saveCover,
    release_mbid: tag?.release_mbid ?? null,
    write_audio_tags: e.writeAudio,
    overwrite_cover: e.overwriteCover,
    overwrite_lyrics: e.overwriteLyrics,
    dedicated_cover: true,
    cover_url: tag?.cover_url ?? null,
  };
}
export function canApply(e: EnrichmentEntry) {
  if (
    e.updateTags &&
    (e.draftDirty || e.result?.tags.length) &&
    (!e.draft.title.trim() ||
      (e.draft.year !== null &&
        (!Number.isInteger(e.draft.year) || e.draft.year < 0 || e.draft.year > 9999)))
  )
    return false;
  const p = applyPayload(e);
  return p.update_library_tags || p.save_lyrics || p.save_cover;
}
export function setCandidate(e: EnrichmentEntry, index: string) {
  e.tagIndex = index;
  const tag = e.result?.tags[Number(index)];
  if (!tag) return;
  e.draft = { ...tag };
  e.draftDirty = false;
  e.saveCover = !!tag.cover_preview && e.missing.missing_cover === true;
}
const message = (error: unknown) => (error instanceof Error ? error.message : String(error));
const entryQueryKey = (e: EnrichmentEntry) =>
  JSON.stringify(
    [e.titleQuery, e.artistQuery].map((s) => s.trim().toLocaleLowerCase().replace(/\s+/g, ' ')),
  );
function hasCachedResult(e: EnrichmentEntry) {
  return !!e.result && e.queryKey === entryQueryKey(e) && e.status !== 'error';
}

/** All mutations target the supplied state (a Svelte proxy in the view, plain state in tests). */
export function createWorkbench(state: WorkbenchState, api: WorkbenchApi) {
  let disposed = false;
  let epoch = 0;
  let abort = new AbortController();
  async function load(targetId?: number) {
    if (state.busy || state.loading || disposed) return false;
    const ticket = ++epoch;
    state.loading = true;
    state.error = '';
    const previous = new Map(state.entries.map((entry) => [entry.track.id, entry]));
    const entries: EnrichmentEntry[] = [];
    try {
      let offset = 0;
      while (!disposed && epoch === ticket) {
        const page = await api.query({ offset, limit: 200, sort: 'added' });
        if (disposed || epoch !== ticket) return false;
        const statuses = await api.statuses(page.tracks.map((t) => t.id));
        if (disposed || epoch !== ticket) return false;
        const byId = new Map(statuses.map((s) => [s.track_id, s]));
        for (const track of page.tracks) {
          const kept = previous.get(track.id);
          if (kept && kept.track.path === track.path) {
            kept.track = track;
            if (byId.has(track.id)) kept.missing = byId.get(track.id)!;
            if (kept.missing.enrichment_ignored) kept.checked = false;
            entries.push(kept);
          } else entries.push(entry(track, byId.get(track.id)));
        }
        offset += page.tracks.length;
        if (!page.tracks.length || offset >= page.total) break;
      }
      state.entries = entries;
      state.now = Math.floor(Date.now() / 1000);
      const focus = targetId ?? state.activeId;
      state.activeId = entries.some(
        (e) =>
          e.track.id === focus &&
          (!e.missing.enrichment_ignored || state.filter === 'all' || targetId !== undefined),
      )
        ? focus!
        : (filteredEntries(state, effectiveFilter(state, state.filter), '')[0]?.track.id ?? null);
      if (targetId && state.activeId !== targetId) state.error = '目标曲目已不在曲库中';
      state.loaded = true;
      return true;
    } catch (error) {
      if (!disposed && epoch === ticket) state.error = message(error);
    } finally {
      if (!disposed && epoch === ticket) state.loading = false;
    }
    return false;
  }
  async function search(e: EnrichmentEntry, valid: () => boolean, forceRefresh: boolean) {
    e.status = 'searching';
    e.error = '';
    try {
      const result = await api.search(
        e.track.id,
        e.titleQuery.trim() || null,
        e.artistQuery.trim(),
        forceRefresh,
      );
      if (!valid()) return;
      if (!result.tags.length && !result.lyrics.length && result.errors.length) {
        e.status = 'error';
        e.error = result.errors.join(' · ');
        return;
      }
      e.result = result;
      e.titleQuery = result.title_query;
      e.artistQuery = result.artist_query;
      e.queryKey = entryQueryKey(e);
      e.tagIndex = '0';
      setCandidate(e, '0');
      e.lyricIndex = '0';
      e.saveCover = !!result.tags[0]?.cover_preview && e.missing.missing_cover === true;
      e.saveLyrics = result.lyrics.length > 0 && e.missing.missing_lyrics === true;
      e.writeAudio = false;
      e.overwriteCover = false;
      e.overwriteLyrics = false;
      e.status = result.tags.length || result.lyrics.length ? 'matched' : 'empty';
    } catch (error) {
      if (valid()) {
        e.status = 'error';
        e.error = message(error);
      }
    }
  }
  async function apply(e: EnrichmentEntry, payload: ApplyEnrichmentRequest, valid: () => boolean) {
    e.error = '';
    try {
      const updated = await api.apply(payload);
      // A committed mutation must refresh global library/player state even after navigation.
      let refreshError = '';
      try {
        await api.refresh(updated);
      } catch {
        refreshError = '已应用；曲库视图暂时无法刷新，请刷新曲目';
      }
      if (!valid()) return;
      e.track = updated;
      e.status = 'applied';
      e.error = refreshError;
      try {
        const statuses = await api.statuses([updated.id]);
        if (valid() && statuses[0]) e.missing = statuses[0];
      } catch {
        if (valid()) e.error = '已应用；本地完整度暂时无法刷新';
      }
    } catch (error) {
      if (valid()) e.error = message(error);
    }
  }
  async function job(kind: 'search' | 'apply', entries: EnrichmentEntry[], forceRefresh = false) {
    if (disposed || state.busy || state.loading || !entries.length) return;
    const ticket = ++epoch;
    const valid = () => !disposed && epoch === ticket;
    abort = new AbortController();
    state.busy = kind;
    state.cancelled = false;
    state.progress = { done: 0, total: entries.length, label: kind === 'search' ? '检索' : '应用' };
    const jobs = entries.map((e) => ({ e, payload: kind === 'apply' ? applyPayload(e) : null }));
    try {
      for (let i = 0; i < jobs.length; i++) {
        if (!valid() || state.cancelled) break;
        if (i) {
          try {
            await api.wait(1100, abort.signal);
          } catch {
            break;
          }
        }
        if (!valid() || state.cancelled) break;
        const { e, payload } = jobs[i];
        if (kind === 'search') await search(e, valid, forceRefresh);
        else await apply(e, payload!, valid);
        if (valid()) state.progress.done++;
      }
    } finally {
      if (valid()) state.busy = null;
    }
  }
  return {
    load,
    async setIgnored(e: EnrichmentEntry, ignored: boolean) {
      if (disposed || state.busy || state.loading) return;
      state.busy = 'ignore';
      e.error = '';
      try {
        await api.setIgnored(e.track.id, ignored);
        e.missing.enrichment_ignored = ignored;
        if (ignored) {
          e.checked = false;
          if (state.activeId === e.track.id && state.filter !== 'all') {
            state.activeId =
              filteredEntries(state, effectiveFilter(state, state.filter), '')[0]?.track.id ?? null;
          }
        }
      } catch (error) {
        e.error = message(error);
      } finally {
        state.busy = null;
      }
    },
    selectAll(entries: EnrichmentEntry[], checked: boolean) {
      if (!state.busy)
        for (const e of entries) e.checked = checked && !e.missing.enrichment_ignored;
    },
    searchCurrent(e: EnrichmentEntry, forceRefresh = false) {
      if (!forceRefresh && hasCachedResult(e)) return Promise.resolve();
      return job('search', [e], forceRefresh);
    },
    applyCurrent(e: EnrichmentEntry) {
      return canApply(e) ? job('apply', [e]) : Promise.resolve();
    },
    batchSearch() {
      return job(
        'search',
        state.entries.filter(
          (e) => e.checked && !e.missing.enrichment_ignored && !hasCachedResult(e),
        ),
      );
    },
    batchApply() {
      return job(
        'apply',
        state.entries.filter(
          (e) =>
            e.checked && !e.missing.enrichment_ignored && e.status === 'matched' && canApply(e),
        ),
      );
    },
    cancel() {
      state.cancelled = true;
      abort.abort();
    },
    dispose() {
      disposed = true;
      ++epoch;
      abort.abort();
    },
  };
}
