import { test } from 'node:test';
import assert from 'node:assert/strict';
import {
  initialWorkbench,
  createWorkbench,
  filteredEntries,
  applyPayload,
  setCandidate,
  canApply,
  effectiveFilter,
} from '../src/lib/utils/enrichment-workbench.ts';

const track = (id) => ({
  id,
  path: `fixture-${id}.flac`,
  title: `Local ${id}`,
  artist: '未知艺术家',
  album: '未知专辑',
  album_artist: '',
  year: null,
  track_number: null,
});
const status = (id) => ({
  track_id: id,
  missing_artist: true,
  missing_album: true,
  missing_cover: true,
  missing_lyrics: true,
});
const candidates = (id) => ({
  title_query: `Query ${id}`,
  artist_query: 'Artist',
  tags: [
    {
      title: `Match ${id}`,
      artist: 'Artist',
      album: 'Album',
      album_artist: 'Artist',
      year: 2025,
      track_number: 1,
      release_mbid: 'id',
      cover_preview: 'data:image/png;base64,AA',
      cover_preview_url: 'https://coverartarchive.org/x',
    },
  ],
  lyrics: [{ id, preview: '[00:01.00]Line', synced_lyrics: '[00:01.00]Line' }],
  errors: [],
});
function fixture(overrides = {}) {
  const state = initialWorkbench();
  const calls = [];
  const api = {
    query: async ({ offset }) => ({
      tracks: offset ? [] : [track(1), track(2), track(3)],
      total: 3,
    }),
    statuses: async (ids) => ids.map(status),
    search: async (id) => {
      calls.push(['search', id]);
      return candidates(id);
    },
    apply: async (p) => {
      calls.push(['apply', p.track_id]);
      return { ...track(p.track_id), ...p.tags };
    },
    refresh: async () => {},
    wait: async () => {},
    ...overrides,
  };
  return { state, calls, work: createWorkbench(state, api) };
}
test('load is local only; filters and selection cover the current filtered set', async () => {
  const { state, calls, work } = fixture();
  await work.load();
  assert.equal(state.entries.length, 3);
  assert.deepEqual(calls, []);
  state.entries[0].missing.missing_cover = false;
  state.entries[1].missing.missing_cover = null;
  assert.deepEqual(
    filteredEntries(state, 'cover', '').map((e) => e.track.id),
    [3],
  );
  work.selectAll(filteredEntries(state, 'cover', ''), true);
  assert.deepEqual(
    state.entries.filter((e) => e.checked).map((e) => e.track.id),
    [3],
  );
  assert.deepEqual(
    filteredEntries(state, 'all', 'Local 2').map((e) => e.track.id),
    [2],
  );
});

test('recent and recent-incomplete filters respect seven days, complete metadata and ignored markers', async () => {
  const now = 1800000000,
    week = 7 * 86400;
  const f = fixture();
  await f.work.load();
  const [old, complete, recent] = f.state.entries;
  old.track.added_at = now - week - 1;
  complete.track.added_at = now - 1;
  complete.missing = {
    ...status(2),
    missing_artist: false,
    missing_album: false,
    missing_cover: false,
    missing_lyrics: false,
  };
  recent.track.added_at = now - week;
  const ids = (filter) => filteredEntries(f.state, filter, '', now).map((e) => e.track.id);
  assert.deepEqual(ids('recent'), [2, 3]);
  assert.deepEqual(ids('recent-info'), [3]);
  recent.missing.enrichment_ignored = true;
  assert.deepEqual(ids('recent'), [2]);
  assert.deepEqual(ids('info'), [1]);
  assert.deepEqual(ids('cover'), [1]);
  assert.deepEqual(ids('lyrics'), [1]);
  assert.deepEqual(ids('incomplete'), [1]);
  assert.deepEqual(ids('all'), [1, 2, 3]);
  complete.track.added_at = now + 10;
  assert.deepEqual(ids('recent'), []);
  assert.equal(effectiveFilter(f.state, 'recent', now), 'incomplete');
  old.track.added_at = now - 1;
  assert.equal(effectiveFilter(f.state, 'recent', now), 'recent');
  assert.equal(effectiveFilter(f.state, 'all', now), 'all');
});

test('ignore clears selection only after success and bulk work skips ignored tracks', async () => {
  const calls = [];
  let fail = false;
  const f = fixture({
    setIgnored: async (id, ignored) => {
      calls.push([id, ignored]);
      if (fail) throw 'METADATA_DATABASE';
    },
  });
  await f.work.load();
  f.work.selectAll(f.state.entries, true);
  const entry = f.state.entries[0];
  fail = true;
  await f.work.setIgnored(entry, true);
  assert.equal(entry.checked, true);
  assert.equal(entry.missing.enrichment_ignored ?? false, false);
  fail = false;
  await f.work.setIgnored(entry, true);
  assert.equal(entry.checked, false);
  assert.equal(entry.missing.enrichment_ignored, true);
  f.work.selectAll(f.state.entries, true);
  await f.work.batchSearch();
  assert.deepEqual(
    f.calls.filter((c) => c[0] === 'search'),
    [
      ['search', 2],
      ['search', 3],
    ],
  );
  await f.work.setIgnored(entry, false);
  assert.equal(entry.missing.enrichment_ignored, false);
  assert.deepEqual(calls, [
    [1, true],
    [1, true],
    [1, false],
  ]);
});

test('initial focus skips persisted ignored entries while all tracks can still select them', async () => {
  const f = fixture({
    statuses: async (ids) => ids.map((id) => ({ ...status(id), enrichment_ignored: id === 1 })),
  });
  await f.work.load();
  assert.equal(f.state.activeId, 2);
  f.state.filter = 'all';
  f.state.activeId = 1;
  await f.work.load();
  assert.equal(f.state.activeId, 1);
});
test('batch search is sequential and rate limited, preserves per-track previews, and never applies', async () => {
  let active = 0,
    max = 0;
  const order = [];
  const { state, work, calls } = fixture({
    search: async (id) => {
      active++;
      max = Math.max(max, active);
      order.push(id);
      await Promise.resolve();
      active--;
      return candidates(id);
    },
    wait: async (ms) => {
      assert.equal(ms, 1100);
      order.push('wait');
    },
  });
  await work.load();
  work.selectAll(state.entries, true);
  await work.batchSearch();
  assert.equal(max, 1);
  assert.deepEqual(order, [1, 'wait', 2, 'wait', 3]);
  assert.deepEqual(
    state.entries.map((e) => e.status),
    ['matched', 'matched', 'matched'],
  );
  assert.equal(state.entries[1].result.tags[0].title, 'Match 2');
  assert.deepEqual(calls, []);
});
test('cancel stops scheduling after the in-flight search; disposing discards stale results', async () => {
  let resolve;
  const pending = new Promise((r) => {
    resolve = r;
  });
  const { state, work, calls } = fixture({
    search: (id) => {
      calls.push(['search', id]);
      return pending;
    },
  });
  await work.load();
  work.selectAll(state.entries, true);
  const batch = work.batchSearch();
  assert.equal(state.entries[0].status, 'searching');
  work.cancel();
  resolve(candidates(1));
  await batch;
  assert.deepEqual(calls, [['search', 1]]);
  assert.equal(state.entries[0].status, 'matched');
  assert.equal(state.entries[1].status, 'idle');
  let release;
  const f = fixture({
    search: () =>
      new Promise((r) => {
        release = r;
      }),
  });
  await f.work.load();
  const search = f.work.searchCurrent(f.state.entries[0]);
  f.work.dispose();
  release(candidates(1));
  await search;
  assert.equal(f.state.entries[0].result, null);
});
test('only selected matched entries apply; failed writes keep local data and do not stop later tracks', async () => {
  const f = fixture({
    apply: async (p) => {
      f.calls.push(['apply', p.track_id]);
      if (p.track_id === 1) throw 'METADATA_FILE_EXISTS';
      return { ...track(p.track_id), ...p.tags };
    },
  });
  await f.work.load();
  f.work.selectAll(f.state.entries, true);
  await f.work.batchSearch();
  f.state.entries[2].checked = false;
  const payload = applyPayload(f.state.entries[0]);
  assert.equal(payload.write_audio_tags, false);
  assert.equal(payload.overwrite_cover, false);
  assert.equal(payload.dedicated_cover, true);
  await f.work.batchApply();
  assert.equal(f.state.entries[0].track.title, 'Local 1');
  assert.equal(f.state.entries[0].error, 'METADATA_FILE_EXISTS');
  assert.equal(f.state.entries[1].status, 'applied');
  assert.equal(f.state.entries[1].track.title, 'Match 2');
  assert.equal(f.state.entries[2].status, 'matched');
  assert.deepEqual(
    f.calls.filter((c) => c[0] === 'apply'),
    [
      ['apply', 1],
      ['apply', 2],
    ],
  );
});
test('existing outputs default to preserved and bulk apply uses a frozen payload', async () => {
  let release;
  const f = fixture({
    apply: (p) =>
      new Promise((r) => {
        release = () => r({ ...track(p.track_id), ...p.tags });
      }),
  });
  await f.work.load();
  const entry = f.state.entries[0];
  entry.missing.missing_cover = false;
  entry.missing.missing_lyrics = false;
  await f.work.searchCurrent(entry);
  assert.equal(entry.saveCover, false);
  assert.equal(entry.saveLyrics, false);
  entry.checked = true;
  const apply = f.work.batchApply();
  entry.result.tags[0].title = 'Changed while pending';
  release();
  await apply;
  assert.equal(entry.track.title, 'Match 1');
});

test('cancelling an in-flight apply preserves its completed write and prevents the next write', async () => {
  let release;
  const f = fixture({
    apply: (p) => {
      f.calls.push(['apply', p.track_id]);
      return new Promise((resolve) => {
        release = () => resolve({ ...track(p.track_id), ...p.tags });
      });
    },
  });
  await f.work.load();
  f.work.selectAll(f.state.entries, true);
  await f.work.batchSearch();
  const job = f.work.batchApply();
  f.work.cancel();
  release();
  await job;
  assert.equal(f.state.entries[0].status, 'applied');
  assert.equal(f.state.entries[1].status, 'matched');
  assert.deepEqual(
    f.calls.filter((c) => c[0] === 'apply'),
    [['apply', 1]],
  );
});

test('a committed apply remains applied if shared-state refresh fails', async () => {
  const f = fixture({
    refresh: async () => {
      throw 'fixture refresh failure';
    },
  });
  await f.work.load();
  const entry = f.state.entries[0];
  await f.work.searchCurrent(entry);
  await f.work.applyCurrent(entry);
  assert.equal(entry.status, 'applied');
  assert.equal(entry.track.title, 'Match 1');
  assert.match(entry.error, /已应用/);
});

test('load pages through a library larger than one IPC batch and checks each local status once', async () => {
  const pages = [],
    statuses = [];
  const f = fixture({
    query: async ({ offset, limit }) => {
      pages.push(offset);
      return {
        tracks: Array.from({ length: Math.min(limit, 205 - offset) }, (_, i) =>
          track(offset + i + 1),
        ),
        total: 205,
      };
    },
    statuses: async (ids) => {
      statuses.push(ids.length);
      return ids.map(status);
    },
  });
  await f.work.load(205);
  assert.deepEqual(pages, [0, 200]);
  assert.deepEqual(statuses, [200, 5]);
  assert.equal(f.state.entries.length, 205);
  assert.equal(f.state.activeId, 205);
  assert.deepEqual(f.calls, []);
});

test('manual target draft applies with no candidates, stays separate from the local track, and survives focus', async () => {
  const f = fixture({
    search: async () => ({ tags: [], lyrics: [], errors: [], title_query: 'q', artist_query: '' }),
  });
  await f.work.load();
  const entry = f.state.entries[0];
  await f.work.searchCurrent(entry);
  assert.equal(canApply(entry), false);
  entry.draft.title = '手填歌曲';
  entry.draft.artist = '手填艺术家';
  entry.draft.album = '手填专辑';
  entry.draft.year = 2024;
  entry.draftDirty = true;
  assert.equal(canApply(entry), true);
  entry.draft.year = 2024.5;
  assert.equal(canApply(entry), false);
  entry.draft.year = 2024;
  assert.equal(entry.track.title, 'Local 1');
  await f.work.applyCurrent(entry);
  assert.equal(entry.status, 'applied');
  assert.equal(entry.track.title, '手填歌曲');
  assert.equal(entry.track.year, 2024);
});

test('editing a candidate target leaves original data intact and choosing another replaces only the draft', async () => {
  const f = fixture();
  await f.work.load();
  const entry = f.state.entries[0];
  await f.work.searchCurrent(entry);
  entry.result.tags.push({
    ...entry.result.tags[0],
    title: '另一候选',
    source: 'iTunes',
    cover_url: 'https://is1-ssl.mzstatic.com/image/thumb/a/600x600bb.jpg',
  });
  entry.draft.title = '简体目标';
  entry.draftDirty = true;
  assert.equal(entry.result.tags[0].title, 'Match 1');
  assert.equal(applyPayload(entry).tags.title, '简体目标');
  setCandidate(entry, '1');
  assert.equal(entry.draft.title, '另一候选');
  assert.equal(
    applyPayload(entry).cover_url,
    'https://is1-ssl.mzstatic.com/image/thumb/a/600x600bb.jpg',
  );
});

test('incremental library sync retains matching entries and drafts, adds tracks, and drops removed IDs', async () => {
  let tracks = [track(1), track(2)];
  const f = fixture({ query: async () => ({ tracks, total: tracks.length }) });
  await f.work.load();
  const kept = f.state.entries[0];
  await f.work.searchCurrent(kept);
  kept.checked = true;
  kept.draft.title = 'Keep draft';
  kept.draftDirty = true;
  f.state.activeId = 1;
  tracks = [{ ...track(1), album: 'Updated local album' }, track(3)];
  await f.work.load();
  assert.equal(f.state.entries[0], kept);
  assert.equal(kept.draft.title, 'Keep draft');
  assert.equal(kept.checked, true);
  assert.equal(kept.track.album, 'Updated local album');
  assert.deepEqual(
    f.state.entries.map((e) => e.track.id),
    [1, 3],
  );
  assert.equal(f.state.activeId, 1);
  assert.equal(kept.result.tags[0].title, 'Match 1');
});

test('ordinary focus search reuses results; explicit refresh and changed queries invoke the requested mode', async () => {
  const args = [];
  const f = fixture({
    search: async (...a) => {
      args.push(a);
      return candidates(a[0]);
    },
  });
  await f.work.load();
  const entry = f.state.entries[0];
  await f.work.searchCurrent(entry);
  entry.draft.title = 'Keep manual target';
  entry.draftDirty = true;
  await f.work.searchCurrent(entry);
  assert.equal(args.length, 1);
  assert.equal(entry.draft.title, 'Keep manual target');
  await f.work.searchCurrent(entry, true);
  assert.equal(args.length, 2);
  assert.equal(args[1][3], true);
  entry.titleQuery = 'Changed keywords';
  await f.work.searchCurrent(entry);
  assert.equal(args.length, 3);
  assert.equal(args[2][3], false);
  assert.equal(args[2][1], 'Changed keywords');
});

test('failed forced refresh keeps the previous candidates, target draft and output choices', async () => {
  let attempts = 0;
  const f = fixture({
    search: async () =>
      ++attempts === 1
        ? candidates(1)
        : {
            tags: [],
            lyrics: [],
            errors: ['METADATA_MB_NETWORK'],
            title_query: 'q',
            artist_query: '',
          },
  });
  await f.work.load();
  const entry = f.state.entries[0];
  await f.work.searchCurrent(entry);
  const previous = entry.result;
  entry.draft.title = 'Keep manual target';
  entry.draftDirty = true;
  entry.saveLyrics = false;
  await f.work.searchCurrent(entry, true);
  assert.equal(entry.result, previous);
  assert.equal(entry.draft.title, 'Keep manual target');
  assert.equal(entry.saveLyrics, false);
  assert.equal(entry.status, 'error');
  assert.equal(entry.error, 'METADATA_MB_NETWORK');
});
