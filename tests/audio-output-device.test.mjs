import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import ts from 'typescript';

test('output device services preserve null/default and return authoritative snapshots', async () => {
  const calls = [];
  const snapshot = { revision: 17, output_device_id: 'Headphones' };
  const code = ts.transpileModule(readFileSync('src/lib/services/music.ts', 'utf8'), {
    compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 },
  }).outputText;
  const dependencies = {
    '@tauri-apps/api/core': {
      isTauri: () => true,
      invoke: async (cmd, args) => {
        calls.push({ cmd, args });
        return cmd === 'player_output_devices' ? ['Headphones'] : snapshot;
      },
    },
    '@tauri-apps/api/window': {},
    '@tauri-apps/api/event': {},
    '../utils/cover-loader': { createCoverLoader: () => ({ stats: {} }) },
  };
  const api = {};
  new Function('require', 'exports', code)((name) => {
    assert.ok(name in dependencies);
    return dependencies[name];
  }, api);
  assert.equal(typeof api.outputDevices, 'function');
  assert.deepEqual(await api.outputDevices(), ['Headphones']);
  assert.equal(await api.setOutputDevice('Headphones'), snapshot);
  assert.equal(await api.setOutputDevice(null), snapshot);
  assert.deepEqual(calls, [
    { cmd: 'player_output_devices', args: undefined },
    { cmd: 'player_set_output_device', args: { device: 'Headphones' } },
    { cmd: 'player_set_output_device', args: { device: null } },
  ]);
});
