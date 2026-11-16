import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import ts from 'typescript';

// Compile the real TS module; stub only its environment/dependencies, never the functions under test.
function moduleFrom(path, dependencies) {
  const code = ts.transpileModule(readFileSync(path, 'utf8'), {
    compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
  }).outputText;
  const exports = {};
  new Function('require', 'exports', code)((name) => {
    assert.ok(name in dependencies, `unexpected dependency: ${name}`);
    return dependencies[name];
  }, exports);
  return exports;
}

function service() {
  const calls = [];
  const api = moduleFrom('src/lib/services/music.ts', {
    '@tauri-apps/api/core': {
      isTauri: () => true,
      invoke: async (cmd, args) => {
        calls.push({ cmd, args });
      },
    },
    '@tauri-apps/api/window': {},
    '../utils/cover-loader': { createCoverLoader: () => ({ stats: {} }) },
  });
  return { api, calls };
}

test('management IPC preserves camelCase IDs and zero-based playlist positions', async () => {
  const { api, calls } = service();
  await api.removeFolder('C:\\Fixture');
  await api.reorderPlaylist(8, 0, 2);
  assert.deepEqual(calls, [
    { cmd: 'library_remove_folder', args: { path: 'C:\\Fixture' } },
    { cmd: 'library_playlist_edit', args: { action: 'reorder', playlistId: 8, from: 0, to: 2 } },
  ]);
});

test('song menu offers reorder only with unfiltered playlist context and disables both boundaries', async () => {
  const { api } = service();
  const actions = moduleFrom('src/lib/actions.ts', {
    'lucide-svelte': {},
    './services/music': api,
    './stores/player.svelte': {},
    './stores/library.svelte': { library: { summary: { playlists: [] } } },
    './stores/nav.svelte': {},
    './stores/ui.svelte': {},
    './utils/playlist-name': {},
    './utils/collections': {},
  });
  const track = { id: 3, title: 'Track' };
  const moves = [];
  const menu = (playlistId, index, total, moving = false) =>
    actions
      .songMenu(track, {
        playlistId,
        onDetails() {},
        ...(index == null ? {} : { order: { index, total, moving, move: (to) => moves.push(to) } }),
      })
      .filter((item) => ['上移一位', '下移一位'].includes(item.label));
  assert.deepEqual(menu(null, 0, 3), []);
  assert.deepEqual(menu(8, null, 3), [], 'filtered/no order context');
  for (const [index, total, disabled] of [
    [0, 3, [true, false]],
    [1, 3, [false, false]],
    [2, 3, [false, true]],
    [0, 1, [true, true]],
    [0, 0, [true, true]],
  ]) {
    const items = menu(8, index, total);
    assert.deepEqual(
      items.map((item) => !!item.disabled),
      disabled,
    );
  }
  const middle = menu(8, 1, 3);
  await middle[0].action();
  await middle[1].action();
  assert.deepEqual(moves, [0, 2]);
  const first = menu(8, 0, 3);
  await first[0].action();
  const busy = menu(8, 1, 3, true);
  await busy[0].action();
  await busy[1].action();
  assert.deepEqual(
    busy.map((item) => item.disabled),
    [true, true],
  );
  assert.deepEqual(moves, [0, 2], 'disabled items do not dispatch moves');
});
