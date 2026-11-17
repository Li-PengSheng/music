import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import ts from 'typescript';

function load() {
  const code = ts.transpileModule(readFileSync('src/lib/utils/ai-tools.ts', 'utf8'), {
    compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
  }).outputText;
  const api = {};
  new Function('require', 'exports', code)(() => ({}), api);
  return api;
}
function fixture() {
  let snapshot = { playback_state: 'paused', volume: 0.7, error: null };
  const tracks = [
    {
      id: 1,
      title: 'Song',
      artist: 'Artist',
      album: 'Album',
      duration_ms: 180000,
      path: 'C:/private/song.flac',
    },
  ];
  const playlists = [];
  const appearance = {};
  const d = {
    summary: async () => ({
      tracks: 1,
      folders: ['C:/private'],
      albums: Array.from({ length: 31 }, (_, i) => ({
        name: `Album ${i}`,
        key: 'private',
        count: 2,
      })),
      artists: [{ name: 'Artist', count: 1 }],
      playlists,
    }),
    query: async () => ({ tracks, total: 1 }),
    track: async (id) => {
      if (id !== 1) throw Error('C:/private/missing');
      return tracks[0];
    },
    createPlaylist: async (name) => {
      playlists.push({ id: 7, name, tracks: [] });
      return 7;
    },
    addToPlaylist: async (_, id) => {
      playlists[0].tracks.push(id);
    },
    refresh: async () => {
      appearance.refreshed = true;
    },
    queueTracks: async (ids, replace) => {
      assert.equal(replace, true);
      snapshot = { ...snapshot, playback_state: 'playing', queue: ids };
      return snapshot;
    },
    setMode: async (mode) => {
      snapshot = { ...snapshot, playback_mode: mode };
      return snapshot;
    },
    play: async () => ({ ...snapshot, playback_state: 'playing' }),
    pause: async () => ({ ...snapshot, playback_state: 'paused' }),
    queueAction: async (action) => ({ ...snapshot, action }),
    setVolume: async (volume) => ({ ...snapshot, volume }),
    runPlayer: async (action) => {
      snapshot = await action();
      return true;
    },
    getPlayer: () => snapshot,
    setTheme: (theme) => {
      appearance.theme = theme;
    },
    setPalette: (palette) => {
      appearance.palette = palette;
    },
    setFont: (font) => {
      appearance.font = font;
    },
    lyrics: async (open) => {
      appearance.lyrics = open;
      return open;
    },
  };
  return { d, playlists, appearance, getSnapshot: () => snapshot };
}

test('AI tools minimize metadata and never return local paths or unbounded lists', async () => {
  const { createAiToolExecutor } = load();
  const f = fixture();
  const execute = createAiToolExecutor(f.d);
  const overview = await execute('get_library_overview', {});
  assert.equal(overview.ok, true);
  assert.equal(overview.albums.length, 30);
  assert.equal(JSON.stringify(overview).includes('private'), false);
  const found = await execute('search_library', { keyword: 'Song', limit: 50, sort: 'title' });
  assert.equal(found.tracks[0].id, 1);
  assert.equal(JSON.stringify(found).includes('path'), false);
  assert.equal((await execute('search_library', { keyword: 'a', limit: 51 })).ok, false);
});
test('AI playlist creation validates IDs before writes, refreshes and reports partial failure', async () => {
  const { createAiToolExecutor } = load();
  const f = fixture();
  const execute = createAiToolExecutor(f.d);
  assert.equal(
    (await execute('create_playlist_with_tracks', { name: 'Mix', ids: [999] })).ok,
    false,
  );
  assert.equal(f.playlists.length, 0);
  const created = await execute('create_playlist_with_tracks', { name: 'Mix', ids: [1, 1] });
  assert.equal(created.ok, true);
  assert.deepEqual(f.playlists[0].tracks, [1]);
  assert.equal(f.appearance.refreshed, true);
  f.d.addToPlaylist = async () => {
    throw Error('C:/private/database');
  };
  const failed = await createAiToolExecutor(f.d)('create_playlist_with_tracks', {
    name: 'Partial',
    ids: [1],
  });
  assert.equal(failed.ok, false);
  assert.equal(failed.playlist_id, 7);
  assert.equal(failed.added, 0);
  assert.equal(JSON.stringify(failed).includes('private'), false);
});
test('AI playback accepts authoritative snapshots and validates all appearance fields before mutation', async () => {
  const { createAiToolExecutor } = load();
  const f = fixture();
  const execute = createAiToolExecutor(f.d);
  assert.equal((await execute('play_tracks', { ids: [1], mode: 'shuffle' })).ok, true);
  assert.equal(f.getSnapshot().playback_state, 'playing');
  assert.equal(f.getSnapshot().playback_mode, 'shuffle');
  assert.equal((await execute('control_playback', { action: 'volume', volume: 30 })).ok, true);
  assert.equal(f.getSnapshot().volume, 0.3);
  assert.equal((await execute('control_playback', { action: 'volume', volume: -1 })).ok, false);
  assert.equal((await execute('set_appearance', { theme: 'dark', palette: 'unknown' })).ok, false);
  assert.deepEqual(f.appearance, {});
  assert.equal(
    (
      await execute('set_appearance', {
        theme: 'dark',
        palette: 'amber',
        font: 'serif',
        desktop_lyrics: true,
      })
    ).ok,
    true,
  );
  assert.deepEqual(f.appearance, { theme: 'dark', palette: 'amber', font: 'serif', lyrics: true });
  f.d.runPlayer = async () => false;
  assert.equal((await createAiToolExecutor(f.d)('control_playback', { action: 'play' })).ok, false);
});
test('AI executor refuses unknown tools, non-object arguments and cancelled mutations', async () => {
  const { createAiToolExecutor } = load();
  const f = fixture();
  const abort = new AbortController();
  abort.abort();
  const execute = createAiToolExecutor(f.d);
  for (const input of [null, [], { ids: [-1] }, { ids: ['1'] }])
    assert.equal((await execute('play_tracks', input)).ok, false);
  assert.equal((await execute('run_shell', { command: 'erase' })).ok, false);
  assert.equal((await execute('play_tracks', { ids: [1], extra: true })).ok, false);
  assert.equal(
    (await execute('create_playlist_with_tracks', { name: 'No', ids: [1] }, abort.signal)).ok,
    false,
  );
  assert.equal(f.playlists.length, 0);
});
